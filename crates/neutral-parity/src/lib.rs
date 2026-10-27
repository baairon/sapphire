//! Replays oracle test vectors against ported functions. std only.
//! The vector format is described in docs/harness.md, section Vectors.

use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    In,
    Out,
    InOut,
}

#[derive(Debug, Clone)]
pub enum Arg {
    Int { bits: u32, value: u64 },
    F32(u32),
    F64(u64),
    Buf { size: usize, off: usize, dir: Dir, data: Vec<u8> },
}

#[derive(Debug, Clone)]
pub struct Ret {
    pub spec: String,
    pub bits: u32,
    pub value: u128,
}

#[derive(Debug, Clone)]
pub struct Case {
    pub index: usize,
    pub status: String,
    pub nondeterministic: bool,
    pub abi_ok: bool,
    pub args: Vec<Arg>,
    pub rets: Vec<Ret>,
    /// Final contents of every `out` or `inout` buffer, by argument index.
    pub bufs: Vec<(usize, Vec<u8>)>,
}

impl Case {
    pub fn ok(&self) -> bool {
        self.status == "ok" && !self.nondeterministic
    }

    pub fn int(&self, i: usize) -> u64 {
        match &self.args[i] {
            Arg::Int { value, .. } => *value,
            other => panic!("arg {} is not an int: {:?}", i, other),
        }
    }

    pub fn f32_bits(&self, i: usize) -> u32 {
        match &self.args[i] {
            Arg::F32(b) => *b,
            other => panic!("arg {} is not an f32: {:?}", i, other),
        }
    }

    pub fn f32(&self, i: usize) -> f32 {
        f32::from_bits(self.f32_bits(i))
    }

    pub fn f64_bits(&self, i: usize) -> u64 {
        match &self.args[i] {
            Arg::F64(b) => *b,
            other => panic!("arg {} is not an f64: {:?}", i, other),
        }
    }

    pub fn f64(&self, i: usize) -> f64 {
        f64::from_bits(self.f64_bits(i))
    }

    /// The whole region the original received for buffer argument `i`.
    pub fn buf(&self, i: usize) -> &[u8] {
        match &self.args[i] {
            Arg::Buf { data, .. } => data,
            other => panic!("arg {} is not a buffer: {:?}", i, other),
        }
    }

