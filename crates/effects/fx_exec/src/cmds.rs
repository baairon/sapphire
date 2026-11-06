//! Command-level executor: runs a plan in the EP-3 text form (allocate-images, convert-from-source, run-primitive,
//! copy-images, constant, free-images, convert-to-dest, exec-linewise), with the original's buffer reuse. It is the
//! second path to the same builtin crates: node-level (`exec`) uses NV-3's bit-exact lanes, this path uses the lanes as
//! printed (6 significant digits), so it is for structural and plane cross-checks against original plans.
//! f32 only: convert-from-source and convert-to-dest are the identity on f32 planes (other formats NOT REBUILT).

use crate::exec::{run_host, HostFrames};
use crate::kernels;
use crate::selector::Tables;
use crate::types::{Arg, ExecError, Image, KParams, Val};
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Sx {
    Atom(String),
    List(Vec<Sx>),
    Br(Vec<String>),
}

pub fn parse_sx(text: &str) -> Result<Sx, ExecError> {
    let t = text.replace("\\n", " ");
    let mut toks: Vec<String> = Vec::new();
    let mut cur = String::new();
    for ch in t.chars() {
        match ch {
            '(' | ')' | '[' | ']' => {
                if !cur.is_empty() {
                    toks.push(std::mem::take(&mut cur));
                }
                toks.push(ch.to_string());
            }
            c if c.is_whitespace() => {
                if !cur.is_empty() {
                    toks.push(std::mem::take(&mut cur));
                }
            }
            c => cur.push(c),
        }
    }
    if !cur.is_empty() {
        toks.push(cur);
    }
    let mut i = 0usize;
    let r = parse_item(&toks, &mut i, 0)?;
    Ok(r)
}

fn parse_item(toks: &[String], i: &mut usize, depth: usize) -> Result<Sx, ExecError> {
    if depth > 64 {
        return Err(ExecError::BadPlan("nesting too deep".into()));
    }
    let t = toks.get(*i).ok_or_else(|| ExecError::BadPlan("unexpected end of command".into()))?;
    *i += 1;
    match t.as_str() {
        "(" => {
            let mut v = Vec::new();
            loop {
                match toks.get(*i).map(|s| s.as_str()) {
                    None => return Err(ExecError::BadPlan("missing )".into())),
                    Some(")") => {
                        *i += 1;
                        return Ok(Sx::List(v));
                    }
                    Some(_) => v.push(parse_item(toks, i, depth + 1)?),
                }
            }
        }
        "[" => {
            let mut v = Vec::new();
            loop {
                match toks.get(*i).map(|s| s.as_str()) {
                    None => return Err(ExecError::BadPlan("missing ]".into())),
                    Some("]") => {
                        *i += 1;
                        return Ok(Sx::Br(v));
                    }
                    Some(a) => {
                        v.push(a.to_string());
                        *i += 1;
                    }
                }
            }
        }
        ")" | "]" => Err(ExecError::BadPlan("unbalanced bracket".into())),
        a => Ok(Sx::Atom(a.to_string())),
    }
}

/// The printed command texts of an EP-3 dump (`CMD <seq> <type> <text> <addr>` rows).
pub fn commands_of_dump(dump: &str) -> Vec<String> {
    dump.lines()
        .filter(|l| l.starts_with("CMD\t"))
        .filter_map(|l| l.split('\t').nth(3).map(|s| s.to_string()))
        .collect()
}

#[derive(Debug)]
pub struct CmdOutput {
    pub dest: Option<Image>,
    /// the host destination memory written by the native convert-to-dest (only when a `HostConvert` was given)
    pub host_dest: Option<Vec<f32>>,
    pub host_wrote: bool,
    /// the 'ae32' destination memory (A,R,G,B u32 words) written by the code-14 bodies (only on the 'ae32' route)
    pub ae32_dest: Option<Vec<u32>>,
    pub saved_prev_dest: Option<Image>,
    pub trace: Vec<String>,
}

struct St<'a> {
    refs: BTreeMap<String, Val>,
    inputs: &'a [Image],
    n: usize,
    w: usize,
    h: usize,
    xy: Option<&'a Image>,
    host: &'a HostFrames,
    kp: &'a KParams,
    hc: Option<&'a crate::hostconv::HostConvert>,
    tables: Tables,
    out: CmdOutput,
}

