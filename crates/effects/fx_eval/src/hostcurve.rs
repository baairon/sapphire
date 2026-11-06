//! TMR1 / INT20 step T: get_curve_val through tickmap::map_param on the claim route (W334 (e) 2, W479).
//! The host's checkout answers come from a HOST-CURVE-VALUES input, a text file:
//!   `host <field> <int>`           field in effect_ref, time_mult, time_scale, double_flag, global, selector (host record fields)
//!   time_frames ([pinfo]) is not in the file: fx_run fills it from --host-frame ([ctx], INFERRED to be the same field)
//!   `value <param> <index> <tick> <step> <scale> <status> <raw>` one checkout answer per key; status 0 = success; raw = the int32 at pd
//!   `point <param> <field> <value>`  the recorded point fields of one parameter (step PW); every field in POINT_FIELDS is required, none is defaulted
//! The parameter record fields are defined in effects_rtdata (styles/<style>.rs, field `tickmap`; BR4 MEASURED); a parameter without a row is refused by name.
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default)]
pub struct HostCurveValues {
    pub host: BTreeMap<String, i64>,
    pub time_frames: Option<f32>,
    /// the file path as given (for refusal texts)
    pub source: String,
    /// set by fx_run: the claim ('ae32') route, where the global is HOST-APP's 1
    pub ae32: bool,
    pub values: BTreeMap<(String, i32, i32, i32, i32), (i32, i32)>,
    /// step PW: parameter -> field -> value text, from `point` lines
    pub points: BTreeMap<String, BTreeMap<String, String>>,
    /// step TPU (W662 (c) 1): parameter -> field -> hex text, from `unit` lines (REC_UNIT_KEY, x3 32-bit; d4, d5 64-bit; all four required)
    pub units: BTreeMap<String, BTreeMap<String, String>>,
}

/// Fields a `unit` entry must carry (step TPU, W662 (c) 1).
pub const UNIT_FIELDS: [&str; 4] = [crate::judge_keys::REC_UNIT_KEY, "x3", "d4", "d5"];

/// Fields a `point` record must carry (list is `none` or the host object type; prelude_status and prelude_type are also required when list is none).
pub const POINT_FIELDS: [&str; 20] = ["list", "gid", "selector", "rtype", "size_x", "size_y", crate::judge_keys::REC_POINT_KEY_A, crate::judge_keys::REC_POINT_KEY_B, crate::judge_keys::REC_POINT_KEY_C, crate::judge_keys::REC_POINT_KEY_D, "off_a", "off_b", "scaled", "cur_time", "time_step", "time_scale", "px", "py", "d4", "d5"];
pub const POINT_UNIT_FIELDS: [&str; 2] = [crate::judge_keys::REC_UNIT_KEY, "x3"];

impl HostCurveValues {
    pub fn parse(text: &str) -> Result<HostCurveValues, String> {
        let mut h = HostCurveValues::default();
        for (ln, l) in text.lines().enumerate() {
            let f: Vec<&str> = l.split_whitespace().collect();
            if f.is_empty() || f[0].starts_with('#') {
                continue;
            }
            let bad = || format!("HOST-CURVE-VALUES line {}: unreadable", ln + 1);
            match f[0] {
                "host" if f.len() == 3 => {
                    h.host.insert(f[1].to_string(), f[2].parse::<i64>().map_err(|_| bad())?);
                }
                "value" if f.len() == 8 => {
                    let p = |i: usize| f[i].parse::<i32>().map_err(|_| bad());
                    h.values.insert((f[1].to_string(), p(2)?, p(3)?, p(4)?, p(5)?), (p(6)?, p(7)?));
                }
                "point" if f.len() == 4 => {
                    h.points.entry(f[1].to_string()).or_default().insert(f[2].to_string(), f[3].to_string());
                }
                "unit" if f.len() == 4 => {
                    h.units.entry(f[1].to_string()).or_default().insert(f[2].to_string(), f[3].to_string());
                }
                _ => return Err(bad()),
            }
        }
        Ok(h)
    }
}