    /// Offset of the pointer inside the region (pointer = region start + offset).
    pub fn buf_off(&self, i: usize) -> usize {
        match &self.args[i] {
            Arg::Buf { off, .. } => *off,
            other => panic!("arg {} is not a buffer: {:?}", i, other),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VecFile {
    pub name: String,
    pub entry: u64,
    pub seed: u64,
    pub cases: Vec<Case>,
}

/// What a ported function produced for a case.
#[derive(Debug, Default, Clone)]
pub struct Outcome {
    /// One value per `ret` in the spec, in order (narrow values zero-extended).
    pub rets: Vec<u128>,
    /// Final contents (whole region) of each `out` or `inout` buffer, by argument index.
    pub bufs: Vec<(usize, Vec<u8>)>,
}

fn hex_u128(s: &str) -> Result<u128, String> {
    let h = s.strip_prefix("0x").ok_or_else(|| format!("expected 0x prefix: {}", s))?;
    u128::from_str_radix(h, 16).map_err(|e| format!("bad hex {}: {}", s, e))
}

fn hex_bytes(s: &str) -> Result<Vec<u8>, String> {
    let h = s.strip_prefix("0x").ok_or_else(|| format!("expected 0x prefix: {}", s))?;
    if h.len() % 2 != 0 {
        return Err("odd hex length".to_string());
    }
    (0..h.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&h[i..i + 2], 16).map_err(|e| e.to_string()))
        .collect()
}

fn num<T: std::str::FromStr>(tok: Option<&str>, what: &str) -> Result<T, String> {
    tok.ok_or_else(|| format!("missing {}", what))?
        .parse::<T>()
        .map_err(|_| format!("bad {}", what))
}

pub fn parse(text: &str) -> Result<VecFile, String> {
    let mut vf = VecFile { name: String::new(), entry: 0, seed: 0, cases: Vec::new() };
    let mut cur: Option<Case> = None;
    for (ln, line) in text.lines().enumerate() {
        let line = line.trim_end();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let at = |m: String| format!("line {}: {}", ln + 1, m);
        let mut it = line.split_whitespace();
        let tag = it.next().unwrap();
        match tag {
            "fn" => {
                vf.name = it.next().ok_or_else(|| at("missing name".into()))?.to_string();
                for kv in it {
                    if let Some(v) = kv.strip_prefix("entry=0x") {
                        vf.entry = u64::from_str_radix(v, 16).map_err(|_| at("bad entry".into()))?;
                    } else if let Some(v) = kv.strip_prefix("seed=") {
                        vf.seed = v.parse().map_err(|_| at("bad seed".into()))?;
                    }
                }
            }
            "case" => {
                let index: usize = num(it.next(), "case index").map_err(at)?;
                let mut rest: Vec<&str> = it.collect();
                let nondet = rest.last() == Some(&"NONDETERMINISTIC");
                if nondet {
                    rest.pop();
                }
                cur = Some(Case {
                    index,
                    status: rest.join(" "),
                    nondeterministic: nondet,
                    abi_ok: true,
                    args: Vec::new(),
                    rets: Vec::new(),
                    bufs: Vec::new(),
                });
            }
            "a" => {
                let c = cur.as_mut().ok_or_else(|| at("a outside case".into()))?;
                let idx: usize = num(it.next(), "arg index").map_err(at)?;
                if idx != c.args.len() {
                    return Err(at(format!("arg index {} out of order", idx)));
                }
                let kind = it.next().ok_or_else(|| at("missing arg kind".into()))?;
                let arg = match kind {
                    "int" => {
                        let bits: u32 = num(it.next(), "bits").map_err(at)?;
                        let v = hex_u128(it.next().unwrap_or("")).map_err(at)?;
                        Arg::Int { bits, value: v as u64 }
                    }
                    "f32" => Arg::F32(hex_u128(it.next().unwrap_or("")).map_err(at)? as u32),
                    "f64" => Arg::F64(hex_u128(it.next().unwrap_or("")).map_err(at)? as u64),
                    "buf" => {
                        let size: usize = num(it.next(), "size").map_err(at)?;
                        let off: usize = num(it.next(), "off").map_err(at)?;
                        let dir = match it.next() {
                            Some("in") => Dir::In,
                            Some("out") => Dir::Out,
                            Some("inout") => Dir::InOut,
                            _ => return Err(at("bad buffer dir".into())),
                        };
                        let data = hex_bytes(it.next().unwrap_or("")).map_err(at)?;
                        if data.len() != size {
                            return Err(at(format!("buffer size {} but {} bytes", size, data.len())));
                        }
                        Arg::Buf { size, off, dir, data }
                    }
                    other => return Err(at(format!("unknown arg kind {}", other))),
                };
                c.args.push(arg);
            }
            "abi" => {
                let c = cur.as_mut().ok_or_else(|| at("abi outside case".into()))?;
                c.abi_ok = it.next() == Some("ok");
            }
            "r" => {
                let c = cur.as_mut().ok_or_else(|| at("r outside case".into()))?;
                let _j: usize = num(it.next(), "ret index").map_err(at)?;
                let spec = it.next().ok_or_else(|| at("missing ret spec".into()))?.to_string();
                let tok = it.next().unwrap_or("");
                let value = hex_u128(tok).map_err(at)?;
                let bits = (tok.len().saturating_sub(2) * 4) as u32;
                c.rets.push(Ret { spec, bits, value });
            }
            "b" => {
                let c = cur.as_mut().ok_or_else(|| at("b outside case".into()))?;
                let i: usize = num(it.next(), "buffer index").map_err(at)?;
                let data = hex_bytes(it.next().unwrap_or("")).map_err(at)?;
                c.bufs.push((i, data));
            }
            "end" => {
                let c = cur.take().ok_or_else(|| at("end outside case".into()))?;
                vf.cases.push(c);
            }
            other => return Err(at(format!("unknown tag {}", other))),
        }
    }
    if cur.is_some() {
        return Err("file ends inside a case".to_string());
    }
    Ok(vf)
}

pub fn load(path: &Path) -> Result<VecFile, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {}", path.display(), e))?;
    parse(&text)
}

/// Equal bits, or both NaN (payload and sign of a NaN are not compared).
pub fn f32_same(a: u32, b: u32) -> bool {
    a == b || (f32::from_bits(a).is_nan() && f32::from_bits(b).is_nan())
}

pub fn f64_same(a: u64, b: u64) -> bool {
    a == b || (f64::from_bits(a).is_nan() && f64::from_bits(b).is_nan())
}

#[derive(Debug, Clone)]
pub struct Report {
    pub name: String,
    pub total: usize,
    /// Cases whose status was ok and deterministic, and were therefore compared.
    pub compared: usize,
    pub skipped: usize,
    pub failed: usize,
    /// First mismatches (at most 20).
    pub mismatches: Vec<String>,
}

impl Report {
    /// Panics unless every compared case matched and at least `min_compared` cases were compared.
    pub fn assert_ok(&self, min_compared: usize) {
        if self.failed > 0 || self.compared < min_compared {
            panic!(
                "{}: {} of {} compared cases failed ({} skipped, need at least {} compared)\n{}",
                self.name,
                self.failed,
                self.compared,
                self.skipped,
                min_compared,
                self.mismatches.join("\n")
            );
        }
    }
}

pub fn check(vf: &VecFile, mut f: impl FnMut(&Case) -> Outcome) -> Report {
    let mut rep = Report {
        name: vf.name.clone(),
        total: vf.cases.len(),
        compared: 0,
        skipped: 0,
        failed: 0,
        mismatches: Vec::new(),
    };
    for c in &vf.cases {
        if !c.ok() {
            rep.skipped += 1;
            continue;
        }
        rep.compared += 1;
        let got = f(c);
        let mut errs: Vec<String> = Vec::new();
        if got.rets.len() != c.rets.len() {
            errs.push(format!("expected {} return values, got {}", c.rets.len(), got.rets.len()));
        } else {
            for (j, (g, w)) in got.rets.iter().zip(&c.rets).enumerate() {
                let mask = if w.bits >= 128 { u128::MAX } else { (1u128 << w.bits) - 1 };
                let g = *g & mask;
                let same = if w.spec.ends_with(":f32") {
                    f32_same(g as u32, w.value as u32)
                } else if w.spec.ends_with(":f64") {
                    f64_same(g as u64, w.value as u64)
                } else {
                    g == w.value
                };
                if !same {
                    errs.push(format!("ret {} ({}) got 0x{:x} want 0x{:x}", j, w.spec, g, w.value));
                }
            }
        }
        for (i, want) in &c.bufs {
            match got.bufs.iter().find(|(gi, _)| gi == i) {
                None => errs.push(format!("buffer {} not produced", i)),
                Some((_, gb)) => {
                    if gb != want {
                        let at = gb.iter().zip(want).position(|(a, b)| a != b).unwrap_or(gb.len().min(want.len()));
                        errs.push(format!("buffer {} differs at byte {} (len got {} want {})", i, at, gb.len(), want.len()));
                    }
                }
            }
        }
        if !errs.is_empty() {
            rep.failed += 1;
            if rep.mismatches.len() < 20 {
                rep.mismatches.push(format!("case {}: {}", c.index, errs.join("; ")));
            }
        }
    }
    rep
}

#[cfg(test)]
mod tests {
    use super::*;

