//! Reader for the harness run folders (`oracle/img_oracle.py`, manifest schema `fx_oracle_v1`) with a small std-only JSON parser.
//!
//! A run folder holds `manifest.json` plus raw little-endian f32 files of `height * stride` floats, starting at the plane pointer
//! the emulated original saw. T2 tests read inputs and expected outputs from these files; they never regenerate them.

use crate::plane::read_f32_le;
use std::path::{Path, PathBuf};

/// Minimal JSON value. Numbers keep their f64 value; integers up to 2^53 are exact.
#[derive(Clone, Debug, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Arr(Vec<Json>),
    Obj(Vec<(String, Json)>),
}

impl Json {
    pub fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Json::Obj(v) => v.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        if let Json::Str(s) = self { Some(s) } else { None }
    }
    pub fn as_f64(&self) -> Option<f64> {
        if let Json::Num(n) = self { Some(*n) } else { None }
    }
    pub fn as_usize(&self) -> Option<usize> {
        self.as_f64().map(|n| n as usize)
    }
    pub fn as_arr(&self) -> Option<&[Json]> {
        if let Json::Arr(a) = self { Some(a) } else { None }
    }
    pub fn as_obj(&self) -> Option<&[(String, Json)]> {
        if let Json::Obj(o) = self { Some(o) } else { None }
    }
}

struct Parser<'a> {
    s: &'a [u8],
    i: usize,
}

impl<'a> Parser<'a> {
    fn ws(&mut self) {
        while self.i < self.s.len() && matches!(self.s[self.i], b' ' | b'\t' | b'\n' | b'\r') {
            self.i += 1;
        }
    }
    fn err<T>(&self, m: &str) -> Result<T, String> {
        Err(format!("json error at byte {}: {}", self.i, m))
    }
    fn lit(&mut self, word: &str, v: Json) -> Result<Json, String> {
        if self.s[self.i..].starts_with(word.as_bytes()) {
            self.i += word.len();
            Ok(v)
        } else {
            self.err("bad literal")
        }
    }
    fn value(&mut self) -> Result<Json, String> {
        self.ws();
        match self.s.get(self.i) {
            None => self.err("unexpected end"),
            Some(b'n') => self.lit("null", Json::Null),
            Some(b't') => self.lit("true", Json::Bool(true)),
            Some(b'f') => self.lit("false", Json::Bool(false)),
            Some(b'"') => Ok(Json::Str(self.string()?)),
            Some(b'[') => {
                self.i += 1;
                let mut v = Vec::new();
                loop {
                    self.ws();
                    if self.s.get(self.i) == Some(&b']') {
                        self.i += 1;
                        return Ok(Json::Arr(v));
                    }
                    v.push(self.value()?);
                    self.ws();
                    match self.s.get(self.i) {
                        Some(b',') => self.i += 1,
                        Some(b']') => {}
                        _ => return self.err("expected , or ]"),
                    }
                }
            }
            Some(b'{') => {
                self.i += 1;
                let mut v = Vec::new();
                loop {
                    self.ws();
                    if self.s.get(self.i) == Some(&b'}') {
                        self.i += 1;
                        return Ok(Json::Obj(v));
                    }
                    let k = self.string()?;
                    self.ws();
                    if self.s.get(self.i) != Some(&b':') {
                        return self.err("expected :");
                    }
                    self.i += 1;
                    let val = self.value()?;
                    v.push((k, val));
                    self.ws();
                    match self.s.get(self.i) {
                        Some(b',') => self.i += 1,
                        Some(b'}') => {}
                        _ => return self.err("expected , or }"),
                    }
                }
            }
            Some(_) => {
                let st = self.i;
                while self.i < self.s.len() && matches!(self.s[self.i], b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9') {
                    self.i += 1;
                }
                let t = std::str::from_utf8(&self.s[st..self.i]).unwrap_or("");
                t.parse::<f64>().map(Json::Num).or_else(|_| self.err("bad number"))
            }
        }
    }
    fn string(&mut self) -> Result<String, String> {
        if self.s.get(self.i) != Some(&b'"') {
            return self.err("expected string");
        }
        self.i += 1;
        let mut out = Vec::new();
        loop {
            match self.s.get(self.i) {
                None => return self.err("unterminated string"),
                Some(b'"') => {
                    self.i += 1;
                    return String::from_utf8(out).map_err(|_| "bad utf8 in string".to_string());
                }
                Some(b'\\') => {
                    self.i += 1;
                    let c = *self.s.get(self.i).ok_or("bad escape")?;
                    self.i += 1;
                    match c {
                        b'n' => out.push(b'\n'),
                        b't' => out.push(b'\t'),
                        b'r' => out.push(b'\r'),
                        b'b' => out.push(8),
                        b'f' => out.push(12),
                        b'/' | b'\\' | b'"' => out.push(c),
                        b'u' => {
                            let h = std::str::from_utf8(self.s.get(self.i..self.i + 4).ok_or("bad \\u")?).map_err(|_| "bad \\u")?;
                            let cp = u32::from_str_radix(h, 16).map_err(|_| "bad \\u")?;
                            self.i += 4;
                            let ch = char::from_u32(cp).unwrap_or('?');
                            let mut b = [0u8; 4];
                            out.extend_from_slice(ch.encode_utf8(&mut b).as_bytes());
                        }
                        _ => return self.err("bad escape"),
                    }
                }
                Some(&c) => {
                    out.push(c);
                    self.i += 1;
                }
            }
        }
    }
}

