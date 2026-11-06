//! Parity of the effect stages: every in-profile frozen member goes through
//! `sapphire stage apply` and its relaid 'ae32' words are compared, by full SHA-256, with the
//! recorded prediction.
//!
//! The members are listed in `$VEC_DIR/fx_parity/members.tsv` (one line per member: stage, width,
//! height, frame index, source word files, frame word files, parameters, expected hashes, name).
//! The test needs `VEC_DIR` and an `fx_run` (named by `SAPPHIRE_FX_RUN`, or beside the test binaries).
//! Without them it prints `NOT RUN` and passes.

use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn sha256(data: &[u8]) -> String {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74,
        0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d,
        0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e,
        0x92722c85, 0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5,
        0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];
    let mut h: [u32; 8] = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19];
    let mut msg = data.to_vec();
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&((data.len() as u64) * 8).to_be_bytes());
    for chunk in msg.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([chunk[i * 4], chunk[i * 4 + 1], chunk[i * 4 + 2], chunk[i * 4 + 3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }
        let mut v = h;
        for i in 0..64 {
            let s1 = v[4].rotate_right(6) ^ v[4].rotate_right(11) ^ v[4].rotate_right(25);
            let ch = (v[4] & v[5]) ^ (!v[4] & v[6]);
            let t1 = v[7].wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0 = v[0].rotate_right(2) ^ v[0].rotate_right(13) ^ v[0].rotate_right(22);
            let maj = (v[0] & v[1]) ^ (v[0] & v[2]) ^ (v[1] & v[2]);
            let t2 = s0.wrapping_add(maj);
            v = [t1.wrapping_add(t2), v[0], v[1], v[2], v[3].wrapping_add(t1), v[4], v[5], v[6]];
        }
        for i in 0..8 {
            h[i] = h[i].wrapping_add(v[i]);
        }
    }
    h.iter().map(|x| format!("{:08x}", x)).collect()
}

/// 'ae32' words (A, R, G, B per pixel) to a gbrapf32le frame, as the stage reads it.
fn words_to_frame(words: &[u8], n: usize) -> Vec<u8> {
    let w = |i: usize| &words[i * 4..i * 4 + 4];
    let mut out = Vec::with_capacity(n * 16);
    for plane in [2usize, 3, 1, 0] {
        for i in 0..n {
            out.extend_from_slice(w(i * 4 + plane));
        }
    }
    out
}

/// A gbrapf32le frame back to 'ae32' words.
fn frame_to_words(frame: &[u8], n: usize) -> Vec<u8> {
    let mut out = Vec::with_capacity(n * 16);
    for i in 0..n {
        for plane in [3usize, 2, 0, 1] {
            out.extend_from_slice(&frame[(plane * n + i) * 4..(plane * n + i) * 4 + 4]);
        }
    }
    out
}

fn fx_run() -> Option<PathBuf> {
    if let Some(p) = std::env::var_os("SAPPHIRE_FX_RUN") {
        return Some(PathBuf::from(p)).filter(|p| p.is_file());
    }
    let exe = PathBuf::from(env!("CARGO_BIN_EXE_sapphire"));
    Some(exe.parent()?.join("fx_run")).filter(|p| p.is_file())
}

#[test]
fn every_in_profile_member_is_byte_identical() {
    let vec_dir = match std::env::var_os("VEC_DIR") {
        Some(v) => PathBuf::from(v),
        None => {
            println!("NOT RUN: VEC_DIR is not set");
            return;
        }
    };
    let list = vec_dir.join("fx_parity").join("members.tsv");
    if !list.is_file() {
        println!("NOT RUN: {} not found", list.display());
        return;
    }
    let fx = match fx_run() {
        Some(p) => p,
        None => {
            println!("NOT RUN: no fx_run (set SAPPHIRE_FX_RUN)");
            return;
        }
    };
    let text = std::fs::read_to_string(&list).expect("members.tsv");
    let (mut total, mut equal) = (0usize, 0usize);
    let mut failures: Vec<String> = Vec::new();
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let c: Vec<&str> = line.split('\t').collect();
        assert!(c.len() >= 9, "bad members line: {}", line);
        let (stage, w, h, f) = (c[0], c[1].parse::<usize>().unwrap(), c[2].parse::<usize>().unwrap(), c[3].parse::<usize>().unwrap());
        let n = w * h;
        let srcs: Vec<&str> = c[4].split(';').collect();
        let mut frames: std::collections::BTreeMap<usize, Vec<u8>> = std::collections::BTreeMap::new();
        for (i, s) in srcs.iter().enumerate() {
            frames.insert(if srcs.len() > 1 { i } else { f }, std::fs::read(s).expect("source words"));
        }
        for kv in c[5].split(';').filter(|t| !t.is_empty()) {
            let (k, p) = kv.split_once('=').unwrap();
            frames.insert(k.parse().unwrap(), std::fs::read(p).expect("frame words"));
        }
        let fallback = frames.get(&f).cloned().unwrap_or_else(|| frames.values().next().unwrap().clone());
        let count = frames.keys().max().copied().unwrap_or(0).max(f) + 1;
        let mut input = Vec::new();
        for i in 0..count {
            input.extend(words_to_frame(frames.get(&i).unwrap_or(&fallback), n));
        }
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_sapphire"));
        cmd.args(["stage", "apply", stage, "--width", &w.to_string(), "--height", &h.to_string(), "--depth", "32", "--fps", "24/1"]);
        for p in c[6].split('\u{1f}').filter(|t| !t.is_empty()) {
            cmd.args(["--param", p]);
        }
        let mut child = cmd
            .env("SAPPHIRE_FX_RUN", &fx)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn sapphire");
        let mut stdin = child.stdin.take().unwrap();
        let feed = std::thread::spawn(move || {
            let _ = stdin.write_all(&input);
        });
        let ran = child.wait_with_output().expect("wait");
        let _ = feed.join();
        total += 1;
        let fb = n * 16;
        let expect: Vec<&str> = c[7].split(';').collect();
        if ran.status.code() != Some(0) {
            failures.push(format!("{} {}: exit {:?}: {}", stage, c[8], ran.status.code(), String::from_utf8_lossy(&ran.stderr).trim()));
            continue;
        }
        let first = if expect.len() > 1 { 0 } else { f };
        let mut ok = true;
        for (k, want) in expect.iter().enumerate() {
            let at = first + k;
            if ran.stdout.len() < (at + 1) * fb {
                ok = false;
                break;
            }
            let got = sha256(&frame_to_words(&ran.stdout[at * fb..(at + 1) * fb], n));
            if got != *want {
                ok = false;
                failures.push(format!("{} {}: render {} sha {} != recorded {}", stage, c[8], k + 1, got, want));
            }
        }
        if ok {
            equal += 1;
        }
    }
    println!("fx_stage_parity: {} members, {} byte identical", total, equal);
    assert!(failures.is_empty(), "{} members differ:\n{}", failures.len(), failures.join("\n"));
}

#[test]
fn sha256_known_answers() {
    assert_eq!(sha256(b""), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    assert_eq!(sha256(b"abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
}
