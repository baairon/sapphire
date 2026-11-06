"""Check that fx_run renders the 13 effect styles the same way on this machine as on the reference build.

Usage:
  python -I tools/fx_golden.py              render every case and compare with tools/fx_golden.tsv (exit 1 on any difference)
  python -I tools/fx_golden.py --write      render every case and rewrite tools/fx_golden.tsv
  python -I tools/fx_golden.py --bin DIR    use the sapphire and fx_run executables in DIR

Besides the plain cases the table has two more groups. The special group (size suffix `s`) renders every stage on a clip that
holds +inf, -inf, both default NaN patterns, huge and subnormal values and neighbouring words that make invalid operations
(inf - inf, 0 * inf, 0 / 0); it is where the default NaN of an ARM processor would show. The threaded group (suffix `t`)
renders the plain sizes with SAPPHIRE_PAR_MIN=0 and SAPPHIRE_THREADS=3, so row bands run at every size; it must equal the plain rows.

Each case runs one effect stage (`sapphire stage apply STAGE`, which runs fx_run with the stage's canonical
host options from crates/neutral-handoff/data/fx_stages.tsv) on a synthetic clip of 3 frames at depth 32 and
records the SHA-256 of the output, or the exit status (`rcN`) of a case the stage refuses. The clip comes from a fixed seed and a 64-bit linear congruential generator,
so it is the same bytes on every platform. The table holds hashes of this repository's own renders (made by the
Linux build) and nothing else. A difference names the stage and the size. Standard library only.
"""

import argparse
import hashlib
import os
import struct
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
TABLE = os.path.join(ROOT, "tools", "fx_golden.tsv")
STAGES = os.path.join(ROOT, "crates", "neutral-handoff", "data", "fx_stages.tsv")
SIZES = [(64, 32), (50, 7)]
SPECIAL_SIZE = (40, 12)
FRAMES = 3
SEED = 20261106
THREADED = {"SAPPHIRE_PAR_MIN": "0", "SAPPHIRE_THREADS": "3"}
EXE = ".exe" if os.name == "nt" else ""


def synth_clip(width, height, frames, seed):
    """gbrapf32le frames: every sample in [0.05, 1.05), from the LCG 6364136223846793005 / 1442695040888963407."""
    state = seed & 0xFFFFFFFFFFFFFFFF
    out = bytearray()
    for _ in range(frames * 4 * width * height):
        state = (state * 6364136223846793005 + 1442695040888963407) & 0xFFFFFFFFFFFFFFFF
        out += struct.pack("<f", 0.05 + (state >> 40) / float(1 << 24))
    return bytes(out)


SPECIAL_WORDS = [
    0x7F800000,  # +inf
    0xFF800000,  # -inf
    0xFFC00000,  # the x86 default NaN
    0x7FC00000,  # the ARM default NaN
    0x7F7FFFFF,  # largest finite
    0xFF7FFFFF,  # most negative finite
    0x00000001,  # smallest subnormal
    0x807FFFFF,  # largest negative subnormal
    0x00000000,  # +0
    0x80000000,  # -0
    0x7F000000,  # huge
    0x7FA00000,  # a signalling NaN
]