fn atom(s: &Sx) -> Option<&str> {
    if let Sx::Atom(a) = s {
        Some(a.as_str())
    } else {
        None
    }
}

fn refname(s: &Sx) -> Result<String, ExecError> {
    atom(s).filter(|a| a.starts_with(":ref")).map(|a| a.to_string()).ok_or_else(|| ExecError::BadPlan(format!("expected :ref, found {:?}", s)))
}

fn lanes_of(b: &[String]) -> Result<Val, ExecError> {
    let mut v: Vec<f32> = Vec::new();
    for t in b {
        v.push(t.parse::<f32>().map_err(|_| ExecError::BadPlan(format!("bad constant lane {}", t)))?);
    }
    if v.is_empty() || v.len() > 4 {
        return Err(ExecError::BadPlan(format!("constant with {} lanes", v.len())));
    }
    let n = v.len();
    let last = *v.last().unwrap_or(&0.0);
    let mut l = [last.to_bits(); 4];
    for (i, x) in v.iter().enumerate() {
        l[i] = x.to_bits();
    }
    Ok(Val::Const { n, lanes: l })
}

impl<'a> St<'a> {
    fn get(&self, r: &str) -> Result<&Val, ExecError> {
        self.refs.get(r).ok_or_else(|| ExecError::BadPlan(format!("{} used before allocation or after free", r)))
    }

    fn run(&mut self, sx: &Sx) -> Result<(), ExecError> {
        let l = match sx {
            Sx::List(l) => l,
            _ => return Err(ExecError::BadPlan("command is not a list".into())),
        };
        let head = l.first().and_then(atom).ok_or_else(|| ExecError::BadPlan("command without a name".into()))?;
        let n = self.n;
        match head {
            "allocate-images" | "allocate-images-preserved" => {
                let r = refname(l.get(1).ok_or_else(|| ExecError::BadPlan("allocate-images".into()))?)?;
                let ch: usize = l.get(2).and_then(atom).and_then(|a| a.parse().ok()).ok_or_else(|| ExecError::BadPlan("allocate-images channels".into()))?;
                self.refs.insert(r, Val::Img(Rc::new(Image::zeros(ch, n))));
            }
            "convert-from-source" => {
                let idx: usize = l.get(1).and_then(atom).and_then(|a| a.parse().ok()).ok_or_else(|| ExecError::BadPlan("convert-from-source index".into()))?;
                let r = refname(l.get(2).ok_or_else(|| ExecError::BadPlan("convert-from-source ref".into()))?)?;
                if let Some(ae) = self.hc.and_then(|hc| hc.ae32.as_ref()) {
                    // the 'ae32' route, the recorded reader head and plane count of this source (readers)
                    let s = ae.sources.get(idx).ok_or_else(|| ExecError::MissingInput(format!("'ae32' host source {}", idx)))?;
                    let im = crate::hostconv14::ae32_to_image(s, self.w, self.h)?;
                    self.refs.insert(r, Val::Img(Rc::new(im)));
                } else if let Some(hc) = self.hc {
                    // native convert-from-source on HOST MEMORY (format 16, B,G,R,A f32), one converted row per call (cell 1)
                    let mem = hc.src.get(idx).ok_or_else(|| ExecError::MissingInput(format!("host source {}", idx)))?;
                    let im = crate::hostconv::host_to_image(mem, self.w, self.h)?;
                    self.refs.insert(r, Val::Img(Rc::new(im)));
                } else {
                    let im = self.inputs.get(idx).ok_or_else(|| ExecError::MissingInput(format!("source {}", idx)))?;
                    if im.n != n {
                        return Err(ExecError::Shape(format!("source {} has {} samples per plane, frame is {}", idx, im.n, n)));
                    }
                    self.refs.insert(r, Val::Img(Rc::new(im.clone())));
                }
            }
            "copy-images" => {
                let d = refname(l.get(1).ok_or_else(|| ExecError::BadPlan("copy-images dest".into()))?)?;
                let s = refname(l.get(2).ok_or_else(|| ExecError::BadPlan("copy-images src".into()))?)?;
                let v = self.get(&s)?.clone();
                self.refs.insert(d, v);
            }
            "free-images" => {
                for it in &l[1..] {
                    if let Some(a) = atom(it) {
                        if a.starts_with(":ref") {
                            self.refs.remove(a).ok_or_else(|| ExecError::BadPlan(format!("free of {} which is not allocated", a)))?;
                        }
                    }
                }
            }
            "constant" => {
                let b = match l.get(1) {
                    Some(Sx::Br(b)) => b.clone(),
                    _ => return Err(ExecError::BadPlan("constant without lanes".into())),
                };
                let r = refname(l.get(2).ok_or_else(|| ExecError::BadPlan("constant ref".into()))?)?;
                self.refs.insert(r, lanes_of(&b)?);
            }
            "convert-to-dest" => {
                let r = refname(l.get(1).ok_or_else(|| ExecError::BadPlan("convert-to-dest ref".into()))?)?;
                let im = match self.get(&r)? {
                    Val::Img(i) => (**i).clone(),
                    Val::Const { n: ln, lanes } => Image::from_const(lanes, *ln, n),
                    Val::Mip(_) => return Err(ExecError::Shape("convert-to-dest of a ripmap object".into())),
                    Val::Dev(_) => return Err(ExecError::Shape("convert-to-dest of a device image".into())),
                };
                if let Some(ae) = self.hc.and_then(|hc| hc.ae32.as_ref()) {
                    // DEST-ROUTE (code 14 zero-arm bodies and the row copier), never the format-16 writer
                    self.out.ae32_dest = Some(crate::hostconv14::ae32_dest(ae, &im, self.w, self.h)?);
                } else if let Some(hc) = self.hc {
                    // native convert-to-dest on HOST MEMORY with the host's alpha mode
                    let (hd, wrote) = crate::hostconv::image_to_host(&im, self.w, self.h, hc.mode, &hc.dest_prefill)?;
                    self.out.host_dest = Some(hd);
                    self.out.host_wrote = wrote;
                }
                self.out.dest = Some(im);
            }
            "exec-linewise" => {
                // (exec-linewise A B (pre) (commands) (post)): the middle group holds the wrapped commands (f32: identity wrapper)
                for g in l.iter().skip(3) {
                    if let Sx::List(inner) = g {
                        for c in inner {
                            if matches!(c, Sx::List(_)) {
                                self.run(c)?;
                            }
                        }
                    }
                }
            }
            "variable" => {
                // (variable 2 :ref): the xy coordinate variable (NV9-A3), the caller's plane or the host-geometry grid
                let r = refname(l.get(2).ok_or_else(|| ExecError::BadPlan("variable ref".into()))?)?;
                let im = match self.xy {
                    Some(i) => i.clone(),
                    None => crate::kernels2::default_xy(self.kp, self.w, self.h),
                };
                self.refs.insert(r, Val::Img(Rc::new(im)));
            }
            "run-primitive" => self.run_prim(l)?,
            other => return Err(ExecError::Unsupported { class: other.to_string(), reason: "plan command not rebuilt".into() }),
        }
        Ok(())
    }