struct PtHost {
    px: i32,
    py: i32,
    status: i32,
    otype: i32,
}
impl tickmap::PointHost for PtHost {
    fn checkout_point(&mut self, _r: u64, _i: i32, _c: i32, _s: i32, _sc: i32) -> (i32, i32) {
        (self.px, self.py)
    }
    fn checkin(&mut self, _r: u64) {}
    fn checkout_type(&mut self, _r: u64, _i: i32, _c: i32, _s: i32, _sc: i32) -> (i32, i32) {
        (self.status, self.otype)
    }
}

struct HcvHost<'a> {
    prog: &'a str,
    pname: &'a str,
    hcv: &'a HostCurveValues,
    missing: Vec<String>,
    trace: Vec<String>,
}

impl<'a> tickmap::Host for HcvHost<'a> {
    fn checkout(&mut self, r: u64, index: i32, tick: i32, step: i32, scale: i32) -> (i32, i32) {
        self.trace.push(format!("HCV-CHECKOUT\t{}\t{}\t{}\t{}\t{}\t{}", self.pname, r, index, tick, step, scale));
        match self.hcv.values.get(&(self.pname.to_string(), index, tick, step, scale)) {
            Some(&(st, raw)) => (st, raw),
            None => {
                self.missing.push(format!("HOST-CURVE-VALUE-MISSING ({}: index {}, tick {}, step {}, scale {})", self.prog, index, tick, step, scale));
                (1, 0)
            }
        }
    }
    fn checkin(&mut self, _r: u64) {}
    fn fetch_no_time(&mut self, _a: i32, _i: i32, _c: u64, _o: &mut [u8; 41]) {}
}

/// The claim-route fold of `get_curve_val p t`: `cur` = the lanes of argument 0 (the node cells), `n` = their count. Returns the folded lanes, or pushes refusals.
pub fn fold(prog: &str, pname: &str, t: f32, cur: &[f32], n: usize, hcv: &HostCurveValues, diags: &mut Vec<String>) -> Option<Vec<f32>> {
    fold_with_row(record_of(prog, pname), prog, pname, t, cur, n, hcv, diags)
}

/// The nine fields of a parameter record, as `[ptype, nlanes, kind_byte, subtype_byte, flags_a_byte, flags_b_byte, index]`.
pub type RecFields = [i64; 7];

/// The defined parameter record of PROG's parameter PNAME.
pub fn record_of(prog: &str, pname: &str) -> Option<effects_rtdata::TickmapRecord> {
    effects_rtdata::tickmap_record(prog, pname).copied()
}

fn fields_of(r: &effects_rtdata::TickmapRecord) -> RecFields {
    [r.ptype, r.nlanes, r.kind_byte, r.subtype_byte, r.flags_a_byte, r.flags_b_byte, r.index]
}

/// Step TPR (W645 (a) 1), tests: the row of a record table given as TSV text, by program and parameter (the row's nine fields as text).
pub fn table_row<'a>(table: &'a str, prog: &str, pname: &str) -> Option<Vec<&'a str>> {
    table.lines().skip(1).map(|l| l.split('\t').collect::<Vec<_>>()).find(|f| f.len() == 9 && f[0] == prog && f[1] == pname)
}

/// Step TPR (W645 (a) 3), tests: `fold` with the parameter record table given as TSV text; `fold` uses the defined records, so the claim path is
/// unchanged. Tests use it to keep the index-check refusal on a constructed table.
pub fn fold_with_table(table: &str, prog: &str, pname: &str, t: f32, cur: &[f32], n: usize, hcv: &HostCurveValues, diags: &mut Vec<String>) -> Option<Vec<f32>> {
    let row = table_row(table, prog, pname).map(|f| {
        let g = |i: usize| f[i + 1].parse::<i64>().unwrap_or(0);
        [g(1), g(2), g(3), g(4), g(5), g(6), g(7)]
    });
    fold_with_row(row.map(|r| effects_rtdata::TickmapRecord { program: "", param: "", ptype: r[0], nlanes: r[1], kind_byte: r[2], subtype_byte: r[3], flags_a_byte: r[4], flags_b_byte: r[5], index: r[6] }), prog, pname, t, cur, n, hcv, diags)
}