    const SNIP: &str = "# oracle vectors v1
fn demo entry=0x10 args=2 rets=1 seed=7
case 0 ok
a 0 int 32 0x00000001
a 1 int 32 0x00000002
abi ok
r 0 eax 0x00000003
end
case 1 fault READ_UNMAPPED READ_UNMAPPED 0x5000 4
a 0 int 32 0x00000001
a 1 int 32 0x00000002
end
case 2 ok NONDETERMINISTIC
a 0 int 32 0x00000009
a 1 int 32 0x00000009
abi ok
r 0 eax 0x00000012
end
";

    #[test]
    fn parses_and_skips_bad_cases() {
        let vf = parse(SNIP).unwrap();
        assert_eq!(vf.name, "demo");
        assert_eq!(vf.entry, 0x10);
        assert_eq!(vf.seed, 7);
        assert_eq!(vf.cases.len(), 3);
        assert!(vf.cases[0].ok());
        assert!(!vf.cases[1].ok());
        assert!(!vf.cases[2].ok());
        assert_eq!(vf.cases[1].status, "fault READ_UNMAPPED READ_UNMAPPED 0x5000 4");
        assert_eq!(vf.cases[0].rets[0].bits, 32);
    }

    #[test]
    fn matching_port_passes() {
        let vf = parse(SNIP).unwrap();
        let rep = check(&vf, |c| Outcome {
            rets: vec![(c.int(0) as u32).wrapping_add(c.int(1) as u32) as u128],
            bufs: vec![],
        });
        assert_eq!((rep.compared, rep.skipped, rep.failed), (1, 2, 0));
        rep.assert_ok(1);
    }

    #[test]
    fn wrong_port_fails() {
        let vf = parse(SNIP).unwrap();
        let rep = check(&vf, |_| Outcome { rets: vec![99], bufs: vec![] });
        assert_eq!(rep.failed, 1);
        assert!(rep.mismatches[0].contains("want 0x3"));
    }

    #[test]
    fn buffers_and_floats() {
        let text = "fn buf entry=0x0 args=2 rets=1 seed=1
case 0 ok
a 0 buf 4 1 inout 0x01020304
a 1 f32 0x7fc00001
abi ok
r 0 xmm0:f32 0x7fc00000
b 0 0x01020399
end
";
        let vf = parse(text).unwrap();
        assert_eq!(vf.cases[0].buf_off(0), 1);
        let rep = check(&vf, |c| {
            let mut b = c.buf(0).to_vec();
            b[3] = 0x99;
            Outcome { rets: vec![f32::NAN.to_bits() as u128], bufs: vec![(0, b)] }
        });
        rep.assert_ok(1);
        let rep = check(&vf, |c| Outcome { rets: vec![0x7fc00000], bufs: vec![(0, c.buf(0).to_vec())] });
        assert_eq!(rep.failed, 1);
    }
}