    fn run_prim(&mut self, l: &[Sx]) -> Result<(), ExecError> {
        // (run-primitive A B :dest prim arg...)
        let dest = refname(l.get(3).ok_or_else(|| ExecError::BadPlan("run-primitive dest".into()))?)?;
        let prim = l.get(4).and_then(atom).ok_or_else(|| ExecError::BadPlan("run-primitive name".into()))?.to_string();
        let c = kernels::normalize_class(&prim).to_string();
        if !kernels::is_supported(&c) {
            return Err(ExecError::Unsupported { class: c, reason: "no kernel wrapper in fx_exec (builtin not wired or not rebuilt)".into() });
        }
        let n = self.n;
        let mut vals: Vec<Val> = Vec::new();
        for it in &l[5..] {
            match it {
                Sx::Atom(a) if a.starts_with(":ref") => vals.push(self.get(a)?.clone()),
                Sx::Br(b) => vals.push(lanes_of(b)?),
                Sx::List(_) => {} // (clip ..) wildcard: the host frame comes from HostFrames
                Sx::Atom(a) => return Err(ExecError::BadPlan(format!("unexpected argument word {}", a))),
            }
        }
        let args: Vec<Arg> = vals.iter().map(|v| v.as_arg()).collect();
        let operands: Vec<i32> = args.iter().map(|a| a.count()).collect();
        let dest_ch = match self.refs.get(&dest) {
            Some(Val::Img(i)) => i.channels() as i32,
            _ => 0,
        };
        let (img, tag) = if kernels::HOST_CLASSES.contains(&c.as_str()) {
            let mut saved = self.out.saved_prev_dest.take();
            let r = run_host(&c, dest_ch, &args, n, self.host, &mut saved)?;
            self.out.saved_prev_dest = saved;
            (crate::kernels2::Out::Img(r.0), r.1)
        } else {
            let tag = self.tables.select_counts(&c, &operands);
            if tag < 0 {
                return Err(ExecError::NoCombo { class: c, operands });
            }
            // HOST-COUNT = W x H as an explicit value (STATE-A-COUNT: the writer is, [ctx] = W x H, cited; the executor derives it, there is no per-run input); the alias flag is the plan's own: dest ref = first source ref
            let mut kpe = *self.kp;
            if kpe.sharpen_count.is_none() {
                kpe.sharpen_count = Some(n);
            }
            if c == "sharpen" {
                kpe.sharpen_alias = l.get(5).and_then(atom).map(|a| a.starts_with(":ref") && refname(l.get(5).unwrap()).map(|r| r == dest).unwrap_or(false)).unwrap_or(false);
            }
            (kernels::run_any(&c, tag, &args, n, (self.w, self.h), &kpe)?, tag)
        };
        if dest_ch != 0 && img.channels() as i32 != dest_ch {
            self.out.trace.push(format!("note: {} wrote {} channels into a {} channel temp", c, img.channels(), dest_ch));
        }
        self.out.trace.push(format!("{} {:?} -> tag {}", c, operands, tag));
        match img {
            crate::kernels2::Out::Img(i) => self.refs.insert(dest, Val::Img(Rc::new(i))),
            crate::kernels2::Out::Mip(m) => self.refs.insert(dest, Val::Mip(Rc::new(m))),
        };
        Ok(())
    }
}