/// Parse a JSON document.
pub fn parse_json(text: &str) -> Result<Json, String> {
    let mut p = Parser { s: text.as_bytes(), i: 0 };
    let v = p.value()?;
    p.ws();
    if p.i != text.len() {
        return p.err("trailing data");
    }
    Ok(v)
}

/// A data patch the harness applied to emulated memory (S7).
#[derive(Clone, Debug, PartialEq)]
pub struct Patch {
    pub va: u64,
    pub hex: String,
    pub label: String,
}

/// One planes file entry: name -> file name.
#[derive(Clone, Debug, PartialEq)]
pub struct PlaneFile {
    pub name: String,
    pub file: String,
    pub sha256: String,
}

/// One case of a run folder.
#[derive(Clone, Debug)]
pub struct Case {
    pub name: String,
    pub width: usize,
    pub height: usize,
    /// Row pitch in f32 units (width + 3 in the misaligned case).
    pub stride: usize,
    /// Bytes the plane pointers were moved off 16 byte alignment (4 in the misaligned case, else 0).
    pub misalign_bytes: usize,
    pub params: Json,
    pub inputs: Vec<PlaneFile>,
    pub outputs: Vec<PlaneFile>,
    pub status: String,
    pub insns: u64,
    pub seconds: f64,
    dir: PathBuf,
}

impl Case {
    fn file(&self, list: &[PlaneFile], name: &str) -> Result<Vec<f32>, String> {
        let f = list.iter().find(|p| p.name == name).ok_or_else(|| format!("case {}: no plane '{}'", self.name, name))?;
        let v = read_f32_le(&self.dir.join(&f.file))?;
        if v.len() != self.height * self.stride {
            return Err(format!("{}: {} floats, expected height*stride = {}", f.file, v.len(), self.height * self.stride));
        }
        Ok(v)
    }
    /// Input plane `name` as height * stride floats.
    pub fn input(&self, name: &str) -> Result<Vec<f32>, String> {
        self.file(&self.inputs, name)
    }
    /// Expected output plane `name` (what the emulated original wrote) as height * stride floats.
    pub fn output(&self, name: &str) -> Result<Vec<f32>, String> {
        self.file(&self.outputs, name)
    }
    /// A numeric parameter by name.
    pub fn param_f64(&self, key: &str) -> Option<f64> {
        self.params.get(key).and_then(|v| v.as_f64())
    }
    pub fn ok(&self) -> bool {
        self.status == "ok"
    }
}

/// A loaded run folder.
#[derive(Clone, Debug)]
pub struct Manifest {
    pub dir: PathBuf,
    pub entry: String,
    pub engine: String,
    pub image_set: String,
    pub param_set: String,
    pub patches: Vec<Patch>,
    pub cases: Vec<Case>,
}

fn plane_files(j: Option<&Json>) -> Vec<PlaneFile> {
    j.and_then(|o| o.as_obj())
        .map(|o| {
            o.iter()
                .map(|(k, v)| PlaneFile {
                    name: k.clone(),
                    file: v.get("file").and_then(|x| x.as_str()).unwrap_or("").to_string(),
                    sha256: v.get("sha256").and_then(|x| x.as_str()).unwrap_or("").to_string(),
                })
                .collect()
        })
        .unwrap_or_default()
}