/// The fold with the parameter record given (None: the program has no record for the parameter).
pub fn fold_with_row(row: Option<effects_rtdata::TickmapRecord>, prog: &str, pname: &str, t: f32, cur: &[f32], n: usize, hcv: &HostCurveValues, diags: &mut Vec<String>) -> Option<Vec<f32>> {
    let refuse = |d: &mut Vec<String>, m: String| {
        d.push(m);
        None::<Vec<f32>>
    };
    // parameter record
    let f = match row.as_ref().map(fields_of) {
        Some(f) => f,
        None => return refuse(diags, format!("TICKMAP-HOST-FIELD-UNSOURCED (parameter record of {}: {}: ptype, nlanes, kind_byte, subtype_byte, flags_a_byte, flags_b_byte, index)", prog, pname)),
    };
    let g = |i: usize| f[i - 1];
    if g(2) as usize != n {
        return refuse(diags, format!("TICKMAP-HOST-FIELD-UNSOURCED (nlanes: record {} lanes, program node {} lanes for {})", g(2), n, pname));
    }
    let rec = tickmap::ParamRec { ptype: g(1) as i32, nlanes: g(2) as u8, kind_byte: g(3) as u8, subtype_byte: g(4) as u8, flags_a_byte: g(5) as u8, flags_b_byte: g(6) as u8, index: g(7) as i16, rec_unit_value: f32::NAN };
    // INT32 step DF (W553 (a), W582 (g), W608 (h)): the table index is the key sent to the host's checkout; for a claimed style it must equal
    // the parameter's registered host index in STYLE_PARAM_INDEX.tsv, else the line refuses by name.
    if let Some(sr) = crate::units::style_param_row(prog, pname) {
        if sr.host_index != rec.index as i32 {
            return refuse(diags, format!("TICKMAP-INDEX-NOT-REGISTERED ({} {}: table index {}, the style's registered index {}; W553)", prog, pname, rec.index, sr.host_index));
        }
    }
    // INT30 TICKMAP step (TMP1, Batch 69 rule 1): nibble 5 reads [rec] and the pinfo unit fields, which the parameter record table
    // does not carry; the line refuses by name instead of reading a constructed value (the NaN placeholders are never read).
    let pt = hcv.points.get(pname);
    // step TPU (W662 (c) 2): a generic-path unit entry; never together with a point entry, and it never makes a point context
    let un = hcv.units.get(pname);
    if un.is_some() && pt.is_some() {
        return refuse(diags, format!("TICKMAP-UNIT-AND-POINT ({}: {}: a generic-path unit entry and a point entry; W662)", prog, pname));
    }
    if rec.subtype_byte & 0xf == 5 && pt.is_none() && un.is_none() {
        return refuse(diags, format!("TICKMAP-HOST-FIELD-UNSOURCED (unit fields of {}: {}: rec_unit_value, unit_x3, unit_d4, unit_d5 are not recorded; TMP1)", prog, pname));
    }
    let tf = match hcv.time_frames {
        Some(v) => v,
        None => return refuse(diags, "TICKMAP-HOST-FIELD-UNSOURCED (time_frames)".to_string()),
    };
    let need = |k: &str| hcv.host.get(k).copied();
    // W490 (e) 2: the global is HOST-APP's cell (1 in the claim state). On the claim route fx_run supplies 1 and refuses a differing file line; on a state A route the file line (TICKMAP's E1 re-score: 0) is used.
    let global: i64 = match (hcv.ae32, hcv.host.get("global").copied()) {
        (true, None) => 1,
        (true, Some(1)) => 1,
        (true, Some(v)) => return refuse(diags, format!("TICKMAP-GLOBAL-NOT-HOST-APP ({}: global {})", hcv.source, v)),
        (false, Some(v)) => v,
        (false, None) => return refuse(diags, "TICKMAP-HOST-FIELD-UNSOURCED (global)".to_string()),
    };
    for k in ["effect_ref", "time_mult", "time_scale", "double_flag"] {
        if need(k).is_none() {
            return refuse(diags, format!("TICKMAP-HOST-FIELD-UNSOURCED ({})", k));
        }
    }
    if hcv.host.get("selector").copied().unwrap_or(0) != 0 {
        return refuse(diags, format!("ROUTE-SELECTOR-NOT-CLAIMED ({})", prog));
    }
    let pinfo = tickmap::Pinfo { time_frames: tf, selector: 0, unit_x3: f32::NAN, unit_d4: f64::NAN, unit_d5: f64::NAN };
    let mut hr = tickmap::HostRec { effect_ref: need("effect_ref").unwrap() as u64, time_mult: need("time_mult").unwrap() as i32, time_scale: need("time_scale").unwrap() as i32, double_flag: need("double_flag").unwrap() as i32, size_x: 0, size_y: 0 };
    let mut pinfo = pinfo;
    let mut rec = rec;
    if let Some(u) = un {
        for k in UNIT_FIELDS {
            if !u.contains_key(k) {
                return refuse(diags, format!("TICKMAP-HOST-FIELD-UNSOURCED (unit field {} of {}: {}; W662)", k, prog, pname));
            }
        }
        let hx = |k: &str| u64::from_str_radix(u[k].trim_start_matches("0x"), 16).map_err(|_| format!("TICKMAP-HOST-FIELD-UNSOURCED (unit field {} of {}: {} is unreadable; W662)", k, prog, pname));
        let (r6, x3, d4, d5) = match (hx(crate::judge_keys::REC_UNIT_KEY), hx("x3"), hx("d4"), hx("d5")) {
            (Ok(a), Ok(b), Ok(c), Ok(d)) => (a, b, c, d),
            (Err(e), _, _, _) | (_, Err(e), _, _) | (_, _, Err(e), _) | (_, _, _, Err(e)) => return refuse(diags, e),
        };
        for k in ["size_x", "size_y"] {
            if need(k).is_none() {
                return refuse(diags, format!("TICKMAP-HOST-FIELD-UNSOURCED ({})", k));
            }
        }
        rec.rec_unit_value = f32::from_bits(r6 as u32);
        pinfo.unit_x3 = f32::from_bits(x3 as u32);
        pinfo.unit_d4 = f64::from_bits(d4);
        pinfo.unit_d5 = f64::from_bits(d5);
        hr.size_x = need("size_x").unwrap() as i32;
        hr.size_y = need("size_y").unwrap() as i32;
    }
    let mut pthost = PtHost { px: 0, py: 0, status: 0, otype: 0 };
    let mut phr: Option<tickmap::PointHostRec> = None;
    let mut ptype_obj: Option<i32> = None;
    if let Some(m) = pt {
        let miss = |f: &str| format!("TICKMAP-HOST-FIELD-UNSOURCED (point field {} of {}: {}; step PW)", f, prog, pname);
        let mut req: Vec<&str> = POINT_FIELDS.iter().chain(POINT_UNIT_FIELDS.iter()).copied().collect();
        if m.get("list").map(|v| v == "none").unwrap_or(false) {
            req.push("prelude_status");
            req.push("prelude_type");
        }
        for k in req {
            if !m.contains_key(k) {
                return refuse(diags, miss(k));
            }
        }
        let iv = |k: &str| -> Result<i64, String> { m[k].parse::<i64>().map_err(|_| format!("TICKMAP-HOST-FIELD-UNSOURCED (point field {} of {}: {} is unreadable; step PW)", k, prog, pname)) };
        let hx = |k: &str| -> Result<u64, String> { u64::from_str_radix(m[k].trim_start_matches("0x"), 16).map_err(|_| format!("TICKMAP-HOST-FIELD-UNSOURCED (point field {} of {}: {} is unreadable; step PW)", k, prog, pname)) };
        macro_rules! t { ($e:expr) => { match $e { Ok(v) => v, Err(e) => return refuse(diags, e) } }; }
        if t!(iv("gid")) != global {
            return refuse(diags, format!("TICKMAP-HOST-FIELD-UNSOURCED (point field gid of {}: {} differs from the run's global {}; step PW)", prog, pname, global));
        }
        if t!(iv("rtype")) != rec.ptype as i64 {
            return refuse(diags, format!("TICKMAP-HOST-FIELD-UNSOURCED (point field rtype of {}: {} differs from the parameter record; step PW)", prog, pname));
        }
        pinfo.selector = t!(iv("selector")) as i16;
        pinfo.unit_x3 = f32::from_bits(t!(hx("x3")) as u32);
        pinfo.unit_d4 = f64::from_bits(t!(hx("d4")));
        pinfo.unit_d5 = f64::from_bits(t!(hx("d5")));
        rec.rec_unit_value = f32::from_bits(t!(hx(crate::judge_keys::REC_UNIT_KEY)) as u32);
        phr = Some(tickmap::PointHostRec {
            effect_ref: need("effect_ref").unwrap() as u64, cur_time: t!(iv("cur_time")) as i32, time_step: t!(iv("time_step")) as i32, time_scale: t!(iv("time_scale")) as i32,
            size_x: t!(iv("size_x")) as i32, size_y: t!(iv("size_y")) as i32, off_a: t!(iv("off_a")) as i32, off_b: t!(iv("off_b")) as i32, scaled: t!(iv("scaled")) != 0,
            hcv_host_a: t!(iv(crate::judge_keys::REC_POINT_KEY_A)) as i32, hcv_host_b: t!(iv(crate::judge_keys::REC_POINT_KEY_B)) as i32, hcv_host_c: t!(iv(crate::judge_keys::REC_POINT_KEY_C)) as i32, hcv_host_d: t!(iv(crate::judge_keys::REC_POINT_KEY_D)) as i32,
        });
        pthost.px = t!(iv("px")) as i32;
        pthost.py = t!(iv("py")) as i32;
        if m["list"] == "none" {
            pthost.status = t!(iv("prelude_status")) as i32;
            pthost.otype = t!(iv("prelude_type")) as i32;
        } else {
            ptype_obj = Some(t!(iv("list")) as i32);
        }
    }
    let mut dest = [0.0f32; 4];
    for i in 0..n.min(4) {
        dest[i] = cur.get(i).copied().unwrap_or(0.0);
    }
    let mut host = HcvHost { prog, pname, hcv, missing: Vec::new(), trace: Vec::new() };
    // step PW: the point context comes from the run's recorded `point` lines; without it the point refusal stays
    let ctx = phr.as_ref().map(|h| tickmap::PointCtx { h, obj_type: ptype_obj, host: &mut pthost });
    let r = tickmap::map_param(&pinfo, Some(&hr), None, global as i32, &rec, t, &mut host, &mut dest, ctx);
    diags.extend(host.trace.iter().cloned());
    if let Err(e) = r {
        let route = match e {
            tickmap::Error::ColourRoute => "colour".to_string(),
            tickmap::Error::PointRoute => "point".to_string(),
            tickmap::Error::UnitConversion => format!("unit nibble {}", rec.subtype_byte & 0xf),
            e @ (tickmap::Error::PointRecType | tickmap::Error::PointNoTime | tickmap::Error::PointNotRouted | tickmap::Error::N4GenericUnrecorded | tickmap::Error::PointPrelude(_)) => {
                return refuse(diags, tickmap::refusal_text(&e, prog, pname));
            }
            other => format!("{:?}", other),
        };
        return refuse(diags, format!("TICKMAP-ROUTE-UNPORTED ({}: {}: {})", prog, pname, route));
    }
    if let Some(m) = host.missing.first() {
        return refuse(diags, m.clone());
    }
    Some(dest[..n.min(4)].to_vec())
}