/// Runs the printed commands in order. `inputs[i]` is the source image of `convert-from-source i`.
pub fn execute_commands(cmds: &[String], inputs: &[Image], width: usize, height: usize, host: &HostFrames, kp: &KParams) -> Result<CmdOutput, ExecError> {
    execute_commands_xy(cmds, inputs, width, height, host, kp, None)
}

/// As `execute_commands`, with the caller's `xy` plane for the `(variable 2 :ref)` command (None: the host-geometry grid, NV9-A3).
pub fn execute_commands_xy(cmds: &[String], inputs: &[Image], width: usize, height: usize, host: &HostFrames, kp: &KParams, xy: Option<&Image>) -> Result<CmdOutput, ExecError> {
    execute_commands_xy_hc(cmds, inputs, width, height, host, kp, xy, None)
}

/// as `execute_commands_xy`, with the native convert pair wired. With `hc` given, `convert-from-source i` reads HOST MEMORY `hc.src[i]` (format 16)
/// through the native reader and `convert-to-dest` writes HOST MEMORY (`CmdOutput::host_dest`) through the native writer in `hc.mode`.
pub fn execute_commands_hc(cmds: &[String], hc: &crate::hostconv::HostConvert, width: usize, height: usize, host: &HostFrames, kp: &KParams) -> Result<CmdOutput, ExecError> {
    hc.check_claim()?;
    execute_commands_xy_hc(cmds, &[], width, height, host, kp, None, Some(hc))
}

#[allow(clippy::too_many_arguments)]
pub fn execute_commands_xy_hc(cmds: &[String], inputs: &[Image], width: usize, height: usize, host: &HostFrames, kp: &KParams, xy: Option<&Image>, hc: Option<&crate::hostconv::HostConvert>) -> Result<CmdOutput, ExecError> {
    let n = width.checked_mul(height).ok_or(ExecError::Shape("frame size overflow".into()))?;
    let mut st = St {
        refs: BTreeMap::new(),
        inputs,
        n,
        w: width,
        h: height,
        xy,
        host,
        kp,
        hc,
        tables: Tables::embedded(),
        out: CmdOutput { dest: None, host_dest: None, host_wrote: false, ae32_dest: None, saved_prev_dest: None, trace: Vec::new() },
    };
    for c in cmds {
        let sx = parse_sx(c)?;
        st.run(&sx)?;
    }
    Ok(st.out)
}