def synth_special(width, height, frames, seed):
    """The plain clip with special words laid over it: every word whose LCG draw is a multiple of 5 takes a special word,
    and every 7th word (counted from the start of the plane) is set so that neighbours are +inf, -inf, 0 and +inf in turn,
    which makes inf - inf and 0 * inf in the blurs and the arithmetic. Deterministic, platform independent."""
    state = (seed ^ 0x5DEECE66D) & 0xFFFFFFFFFFFFFFFF
    out = bytearray()
    pattern = [0x7F800000, 0xFF800000, 0x00000000, 0x7F800000, 0xFF800000, 0x80000000]
    for i in range(frames * 4 * width * height):
        state = (state * 6364136223846793005 + 1442695040888963407) & 0xFFFFFFFFFFFFFFFF
        draw = state >> 33
        if draw % 5 == 0:
            word = SPECIAL_WORDS[(draw // 5) % len(SPECIAL_WORDS)]
        elif i % 7 < len(pattern) and (i // 7) % 3 == 0:
            word = pattern[i % 7]
        else:
            word = struct.unpack("<I", struct.pack("<f", 0.05 + (state >> 40) / float(1 << 24)))[0]
        out += struct.pack("<I", word)
    return bytes(out)


def stage_ids():
    with open(STAGES, encoding="utf-8") as f:
        rows = [line.rstrip("\n").split("\t") for line in f if line.strip()]
    return [r[0] for r in rows[1:]]


def find_bin(given):
    dirs = [given] if given else []
    if not given:
        target = os.environ.get("CARGO_TARGET_DIR") or os.path.join(ROOT, "target")
        dirs.append(os.path.join(target, "release"))
    for d in dirs:
        if os.path.isfile(os.path.join(d, "sapphire" + EXE)) and os.path.isfile(os.path.join(d, "fx_run" + EXE)):
            return d
    sys.exit("fx_golden: no sapphire and fx_run in " + ", ".join(dirs) + " (cargo build --release -p neutral-handoff -p fx_exec)")


def render(bindir, stage, width, height, clip, extra_env=None):
    cmd = [os.path.join(bindir, "sapphire" + EXE), "stage", "apply", stage, "--width", str(width), "--height", str(height),
           "--depth", "32", "--fps", "24", "--frames", str(FRAMES)]
    env = dict(os.environ)
    env["SAPPHIRE_FX_RUN"] = os.path.join(bindir, "fx_run" + EXE)
    env.update(extra_env or {})
    r = subprocess.run(cmd, input=clip, capture_output=True, env=env)
    if r.returncode != 0:
        return "rc{}".format(r.returncode), r.stderr.decode("utf-8", "replace").strip().splitlines()[-1:]
    return hashlib.sha256(r.stdout).hexdigest(), []


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--write", action="store_true")
    ap.add_argument("--bin")
    a = ap.parse_args(argv)
    bindir = find_bin(a.bin)
    got = {}
    for (w, h) in SIZES:
        clip = synth_clip(w, h, FRAMES, SEED)
        for s in stage_ids():
            got[(s, "{}x{}".format(w, h))] = render(bindir, s, w, h, clip)
            got[(s, "{}x{}t".format(w, h))] = render(bindir, s, w, h, clip, THREADED)
    sw, sh = SPECIAL_SIZE
    special = synth_special(sw, sh, FRAMES, SEED)
    for s in stage_ids():
        got[(s, "{}x{}s".format(sw, sh))] = render(bindir, s, sw, sh, special)
    if a.write:
        for k, (v, tail) in sorted(got.items()):
            if v.startswith("rc"):
                print("fx_golden: {} {} ends with {} and is recorded as that outcome: {}".format(k[0], k[1], v, " ".join(tail)))
        with open(TABLE, "w", encoding="utf-8", newline="\n") as f:
            f.write("stage\tsize\tframes\tseed\tsha256\n")
            for (s, size), (v, _) in sorted(got.items()):
                f.write("{}\t{}\t{}\t{}\t{}\n".format(s, size, FRAMES, SEED, v))
        print("fx_golden: wrote {} cases to tools/fx_golden.tsv".format(len(got)))
        return 0
    with open(TABLE, encoding="utf-8") as f:
        want = {}
        for line in list(f)[1:]:
            c = line.rstrip("\n").split("\t")
            if len(c) == 5:
                want[(c[0], c[1])] = c[4]
    failed = []
    for key in sorted(set(got) | set(want)):
        v, tail = got.get(key, ("absent", []))
        if want.get(key) != v:
            failed.append(key)
            print("DIFFERS {} {}: expected {}, rendered {} {}".format(key[0], key[1], want.get(key, "no row"), v, " ".join(tail)).rstrip())
    print("fx_golden: {} cases, {} equal, {} differ".format(len(set(got) | set(want)), len(set(got) | set(want)) - len(failed), len(failed)))
    if failed:
        print("fx_golden: styles that differ: " + ", ".join(sorted({k[0] for k in failed})))
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