impl Manifest {
    /// Load `<dir>/manifest.json`.
    pub fn load(dir: &Path) -> Result<Manifest, String> {
        let text = std::fs::read_to_string(dir.join("manifest.json")).map_err(|e| format!("{}: {}", dir.display(), e))?;
        let j = parse_json(&text)?;
        let s = |k: &str| j.get(k).and_then(|v| v.as_str()).unwrap_or("").to_string();
        if s("schema") != "fx_oracle_v1" {
            return Err(format!("unsupported manifest schema '{}'", s("schema")));
        }
        let patches = j
            .get("patches")
            .and_then(|p| p.as_arr())
            .unwrap_or(&[])
            .iter()
            .map(|p| Patch {
                va: u64::from_str_radix(p.get("va").and_then(|x| x.as_str()).unwrap_or("0x0").trim_start_matches("0x"), 16).unwrap_or(0),
                hex: p.get("hex").and_then(|x| x.as_str()).unwrap_or("").to_string(),
                label: p.get("label").and_then(|x| x.as_str()).unwrap_or("").to_string(),
            })
            .collect();
        let mut cases = Vec::new();
        for c in j.get("cases").and_then(|c| c.as_arr()).unwrap_or(&[]) {
            let n = |k: &str| c.get(k).and_then(|v| v.as_usize()).unwrap_or(0);
            cases.push(Case {
                name: c.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                width: n("width"),
                height: n("height"),
                stride: n("stride"),
                misalign_bytes: n("misalign_bytes"),
                params: c.get("params").cloned().unwrap_or(Json::Null),
                inputs: plane_files(c.get("inputs")),
                outputs: plane_files(c.get("outputs")),
                status: c.get("status").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                insns: c.get("insns").and_then(|v| v.as_f64()).unwrap_or(0.0) as u64,
                seconds: c.get("seconds").and_then(|v| v.as_f64()).unwrap_or(0.0),
                dir: dir.to_path_buf(),
            });
        }
        Ok(Manifest { dir: dir.to_path_buf(), entry: s("entry"), engine: s("engine"), image_set: s("image_set"), param_set: s("param_set"), patches, cases })
    }

    pub fn case(&self, name: &str) -> Option<&Case> {
        self.cases.iter().find(|c| c.name == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plane::write_f32_le;

    #[test]
    fn json_basics() {
        let j = parse_json(r#" {"a": [1, 2.5, -3e2, true, null], "b": {"c": "x\"y\nA"}, "d": []} "#).unwrap();
        assert_eq!(j.get("a").unwrap().as_arr().unwrap().len(), 5);
        assert_eq!(j.get("a").unwrap().as_arr().unwrap()[2].as_f64(), Some(-300.0));
        assert_eq!(j.get("b").unwrap().get("c").unwrap().as_str(), Some("x\"y\nA"));
        assert!(parse_json("{\"a\": }").is_err());
        assert!(parse_json("[1, 2] x").is_err());
    }

    #[test]
    fn manifest_round_trip() {
        let dir = std::env::temp_dir().join("fx_common_manifest_test");
        std::fs::create_dir_all(&dir).unwrap();
        write_f32_le(&dir.join("c.x.f32"), &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
        write_f32_le(&dir.join("c.out.f32"), &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
        let text = r#"{"schema":"fx_oracle_v1","entry":"0x1","engine":"plain","image_set":"t","param_set":"p",
            "patches":[{"va":"0x1","hex":"0000803f","label":"A1"}],
            "cases":[{"name":"c","width":2,"height":2,"stride":3,"misalign_bytes":4,"params":{"p":0.25},
            "inputs":{"x":{"file":"c.x.f32","sha256":"00"}},"outputs":{"out":{"file":"c.out.f32","sha256":"00"}},
            "status":"ok","insns":12,"seconds":0.5}]}"#;
        std::fs::write(dir.join("manifest.json"), text).unwrap();
        let m = Manifest::load(&dir).unwrap();
        assert_eq!((m.entry.as_str(), m.engine.as_str()), ("0x1", "plain"));
        assert_eq!(m.patches[0].va, 0x1);
        let c = m.case("c").unwrap();
        assert_eq!((c.width, c.height, c.stride, c.misalign_bytes), (2, 2, 3, 4));
        assert_eq!(c.param_f64("p"), Some(0.25));
        assert_eq!(c.input("x").unwrap().len(), 6);
        assert_eq!(c.output("out").unwrap()[5], 6.0);
        assert!(c.input("nope").is_err());
        assert!(c.ok());
        std::fs::remove_dir_all(&dir).ok();
    }
}
