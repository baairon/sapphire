//! Native builders of the programs the registered styles reach: the same `Corpus` structures `Corpus::from_programs_tsv` produces, made through
//! a small constructor API instead of parsing program text. One function per program row (def_effect, defun, layout_effect).
//!
//! The text parser stays as code (`parse`, `Corpus::from_programs_tsv`); no product path reads a programs table. The builders are written
//! in the arena order the parser uses (parameter defaults first, then the body in post order), so the structures are equal node for node.

use crate::corpus::Corpus;
use crate::layoutmerge::{read_layout_sx, LayoutInfo, Sx};
use crate::parse::{flag_info, Arena, Bind, DefaultVal, DefunDef, EffectDef, InputDef, PKind, Param};

/// A program expression before it is placed in the arena.
pub enum X {
    Num(f32),
    Vec(&'static [f32]),
    Str(&'static str),
    Id(&'static str),
    Call(&'static str, Vec<X>),
    Let(Vec<(&'static [&'static str], X)>, Box<X>),
}

/// A parameter default.
pub enum D {
    N(f32),
    V(&'static [f32]),
    S(&'static str),
    W(&'static str),
}

fn n(v: f32) -> X { X::Num(v) }
fn v(l: &'static [f32]) -> X { X::Vec(l) }
fn s(t: &'static str) -> X { X::Str(t) }
fn id(t: &'static str) -> X { X::Id(t) }
fn call(h: &'static str, args: Vec<X>) -> X { X::Call(h, args) }
fn lt(binds: Vec<(&'static [&'static str], X)>, result: X) -> X { X::Let(binds, Box::new(result)) }
fn a(t: &str) -> Sx { Sx::Atom(t.to_string()) }
fn l(v: Vec<Sx>) -> Sx { Sx::List(v) }
fn strv(v: &[&str]) -> Vec<String> { v.iter().map(|x| x.to_string()).collect() }
fn inp(name: &str, default: Option<&str>) -> InputDef { InputDef { name: name.to_string(), default: default.map(|d| d.to_string()) } }

/// Place an expression in the arena: children first, then the node itself; a let places its bindings, its result, then itself.
fn lower(ar: &mut Arena, e: &X) -> usize {
    match e {
        X::Num(x) => ar.push(PKind::Scalar(*x as f64)),
        X::Vec(x) => ar.push(PKind::Vector(x.iter().map(|y| *y as f64).collect())),
        X::Str(t) => ar.push(PKind::Str(t.to_string())),
        X::Id(t) => ar.push(PKind::Ident(t.to_string())),
        X::Call(h, args) => {
            let ids: Vec<usize> = args.iter().map(|g| lower(ar, g)).collect();
            ar.push(PKind::Call { head: h.to_string(), args: ids })
        }
        X::Let(binds, res) => {
            let mut bs: Vec<Bind> = Vec::new();
            for (names, rhs) in binds {
                let r = lower(ar, rhs);
                bs.push(Bind { names: names.iter().map(|x| x.to_string()).collect(), rhs: r });
            }
            let result = lower(ar, res);
            let defined: Vec<&String> = bs.iter().flat_map(|b| b.names.iter()).collect();
            let mut all = Vec::new();
            for b in &bs {
                ar.refs(b.rhs, &mut all);
            }
            ar.refs(result, &mut all);
            let nonlocal: Vec<String> = all.into_iter().filter(|r| !defined.iter().any(|d| *d == r)).collect();
            ar.push(PKind::Let { binds: bs, result, nonlocal })
        }
    }
}

/// One parameter: its default node goes in the arena, the flag record is derived from the flag tokens.
fn param(ar: &mut Arena, name: &str, d: D, flags: &[&str]) -> Param {
    let (default, rhs) = match d {
        D::N(x) => (DefaultVal::Scalar(x as f64), ar.push(PKind::Scalar(x as f64))),
        D::S(t) => (DefaultVal::Str(t.to_string()), ar.push(PKind::Str(t.to_string()))),
        D::W(t) => (DefaultVal::Word(t.to_string()), ar.push(PKind::Ident(t.to_string()))),
        D::V(x) => {
            let w: Vec<f64> = x.iter().map(|y| *y as f64).collect();
            (DefaultVal::Vector(w.clone()), ar.push(PKind::Vector(w)))
        }
    };
    let flags = strv(flags);
    let info = flag_info(&flags);
    Param { name: name.to_string(), default, flags, info, rhs }
}

impl Corpus {
    /// The corpus of the programs the registered styles reach (def_effect, defun and layout rows), built natively. Same bookkeeping as `add_text`:
    /// the last definition of a name wins.
    pub fn native() -> Corpus {
        let mut c = Corpus::new();
        let effs = effects(&mut c.arena);
        for e in effs {
            let i = c.effects.len();
            if c.effect_idx.insert(e.name.clone(), i).is_some() {
                c.dup_effects.push(e.name.clone());
            }
            c.effects.push(e);
        }
        let defs = defuns(&mut c.arena);
        for d in defs {
            let i = c.defuns.len();
            if c.defun_idx.insert(d.name.clone(), i).is_some() {
                c.dup_defuns.push(d.name.clone());
            }
            c.defuns.push(d);
        }
        for row in layouts() {
            c.layout_rows += 1;
            c.layout_sx.push(row);
        }
        c
    }
}

/// The registered program names the native corpus holds (the `_Autogen` programs and Drop_Shadow): the last header name of each layout row.
pub fn registered_names() -> Vec<String> {
    layouts().iter().filter_map(|r| crate::layoutmerge::header_names(r).last().cloned()).collect()
}

/// Is `name` one of the registered programs of the native corpus?
pub fn has_program(name: &str) -> bool {
    registered_names().iter().any(|x| x == name)
}

/// The native layout row for `prog`, read by the layout reader (None when the native corpus has no row for it).
pub fn layout_info(prog: &str) -> Result<Option<LayoutInfo>, String> {
    let rows: Vec<Vec<Sx>> = layouts().into_iter().filter(|r| crate::layoutmerge::header_names(r).last().map(|x| x == prog).unwrap_or(false)).collect();
    match rows.len() {
        0 => Ok(None),
        1 => read_layout_sx(rows.into_iter().next().unwrap()).map(Some).map_err(|e| format!("LAYOUT-ROW-UNREADABLE {}: {}", prog, e)),
        k => Err(format!("LAYOUT-ROW-AMBIGUOUS: {} layout rows are registered for {}", k, prog)),
    }
}

fn eff_blur_color(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "blur_amount", D::N(0.4), &[":blur_width", "auto_anim_floor0", "widget", "ellipse.width", "wenable", "21"]),
        param(a, "blur_rel", D::V(&[1.0, 1.0]), &["widget", "ellipse.rel_xy"]),
        param(a, "brightness", D::N(1.0), &[]),
        param(a, "scale_opacity", D::N(1.0), &["slider_max", "1"]),
        param(a, "offset_darks", D::N(0.0), &["min", "*", "slider_range", "-5", "1", "big_slider_range", "-8", "2"]),
        param(a, "mix_with_source", D::N(0.0), &["max", "1"]),
        param(a, "filter", D::N(2.0), &["popup", "Box", "Triangle", "Gauss"]),
        param(a, "subpixel", D::N(1.0), &["boolean"]),
        param(a, "invert_mask", D::N(0.0), &["show_with_mask", "boolean"]),
        param(a, "mask_use", D::N(0.0), &["show_with_mask", "popup", "Luma", "Alpha"]),
    ];
    let body = lower(a, &lt(vec![
        (&["m"], call("if+", vec![
            id("mask_use"),
            call("coerce_to_alpha", vec![id("Mask")]),
            call("monochrome", vec![id("Mask")]),
        ])),
        (&["mask"], call("if+", vec![id("invert_mask"), call("one_minus", vec![id("m")]), id("m")])),
        (&["scale_rgba"], call("if_neq_const", vec![
            id("Mask"),
            n(1.0),
            call("v3_to_v4_if4", vec![
                call("SCALE_LIGHTS", vec![id("brightness"), id("offset_darks")]),
                id("scale_opacity"),
                id("Source"),
            ]),
            call("v3_to_v4", vec![
                call("SCALE_LIGHTS", vec![id("brightness"), id("offset_darks")]),
                id("scale_opacity"),
            ]),
        ])),
        (&["blurred"], call("if_neq_const", vec![
            id("Mask"),
            n(1.0),
            call("*", vec![
                id("scale_rgba"),
                call("masked_blur", vec![
                    id("Source"),
                    id("mask"),
                    call("*", vec![id("blur_amount"), id("blur_rel")]),
                    id("filter"),
                    id("subpixel"),
                ]),
            ]),
            call("blur", vec![
                id("Source"),
                call("*", vec![id("blur_amount"), id("blur_rel")]),
                id("filter"),
                id("scale_rgba"),
                id("subpixel"),
            ]),
        ])),
    ], call("interp", vec![
        call("add_rgb", vec![id("blurred"), id("offset_darks")]),
        id("Source"),
        id("mix_with_source"),
    ])));
    EffectDef { name: "Blur_Color".into(), va: "0x13b4c00".into(), inputs: vec![inp("Source", None), inp("Mask", Some("1"))], flags: strv(&["rgba_ok"]), params, body }
}

fn eff_blur_color_autogen(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "blur_amount", D::N(0.4), &["normalize_scale", "4", "slider_max", "2", "auto_anim_floor0", "widget", "ellipse.width", "wenable", "21"]),
        param(a, "blur_rel", D::V(&[1.0, 1.0]), &["widget", "ellipse.rel_xy"]),
        param(a, "brightness", D::N(1.0), &[]),
        param(a, "scale_opacity", D::N(1.0), &["slider_max", "1"]),
        param(a, "offset_darks", D::N(0.0), &["min", "*", "slider_range", "-5", "1", "big_slider_range", "-8", "2"]),
        param(a, "mix_with_source", D::N(0.0), &["max", "1"]),
        param(a, "filter", D::N(2.0), &["popup", "Box", "Triangle", "Gauss"]),
        param(a, "subpixel", D::N(1.0), &["boolean"]),
        param(a, "invert_mask", D::N(0.0), &["show_with_mask", "boolean"]),
        param(a, "mask_use", D::N(0.0), &["show_with_mask", "popup", "Luma", "Alpha"]),
        param(a, "mocha_project", D::N(0.0), &["optional", "mocha_data", "builder_disable", "shared"]),
        param(a, "blur_mocha_mask", D::N(0.0), &["optional", "disable_motion_blur", "normalize_scale", "4", "slider_max", "0.2", "builder_disable", "shared"]),
        param(a, "mocha_mask_opacity", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "1", "shared"]),
        param(a, "invert_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "resize_mocha_mask", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_x", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_y", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "shift_mocha_mask", D::V(&[0.0, 0.0]), &["optional", "disable_motion_blur", "builder_disable", "normalize_scale", "1", "min", "*", "slider_range", "-2", "2", "shared"]),
        param(a, "bypass_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "show_mocha_mask_only", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "combine_masks", D::N(0.0), &["optional", "popup", "Union", "Intersect", "Mocha_Only", "builder_disable", "shared"]),
    ];
    let body = lower(a, &lt(vec![
        (&["blur_mask"], n(0.0)),
        (&["combined_mask", "mocha_mask"], call("PROCESS_MASKS", vec![
            id("Mask"),
            id("invert_mask"),
            id("blur_mask"),
            id("mask_use"),
            id("mocha_project"),
            id("bypass_mocha_mask"),
            id("invert_mocha_mask"),
            id("blur_mocha_mask"),
            id("combine_masks"),
            id("mocha_mask_opacity"),
            id("resize_mocha_mask"),
            id("resize_mocha_rel_x"),
            id("resize_mocha_rel_y"),
            id("shift_mocha_mask"),
        ])),
        (&["result"], call("Blur_Color", vec![
            id("Source"),
            id("combined_mask"),
            id("blur_amount"),
            id("blur_rel"),
            id("brightness"),
            id("scale_opacity"),
            id("offset_darks"),
            id("mix_with_source"),
            id("filter"),
            id("subpixel"),
            n(0.0),
            n(0.0),
        ])),
    ], call("if+", vec![id("show_mocha_mask_only"), id("mocha_mask"), id("result")])));
    EffectDef { name: "Blur_Color_Autogen".into(), va: "0x13b5040".into(), inputs: vec![inp("Source", None), inp("Mask", Some("1"))], flags: strv(&["rgba_ok"]), params, body }
}

fn eff_blur_mono(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "blur_amount", D::N(0.4), &[":blur_width", "auto_anim_floor0", "widget", "ellipse.width", "wenable", "21"]),
        param(a, "blur_rel", D::V(&[1.0, 1.0]), &["widget", "ellipse.rel_xy"]),
        param(a, "brightness", D::N(1.0), &[]),
        param(a, "scale_opacity", D::N(1.0), &["slider_max", "1"]),
        param(a, "offset_darks", D::N(0.0), &["min", "*", "slider_range", "-5", "1", "big_slider_range", "-8", "2"]),
        param(a, "mix_with_source", D::N(0.0), &["max", "1"]),
        param(a, "filter", D::N(2.0), &["popup", "Box", "Triangle", "Gauss"]),
        param(a, "subpixel", D::N(1.0), &["boolean"]),
        param(a, "invert_mask", D::N(0.0), &["show_with_mask", "boolean"]),
        param(a, "mask_use", D::N(0.0), &["show_with_mask", "popup", "Luma", "Alpha"]),
    ];
    let body = lower(a, &lt(vec![
        (&["bw"], call("monochrome_a", vec![id("Source")])),
        (&["m"], call("if+", vec![
            id("mask_use"),
            call("coerce_to_alpha", vec![id("Mask")]),
            call("monochrome", vec![id("Mask")]),
        ])),
        (&["mask"], call("if+", vec![id("invert_mask"), call("one_minus", vec![id("m")]), id("m")])),
        (&["scale_luma_alpha"], call("if_neq_const", vec![
            id("Mask"),
            n(1.0),
            call("v3_to_v4_if4", vec![
                call("SCALE_LIGHTS", vec![id("brightness"), id("offset_darks")]),
                id("scale_opacity"),
                id("bw"),
            ]),
            call("v2", vec![
                call("SCALE_LIGHTS", vec![id("brightness"), id("offset_darks")]),
                id("scale_opacity"),
            ]),
        ])),
        (&["blurred"], call("if_neq_const", vec![
            id("Mask"),
            n(1.0),
            call("*", vec![
                id("scale_luma_alpha"),
                call("masked_blur", vec![
                    id("bw"),
                    id("mask"),
                    call("*", vec![id("blur_amount"), id("blur_rel")]),
                    id("filter"),
                    id("subpixel"),
                ]),
            ]),
            call("blur", vec![
                id("bw"),
                call("*", vec![id("blur_amount"), id("blur_rel")]),
                id("filter"),
                id("scale_luma_alpha"),
                id("subpixel"),
            ]),
        ])),
    ], call("interp", vec![
        call("add_rgb", vec![id("blurred"), id("offset_darks")]),
        id("bw"),
        id("mix_with_source"),
    ])));
    EffectDef { name: "Blur_Mono".into(), va: "0x13b65e0".into(), inputs: vec![inp("Source", None), inp("Mask", Some("1"))], flags: strv(&["rgba_ok"]), params, body }
}

fn eff_blur_mono_autogen(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "blur_amount", D::N(0.4), &["normalize_scale", "4", "slider_max", "2", "auto_anim_floor0", "widget", "ellipse.width", "wenable", "21"]),
        param(a, "blur_rel", D::V(&[1.0, 1.0]), &["widget", "ellipse.rel_xy"]),
        param(a, "brightness", D::N(1.0), &[]),
        param(a, "scale_opacity", D::N(1.0), &["slider_max", "1"]),
        param(a, "offset_darks", D::N(0.0), &["min", "*", "slider_range", "-5", "1", "big_slider_range", "-8", "2"]),
        param(a, "mix_with_source", D::N(0.0), &["max", "1"]),
        param(a, "filter", D::N(2.0), &["popup", "Box", "Triangle", "Gauss"]),
        param(a, "subpixel", D::N(1.0), &["boolean"]),
        param(a, "invert_mask", D::N(0.0), &["show_with_mask", "boolean"]),
        param(a, "mask_use", D::N(0.0), &["show_with_mask", "popup", "Luma", "Alpha"]),
        param(a, "mocha_project", D::N(0.0), &["optional", "mocha_data", "builder_disable", "shared"]),
        param(a, "blur_mocha_mask", D::N(0.0), &["optional", "disable_motion_blur", "normalize_scale", "4", "slider_max", "0.2", "builder_disable", "shared"]),
        param(a, "mocha_mask_opacity", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "1", "shared"]),
        param(a, "invert_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "resize_mocha_mask", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_x", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_y", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "shift_mocha_mask", D::V(&[0.0, 0.0]), &["optional", "disable_motion_blur", "builder_disable", "normalize_scale", "1", "min", "*", "slider_range", "-2", "2", "shared"]),
        param(a, "bypass_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "show_mocha_mask_only", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "combine_masks", D::N(0.0), &["optional", "popup", "Union", "Intersect", "Mocha_Only", "builder_disable", "shared"]),
    ];
    let body = lower(a, &lt(vec![
        (&["blur_mask"], n(0.0)),
        (&["combined_mask", "mocha_mask"], call("PROCESS_MASKS", vec![
            id("Mask"),
            id("invert_mask"),
            id("blur_mask"),
            id("mask_use"),
            id("mocha_project"),
            id("bypass_mocha_mask"),
            id("invert_mocha_mask"),
            id("blur_mocha_mask"),
            id("combine_masks"),
            id("mocha_mask_opacity"),
            id("resize_mocha_mask"),
            id("resize_mocha_rel_x"),
            id("resize_mocha_rel_y"),
            id("shift_mocha_mask"),
        ])),
        (&["result"], call("Blur_Mono", vec![
            id("Source"),
            id("combined_mask"),
            id("blur_amount"),
            id("blur_rel"),
            id("brightness"),
            id("scale_opacity"),
            id("offset_darks"),
            id("mix_with_source"),
            id("filter"),
            id("subpixel"),
            n(0.0),
            n(0.0),
        ])),
    ], call("if+", vec![id("show_mocha_mask_only"), id("mocha_mask"), id("result")])));
    EffectDef { name: "Blur_Mono_Autogen".into(), va: "0x13b6a20".into(), inputs: vec![inp("Source", None), inp("Mask", Some("1"))], flags: strv(&["rgba_ok"]), params, body }
}

fn eff_blur_motion_curves_no_transform(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "blur_res", D::N(0.0), &["popup", "Full", "Half", "Quarter"]),
        param(a, "center", D::V(&[0.0, 0.0]), &[]),
        param(a, "exposure_bias", D::N(0.5), &["max", "1"]),
        param(a, "rotate", D::N(0.0), &["angle", "widget", "rectshift.rotate", "instance", "2"]),
        param(a, "scale_result", D::N(1.0), &[]),
        param(a, "shift", D::V(&[0.0, 0.0]), &["min", "*", "slider_range", "-1", "1", "big_slider_range", "-2", "2", "force_slider"]),
        param(a, "shutter_duration", D::N(1.0), &["slider_max", "2"]),
        param(a, "shutter_shift", D::N(0.0), &["min", "*", "slider_range", "-1", "1"]),
        param(a, "subpixel", D::N(1.0), &["boolean"]),
        param(a, "wrap", D::V(&[0.0, 0.0]), &["popup", "No", "Tile", "Reflect"]),
        param(a, "z_dist", D::N(1.0), &["min", ".001", "slider_range", ".1", "5", "big_slider_range", ".01", "10", "widget", "rectshift.z_dist", "instance", "2"]),
    ];
    let body = lower(a, &lt(vec![
        (&["from_rotate", "from_z_dist", "from_shift", "to_rotate", "to_z_dist", "to_shift"], call("blur_motion_curves_from_to", vec![
            id("rotate"),
            id("shift"),
            id("shutter_duration"),
            id("shutter_shift"),
            id("z_dist"),
        ])),
        (&["xyc"], call("-", vec![id("xy"), id("center")])),
        (&["r0"], call("/", vec![id("from_rotate"), n(-360.0)])),
        (&["r1"], call("/", vec![id("to_rotate"), n(-360.0)])),
        (&["s0"], call("-", vec![id("center"), id("from_shift")])),
        (&["s1"], call("-", vec![id("center"), id("to_shift")])),
        (&["half_inv_z"], call("/", vec![n(0.5), id("z_dist")])),
        (&["dv"], call("rotate", vec![
            call("+", vec![
                call("-", vec![
                    call("*", vec![
                        call("rotate", vec![id("xyc"), id("r1"), v(&[0.0, 0.0])]),
                        call("*", vec![id("to_z_dist"), id("half_inv_z")]),
                    ]),
                    call("*", vec![
                        call("rotate", vec![id("xyc"), id("r0"), v(&[0.0, 0.0])]),
                        call("*", vec![id("from_z_dist"), id("half_inv_z")]),
                    ]),
                ]),
                call("*", vec![call("-", vec![id("s1"), id("s0")]), id("half_inv_z")]),
            ]),
            call("/", vec![id("rotate"), n(360.0)]),
            v(&[0.0, 0.0]),
        ])),
        (&["from_rel_bright"], call("one_minus", vec![id("exposure_bias")])),
        (&["to_rel_bright"], id("exposure_bias")),
    ], call("blur_warp", vec![
        call("make_mipmap", vec![id("Source"), id("wrap"), n(0.0)]),
        call("-", vec![id("xy"), id("dv")]),
        call("+", vec![id("xy"), id("dv")]),
        call("v2", vec![id("from_rel_bright"), id("to_rel_bright")]),
        id("scale_result"),
        id("wrap"),
        id("subpixel"),
        id("blur_res"),
    ])));
    EffectDef { name: "Blur_Motion_Curves_No_Transform".into(), va: "0x13b7fa0".into(), inputs: vec![inp("Source", None)], flags: strv(&["rgba_ok", "share_all", "warp_crop"]), params, body }
}

fn eff_blur_motion_curves_no_transform_autogen(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "blur_res", D::N(0.0), &["popup", "Full", "Half", "Quarter"]),
        param(a, "center", D::V(&[0.0, 0.0]), &[]),
        param(a, "exposure_bias", D::N(0.5), &["max", "1"]),
        param(a, "rotate", D::N(0.0), &["angle", "widget", "rectshift.rotate", "instance", "2"]),
        param(a, "scale_result", D::N(1.0), &[]),
        param(a, "shift", D::V(&[0.0, 0.0]), &["min", "*", "slider_range", "-1", "1", "big_slider_range", "-2", "2", "force_slider"]),
        param(a, "shutter_duration", D::N(1.0), &["slider_max", "2"]),
        param(a, "shutter_shift", D::N(0.0), &["min", "*", "slider_range", "-1", "1"]),
        param(a, "subpixel", D::N(1.0), &["boolean"]),
        param(a, "wrap", D::V(&[0.0, 0.0]), &["popup", "No", "Tile", "Reflect"]),
        param(a, "z_dist", D::N(1.0), &["min", ".001", "slider_range", ".1", "5", "big_slider_range", ".01", "10", "widget", "rectshift.z_dist", "instance", "2"]),
        param(a, "mocha_project", D::N(0.0), &["optional", "mocha_data", "builder_disable", "shared"]),
        param(a, "blur_mocha_mask", D::N(0.0), &["optional", "disable_motion_blur", "normalize_scale", "4", "slider_max", "0.2", "builder_disable", "shared"]),
        param(a, "mocha_mask_opacity", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "1", "shared"]),
        param(a, "invert_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "resize_mocha_mask", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_x", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_y", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "shift_mocha_mask", D::V(&[0.0, 0.0]), &["optional", "disable_motion_blur", "builder_disable", "normalize_scale", "1", "min", "*", "slider_range", "-2", "2", "shared"]),
        param(a, "bypass_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "show_mocha_mask_only", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "combine_masks", D::N(0.0), &["optional", "popup", "Union", "Intersect", "Mocha_Only", "builder_disable", "shared"]),
        param(a, "mask_use", D::N(0.0), &["show_with_mask", "optional", "shared", "popup", "Luma", "Alpha"]),
        param(a, "blur_mask", D::N(0.050000004), &["show_with_mask", "optional", "shared", "show_with_mask", "normalize_scale", "4", "slider_max", "2"]),
        param(a, "invert_mask", D::N(0.0), &["show_with_mask", "optional", "shared", "boolean"]),
    ];
    let body = lower(a, &lt(vec![
        (&["combined_mask", "mocha_mask"], call("PROCESS_MASKS", vec![
            id("Mask"),
            id("invert_mask"),
            id("blur_mask"),
            id("mask_use"),
            id("mocha_project"),
            id("bypass_mocha_mask"),
            id("invert_mocha_mask"),
            id("blur_mocha_mask"),
            id("combine_masks"),
            id("mocha_mask_opacity"),
            id("resize_mocha_mask"),
            id("resize_mocha_rel_x"),
            id("resize_mocha_rel_y"),
            id("shift_mocha_mask"),
        ])),
        (&["result"], call("Blur_Motion_Curves_No_Transform", vec![
            id("Source"),
            id("blur_res"),
            id("center"),
            id("exposure_bias"),
            id("rotate"),
            id("scale_result"),
            id("shift"),
            id("shutter_duration"),
            id("shutter_shift"),
            id("subpixel"),
            id("wrap"),
            id("z_dist"),
        ])),
    ], call("if+", vec![
        id("show_mocha_mask_only"),
        id("mocha_mask"),
        call("dissolve", vec![
            id("Source"),
            call("coerce_up", vec![id("result"), id("Source")]),
            id("combined_mask"),
        ]),
    ])));
    EffectDef { name: "Blur_Motion_Curves_No_Transform_Autogen".into(), va: "0x13b84e0".into(), inputs: vec![inp("Source", None), inp("Mask", Some("1"))], flags: strv(&["rgba_ok", "share_all", "warp_crop", "rgba_ok_for_input", "1"]), params, body }
}

fn eff_diffuse(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "blur_mask", D::N(0.0), &[":blur_mask"]),
        param(a, "diffuse_amount", D::N(0.2), &["normalize_scale", "2", "slider_max", "1", "big_slider_max", "1", "auto_anim_floor0"]),
        param(a, "invert_mask", D::N(0.0), &["show_with_mask", "boolean"]),
        param(a, "mask_use", D::N(0.0), &["show_with_mask", "popup", "Luma", "Alpha"]),
        param(a, "rel_amount", D::V(&[1.0, 1.0]), &[]),
        param(a, "wrap", D::V(&[2.0, 2.0]), &["popup", "No", "Tile", "Reflect"]),
    ];
    let body = lower(a, &lt(vec![
        (&["mask"], call("WARP_MASK", vec![id("Mask"), id("invert_mask"), id("blur_mask"), id("mask_use")])),
    ], call("warp_rel", vec![
        call("make_mipmap", vec![id("Source"), id("wrap"), n(0.0)]),
        call("*", vec![
            call("static2", vec![]),
            call("*", vec![call("*", vec![id("diffuse_amount"), id("rel_amount")]), id("mask")]),
        ]),
        id("wrap"),
        n(0.0),
    ])));
    EffectDef { name: "Diffuse".into(), va: "0x13d4b80".into(), inputs: vec![inp("Source", None), inp("Mask", Some("1"))], flags: strv(&["rgba_ok", "warp_crop"]), params, body }
}

fn eff_diffuse_autogen(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "blur_mask", D::N(0.0), &["show_with_mask", "normalize_scale", "4", "slider_max", "2"]),
        param(a, "diffuse_amount", D::N(0.2), &["normalize_scale", "2", "slider_max", "1", "big_slider_max", "1", "auto_anim_floor0"]),
        param(a, "invert_mask", D::N(0.0), &["show_with_mask", "boolean"]),
        param(a, "mask_use", D::N(0.0), &["show_with_mask", "popup", "Luma", "Alpha"]),
        param(a, "rel_amount", D::V(&[1.0, 1.0]), &[]),
        param(a, "wrap", D::V(&[2.0, 2.0]), &["popup", "No", "Tile", "Reflect"]),
        param(a, "mocha_project", D::N(0.0), &["optional", "mocha_data", "builder_disable", "shared"]),
        param(a, "blur_mocha_mask", D::N(0.0), &["optional", "disable_motion_blur", "normalize_scale", "4", "slider_max", "0.2", "builder_disable", "shared"]),
        param(a, "mocha_mask_opacity", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "1", "shared"]),
        param(a, "invert_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "resize_mocha_mask", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_x", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_y", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "shift_mocha_mask", D::V(&[0.0, 0.0]), &["optional", "disable_motion_blur", "builder_disable", "normalize_scale", "1", "min", "*", "slider_range", "-2", "2", "shared"]),
        param(a, "bypass_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "show_mocha_mask_only", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "combine_masks", D::N(0.0), &["optional", "popup", "Union", "Intersect", "Mocha_Only", "builder_disable", "shared"]),
    ];
    let body = lower(a, &lt(vec![
        (&["combined_mask", "mocha_mask"], call("PROCESS_MASKS", vec![
            id("Mask"),
            id("invert_mask"),
            id("blur_mask"),
            id("mask_use"),
            id("mocha_project"),
            id("bypass_mocha_mask"),
            id("invert_mocha_mask"),
            id("blur_mocha_mask"),
            id("combine_masks"),
            id("mocha_mask_opacity"),
            id("resize_mocha_mask"),
            id("resize_mocha_rel_x"),
            id("resize_mocha_rel_y"),
            id("shift_mocha_mask"),
        ])),
        (&["result"], call("Diffuse", vec![
            id("Source"),
            id("combined_mask"),
            n(0.0),
            id("diffuse_amount"),
            n(0.0),
            n(0.0),
            id("rel_amount"),
            id("wrap"),
        ])),
    ], call("if+", vec![id("show_mocha_mask_only"), id("mocha_mask"), id("result")])));
    EffectDef { name: "Diffuse_Autogen".into(), va: "0x13d4da0".into(), inputs: vec![inp("Source", None), inp("Mask", Some("1"))], flags: strv(&["rgba_ok", "warp_crop"]), params, body }
}

fn eff_distort_rgb(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "amount", D::N(1.0), &["min", "*", "slider_range", "-2", "2", "big_slider_range", "-5", "5", "auto_anim_floor0"]),
        param(a, "amount_rel", D::V(&[1.0, 1.0]), &[]),
        param(a, "blur_lens", D::N(0.4), &["normalize_scale", "4", "slider_max", "2"]),
        param(a, "blur_mask", D::N(0.0), &[":blur_mask"]),
        param(a, "filter", D::N(1.0), &["boolean"]),
        param(a, "invert_mask", D::N(0.0), &["show_with_mask", "boolean"]),
        param(a, "mask_use", D::N(0.0), &["show_with_mask", "popup", "Luma", "Alpha"]),
        param(a, "rotate_warp_dir", D::N(0.0), &["angle"]),
        param(a, "warp_alpha", D::N(0.75), &["min", "*", "step", ".0005"]),
        param(a, "warp_blue", D::N(1.0), &["min", "*", "slider_range", "-2", "2"]),
        param(a, "warp_green", D::N(0.75), &["min", "*", "slider_range", "-2", "2"]),
        param(a, "warp_red", D::N(0.5), &["min", "*", "slider_range", "-2", "2"]),
        param(a, "wrap", D::V(&[2.0, 2.0]), &["popup", "No", "Tile", "Reflect"]),
    ];
    let body = lower(a, &lt(vec![
        (&["mask"], call("WARP_MASK", vec![id("Mask"), id("invert_mask"), id("blur_mask"), id("mask_use")])),
        (&["scale"], call("*", vec![id("amount"), call("+", vec![id("blur_lens"), n(0.1)])])),
        (&["grad"], call("rotate", vec![
            call("*", vec![
                id("amount_rel"),
                call("grad", vec![call("monochrome", vec![id("Lens")]), id("blur_lens"), id("scale"), n(1.0)]),
            ]),
            call("/", vec![id("rotate_warp_dir"), n(360.0)]),
            v(&[0.0, 0.0]),
        ])),
        (&["coords"], call("*", vec![id("grad"), id("mask")])),
        (&["dest3"], call("v3", vec![
            call("warp_rel", vec![
                call("make_mipmap", vec![
                    call("element_of", vec![id("Source"), n(0.0)]),
                    id("wrap"),
                    id("filter"),
                ]),
                call("*", vec![id("coords"), id("warp_red")]),
                id("wrap"),
                id("filter"),
            ]),
            call("warp_rel", vec![
                call("make_mipmap", vec![
                    call("element_of", vec![id("Source"), n(1.0)]),
                    id("wrap"),
                    id("filter"),
                ]),
                call("*", vec![id("coords"), id("warp_green")]),
                id("wrap"),
                id("filter"),
            ]),
            call("warp_rel", vec![
                call("make_mipmap", vec![
                    call("element_of", vec![id("Source"), n(2.0)]),
                    id("wrap"),
                    id("filter"),
                ]),
                call("*", vec![id("coords"), id("warp_blue")]),
                id("wrap"),
                id("filter"),
            ]),
        ])),
        (&["alpha"], call("warp_rel", vec![
            call("make_mipmap", vec![
                call("element_of", vec![id("Source"), n(3.0)]),
                id("wrap"),
                id("filter"),
            ]),
            call("*", vec![id("coords"), id("warp_alpha")]),
            id("wrap"),
            id("filter"),
        ])),
    ], call("v3_to_v4_if4", vec![id("dest3"), id("alpha"), id("Source")])));
    EffectDef { name: "Distort_RGB".into(), va: "0x13e4aa0".into(), inputs: vec![inp("Source", None), inp("Lens", Some("Source")), inp("Mask", Some("1"))], flags: strv(&["generates_color", "warp_crop"]), params, body }
}

fn eff_distort_rgb_autogen(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "amount", D::N(1.0), &["min", "*", "slider_range", "-2", "2", "big_slider_range", "-5", "5", "auto_anim_floor0"]),
        param(a, "amount_rel", D::V(&[1.0, 1.0]), &[]),
        param(a, "blur_lens", D::N(0.4), &["normalize_scale", "4", "slider_max", "2"]),
        param(a, "blur_mask", D::N(0.0), &["show_with_mask", "normalize_scale", "4", "slider_max", "2"]),
        param(a, "filter", D::N(1.0), &["boolean"]),
        param(a, "invert_mask", D::N(0.0), &["show_with_mask", "boolean"]),
        param(a, "mask_use", D::N(0.0), &["show_with_mask", "popup", "Luma", "Alpha"]),
        param(a, "rotate_warp_dir", D::N(0.0), &["angle"]),
        param(a, "warp_alpha", D::N(0.75), &["min", "*", "step", ".0005"]),
        param(a, "warp_blue", D::N(1.0), &["min", "*", "slider_range", "-2", "2"]),
        param(a, "warp_green", D::N(0.75), &["min", "*", "slider_range", "-2", "2"]),
        param(a, "warp_red", D::N(0.5), &["min", "*", "slider_range", "-2", "2"]),
        param(a, "wrap", D::V(&[2.0, 2.0]), &["popup", "No", "Tile", "Reflect"]),
        param(a, "mocha_project", D::N(0.0), &["optional", "mocha_data", "builder_disable", "shared"]),
        param(a, "blur_mocha_mask", D::N(0.0), &["optional", "disable_motion_blur", "normalize_scale", "4", "slider_max", "0.2", "builder_disable", "shared"]),
        param(a, "mocha_mask_opacity", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "1", "shared"]),
        param(a, "invert_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "resize_mocha_mask", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_x", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_y", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "shift_mocha_mask", D::V(&[0.0, 0.0]), &["optional", "disable_motion_blur", "builder_disable", "normalize_scale", "1", "min", "*", "slider_range", "-2", "2", "shared"]),
        param(a, "bypass_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "show_mocha_mask_only", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "combine_masks", D::N(0.0), &["optional", "popup", "Union", "Intersect", "Mocha_Only", "builder_disable", "shared"]),
    ];
    let body = lower(a, &lt(vec![
        (&["combined_mask", "mocha_mask"], call("PROCESS_MASKS", vec![
            id("Mask"),
            id("invert_mask"),
            id("blur_mask"),
            id("mask_use"),
            id("mocha_project"),
            id("bypass_mocha_mask"),
            id("invert_mocha_mask"),
            id("blur_mocha_mask"),
            id("combine_masks"),
            id("mocha_mask_opacity"),
            id("resize_mocha_mask"),
            id("resize_mocha_rel_x"),
            id("resize_mocha_rel_y"),
            id("shift_mocha_mask"),
        ])),
        (&["result"], call("Distort_RGB", vec![
            id("Source"),
            id("Lens"),
            id("combined_mask"),
            id("amount"),
            id("amount_rel"),
            id("blur_lens"),
            n(0.0),
            id("filter"),
            n(0.0),
            n(0.0),
            id("rotate_warp_dir"),
            id("warp_alpha"),
            id("warp_blue"),
            id("warp_green"),
            id("warp_red"),
            id("wrap"),
        ])),
    ], call("if+", vec![id("show_mocha_mask_only"), id("mocha_mask"), id("result")])));
    EffectDef { name: "Distort_RGB_Autogen".into(), va: "0x13e4fe0".into(), inputs: vec![inp("Source", None), inp("Lens", Some("Source")), inp("Mask", Some("1"))], flags: strv(&["generates_color", "warp_crop"]), params, body }
}

fn eff_drop_shadow(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "shadow_blur", D::N(0.1), &["normalize_scale", "4", "slider_max", "2"]),
        param(a, "color", D::V(&[0.0, 0.0, 0.0]), &["color"]),
        param(a, "comp_premult", D::N(1.0), &["boolean"]),
        param(a, "fg_opacity", D::N(1.0), &[]),
        param(a, "invert_matte", D::N(0.0), &["boolean", "show_with_mask"]),
        param(a, "matte_use", D::N(1.0), &["popup", "Luma", "Alpha", "show_with_mask"]),
        param(a, "shadow_opacity", D::N(1.0), &["auto_anim_floor0"]),
        param(a, "shift", D::V(&[0.042000003, -0.042000003]), &[]),
        param(a, "mocha_project", D::N(0.0), &["optional", "mocha_data", "builder_disable", "shared"]),
        param(a, "blur_mocha_mask", D::N(0.0), &["optional", "disable_motion_blur", "normalize_scale", "4", "slider_max", "0.2", "builder_disable", "shared"]),
        param(a, "mocha_mask_opacity", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "1", "shared"]),
        param(a, "invert_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "resize_mocha_mask", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_x", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_y", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "shift_mocha_mask", D::V(&[0.0, 0.0]), &["optional", "disable_motion_blur", "builder_disable", "normalize_scale", "1", "min", "*", "slider_range", "-2", "2", "shared"]),
        param(a, "bypass_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "show_mocha_mask_only", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "combine_masks", D::N(0.0), &["optional", "popup", "Union", "Intersect", "Mocha_Only", "builder_disable", "shared"]),
    ];
    let body = lower(a, &lt(vec![
        (&["back_shadow"], call("if_neq_const", vec![
            id("Background"),
            n(0.0),
            call("dissolve", vec![id("Background"), id("color"), id("shadow")]),
            call("*", vec![call("v3_to_v4_if4", vec![id("color"), n(1.0), id("Output")]), id("shadow")]),
        ])),
        (&["mocha_mask"], call("MOCHA_MASK", vec![
            id("mocha_project"),
            id("bypass_mocha_mask"),
            id("invert_mocha_mask"),
            id("blur_mocha_mask"),
            id("mocha_mask_opacity"),
            id("resize_mocha_mask"),
            id("resize_mocha_rel_x"),
            id("resize_mocha_rel_y"),
            id("shift_mocha_mask"),
        ])),
        (&["a", "shadow"], call("drop_shadow_shadow", vec![
            id("Matte"),
            id("shadow_blur"),
            id("invert_matte"),
            id("matte_use"),
            id("shadow_opacity"),
            id("shift"),
            id("mocha_mask"),
            id("combine_masks"),
        ])),
        (&["result"], call("drop_shadow_combine", vec![
            id("Foreground"),
            id("a"),
            id("back_shadow"),
            id("comp_premult"),
            id("fg_opacity"),
        ])),
    ], call("if+", vec![id("show_mocha_mask_only"), id("mocha_mask"), id("result")])));
    EffectDef { name: "Drop_Shadow".into(), va: "0x13e65a0".into(), inputs: vec![inp("Foreground", None), inp("Background", Some("0")), inp("Matte", Some("Foreground"))], flags: strv(&["rgba_ok", "generates_color", "generates_alpha"]), params, body }
}

fn eff_hue_sat_bright(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "brightness", D::N(1.0), &[]),
        param(a, "hue_shift", D::N(0.0), &[]),
        param(a, "offset_darks", D::N(0.0), &[":offsetdarks"]),
        param(a, "preserve_luma", D::N(0.0), &["boolean"]),
        param(a, "saturation", D::N(1.0), &[":saturation"]),
        param(a, "scale_colors", D::V(&[1.0, 1.0, 1.0]), &["color"]),
    ];
    let body = lower(a, &lt(vec![
        (&["d0"], call("sat_scale", vec![
            call("if+", vec![
                id("preserve_luma"),
                call("hue_shift_preserve_luma", vec![id("Source"), id("hue_shift")]),
                call("hue_shift", vec![id("Source"), id("hue_shift")]),
            ]),
            id("saturation"),
        ])),
    ], call("SCALE_LIGHTS_OFFSET_DARKS", vec![
        id("d0"),
        call("*", vec![id("brightness"), id("scale_colors")]),
        id("offset_darks"),
        id("Source"),
    ])));
    EffectDef { name: "Hue_Sat_Bright".into(), va: "0x141fac0".into(), inputs: vec![inp("Source", None)], flags: strv(&["generates_color"]), params, body }
}

fn eff_hue_sat_bright_autogen(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "brightness", D::N(1.0), &[]),
        param(a, "hue_shift", D::N(0.0), &[]),
        param(a, "offset_darks", D::N(0.0), &["min", "-8", "max", "2", "slider_range", "-5", "1", "big_slider_range", "-8", "2"]),
        param(a, "preserve_luma", D::N(0.0), &["boolean"]),
        param(a, "saturation", D::N(1.0), &["min", "-2", "max", "8", "slider_range", "0", "2", "big_slider_range", "-2", "8"]),
        param(a, "scale_colors", D::V(&[1.0, 1.0, 1.0]), &["color"]),
        param(a, "mocha_project", D::N(0.0), &["optional", "mocha_data", "builder_disable", "shared"]),
        param(a, "blur_mocha_mask", D::N(0.0), &["optional", "disable_motion_blur", "normalize_scale", "4", "slider_max", "0.2", "builder_disable", "shared"]),
        param(a, "mocha_mask_opacity", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "1", "shared"]),
        param(a, "invert_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "resize_mocha_mask", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_x", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_y", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "shift_mocha_mask", D::V(&[0.0, 0.0]), &["optional", "disable_motion_blur", "builder_disable", "normalize_scale", "1", "min", "*", "slider_range", "-2", "2", "shared"]),
        param(a, "bypass_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "show_mocha_mask_only", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "combine_masks", D::N(0.0), &["optional", "popup", "Union", "Intersect", "Mocha_Only", "builder_disable", "shared"]),
        param(a, "mask_use", D::N(0.0), &["show_with_mask", "optional", "shared", "popup", "Luma", "Alpha"]),
        param(a, "blur_mask", D::N(0.050000004), &["show_with_mask", "optional", "shared", "show_with_mask", "normalize_scale", "4", "slider_max", "2"]),
        param(a, "invert_mask", D::N(0.0), &["show_with_mask", "optional", "shared", "boolean"]),
    ];
    let body = lower(a, &lt(vec![
        (&["combined_mask", "mocha_mask"], call("PROCESS_MASKS", vec![
            id("Mask"),
            id("invert_mask"),
            id("blur_mask"),
            id("mask_use"),
            id("mocha_project"),
            id("bypass_mocha_mask"),
            id("invert_mocha_mask"),
            id("blur_mocha_mask"),
            id("combine_masks"),
            id("mocha_mask_opacity"),
            id("resize_mocha_mask"),
            id("resize_mocha_rel_x"),
            id("resize_mocha_rel_y"),
            id("shift_mocha_mask"),
        ])),
        (&["result"], call("Hue_Sat_Bright", vec![
            id("Source"),
            id("brightness"),
            id("hue_shift"),
            id("offset_darks"),
            id("preserve_luma"),
            id("saturation"),
            id("scale_colors"),
        ])),
    ], call("if+", vec![
        id("show_mocha_mask_only"),
        id("mocha_mask"),
        call("dissolve", vec![
            id("Source"),
            call("coerce_up", vec![id("result"), id("Source")]),
            id("combined_mask"),
        ]),
    ])));
    EffectDef { name: "Hue_Sat_Bright_Autogen".into(), va: "0x141fc80".into(), inputs: vec![inp("Source", None), inp("Mask", Some("1"))], flags: strv(&["generates_color", "rgba_ok_for_input", "1"]), params, body }
}

fn eff_sharpen_best_quality(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "invert_mask", D::N(0.0), &["boolean", "show_with_mask", "shared", "button", "8"]),
        param(a, "edge_threshold", D::N(0.15), &["slider_max", "1", "button", "15"]),
        param(a, "small_detail_size", D::N(0.010000001), &["normalize_scale", "4", "slider_max", "0.1", "widget", "ellipse.width"]),
        param(a, "scale_tiny_details", D::N(3.0), &["slider_max", "5"]),
        param(a, "scale_small_details", D::N(1.5), &["slider_max", "5"]),
        param(a, "scale_medium_details", D::N(1.0), &["slider_max", "5"]),
        param(a, "scale_large_details", D::N(2.0), &["slider_max", "5"]),
        param(a, "sharpen_amp", D::N(1.0), &["button", "1", "min", "*", "slider_range", "0", "10", "shared", "auto_anim_floor0"]),
        param(a, "sharpen_luma", D::N(1.0), &["button", "4", "shared"]),
        param(a, "sharpen_chroma", D::N(1.0), &["shared"]),
        param(a, "sharpen_red", D::N(1.0), &["button", "11", "min", "*", "slider_range", "0", "5", "shared"]),
        param(a, "sharpen_green", D::N(1.0), &["min", "*", "slider_range", "0", "5", "shared"]),
        param(a, "sharpen_blue", D::N(1.0), &["min", "*", "slider_range", "0", "5", "shared"]),
        param(a, "show", D::N(0.0), &[]),
        param(a, "mask_use", D::N(0.0), &["show_with_mask", "optional", "shared", "popup", "Luma", "Alpha"]),
        param(a, "blur_mask", D::N(0.050000004), &["show_with_mask", "optional", "shared", "old_default", "v11", "0", ":blur_mask"]),
    ];
    let body = lower(a, &lt(vec![
        (&["m"], call("if+", vec![
            id("mask_use"),
            call("coerce_to_alpha", vec![id("Mask")]),
            call("monochrome", vec![id("Mask")]),
        ])),
        (&["mask"], call("blur", vec![
            call("if+", vec![id("invert_mask"), call("one_minus", vec![id("m")]), id("m")]),
            id("blur_mask"),
            n(3.0),
            n(1.0),
            n(0.0),
        ])),
        (&["blur_type"], call("abs", vec![n(4.0)])),
        (&["deriv_io"], call("v2", vec![n(0.0), n(0.0)])),
        (&["levels"], call("abs", vec![n(4.0)])),
        (&["show_1"], call("-", vec![id("show"), n(1.0)])),
        (&["sigmoid_scale"], call("v4", vec![
            id("scale_tiny_details"),
            id("scale_small_details"),
            id("scale_medium_details"),
            id("scale_large_details"),
        ])),
        (&["tiny_detail_size"], call("*", vec![n(0.5), id("small_detail_size")])),
        (&["rel_chan_luma"], call("*", vec![
            id("sharpen_luma"),
            call("*", vec![id("sharpen_amp"), call("v3", vec![n(1.0), n(1.0), n(1.0)])]),
        ])),
        (&["rel_chan_chroma"], call("*", vec![
            id("sharpen_chroma"),
            call("*", vec![id("sharpen_amp"), call("v3", vec![n(1.0), n(1.0), n(1.0)])]),
        ])),
        (&["rel_chan_rgb"], call("*", vec![
            id("sharpen_amp"),
            call("v3", vec![id("sharpen_red"), id("sharpen_green"), id("sharpen_blue")]),
        ])),
        (&["luma"], call("monochrome", vec![id("Source")])),
        (&["chroma"], call("-", vec![id("Source"), id("luma")])),
        (&["sharp_luma"], call("edge_aware_sharpen", vec![
            id("luma"),
            id("mask"),
            id("blur_type"),
            id("deriv_io"),
            id("levels"),
            id("tiny_detail_size"),
            id("edge_threshold"),
            id("sigmoid_scale"),
            id("rel_chan_luma"),
            id("show_1"),
        ])),
        (&["sharp_chroma"], call("edge_aware_sharpen", vec![
            id("chroma"),
            id("mask"),
            id("blur_type"),
            id("deriv_io"),
            id("levels"),
            id("tiny_detail_size"),
            id("edge_threshold"),
            id("sigmoid_scale"),
            id("rel_chan_chroma"),
            id("show_1"),
        ])),
        (&["sharp_all"], call("edge_aware_sharpen", vec![
            id("Source"),
            id("mask"),
            id("blur_type"),
            id("deriv_io"),
            id("levels"),
            id("tiny_detail_size"),
            id("edge_threshold"),
            id("sigmoid_scale"),
            id("rel_chan_rgb"),
            id("show_1"),
        ])),
    ], call("if_eq", vec![
        id("sharpen_luma"),
        id("sharpen_chroma"),
        id("sharp_all"),
        call("+", vec![id("sharp_luma"), id("sharp_chroma")]),
    ])));
    EffectDef { name: "Sharpen_Best_Quality".into(), va: "0x1476f20".into(), inputs: vec![inp("Source", None), inp("Mask", Some("1"))], flags: strv(&["no_alpha_only", "old_default_effect", "v7", "1", "no_tiled_rendering", "effect_popup_name", "Quality"]), params, body }
}

fn eff_sharpen_best_quality_autogen(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "invert_mask", D::N(0.0), &["boolean", "show_with_mask", "shared", "button", "8"]),
        param(a, "edge_threshold", D::N(0.15), &["slider_max", "1", "button", "15"]),
        param(a, "small_detail_size", D::N(0.010000001), &["normalize_scale", "4", "slider_max", "0.1", "widget", "ellipse.width"]),
        param(a, "scale_tiny_details", D::N(3.0), &["slider_max", "5"]),
        param(a, "scale_small_details", D::N(1.5), &["slider_max", "5"]),
        param(a, "scale_medium_details", D::N(1.0), &["slider_max", "5"]),
        param(a, "scale_large_details", D::N(2.0), &["slider_max", "5"]),
        param(a, "sharpen_amp", D::N(1.0), &["button", "1", "min", "*", "slider_range", "0", "10", "shared", "auto_anim_floor0"]),
        param(a, "sharpen_luma", D::N(1.0), &["button", "4", "shared"]),
        param(a, "sharpen_chroma", D::N(1.0), &["shared"]),
        param(a, "sharpen_red", D::N(1.0), &["button", "11", "min", "*", "slider_range", "0", "5", "shared"]),
        param(a, "sharpen_green", D::N(1.0), &["min", "*", "slider_range", "0", "5", "shared"]),
        param(a, "sharpen_blue", D::N(1.0), &["min", "*", "slider_range", "0", "5", "shared"]),
        param(a, "show", D::N(0.0), &[]),
        param(a, "mask_use", D::N(0.0), &["show_with_mask", "optional", "shared", "popup", "Luma", "Alpha"]),
        param(a, "blur_mask", D::N(0.050000004), &["show_with_mask", "optional", "shared", "old_default", "v11", "0", "show_with_mask", "normalize_scale", "4", "slider_max", "2"]),
        param(a, "mocha_project", D::N(0.0), &["optional", "mocha_data", "builder_disable", "shared"]),
        param(a, "blur_mocha_mask", D::N(0.0), &["optional", "disable_motion_blur", "normalize_scale", "4", "slider_max", "0.2", "builder_disable", "shared"]),
        param(a, "mocha_mask_opacity", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "1", "shared"]),
        param(a, "invert_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "resize_mocha_mask", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_x", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_y", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "shift_mocha_mask", D::V(&[0.0, 0.0]), &["optional", "disable_motion_blur", "builder_disable", "normalize_scale", "1", "min", "*", "slider_range", "-2", "2", "shared"]),
        param(a, "bypass_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "show_mocha_mask_only", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "combine_masks", D::N(0.0), &["optional", "popup", "Union", "Intersect", "Mocha_Only", "builder_disable", "shared"]),
    ];
    let body = lower(a, &lt(vec![
        (&["combined_mask", "mocha_mask"], call("PROCESS_MASKS", vec![
            id("Mask"),
            id("invert_mask"),
            id("blur_mask"),
            id("mask_use"),
            id("mocha_project"),
            id("bypass_mocha_mask"),
            id("invert_mocha_mask"),
            id("blur_mocha_mask"),
            id("combine_masks"),
            id("mocha_mask_opacity"),
            id("resize_mocha_mask"),
            id("resize_mocha_rel_x"),
            id("resize_mocha_rel_y"),
            id("shift_mocha_mask"),
        ])),
        (&["result"], call("Sharpen_Best_Quality", vec![
            id("Source"),
            id("combined_mask"),
            n(0.0),
            id("edge_threshold"),
            id("small_detail_size"),
            id("scale_tiny_details"),
            id("scale_small_details"),
            id("scale_medium_details"),
            id("scale_large_details"),
            id("sharpen_amp"),
            id("sharpen_luma"),
            id("sharpen_chroma"),
            id("sharpen_red"),
            id("sharpen_green"),
            id("sharpen_blue"),
            id("show"),
            n(0.0),
            n(0.0),
        ])),
    ], call("if+", vec![id("show_mocha_mask_only"), id("mocha_mask"), id("result")])));
    EffectDef { name: "Sharpen_Best_Quality_Autogen".into(), va: "0x1477780".into(), inputs: vec![inp("Source", None), inp("Mask", Some("1"))], flags: strv(&["no_alpha_only", "old_default_effect", "v7", "1", "no_tiled_rendering", "effect_popup_name", "Quality"]), params, body }
}

fn eff_sharpen_fast(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "sharpen_amp", D::N(1.0), &["min", "*", "slider_range", "0", "10", "auto_anim_floor0", "shared", "big_slider_range", "0", "10"]),
        param(a, "sharpen_blue", D::N(1.0), &["min", "*", "slider_range", "0", "5", "shared"]),
        param(a, "sharpen_chroma", D::N(1.0), &["shared"]),
        param(a, "sharpen_green", D::N(1.0), &["min", "*", "slider_range", "0", "5", "shared"]),
        param(a, "sharpen_luma", D::N(1.0), &["shared"]),
        param(a, "sharpen_red", D::N(1.0), &["min", "*", "slider_range", "0", "5", "shared"]),
        param(a, "sharpen_width", D::N(0.111999996), &["normalize_scale", "4", "slider_max", "2"]),
        param(a, "mask_use", D::N(0.0), &["show_with_mask", "optional", "shared", "popup", "Luma", "Alpha"]),
        param(a, "blur_mask", D::N(0.050000004), &["show_with_mask", "optional", "shared", "show_with_mask", "normalize_scale", "4", "slider_max", "2"]),
        param(a, "invert_mask", D::N(0.0), &["show_with_mask", "optional", "shared", "boolean"]),
    ];
    let body = lower(a, &lt(vec![
        (&["m"], call("if+", vec![
            id("mask_use"),
            call("coerce_to_alpha", vec![id("Mask")]),
            call("monochrome", vec![id("Mask")]),
        ])),
        (&["mask"], call("blur", vec![
            call("if+", vec![id("invert_mask"), call("one_minus", vec![id("m")]), id("m")]),
            id("blur_mask"),
            n(3.0),
            n(1.0),
            n(0.0),
        ])),
        (&["luma"], call("monochrome", vec![id("Source")])),
        (&["chroma"], call("-", vec![id("Source"), id("luma")])),
        (&["sharp_luma"], call("sharpen", vec![
            id("luma"),
            call("*", vec![id("sharpen_amp"), id("sharpen_luma")]),
            id("sharpen_width"),
        ])),
        (&["sharp_chroma"], call("sharpen", vec![
            id("chroma"),
            call("*", vec![
                call("*", vec![id("sharpen_amp"), id("sharpen_chroma")]),
                call("v3", vec![id("sharpen_red"), id("sharpen_green"), id("sharpen_blue")]),
            ]),
            id("sharpen_width"),
        ])),
        (&["sharp_all"], call("sharpen", vec![
            id("Source"),
            call("*", vec![
                call("*", vec![id("sharpen_amp"), id("sharpen_luma")]),
                call("v3", vec![id("sharpen_red"), id("sharpen_green"), id("sharpen_blue")]),
            ]),
            id("sharpen_width"),
        ])),
    ], call("dissolve", vec![
        id("Source"),
        call("if_eq", vec![
            id("sharpen_luma"),
            id("sharpen_chroma"),
            id("sharp_all"),
            call("+", vec![id("sharp_luma"), id("sharp_chroma")]),
        ]),
        id("mask"),
    ])));
    EffectDef { name: "Sharpen_Fast".into(), va: "0x1478160".into(), inputs: vec![inp("Source", None), inp("Mask", Some("1"))], flags: strv(&["no_alpha_only"]), params, body }
}

fn eff_sharpen_fast_autogen(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "sharpen_amp", D::N(1.0), &["min", "*", "slider_range", "0", "10", "auto_anim_floor0", "shared", "big_slider_range", "0", "10"]),
        param(a, "sharpen_blue", D::N(1.0), &["min", "*", "slider_range", "0", "5", "shared"]),
        param(a, "sharpen_chroma", D::N(1.0), &["shared"]),
        param(a, "sharpen_green", D::N(1.0), &["min", "*", "slider_range", "0", "5", "shared"]),
        param(a, "sharpen_luma", D::N(1.0), &["shared"]),
        param(a, "sharpen_red", D::N(1.0), &["min", "*", "slider_range", "0", "5", "shared"]),
        param(a, "sharpen_width", D::N(0.111999996), &["normalize_scale", "4", "slider_max", "2"]),
        param(a, "mask_use", D::N(0.0), &["show_with_mask", "optional", "shared", "popup", "Luma", "Alpha"]),
        param(a, "blur_mask", D::N(0.050000004), &["show_with_mask", "optional", "shared", "show_with_mask", "normalize_scale", "4", "slider_max", "2"]),
        param(a, "invert_mask", D::N(0.0), &["show_with_mask", "optional", "shared", "boolean"]),
        param(a, "mocha_project", D::N(0.0), &["optional", "mocha_data", "builder_disable", "shared"]),
        param(a, "blur_mocha_mask", D::N(0.0), &["optional", "disable_motion_blur", "normalize_scale", "4", "slider_max", "0.2", "builder_disable", "shared"]),
        param(a, "mocha_mask_opacity", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "1", "shared"]),
        param(a, "invert_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "resize_mocha_mask", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_x", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_y", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "shift_mocha_mask", D::V(&[0.0, 0.0]), &["optional", "disable_motion_blur", "builder_disable", "normalize_scale", "1", "min", "*", "slider_range", "-2", "2", "shared"]),
        param(a, "bypass_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "show_mocha_mask_only", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "combine_masks", D::N(0.0), &["optional", "popup", "Union", "Intersect", "Mocha_Only", "builder_disable", "shared"]),
    ];
    let body = lower(a, &lt(vec![
        (&["combined_mask", "mocha_mask"], call("PROCESS_MASKS", vec![
            id("Mask"),
            id("invert_mask"),
            id("blur_mask"),
            id("mask_use"),
            id("mocha_project"),
            id("bypass_mocha_mask"),
            id("invert_mocha_mask"),
            id("blur_mocha_mask"),
            id("combine_masks"),
            id("mocha_mask_opacity"),
            id("resize_mocha_mask"),
            id("resize_mocha_rel_x"),
            id("resize_mocha_rel_y"),
            id("shift_mocha_mask"),
        ])),
        (&["result"], call("Sharpen_Fast", vec![
            id("Source"),
            id("combined_mask"),
            id("sharpen_amp"),
            id("sharpen_blue"),
            id("sharpen_chroma"),
            id("sharpen_green"),
            id("sharpen_luma"),
            id("sharpen_red"),
            id("sharpen_width"),
            n(0.0),
            n(0.0),
            n(0.0),
        ])),
    ], call("if+", vec![id("show_mocha_mask_only"), id("mocha_mask"), id("result")])));
    EffectDef { name: "Sharpen_Fast_Autogen".into(), va: "0x1478680".into(), inputs: vec![inp("Source", None), inp("Mask", Some("1"))], flags: strv(&["no_alpha_only"]), params, body }
}

fn eff_time_warp_rgb(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "clamp_chroma", D::N(1.0), &["max", "1"]),
        param(a, "shift_frames_blue", D::N(-1.0), &["min", "*", "int", "slider_range", "-30", "30"]),
        param(a, "shift_frames_green", D::N(0.0), &["min", "*", "int", "slider_range", "-30", "30"]),
        param(a, "shift_frames_red", D::N(1.0), &["min", "*", "int", "slider_range", "-30", "30"]),
    ];
    let body = lower(a, &lt(vec![
        (&["dest0"], call("v3", vec![
            call("element_of", vec![
                call("get_frame", vec![
                    id("Source"),
                    call("+", vec![call("get_frame_number", vec![]), id("shift_frames_red")]),
                    n(0.0),
                ]),
                n(0.0),
            ]),
            call("element_of", vec![
                call("get_frame", vec![
                    id("Source"),
                    call("+", vec![call("get_frame_number", vec![]), id("shift_frames_green")]),
                    n(0.0),
                ]),
                n(1.0),
            ]),
            call("element_of", vec![
                call("get_frame", vec![
                    id("Source"),
                    call("+", vec![call("get_frame_number", vec![]), id("shift_frames_blue")]),
                    n(0.0),
                ]),
                n(2.0),
            ]),
        ])),
        (&["c"], call("-", vec![call("max_element", vec![id("dest0")]), call("min_element", vec![id("dest0")])])),
    ], call("if>", vec![
        n(1.0),
        id("clamp_chroma"),
        call("sat_scale", vec![
            id("dest0"),
            call("min", vec![n(1.0), call("/", vec![id("clamp_chroma"), id("c")])]),
        ]),
        id("dest0"),
    ])));
    EffectDef { name: "Time_Warp_RGB".into(), va: "0x149e640".into(), inputs: vec![inp("Source", None)], flags: strv(&[]), params, body }
}

fn eff_time_warp_rgb_autogen(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "clamp_chroma", D::N(1.0), &["max", "1"]),
        param(a, "shift_frames_blue", D::N(-1.0), &["min", "*", "int", "slider_range", "-30", "30"]),
        param(a, "shift_frames_green", D::N(0.0), &["min", "*", "int", "slider_range", "-30", "30"]),
        param(a, "shift_frames_red", D::N(1.0), &["min", "*", "int", "slider_range", "-30", "30"]),
        param(a, "mocha_project", D::N(0.0), &["optional", "mocha_data", "builder_disable", "shared"]),
        param(a, "blur_mocha_mask", D::N(0.0), &["optional", "disable_motion_blur", "normalize_scale", "4", "slider_max", "0.2", "builder_disable", "shared"]),
        param(a, "mocha_mask_opacity", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "1", "shared"]),
        param(a, "invert_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "resize_mocha_mask", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_x", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_y", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "shift_mocha_mask", D::V(&[0.0, 0.0]), &["optional", "disable_motion_blur", "builder_disable", "normalize_scale", "1", "min", "*", "slider_range", "-2", "2", "shared"]),
        param(a, "bypass_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "show_mocha_mask_only", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "combine_masks", D::N(0.0), &["optional", "popup", "Union", "Intersect", "Mocha_Only", "builder_disable", "shared"]),
        param(a, "mask_use", D::N(0.0), &["show_with_mask", "optional", "shared", "popup", "Luma", "Alpha"]),
        param(a, "blur_mask", D::N(0.050000004), &["show_with_mask", "optional", "shared", "show_with_mask", "normalize_scale", "4", "slider_max", "2"]),
        param(a, "invert_mask", D::N(0.0), &["show_with_mask", "optional", "shared", "boolean"]),
    ];
    let body = lower(a, &lt(vec![
        (&["combined_mask", "mocha_mask"], call("PROCESS_MASKS", vec![
            id("Mask"),
            id("invert_mask"),
            id("blur_mask"),
            id("mask_use"),
            id("mocha_project"),
            id("bypass_mocha_mask"),
            id("invert_mocha_mask"),
            id("blur_mocha_mask"),
            id("combine_masks"),
            id("mocha_mask_opacity"),
            id("resize_mocha_mask"),
            id("resize_mocha_rel_x"),
            id("resize_mocha_rel_y"),
            id("shift_mocha_mask"),
        ])),
        (&["result"], call("Time_Warp_RGB", vec![
            id("Source"),
            id("clamp_chroma"),
            id("shift_frames_blue"),
            id("shift_frames_green"),
            id("shift_frames_red"),
        ])),
    ], call("if+", vec![
        id("show_mocha_mask_only"),
        id("mocha_mask"),
        call("dissolve", vec![
            id("Source"),
            call("coerce_up", vec![id("result"), id("Source")]),
            id("combined_mask"),
        ]),
    ])));
    EffectDef { name: "Time_Warp_RGB_Autogen".into(), va: "0x149e8e0".into(), inputs: vec![inp("Source", None), inp("Mask", Some("1"))], flags: strv(&["rgba_ok_for_input", "1"]), params, body }
}

fn eff_trails(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "blur_amount", D::N(0.0), &["normalize_scale", "4", "slider_max", "0.4"]),
        param(a, "blur_rel", D::V(&[1.0, 1.0]), &[]),
        param(a, "combine", D::N(0.0), &["shared"]),
        param(a, "fade", D::N(0.8), &["shared", "slider_max", "1"]),
        param(a, "fade_color", D::V(&[1.0, 1.0, 1.0]), &["shared", "color"]),
        param(a, "new_color", D::V(&[1.0, 1.0, 1.0]), &["shared", "color"]),
        param(a, "new_opacity", D::N(1.0), &["shared", "slider_max", "2", "big_slider_max", "2"]),
        param(a, "prev_hue_shift", D::N(0.0), &["shared", ":hueshift"]),
    ];
    let body = lower(a, &lt(vec![
        (&["s"], call("*", vec![id("Source"), call("*", vec![id("new_opacity"), id("new_color")])])),
        (&["init_prev"], call("feedback_init_prev", vec![id("combine"), id("s")])),
        (&["prev"], call("*", vec![
            call("hue_shift", vec![
                call("blur", vec![
                    call("get_prev_dest", vec![id("init_prev")]),
                    call("*", vec![id("blur_amount"), id("blur_rel")]),
                    n(2.0),
                    n(1.0),
                    n(1.0),
                ]),
                id("prev_hue_shift"),
            ]),
            call("*", vec![id("fade"), call("coerce_up", vec![id("fade_color"), id("Source")])]),
        ])),
    ], call("feedback_combine", vec![id("combine"), id("fade"), id("prev"), id("s")])));
    EffectDef { name: "Trails".into(), va: "0x149f9a0".into(), inputs: vec![inp("Source", None)], flags: strv(&["rgba_ok", "generates_color", "generates_alpha", "renders_everywhere"]), params, body }
}

fn eff_trails_autogen(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "blur_amount", D::N(0.0), &["normalize_scale", "4", "slider_max", "0.4"]),
        param(a, "blur_rel", D::V(&[1.0, 1.0]), &[]),
        param(a, "combine", D::N(0.0), &["shared"]),
        param(a, "fade", D::N(0.8), &["shared", "slider_max", "1"]),
        param(a, "fade_color", D::V(&[1.0, 1.0, 1.0]), &["shared", "color"]),
        param(a, "new_color", D::V(&[1.0, 1.0, 1.0]), &["shared", "color"]),
        param(a, "new_opacity", D::N(1.0), &["shared", "slider_max", "2", "big_slider_max", "2"]),
        param(a, "prev_hue_shift", D::N(0.0), &["shared", "min", "*", "slider_range", "-1", "1", "big_slider_range", "-1", "1"]),
        param(a, "mocha_project", D::N(0.0), &["optional", "mocha_data", "builder_disable", "shared"]),
        param(a, "blur_mocha_mask", D::N(0.0), &["optional", "disable_motion_blur", "normalize_scale", "4", "slider_max", "0.2", "builder_disable", "shared"]),
        param(a, "mocha_mask_opacity", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "1", "shared"]),
        param(a, "invert_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "resize_mocha_mask", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_x", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_y", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "shift_mocha_mask", D::V(&[0.0, 0.0]), &["optional", "disable_motion_blur", "builder_disable", "normalize_scale", "1", "min", "*", "slider_range", "-2", "2", "shared"]),
        param(a, "bypass_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "show_mocha_mask_only", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "combine_masks", D::N(0.0), &["optional", "popup", "Union", "Intersect", "Mocha_Only", "builder_disable", "shared"]),
        param(a, "mask_use", D::N(0.0), &["show_with_mask", "optional", "shared", "popup", "Luma", "Alpha"]),
        param(a, "blur_mask", D::N(0.050000004), &["show_with_mask", "optional", "shared", "show_with_mask", "normalize_scale", "4", "slider_max", "2"]),
        param(a, "invert_mask", D::N(0.0), &["show_with_mask", "optional", "shared", "boolean"]),
    ];
    let body = lower(a, &lt(vec![
        (&["combined_mask", "mocha_mask"], call("PROCESS_MASKS", vec![
            id("Mask"),
            id("invert_mask"),
            id("blur_mask"),
            id("mask_use"),
            id("mocha_project"),
            id("bypass_mocha_mask"),
            id("invert_mocha_mask"),
            id("blur_mocha_mask"),
            id("combine_masks"),
            id("mocha_mask_opacity"),
            id("resize_mocha_mask"),
            id("resize_mocha_rel_x"),
            id("resize_mocha_rel_y"),
            id("shift_mocha_mask"),
        ])),
        (&["result"], call("Trails", vec![
            id("Source"),
            id("blur_amount"),
            id("blur_rel"),
            id("combine"),
            id("fade"),
            id("fade_color"),
            id("new_color"),
            id("new_opacity"),
            id("prev_hue_shift"),
        ])),
    ], call("if+", vec![
        id("show_mocha_mask_only"),
        id("mocha_mask"),
        call("dissolve", vec![
            id("Source"),
            call("coerce_up", vec![id("result"), id("Source")]),
            id("combined_mask"),
        ]),
    ])));
    EffectDef { name: "Trails_Autogen".into(), va: "0x14a0840".into(), inputs: vec![inp("Source", None), inp("Mask", Some("1"))], flags: strv(&["rgba_ok", "generates_color", "generates_alpha", "renders_everywhere", "rgba_ok_for_input", "1"]), params, body }
}

fn eff_transform_and_blur(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "blur_res", D::N(0.0), &["popup", "Full", "Half", "Quarter"]),
        param(a, "center", D::V(&[0.0, 0.0]), &[]),
        param(a, "exposure_bias", D::N(0.5), &["max", "1"]),
        param(a, "rotate", D::N(0.0), &["angle", "widget", "rectshift.rotate", "instance", "2"]),
        param(a, "scale_result", D::N(1.0), &[]),
        param(a, "shift", D::V(&[0.0, 0.0]), &["min", "*", "slider_range", "-1", "1", "big_slider_range", "-2", "2", "force_slider"]),
        param(a, "shutter_duration", D::N(1.0), &["slider_max", "2"]),
        param(a, "shutter_shift", D::N(0.0), &["min", "*", "slider_range", "-1", "1"]),
        param(a, "subpixel", D::N(1.0), &["boolean"]),
        param(a, "wrap", D::V(&[0.0, 0.0]), &["popup", "No", "Tile", "Reflect"]),
        param(a, "z_dist", D::N(1.0), &["min", ".001", "slider_range", ".1", "5", "big_slider_range", ".01", "10", "widget", "rectshift.z_dist", "instance", "2"]),
    ];
    let body = lower(a, &lt(vec![
        (&["from_rotate", "from_z_dist", "from_shift", "to_rotate", "to_z_dist", "to_shift"], call("blur_motion_curves_from_to", vec![
            id("rotate"),
            id("shift"),
            id("shutter_duration"),
            id("shutter_shift"),
            id("z_dist"),
        ])),
        (&["from_xy"], call("+", vec![
            call("*", vec![
                call("rotate", vec![
                    call("-", vec![id("xy"), id("center")]),
                    call("/", vec![id("from_rotate"), n(-360.0)]),
                    v(&[0.0, 0.0]),
                ]),
                id("from_z_dist"),
            ]),
            call("-", vec![id("center"), id("from_shift")]),
        ])),
        (&["to_xy"], call("+", vec![
            call("*", vec![
                call("rotate", vec![
                    call("-", vec![id("xy"), id("center")]),
                    call("/", vec![id("to_rotate"), n(-360.0)]),
                    v(&[0.0, 0.0]),
                ]),
                id("to_z_dist"),
            ]),
            call("-", vec![id("center"), id("to_shift")]),
        ])),
        (&["from_rel_bright"], call("one_minus", vec![id("exposure_bias")])),
        (&["to_rel_bright"], id("exposure_bias")),
    ], call("blur_warp", vec![
        call("make_mipmap", vec![id("Source"), id("wrap"), n(0.0)]),
        id("from_xy"),
        id("to_xy"),
        call("v2", vec![id("from_rel_bright"), id("to_rel_bright")]),
        id("scale_result"),
        id("wrap"),
        id("subpixel"),
        id("blur_res"),
    ])));
    EffectDef { name: "Transform_and_Blur".into(), va: "0x14a10a0".into(), inputs: vec![inp("Source", None)], flags: strv(&["rgba_ok", "share_all", "warp_crop"]), params, body }
}

fn eff_transform_and_blur_autogen(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "blur_res", D::N(0.0), &["popup", "Full", "Half", "Quarter"]),
        param(a, "center", D::V(&[0.0, 0.0]), &[]),
        param(a, "exposure_bias", D::N(0.5), &["max", "1"]),
        param(a, "rotate", D::N(0.0), &["angle", "widget", "rectshift.rotate", "instance", "2"]),
        param(a, "scale_result", D::N(1.0), &[]),
        param(a, "shift", D::V(&[0.0, 0.0]), &["min", "*", "slider_range", "-1", "1", "big_slider_range", "-2", "2", "force_slider"]),
        param(a, "shutter_duration", D::N(1.0), &["slider_max", "2"]),
        param(a, "shutter_shift", D::N(0.0), &["min", "*", "slider_range", "-1", "1"]),
        param(a, "subpixel", D::N(1.0), &["boolean"]),
        param(a, "wrap", D::V(&[0.0, 0.0]), &["popup", "No", "Tile", "Reflect"]),
        param(a, "z_dist", D::N(1.0), &["min", ".001", "slider_range", ".1", "5", "big_slider_range", ".01", "10", "widget", "rectshift.z_dist", "instance", "2"]),
        param(a, "mocha_project", D::N(0.0), &["optional", "mocha_data", "builder_disable", "shared"]),
        param(a, "blur_mocha_mask", D::N(0.0), &["optional", "disable_motion_blur", "normalize_scale", "4", "slider_max", "0.2", "builder_disable", "shared"]),
        param(a, "mocha_mask_opacity", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "1", "shared"]),
        param(a, "invert_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "resize_mocha_mask", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_x", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_y", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "shift_mocha_mask", D::V(&[0.0, 0.0]), &["optional", "disable_motion_blur", "builder_disable", "normalize_scale", "1", "min", "*", "slider_range", "-2", "2", "shared"]),
        param(a, "bypass_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "show_mocha_mask_only", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "combine_masks", D::N(0.0), &["optional", "popup", "Union", "Intersect", "Mocha_Only", "builder_disable", "shared"]),
        param(a, "mask_use", D::N(0.0), &["show_with_mask", "optional", "shared", "popup", "Luma", "Alpha"]),
        param(a, "blur_mask", D::N(0.050000004), &["show_with_mask", "optional", "shared", "show_with_mask", "normalize_scale", "4", "slider_max", "2"]),
        param(a, "invert_mask", D::N(0.0), &["show_with_mask", "optional", "shared", "boolean"]),
    ];
    let body = lower(a, &lt(vec![
        (&["combined_mask", "mocha_mask"], call("PROCESS_MASKS", vec![
            id("Mask"),
            id("invert_mask"),
            id("blur_mask"),
            id("mask_use"),
            id("mocha_project"),
            id("bypass_mocha_mask"),
            id("invert_mocha_mask"),
            id("blur_mocha_mask"),
            id("combine_masks"),
            id("mocha_mask_opacity"),
            id("resize_mocha_mask"),
            id("resize_mocha_rel_x"),
            id("resize_mocha_rel_y"),
            id("shift_mocha_mask"),
        ])),
        (&["result"], call("Transform_and_Blur", vec![
            id("Source"),
            id("blur_res"),
            id("center"),
            id("exposure_bias"),
            id("rotate"),
            id("scale_result"),
            id("shift"),
            id("shutter_duration"),
            id("shutter_shift"),
            id("subpixel"),
            id("wrap"),
            id("z_dist"),
        ])),
    ], call("if+", vec![
        id("show_mocha_mask_only"),
        id("mocha_mask"),
        call("dissolve", vec![
            id("Source"),
            call("coerce_up", vec![id("result"), id("Source")]),
            id("combined_mask"),
        ]),
    ])));
    EffectDef { name: "Transform_and_Blur_Autogen".into(), va: "0x14a1540".into(), inputs: vec![inp("Source", None), inp("Mask", Some("1"))], flags: strv(&["rgba_ok", "share_all", "warp_crop", "rgba_ok_for_input", "1"]), params, body }
}

fn eff_warp_perspective(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "comp_premult", D::N(0.0), &["shared", "boolean"]),
        param(a, "blur_mask", D::N(0.0), &[":blur_mask"]),
        param(a, "filter", D::N(1.0), &["boolean"]),
        param(a, "invert_mask", D::N(0.0), &["show_with_mask", "boolean"]),
        param(a, "latitude", D::N(35.0), &["auto_anim_floor0"]),
        param(a, "mask_use", D::N(0.0), &["show_with_mask", "popup", "Luma", "Alpha"]),
        param(a, "roll", D::N(0.0), &["angle"]),
        param(a, "shift", D::V(&[0.0, 0.0]), &["min", "*", "slider_range", "-1", "1", "big_slider_range", "-2", "2"]),
        param(a, "swing", D::N(0.0), &["angle"]),
        param(a, "tele_lens_width", D::N(1.0), &["min", ".2"]),
        param(a, "wrap", D::V(&[1.0, 1.0]), &["popup", "No", "Tile", "Reflect"]),
        param(a, "wrap_above_horizon", D::N(0.0), &["boolean"]),
        param(a, "z_dist", D::N(3.0), &["slider_range", ".1", "10", "auto_anim_floor1"]),
    ];
    let body = lower(a, &lt(vec![
        (&["source"], call("maybe_add_alpha", vec![id("Source"), id("SparksMatte"), id("comp_premult")])),
        (&["xy0"], call("rotate", vec![id("xy"), call("/", vec![id("roll"), n(-360.0)]), v(&[0.0, 0.0])])),
        (&["coords"], call("-", vec![
            call("rotate", vec![
                call("perspective", vec![
                    call("force_no_cache", vec![id("xy0")]),
                    id("z_dist"),
                    call("/", vec![id("latitude"), n(90.0)]),
                    call("/", vec![n(1.0), id("tele_lens_width")]),
                ]),
                call("/", vec![id("swing"), n(-360.0)]),
                v(&[0.0, 0.0]),
            ]),
            id("shift"),
        ])),
        (&["mask"], call("WARP_MASK", vec![id("Mask"), id("invert_mask"), id("blur_mask"), id("mask_use")])),
        (&["alpha"], call("if+", vec![
            call("+", vec![
                id("wrap_above_horizon"),
                call("if>", vec![n(10.0), call("abs", vec![id("latitude")]), n(1.0), n(0.0)]),
            ]),
            n(1.0),
            call("force_no_cache", vec![
                call("perspective_alpha", vec![
                    id("xy0"),
                    id("z_dist"),
                    call("/", vec![id("latitude"), n(90.0)]),
                    call("/", vec![n(1.0), id("tele_lens_width")]),
                ]),
            ]),
        ])),
        (&["d0"], call("motion_blur", vec![
            call("*", vec![
                call("warp", vec![
                    call("make_mipmap", vec![id("source"), id("wrap"), id("filter")]),
                    call("interp", vec![id("xy"), id("coords"), id("mask")]),
                    id("wrap"),
                    id("filter"),
                ]),
                id("alpha"),
            ]),
        ])),
    ], call("warp_combine", vec![id("d0"), id("Back")])));
    EffectDef { name: "Warp_Perspective".into(), va: "0x14ab740".into(), inputs: vec![inp("Mask", Some("1")), inp("Source", None), inp("SparksMatte", Some("1")), inp("Back", Some("0"))], flags: strv(&["rgba_ok", "warp_crop"]), params, body }
}

fn eff_warp_perspective_autogen(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "comp_premult", D::N(0.0), &["shared", "boolean"]),
        param(a, "blur_mask", D::N(0.0), &["show_with_mask", "normalize_scale", "4", "slider_max", "2"]),
        param(a, "filter", D::N(1.0), &["boolean"]),
        param(a, "invert_mask", D::N(0.0), &["show_with_mask", "boolean"]),
        param(a, "latitude", D::N(35.0), &["auto_anim_floor0"]),
        param(a, "mask_use", D::N(0.0), &["show_with_mask", "popup", "Luma", "Alpha"]),
        param(a, "roll", D::N(0.0), &["angle"]),
        param(a, "shift", D::V(&[0.0, 0.0]), &["min", "*", "slider_range", "-1", "1", "big_slider_range", "-2", "2"]),
        param(a, "swing", D::N(0.0), &["angle"]),
        param(a, "tele_lens_width", D::N(1.0), &["min", ".2"]),
        param(a, "wrap", D::V(&[1.0, 1.0]), &["popup", "No", "Tile", "Reflect"]),
        param(a, "wrap_above_horizon", D::N(0.0), &["boolean"]),
        param(a, "z_dist", D::N(3.0), &["slider_range", ".1", "10", "auto_anim_floor1"]),
        param(a, "mocha_project", D::N(0.0), &["optional", "mocha_data", "builder_disable", "shared"]),
        param(a, "blur_mocha_mask", D::N(0.0), &["optional", "disable_motion_blur", "normalize_scale", "4", "slider_max", "0.2", "builder_disable", "shared"]),
        param(a, "mocha_mask_opacity", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "1", "shared"]),
        param(a, "invert_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "resize_mocha_mask", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_x", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_y", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "shift_mocha_mask", D::V(&[0.0, 0.0]), &["optional", "disable_motion_blur", "builder_disable", "normalize_scale", "1", "min", "*", "slider_range", "-2", "2", "shared"]),
        param(a, "bypass_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "show_mocha_mask_only", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "combine_masks", D::N(0.0), &["optional", "popup", "Union", "Intersect", "Mocha_Only", "builder_disable", "shared"]),
    ];
    let body = lower(a, &lt(vec![
        (&["combined_mask", "mocha_mask"], call("PROCESS_MASKS", vec![
            id("Mask"),
            id("invert_mask"),
            id("blur_mask"),
            id("mask_use"),
            id("mocha_project"),
            id("bypass_mocha_mask"),
            id("invert_mocha_mask"),
            id("blur_mocha_mask"),
            id("combine_masks"),
            id("mocha_mask_opacity"),
            id("resize_mocha_mask"),
            id("resize_mocha_rel_x"),
            id("resize_mocha_rel_y"),
            id("shift_mocha_mask"),
        ])),
        (&["result"], call("Warp_Perspective", vec![
            id("combined_mask"),
            id("Source"),
            id("SparksMatte"),
            id("Back"),
            id("comp_premult"),
            n(0.0),
            id("filter"),
            n(0.0),
            id("latitude"),
            n(0.0),
            id("roll"),
            id("shift"),
            id("swing"),
            id("tele_lens_width"),
            id("wrap"),
            id("wrap_above_horizon"),
            id("z_dist"),
        ])),
    ], call("if+", vec![id("show_mocha_mask_only"), id("mocha_mask"), id("result")])));
    EffectDef { name: "Warp_Perspective_Autogen".into(), va: "0x14abc00".into(), inputs: vec![inp("Mask", Some("1")), inp("Source", None), inp("SparksMatte", Some("1")), inp("Back", Some("0"))], flags: strv(&["rgba_ok", "warp_crop"]), params, body }
}

fn defun_combine_masks(a: &mut Arena) -> DefunDef {
    let body = lower(a, &call("if_neq_const", vec![
        id("Mask"),
        n(1.0),
        call("if_neq_const", vec![
            id("m2"),
            n(1.0),
            call("switch", vec![
                id("combine_mode"),
                n(0.0),
                call("max", vec![id("m1"), id("m2")]),
                n(1.0),
                call("min", vec![id("m1"), id("m2")]),
                n(2.0),
                id("m2"),
                call("max", vec![id("m1"), id("m2")]),
            ]),
            id("m1"),
        ]),
        id("m2"),
    ]));
    DefunDef { name: "COMBINE_MASKS".into(), va: "0x13bc120".into(), args: strv(&["Mask", "m1", "m2", "combine_mode"]), body }
}

fn defun_mocha_mask(a: &mut Arena) -> DefunDef {
    let body = lower(a, &lt(vec![
        (&["m1"], call("mocha_render", vec![
            id("mocha_project"),
            call("*", vec![
                id("resize_mocha_mask"),
                call("v2", vec![id("resize_mocha_rel_x"), id("resize_mocha_rel_y")]),
            ]),
            id("shift_mocha_mask"),
        ])),
        (&["m2"], call("if+", vec![id("invert_mocha_mask"), call("one_minus", vec![id("m1")]), id("m1")])),
        (&["mocha_mask"], call("blur", vec![
            call("*", vec![id("m2"), id("mocha_mask_opacity")]),
            id("blur_mocha_mask"),
            n(2.0),
            n(1.0),
            n(1.0),
        ])),
    ], call("if+", vec![id("bypass_mocha_mask"), n(1.0), id("mocha_mask")])));
    DefunDef { name: "MOCHA_MASK".into(), va: "0x1449d00".into(), args: strv(&["mocha_project", "bypass_mocha_mask", "invert_mocha_mask", "blur_mocha_mask", "mocha_mask_opacity", "resize_mocha_mask", "resize_mocha_rel_x", "resize_mocha_rel_y", "shift_mocha_mask"]), body }
}

fn defun_process_masks(a: &mut Arena) -> DefunDef {
    let body = lower(a, &lt(vec![
        (&["m"], call("if+", vec![
            id("mask_use"),
            call("coerce_to_alpha", vec![id("Mask")]),
            call("monochrome", vec![id("Mask")]),
        ])),
        (&["mask"], call("blur", vec![
            call("if+", vec![id("invert_mask"), call("one_minus", vec![id("m")]), id("m")]),
            id("blur_mask"),
            n(2.0),
            n(1.0),
            n(0.0),
        ])),
        (&["mocha_mask"], call("MOCHA_MASK", vec![
            id("mocha_project"),
            id("bypass_mocha_mask"),
            id("invert_mocha_mask"),
            id("blur_mocha_mask"),
            id("mocha_mask_opacity"),
            id("resize_mocha_mask"),
            id("resize_mocha_rel_x"),
            id("resize_mocha_rel_y"),
            id("shift_mocha_mask"),
        ])),
        (&["combined_mask"], call("COMBINE_MASKS", vec![id("Mask"), id("mask"), id("mocha_mask"), id("combine_mode")])),
    ], call("values", vec![id("combined_mask"), id("mocha_mask")])));
    DefunDef { name: "PROCESS_MASKS".into(), va: "0x14626e0".into(), args: strv(&["Mask", "invert_mask", "blur_mask", "mask_use", "mocha_project", "bypass_mocha_mask", "invert_mocha_mask", "blur_mocha_mask", "combine_mode", "mocha_mask_opacity", "resize_mocha_mask", "resize_mocha_rel_x", "resize_mocha_rel_y", "shift_mocha_mask"]), body }
}

fn defun_scale_lights(a: &mut Arena) -> DefunDef {
    let body = lower(a, &call("*", vec![
        call("/", vec![id("scale_lights"), call("abs", vec![id("scale_lights")])]),
        call("max", vec![n(0.0), call("-", vec![call("abs", vec![id("scale_lights")]), id("offset_darks")])]),
    ]));
    DefunDef { name: "SCALE_LIGHTS".into(), va: "0x14700c0".into(), args: strv(&["scale_lights", "offset_darks"]), body }
}

fn defun_scale_lights_offset_darks(a: &mut Arena) -> DefunDef {
    let body = lower(a, &call("+", vec![
        call("*", vec![
            id("src"),
            call("v3_to_v4_if4", vec![
                call("SCALE_LIGHTS", vec![id("scale_lights"), id("offset_darks")]),
                n(1.0),
                id("to_vect_type"),
            ]),
        ]),
        call("v3_to_v4_if4", vec![id("offset_darks"), n(0.0), id("to_vect_type")]),
    ]));
    DefunDef { name: "SCALE_LIGHTS_OFFSET_DARKS".into(), va: "0x1470160".into(), args: strv(&["src", "scale_lights", "offset_darks", "to_vect_type"]), body }
}

fn defun_warp_mask(a: &mut Arena) -> DefunDef {
    let body = lower(a, &call("blur", vec![
        call("if+", vec![
            id("invert_mask"),
            call("one_minus", vec![
                call("if+", vec![
                    id("mask_use"),
                    call("coerce_to_alpha", vec![id("Mask")]),
                    call("monochrome", vec![id("Mask")]),
                ]),
            ]),
            call("if+", vec![
                id("mask_use"),
                call("coerce_to_alpha", vec![id("Mask")]),
                call("monochrome", vec![id("Mask")]),
            ]),
        ]),
        id("blur_mask"),
        n(2.0),
        n(1.0),
        n(0.0),
    ]));
    DefunDef { name: "WARP_MASK".into(), va: "0x14a59e0".into(), args: strv(&["Mask", "invert_mask", "blur_mask", "mask_use"]), body }
}

fn defun_blur_mo_curves_get(a: &mut Arena) -> DefunDef {
    let body = lower(a, &call("values", vec![
        call("get_curve_val", vec![id("rotate"), id("time")]),
        call("get_curve_val", vec![id("z_dist"), id("time")]),
        call("get_curve_val", vec![id("shift"), id("time")]),
    ]));
    DefunDef { name: "blur_mo_curves_get".into(), va: "0x14ce440".into(), args: strv(&["time", "rotate", "shift", "z_dist"]), body }
}

fn defun_blur_motion_curves_from_to(a: &mut Arena) -> DefunDef {
    let body = lower(a, &lt(vec![
        (&["from_time"], call("+", vec![id("shutter_shift"), call("*", vec![id("shutter_duration"), n(0.5)])])),
        (&["to_time"], call("-", vec![id("shutter_shift"), call("*", vec![id("shutter_duration"), n(0.5)])])),
        (&["from_rotate", "from_z_dist", "from_shift"], call("blur_mo_curves_get", vec![id("from_time"), id("rotate"), id("shift"), id("z_dist")])),
        (&["to_rotate", "to_z_dist", "to_shift"], call("blur_mo_curves_get", vec![id("to_time"), id("rotate"), id("shift"), id("z_dist")])),
    ], call("values", vec![
        id("from_rotate"),
        id("from_z_dist"),
        id("from_shift"),
        id("to_rotate"),
        id("to_z_dist"),
        id("to_shift"),
    ])));
    DefunDef { name: "blur_motion_curves_from_to".into(), va: "0x14ce7e0".into(), args: strv(&["rotate", "shift", "shutter_duration", "shutter_shift", "z_dist"]), body }
}

fn defun_drop_shadow_combine(a: &mut Arena) -> DefunDef {
    let body = lower(a, &lt(vec![
        (&["f"], call("coerce_to_v3", vec![id("Foreground")])),
    ], call("if+", vec![
        id("comp_premult"),
        call("comp_premult", vec![
            id("back_shadow"),
            call("*", vec![id("fg_opacity"), id("f")]),
            call("*", vec![id("fg_opacity"), id("a")]),
        ]),
        call("dissolve", vec![id("back_shadow"), id("f"), call("*", vec![id("fg_opacity"), id("a")])]),
    ])));
    DefunDef { name: "drop_shadow_combine".into(), va: "0x14d2140".into(), args: strv(&["Foreground", "a", "back_shadow", "comp_premult", "fg_opacity"]), body }
}

fn defun_drop_shadow_shadow(a: &mut Arena) -> DefunDef {
    let body = lower(a, &lt(vec![
        (&["m1"], call("if+", vec![
            id("matte_use"),
            call("coerce_to_alpha", vec![id("Matte")]),
            call("monochrome", vec![id("Matte")]),
        ])),
        (&["a"], call("copy", vec![call("if+", vec![id("invert_matte"), call("one_minus", vec![id("m1")]), id("m1")])])),
        (&["a1"], call("COMBINE_MASKS", vec![id("Matte"), id("a"), id("mocha_mask"), id("combine_masks")])),
        (&["shadow"], call("blur", vec![
            call("warp_rel", vec![
                call("make_mipmap", vec![id("a1"), v(&[0.0, 0.0]), n(0.0)]),
                call("negate", vec![id("shift")]),
                v(&[0.0, 0.0]),
                n(0.0),
            ]),
            id("shadow_blur"),
            n(3.0),
            id("shadow_opacity"),
            n(1.0),
        ])),
    ], call("values", vec![id("a1"), id("shadow")])));
    DefunDef { name: "drop_shadow_shadow".into(), va: "0x14d2260".into(), args: strv(&["Matte", "shadow_blur", "invert_matte", "matte_use", "shadow_opacity", "shift", "mocha_mask", "combine_masks"]), body }
}

fn defun_feedback_combine(a: &mut Arena) -> DefunDef {
    let body = lower(a, &lt(vec![
        (&["cmb"], id("combine")),
    ], call("if>", vec![
        id("cmb"),
        n(5.0),
        call("min", vec![id("prev"), id("s")]),
        call("if>", vec![
            id("cmb"),
            n(4.0),
            call("comp_rgba", vec![id("s"), id("prev")]),
            call("if>", vec![
                id("cmb"),
                n(3.0),
                call("comp_rgba", vec![id("prev"), id("s")]),
                call("if>", vec![
                    id("cmb"),
                    n(2.0),
                    call("+", vec![id("prev"), id("s")]),
                    call("if>", vec![
                        id("cmb"),
                        n(1.0),
                        call("screen_super_if_needed", vec![id("prev"), id("s")]),
                        call("if+", vec![
                            id("cmb"),
                            call("max", vec![id("prev"), id("s")]),
                            call("+", vec![
                                id("prev"),
                                call("*", vec![
                                    id("s"),
                                    call("max", vec![n(0.0), call("one_minus", vec![id("fade")])]),
                                ]),
                            ]),
                        ]),
                    ]),
                ]),
            ]),
        ]),
    ])));
    DefunDef { name: "feedback_combine".into(), va: "0x14d2b40".into(), args: strv(&["combine", "fade", "prev", "s"]), body }
}

fn defun_feedback_init_prev(a: &mut Arena) -> DefunDef {
    let body = lower(a, &call("if>", vec![
        id("combine"),
        n(5.0),
        id("s2"),
        call("if+", vec![id("combine"), call("coerce_up", vec![n(0.0), id("s2")]), id("s2")]),
    ]));
    DefunDef { name: "feedback_init_prev".into(), va: "0x14d2ca0".into(), args: strv(&["combine", "s2"]), body }
}

fn defun_maybe_add_alpha(a: &mut Arena) -> DefunDef {
    let body = lower(a, &lt(vec![
        (&["m"], call("coerce_to_alpha", vec![id("Matte")])),
    ], call("if_neq_const", vec![
        id("Matte"),
        n(1.0),
        call("v3_to_v4", vec![
            call("if+", vec![id("comp_premult"), id("Front"), call("*", vec![id("Front"), id("m")])]),
            id("m"),
        ]),
        id("Front"),
    ])));
    DefunDef { name: "maybe_add_alpha".into(), va: "0x14da980".into(), args: strv(&["Front", "Matte", "comp_premult"]), body }
}

fn defun_warp_combine(a: &mut Arena) -> DefunDef {
    let body = lower(a, &call("comp_rgba", vec![id("back"), id("d0")]));
    DefunDef { name: "warp_combine".into(), va: "0x14e8f64".into(), args: strv(&["d0", "back"]), body }
}

fn layout_blur_color_autogen() -> Vec<Sx> {
    vec![
        a("layout_effect"),
        l(vec![a("Blur_Color"), a("Blur_Color_Autogen")]),
        l(vec![a("Source"), l(vec![a("Matte"), a("Mask")])]),
        l(vec![a("share_all")]),
        l(vec![
            l(vec![a(":modify"), a("mocha_project"), l(vec![a("id_insert"), a("520")])]),
            l(vec![a(":modify"), a("blur_mocha"), a("blur_mocha_mask"), l(vec![a("id_insert"), a("521"), a("group_start"), a("mocha")])]),
            l(vec![a(":modify"), a("mocha_opacity"), a("mocha_mask_opacity"), l(vec![a("id_insert"), a("522")])]),
            l(vec![a(":modify"), a("invert_mocha"), a("invert_mocha_mask"), l(vec![a("id_insert"), a("523")])]),
            l(vec![a(":modify"), a("resize_mocha"), a("resize_mocha_mask"), l(vec![a("id_insert"), a("524")])]),
            l(vec![a(":modify"), a("resize_rel_x"), a("resize_mocha_rel_x"), l(vec![a("id_insert"), a("525")])]),
            l(vec![a(":modify"), a("resize_rel_y"), a("resize_mocha_rel_y"), l(vec![a("id_insert"), a("526")])]),
            l(vec![a(":modify"), a("shift_mocha"), a("shift_mocha_mask"), l(vec![a("id_insert"), a("527")])]),
            l(vec![a(":modify"), a("bypass_mocha"), a("bypass_mocha_mask"), l(vec![a("id_insert"), a("531")])]),
            l(vec![a(":modify"), a("show_mocha_only"), a("show_mocha_mask_only"), l(vec![a("id_insert"), a("529")])]),
            l(vec![a(":modify"), a("combine_masks"), l(vec![a("id_insert"), a("530"), a("group_end"), a("mocha")])]),
            a("blur_amount"),
            a("blur_rel"),
            l(vec![a(":modify"), a("brightness"), l(vec![a("new_column")])]),
            l(vec![a(":modify"), a("scale_opacity"), l(vec![a("id_insert"), a("100")])]),
            a("offset_darks"),
            l(vec![a(":modify"), a("mix_with_source"), l(vec![a("id_insert"), a("101")])]),
            a("filter"),
            a("subpixel"),
            l(vec![a(":modify"), a("invert_matte"), a("invert_mask"), l(vec![a("new_column")])]),
            l(vec![a(":modify"), a("matte_use"), a("mask_use")]),
            l(vec![a(":extra"), a("soft_borders"), a("="), a("0"), l(vec![a("boolean")])]),
            l(vec![a(":extra"), a("opacity"), a("="), a("1"), l(vec![a("popup"), a("All_Opaque"), a("Normal"), a("As_Premult")])]),
        ]),
    ]
}

fn layout_blur_mono_autogen() -> Vec<Sx> {
    vec![
        a("layout_effect"),
        l(vec![a("Blur_Mono"), a("Blur_Mono_Autogen")]),
        l(vec![a("Source"), l(vec![a("Matte"), a("Mask")])]),
        l(vec![a("share_all")]),
        l(vec![
            l(vec![a(":modify"), a("mocha_project"), l(vec![a("id_insert"), a("520")])]),
            l(vec![a(":modify"), a("blur_mocha"), a("blur_mocha_mask"), l(vec![a("id_insert"), a("521"), a("group_start"), a("mocha")])]),
            l(vec![a(":modify"), a("mocha_opacity"), a("mocha_mask_opacity"), l(vec![a("id_insert"), a("522")])]),
            l(vec![a(":modify"), a("invert_mocha"), a("invert_mocha_mask"), l(vec![a("id_insert"), a("523")])]),
            l(vec![a(":modify"), a("resize_mocha"), a("resize_mocha_mask"), l(vec![a("id_insert"), a("524")])]),
            l(vec![a(":modify"), a("resize_rel_x"), a("resize_mocha_rel_x"), l(vec![a("id_insert"), a("525")])]),
            l(vec![a(":modify"), a("resize_rel_y"), a("resize_mocha_rel_y"), l(vec![a("id_insert"), a("526")])]),
            l(vec![a(":modify"), a("shift_mocha"), a("shift_mocha_mask"), l(vec![a("id_insert"), a("527")])]),
            l(vec![a(":modify"), a("bypass_mocha"), a("bypass_mocha_mask"), l(vec![a("id_insert"), a("531")])]),
            l(vec![a(":modify"), a("show_mocha_only"), a("show_mocha_mask_only"), l(vec![a("id_insert"), a("529")])]),
            l(vec![a(":modify"), a("combine_masks"), l(vec![a("id_insert"), a("530"), a("group_end"), a("mocha")])]),
            a("blur_amount"),
            a("blur_rel"),
            l(vec![a(":modify"), a("brightness"), l(vec![a("new_column")])]),
            l(vec![a(":modify"), a("scale_opacity"), l(vec![a("id_insert"), a("100")])]),
            a("offset_darks"),
            l(vec![a(":modify"), a("mix_with_source"), l(vec![a("id_insert"), a("101")])]),
            a("filter"),
            a("subpixel"),
            l(vec![a(":modify"), a("invert_matte"), a("invert_mask"), l(vec![a("new_column")])]),
            l(vec![a(":modify"), a("matte_use"), a("mask_use")]),
            l(vec![a(":extra"), a("soft_borders"), a("="), a("0"), l(vec![a("boolean")])]),
            l(vec![a(":extra"), a("opacity"), a("="), a("1"), l(vec![a("popup"), a("All_Opaque"), a("Normal"), a("As_Premult")])]),
        ]),
    ]
}

fn layout_transform_and_blur_autogen() -> Vec<Sx> {
    vec![
        a("layout_effect"),
        l(vec![a("Transform_and_Blur"), a("Transform_and_Blur_Autogen")]),
        l(vec![a("Source"), a("Mask")]),
        l(vec![]),
        l(vec![
            l(vec![a(":modify"), a("mocha_project"), l(vec![a("id_insert"), a("520")])]),
            l(vec![a(":modify"), a("blur_mocha"), a("blur_mocha_mask"), l(vec![a("id_insert"), a("521"), a("group_start"), a("mocha")])]),
            l(vec![a(":modify"), a("mocha_opacity"), a("mocha_mask_opacity"), l(vec![a("id_insert"), a("522")])]),
            l(vec![a(":modify"), a("invert_mocha"), a("invert_mocha_mask"), l(vec![a("id_insert"), a("523")])]),
            l(vec![a(":modify"), a("resize_mocha"), a("resize_mocha_mask"), l(vec![a("id_insert"), a("524")])]),
            l(vec![a(":modify"), a("resize_rel_x"), a("resize_mocha_rel_x"), l(vec![a("id_insert"), a("525")])]),
            l(vec![a(":modify"), a("resize_rel_y"), a("resize_mocha_rel_y"), l(vec![a("id_insert"), a("526")])]),
            l(vec![a(":modify"), a("shift_mocha"), a("shift_mocha_mask"), l(vec![a("id_insert"), a("527")])]),
            l(vec![a(":modify"), a("bypass_mocha"), a("bypass_mocha_mask"), l(vec![a("id_insert"), a("531")])]),
            l(vec![a(":modify"), a("show_mocha_only"), a("show_mocha_mask_only"), l(vec![a("id_insert"), a("529")])]),
            l(vec![a(":modify"), a("combine_masks"), l(vec![a("id_insert"), a("530"), a("group_end"), a("mocha")])]),
            l(vec![a(":modify"), a("center"), l(vec![a("normalize"), a("widget"), a("cross_circle"), a("name"), a("center"), a("instance"), a("0"), a("widget"), a("rect"), a("name"), a("transform"), a("instance"), a("1"), a("widget"), a("rectshift"), a("name"), a("shift"), a("instance"), a("2")])]),
            l(vec![a(":modify"), a("z_dist"), l(vec![a("widget"), a("rect.z_dist"), a("instance"), a("1")])]),
            l(vec![a(":modify"), a("rotate"), l(vec![a("widget"), a("rect.rotate"), a("instance"), a("1")])]),
            l(vec![a(":modify"), a("shift"), l(vec![a("min"), a("*"), a("normalize_scale"), a("1"), a("slider_range"), a("-1"), a("1"), a("big_slider_range"), a("-2"), a("2"), a("widget"), a("rect.shift"), a("instance"), a("1"), a("widget"), a("rectshift.shift"), a("default_off"), a("instance"), a("2")])]),
            l(vec![a(":modify"), a("shutter_duration"), l(vec![a("new_column")])]),
            a("shutter_shift"),
            a("exposure_bias"),
            l(vec![a(":modify"), a("brightness"), a("scale_result")]),
            a("wrap"),
            l(vec![a(":modify"), a("blur_res"), l(vec![a("new_column")])]),
            a("subpixel"),
            l(vec![a(":extra"), a("opacity"), a("="), a("1"), l(vec![a("ignore"), a("popup"), a("All_Opaque"), a("Normal"), a("As_Premult")])]),
            l(vec![a(":modify"), a("mask_use"), l(vec![a("id_insert"), a("540")])]),
            l(vec![a(":modify"), a("blur_mask"), l(vec![a("id_insert"), a("541")])]),
            l(vec![a(":modify"), a("invert_mask"), l(vec![a("id_insert"), a("542")])]),
        ]),
    ]
}

fn layout_blur_motion_curves_no_transform_autogen() -> Vec<Sx> {
    vec![
        a("layout_effect"),
        l(vec![a("Blur_Only"), a("Blur_Motion_Curves_No_Transform_Autogen")]),
        l(vec![a("Source"), a("Mask")]),
        l(vec![]),
        l(vec![
            l(vec![a(":modify"), a("mocha_project"), l(vec![a("id_insert"), a("520")])]),
            l(vec![a(":modify"), a("blur_mocha"), a("blur_mocha_mask"), l(vec![a("id_insert"), a("521"), a("group_start"), a("mocha")])]),
            l(vec![a(":modify"), a("mocha_opacity"), a("mocha_mask_opacity"), l(vec![a("id_insert"), a("522")])]),
            l(vec![a(":modify"), a("invert_mocha"), a("invert_mocha_mask"), l(vec![a("id_insert"), a("523")])]),
            l(vec![a(":modify"), a("resize_mocha"), a("resize_mocha_mask"), l(vec![a("id_insert"), a("524")])]),
            l(vec![a(":modify"), a("resize_rel_x"), a("resize_mocha_rel_x"), l(vec![a("id_insert"), a("525")])]),
            l(vec![a(":modify"), a("resize_rel_y"), a("resize_mocha_rel_y"), l(vec![a("id_insert"), a("526")])]),
            l(vec![a(":modify"), a("shift_mocha"), a("shift_mocha_mask"), l(vec![a("id_insert"), a("527")])]),
            l(vec![a(":modify"), a("bypass_mocha"), a("bypass_mocha_mask"), l(vec![a("id_insert"), a("531")])]),
            l(vec![a(":modify"), a("show_mocha_only"), a("show_mocha_mask_only"), l(vec![a("id_insert"), a("529")])]),
            l(vec![a(":modify"), a("combine_masks"), l(vec![a("id_insert"), a("530"), a("group_end"), a("mocha")])]),
            l(vec![a(":modify"), a("center"), l(vec![a("normalize"), a("widget"), a("cross_circle"), a("name"), a("center"), a("instance"), a("0"), a("widget"), a("rect"), a("name"), a("transform"), a("instance"), a("1"), a("widget"), a("rectshift"), a("name"), a("shift"), a("instance"), a("2")])]),
            l(vec![a(":modify"), a("z_dist"), l(vec![a("widget"), a("rect.z_dist"), a("instance"), a("1")])]),
            l(vec![a(":modify"), a("rotate"), l(vec![a("widget"), a("rect.rotate"), a("instance"), a("1")])]),
            l(vec![a(":modify"), a("shift"), l(vec![a("min"), a("*"), a("normalize_scale"), a("1"), a("slider_range"), a("-1"), a("1"), a("big_slider_range"), a("-2"), a("2"), a("widget"), a("rect.shift"), a("instance"), a("1"), a("widget"), a("rectshift.shift"), a("default_off"), a("instance"), a("2")])]),
            l(vec![a(":modify"), a("shutter_duration"), l(vec![a("new_column")])]),
            a("shutter_shift"),
            a("exposure_bias"),
            l(vec![a(":modify"), a("brightness"), a("scale_result")]),
            a("wrap"),
            l(vec![a(":modify"), a("blur_res"), l(vec![a("new_column")])]),
            a("subpixel"),
            l(vec![a(":extra"), a("opacity"), a("="), a("1"), l(vec![a("ignore"), a("popup"), a("All_Opaque"), a("Normal"), a("As_Premult")])]),
            l(vec![a(":modify"), a("mask_use"), l(vec![a("id_insert"), a("540")])]),
            l(vec![a(":modify"), a("blur_mask"), l(vec![a("id_insert"), a("541")])]),
            l(vec![a(":modify"), a("invert_mask"), l(vec![a("id_insert"), a("542")])]),
        ]),
    ]
}

fn layout_diffuse_autogen() -> Vec<Sx> {
    vec![
        a("layout_effect"),
        l(vec![a("Diffuse"), a("Diffuse_Autogen")]),
        l(vec![a("Source"), l(vec![a("Matte"), a("Mask")])]),
        l(vec![]),
        l(vec![
            l(vec![a(":modify"), a("mocha_project"), l(vec![a("id_insert"), a("520")])]),
            l(vec![a(":modify"), a("blur_mocha"), a("blur_mocha_mask"), l(vec![a("id_insert"), a("521"), a("group_start"), a("mocha")])]),
            l(vec![a(":modify"), a("mocha_opacity"), a("mocha_mask_opacity"), l(vec![a("id_insert"), a("522")])]),
            l(vec![a(":modify"), a("invert_mocha"), a("invert_mocha_mask"), l(vec![a("id_insert"), a("523")])]),
            l(vec![a(":modify"), a("resize_mocha"), a("resize_mocha_mask"), l(vec![a("id_insert"), a("524")])]),
            l(vec![a(":modify"), a("resize_rel_x"), a("resize_mocha_rel_x"), l(vec![a("id_insert"), a("525")])]),
            l(vec![a(":modify"), a("resize_rel_y"), a("resize_mocha_rel_y"), l(vec![a("id_insert"), a("526")])]),
            l(vec![a(":modify"), a("shift_mocha"), a("shift_mocha_mask"), l(vec![a("id_insert"), a("527")])]),
            l(vec![a(":modify"), a("bypass_mocha"), a("bypass_mocha_mask"), l(vec![a("id_insert"), a("531")])]),
            l(vec![a(":modify"), a("show_mocha_only"), a("show_mocha_mask_only"), l(vec![a("id_insert"), a("529")])]),
            l(vec![a(":modify"), a("combine_masks"), l(vec![a("id_insert"), a("530"), a("group_end"), a("mocha")])]),
            l(vec![a(":modify"), a("diffuse_amount"), l(vec![a("widget"), a("ellipse_rad.width")])]),
            l(vec![a(":modify"), a("rel_amount"), l(vec![a("widget"), a("ellipse_rad.rel_xy")])]),
            a("wrap"),
            l(vec![a(":modify"), a("blur_matte"), a("blur_mask"), l(vec![a("new_column")])]),
            l(vec![a(":modify"), a("invert_matte"), a("invert_mask")]),
            l(vec![a(":modify"), a("matte_use"), a("mask_use")]),
        ]),
    ]
}

fn layout_distort_rgb_autogen() -> Vec<Sx> {
    vec![
        a("layout_effect"),
        l(vec![a("Distort_RGB"), a("Distort_RGB_Autogen")]),
        l(vec![a("Source"), a("Lens"), l(vec![a("Matte"), a("Mask")])]),
        l(vec![a("share_all")]),
        l(vec![
            l(vec![a(":modify"), a("mocha_project"), l(vec![a("id_insert"), a("520")])]),
            l(vec![a(":modify"), a("blur_mocha"), a("blur_mocha_mask"), l(vec![a("id_insert"), a("521"), a("group_start"), a("mocha")])]),
            l(vec![a(":modify"), a("mocha_opacity"), a("mocha_mask_opacity"), l(vec![a("id_insert"), a("522")])]),
            l(vec![a(":modify"), a("invert_mocha"), a("invert_mocha_mask"), l(vec![a("id_insert"), a("523")])]),
            l(vec![a(":modify"), a("resize_mocha"), a("resize_mocha_mask"), l(vec![a("id_insert"), a("524")])]),
            l(vec![a(":modify"), a("resize_rel_x"), a("resize_mocha_rel_x"), l(vec![a("id_insert"), a("525")])]),
            l(vec![a(":modify"), a("resize_rel_y"), a("resize_mocha_rel_y"), l(vec![a("id_insert"), a("526")])]),
            l(vec![a(":modify"), a("shift_mocha"), a("shift_mocha_mask"), l(vec![a("id_insert"), a("527")])]),
            l(vec![a(":modify"), a("bypass_mocha"), a("bypass_mocha_mask"), l(vec![a("id_insert"), a("531")])]),
            l(vec![a(":modify"), a("show_mocha_only"), a("show_mocha_mask_only"), l(vec![a("id_insert"), a("529")])]),
            l(vec![a(":modify"), a("combine_masks"), l(vec![a("id_insert"), a("530"), a("group_end"), a("mocha")])]),
            a("amount"),
            a("blur_lens"),
            a("rotate_warp_dir"),
            l(vec![a(":modify"), a("warp_red"), l(vec![a("new_column")])]),
            a("warp_green"),
            a("warp_blue"),
            l(vec![a(":modify"), a("amount_rel"), l(vec![a("id_insert"), a("100"), a("new_column")])]),
            a("wrap"),
            a("filter"),
            l(vec![a(":modify"), a("blur_matte"), a("blur_mask"), l(vec![a("new_column")])]),
            l(vec![a(":modify"), a("invert_matte"), a("invert_mask")]),
            l(vec![a(":modify"), a("matte_use"), a("mask_use")]),
            l(vec![a(":extra"), a("opacity"), a("="), a("1"), l(vec![a("ignore"), a("popup"), a("All_Opaque"), a("Normal"), a("As_Premult")])]),
            l(vec![a(":hardcode"), a("warp_alpha"), a("="), a("0")]),
        ]),
    ]
}

fn layout_drop_shadow() -> Vec<Sx> {
    vec![
        a("layout_effect"),
        a("Drop_Shadow"),
        l(vec![a("Foreground"), a("Background"), a("Matte")]),
        l(vec![]),
        l(vec![
            l(vec![a(":modify"), a("mocha_project"), l(vec![a("id_insert"), a("520")])]),
            l(vec![a(":modify"), a("blur_mocha"), a("blur_mocha_mask"), l(vec![a("id_insert"), a("521"), a("group_start"), a("mocha")])]),
            l(vec![a(":modify"), a("mocha_opacity"), a("mocha_mask_opacity"), l(vec![a("id_insert"), a("522")])]),
            l(vec![a(":modify"), a("invert_mocha"), a("invert_mocha_mask"), l(vec![a("id_insert"), a("523")])]),
            l(vec![a(":modify"), a("resize_mocha"), a("resize_mocha_mask"), l(vec![a("id_insert"), a("524")])]),
            l(vec![a(":modify"), a("resize_rel_x"), a("resize_mocha_rel_x"), l(vec![a("id_insert"), a("525")])]),
            l(vec![a(":modify"), a("resize_rel_y"), a("resize_mocha_rel_y"), l(vec![a("id_insert"), a("526")])]),
            l(vec![a(":modify"), a("shift_mocha"), a("shift_mocha_mask"), l(vec![a("id_insert"), a("527")])]),
            l(vec![a(":modify"), a("bypass_mocha"), a("bypass_mocha_mask"), l(vec![a("id_insert"), a("531")])]),
            l(vec![a(":modify"), a("show_mocha_only"), a("show_mocha_mask_only"), l(vec![a("id_insert"), a("529")])]),
            l(vec![a(":modify"), a("combine_masks"), l(vec![a("id_insert"), a("530"), a("group_end"), a("mocha")])]),
            l(vec![a(":modify"), a("shadow_color"), a("color")]),
            a("shadow_opacity"),
            l(vec![a(":modify"), a("shadow_blur"), l(vec![a("widget"), a("center_radius_center_rel.radius")])]),
            l(vec![a(":modify"), a("shift"), l(vec![a("min"), a("*"), a("normalize_scale"), a("1"), a("slider_range"), a("-1"), a("1"), a("widget"), a("center_radius_center_rel.center")])]),
            l(vec![a(":modify"), a("fg_opacity"), l(vec![a("max"), a("1")])]),
            a("comp_premult"),
            l(vec![a(":modify"), a("matte_use"), l(vec![a("ignore")])]),
            a("invert_matte"),
            l(vec![a(":extra"), a("expand_borders"), a("="), a("1"), l(vec![a("ignore"), a("boolean")])]),
            l(vec![a(":extra"), a("opacity"), a("="), a("1"), l(vec![a("ignore"), a("popup"), a("All_Opaque"), a("Normal"), a("As_Premult")])]),
        ]),
    ]
}

fn layout_hue_sat_bright_autogen() -> Vec<Sx> {
    vec![
        a("layout_effect"),
        l(vec![a("Hue_Sat_Bright"), a("Hue_Sat_Bright_Autogen")]),
        l(vec![a("Source"), a("Mask")]),
        l(vec![]),
        l(vec![
            l(vec![a(":modify"), a("mocha_project"), l(vec![a("id_insert"), a("520")])]),
            l(vec![a(":modify"), a("blur_mocha"), a("blur_mocha_mask"), l(vec![a("id_insert"), a("521"), a("group_start"), a("mocha")])]),
            l(vec![a(":modify"), a("mocha_opacity"), a("mocha_mask_opacity"), l(vec![a("id_insert"), a("522")])]),
            l(vec![a(":modify"), a("invert_mocha"), a("invert_mocha_mask"), l(vec![a("id_insert"), a("523")])]),
            l(vec![a(":modify"), a("resize_mocha"), a("resize_mocha_mask"), l(vec![a("id_insert"), a("524")])]),
            l(vec![a(":modify"), a("resize_rel_x"), a("resize_mocha_rel_x"), l(vec![a("id_insert"), a("525")])]),
            l(vec![a(":modify"), a("resize_rel_y"), a("resize_mocha_rel_y"), l(vec![a("id_insert"), a("526")])]),
            l(vec![a(":modify"), a("shift_mocha"), a("shift_mocha_mask"), l(vec![a("id_insert"), a("527")])]),
            l(vec![a(":modify"), a("bypass_mocha"), a("bypass_mocha_mask"), l(vec![a("id_insert"), a("531")])]),
            l(vec![a(":modify"), a("show_mocha_only"), a("show_mocha_mask_only"), l(vec![a("id_insert"), a("529")])]),
            l(vec![a(":modify"), a("combine_masks"), l(vec![a("id_insert"), a("530"), a("group_end"), a("mocha")])]),
            l(vec![a(":modify"), a("hue_shift"), l(vec![a("min"), a("-1"), a("max"), a("1")])]),
            a("preserve_luma"),
            a("saturation"),
            a("brightness"),
            a("scale_colors"),
            a("offset_darks"),
            l(vec![a(":modify"), a("mask_use"), l(vec![a("id_insert"), a("540")])]),
            l(vec![a(":modify"), a("blur_mask"), l(vec![a("id_insert"), a("541")])]),
            l(vec![a(":modify"), a("invert_mask"), l(vec![a("id_insert"), a("542")])]),
        ]),
    ]
}

fn layout_sharpen_best_quality_autogen() -> Vec<Sx> {
    vec![
        a("layout_effect"),
        l(vec![a("Best"), a("Sharpen_Best_Quality_Autogen")]),
        l(vec![a("Source"), l(vec![a("Matte"), a("Mask")])]),
        l(vec![]),
        l(vec![
            l(vec![a(":modify"), a("mocha_project"), l(vec![a("id_insert"), a("520")])]),
            l(vec![a(":modify"), a("blur_mocha"), a("blur_mocha_mask"), l(vec![a("id_insert"), a("521"), a("group_start"), a("mocha")])]),
            l(vec![a(":modify"), a("mocha_opacity"), a("mocha_mask_opacity"), l(vec![a("id_insert"), a("522")])]),
            l(vec![a(":modify"), a("invert_mocha"), a("invert_mocha_mask"), l(vec![a("id_insert"), a("523")])]),
            l(vec![a(":modify"), a("resize_mocha"), a("resize_mocha_mask"), l(vec![a("id_insert"), a("524")])]),
            l(vec![a(":modify"), a("resize_rel_x"), a("resize_mocha_rel_x"), l(vec![a("id_insert"), a("525")])]),
            l(vec![a(":modify"), a("resize_rel_y"), a("resize_mocha_rel_y"), l(vec![a("id_insert"), a("526")])]),
            l(vec![a(":modify"), a("shift_mocha"), a("shift_mocha_mask"), l(vec![a("id_insert"), a("527")])]),
            l(vec![a(":modify"), a("bypass_mocha"), a("bypass_mocha_mask"), l(vec![a("id_insert"), a("531")])]),
            l(vec![a(":modify"), a("show_mocha_only"), a("show_mocha_mask_only"), l(vec![a("id_insert"), a("529")])]),
            l(vec![a(":modify"), a("combine_masks"), l(vec![a("id_insert"), a("530"), a("group_end"), a("mocha")])]),
            a("sharpen_amp"),
            l(vec![a(":modify"), a("edge_threshold"), l(vec![a("id_insert"), a("105")])]),
            l(vec![a(":modify"), a("small_detail_size"), l(vec![a("id_insert"), a("106")])]),
            l(vec![a(":modify"), a("scale_tiny_details"), l(vec![a("id_insert"), a("107"), a("group_start_open"), a("Detail_Levels")])]),
            l(vec![a(":modify"), a("scale_small_details"), l(vec![a("id_insert"), a("108")])]),
            l(vec![a(":modify"), a("scale_medium_details"), l(vec![a("id_insert"), a("109")])]),
            l(vec![a(":modify"), a("scale_large_details"), l(vec![a("id_insert"), a("110"), a("group_end"), a("Detail_Levels")])]),
            l(vec![a(":modify"), a("sharpen_luma"), l(vec![a("id_insert"), a("100")])]),
            l(vec![a(":modify"), a("sharpen_chroma"), l(vec![a("id_insert"), a("101")])]),
            l(vec![a(":modify"), a("sharpen_red"), l(vec![a("min"), a("*"), a("id_insert"), a("102")])]),
            l(vec![a(":modify"), a("sharpen_green"), l(vec![a("min"), a("*"), a("id_insert"), a("103")])]),
            l(vec![a(":modify"), a("sharpen_blue"), l(vec![a("min"), a("*"), a("id_insert"), a("104")])]),
            l(vec![a(":extra"), a("opacity"), a("="), a("1"), l(vec![a("ignore"), a("shared"), a("popup"), a("All_Opaque"), a("Normal"), a("As_Premult")])]),
            l(vec![a(":modify"), a("mask_use"), l(vec![a("id_insert"), a("540")])]),
            l(vec![a(":modify"), a("blur_mask"), l(vec![a("id_insert"), a("541")])]),
            l(vec![a(":modify"), a("invert_mask"), l(vec![a("id_insert"), a("542")])]),
            l(vec![a(":hardcode"), a("show"), a("="), a("0")]),
        ]),
    ]
}

fn layout_sharpen_fast_autogen() -> Vec<Sx> {
    vec![
        a("layout_effect"),
        l(vec![a("Fast"), a("Sharpen_Fast_Autogen")]),
        l(vec![a("Source"), l(vec![a("Matte"), a("Mask")])]),
        l(vec![]),
        l(vec![
            l(vec![a(":modify"), a("mocha_project"), l(vec![a("id_insert"), a("520")])]),
            l(vec![a(":modify"), a("blur_mocha"), a("blur_mocha_mask"), l(vec![a("id_insert"), a("521"), a("group_start"), a("mocha")])]),
            l(vec![a(":modify"), a("mocha_opacity"), a("mocha_mask_opacity"), l(vec![a("id_insert"), a("522")])]),
            l(vec![a(":modify"), a("invert_mocha"), a("invert_mocha_mask"), l(vec![a("id_insert"), a("523")])]),
            l(vec![a(":modify"), a("resize_mocha"), a("resize_mocha_mask"), l(vec![a("id_insert"), a("524")])]),
            l(vec![a(":modify"), a("resize_rel_x"), a("resize_mocha_rel_x"), l(vec![a("id_insert"), a("525")])]),
            l(vec![a(":modify"), a("resize_rel_y"), a("resize_mocha_rel_y"), l(vec![a("id_insert"), a("526")])]),
            l(vec![a(":modify"), a("shift_mocha"), a("shift_mocha_mask"), l(vec![a("id_insert"), a("527")])]),
            l(vec![a(":modify"), a("bypass_mocha"), a("bypass_mocha_mask"), l(vec![a("id_insert"), a("531")])]),
            l(vec![a(":modify"), a("show_mocha_only"), a("show_mocha_mask_only"), l(vec![a("id_insert"), a("529")])]),
            l(vec![a(":modify"), a("combine_masks"), l(vec![a("id_insert"), a("530"), a("group_end"), a("mocha")])]),
            a("sharpen_amp"),
            a("sharpen_width"),
            l(vec![a(":modify"), a("sharpen_luma"), l(vec![a("id_insert"), a("100")])]),
            l(vec![a(":modify"), a("sharpen_chroma"), l(vec![a("id_insert"), a("101")])]),
            l(vec![a(":modify"), a("sharpen_red"), l(vec![a("min"), a("*"), a("id_insert"), a("102"), a("new_column")])]),
            l(vec![a(":modify"), a("sharpen_green"), l(vec![a("min"), a("*"), a("id_insert"), a("103")])]),
            l(vec![a(":modify"), a("sharpen_blue"), l(vec![a("min"), a("*"), a("id_insert"), a("104")])]),
            l(vec![a(":extra"), a("opacity"), a("="), a("1"), l(vec![a("ignore"), a("shared"), a("popup"), a("All_Opaque"), a("Normal"), a("As_Premult")])]),
            l(vec![a(":modify"), a("mask_use"), l(vec![a("id_insert"), a("540")])]),
            l(vec![a(":modify"), a("blur_mask"), l(vec![a("id_insert"), a("541")])]),
            l(vec![a(":modify"), a("invert_mask"), l(vec![a("id_insert"), a("542")])]),
        ]),
    ]
}

fn layout_time_warp_rgb_autogen() -> Vec<Sx> {
    vec![
        a("layout_effect"),
        l(vec![a("Time_Warp_RGB"), a("Time_Warp_RGB_Autogen")]),
        l(vec![a("Source"), a("Mask")]),
        l(vec![a("cache_alt_frames")]),
        l(vec![
            l(vec![a(":modify"), a("mocha_project"), l(vec![a("id_insert"), a("520")])]),
            l(vec![a(":modify"), a("blur_mocha"), a("blur_mocha_mask"), l(vec![a("id_insert"), a("521"), a("group_start"), a("mocha")])]),
            l(vec![a(":modify"), a("mocha_opacity"), a("mocha_mask_opacity"), l(vec![a("id_insert"), a("522")])]),
            l(vec![a(":modify"), a("invert_mocha"), a("invert_mocha_mask"), l(vec![a("id_insert"), a("523")])]),
            l(vec![a(":modify"), a("resize_mocha"), a("resize_mocha_mask"), l(vec![a("id_insert"), a("524")])]),
            l(vec![a(":modify"), a("resize_rel_x"), a("resize_mocha_rel_x"), l(vec![a("id_insert"), a("525")])]),
            l(vec![a(":modify"), a("resize_rel_y"), a("resize_mocha_rel_y"), l(vec![a("id_insert"), a("526")])]),
            l(vec![a(":modify"), a("shift_mocha"), a("shift_mocha_mask"), l(vec![a("id_insert"), a("527")])]),
            l(vec![a(":modify"), a("bypass_mocha"), a("bypass_mocha_mask"), l(vec![a("id_insert"), a("531")])]),
            l(vec![a(":modify"), a("show_mocha_only"), a("show_mocha_mask_only"), l(vec![a("id_insert"), a("529")])]),
            l(vec![a(":modify"), a("combine_masks"), l(vec![a("id_insert"), a("530"), a("group_end"), a("mocha")])]),
            l(vec![a(":modify"), a("red_shift_frames"), a("shift_frames_red")]),
            l(vec![a(":modify"), a("green_shift_frames"), a("shift_frames_green")]),
            l(vec![a(":modify"), a("blue_shift_frames"), a("shift_frames_blue")]),
            a("clamp_chroma"),
            l(vec![a(":extra"), a("input_opacity"), a("="), a("1"), l(vec![a("new_column"), a("ignore"), a("popup"), a("All_Opaque"), a("Normal"), a("As_Premult")])]),
            l(vec![a(":extra"), a("output_opacity"), a("="), a("0"), l(vec![a("ignore"), a("popup"), a("All_Opaque"), a("Copy_From_Input")])]),
            l(vec![a(":modify"), a("mask_use"), l(vec![a("id_insert"), a("540")])]),
            l(vec![a(":modify"), a("blur_mask"), l(vec![a("id_insert"), a("541")])]),
            l(vec![a(":modify"), a("invert_mask"), l(vec![a("id_insert"), a("542")])]),
        ]),
    ]
}

fn layout_trails_autogen() -> Vec<Sx> {
    vec![
        a("layout_effect"),
        l(vec![a("Trails"), a("Trails_Autogen")]),
        l(vec![a("Source"), a("Mask")]),
        l(vec![]),
        l(vec![
            l(vec![a(":modify"), a("mocha_project"), l(vec![a("id_insert"), a("520")])]),
            l(vec![a(":modify"), a("blur_mocha"), a("blur_mocha_mask"), l(vec![a("id_insert"), a("521"), a("group_start"), a("mocha")])]),
            l(vec![a(":modify"), a("mocha_opacity"), a("mocha_mask_opacity"), l(vec![a("id_insert"), a("522")])]),
            l(vec![a(":modify"), a("invert_mocha"), a("invert_mocha_mask"), l(vec![a("id_insert"), a("523")])]),
            l(vec![a(":modify"), a("resize_mocha"), a("resize_mocha_mask"), l(vec![a("id_insert"), a("524")])]),
            l(vec![a(":modify"), a("resize_rel_x"), a("resize_mocha_rel_x"), l(vec![a("id_insert"), a("525")])]),
            l(vec![a(":modify"), a("resize_rel_y"), a("resize_mocha_rel_y"), l(vec![a("id_insert"), a("526")])]),
            l(vec![a(":modify"), a("shift_mocha"), a("shift_mocha_mask"), l(vec![a("id_insert"), a("527")])]),
            l(vec![a(":modify"), a("bypass_mocha"), a("bypass_mocha_mask"), l(vec![a("id_insert"), a("531")])]),
            l(vec![a(":modify"), a("show_mocha_only"), a("show_mocha_mask_only"), l(vec![a("id_insert"), a("529")])]),
            l(vec![a(":modify"), a("combine_masks"), l(vec![a("id_insert"), a("530"), a("group_end"), a("mocha")])]),
            l(vec![a(":modify"), a("prev_brightness"), a("fade")]),
            l(vec![a(":modify"), a("prev_color"), a("fade_color")]),
            a("prev_hue_shift"),
            l(vec![a(":modify"), a("combine_new"), a("combine"), l(vec![a("popup"), a("Ave"), a("Max"), a("Screen"), a("Add"), a("Over"), a("Under"), a("Min")])]),
            a("new_color"),
            l(vec![a(":modify"), a("new_opacity"), l(vec![a("max"), a("10")])]),
            l(vec![a(":modify"), a("blur_amount"), l(vec![a("new_column")])]),
            a("blur_rel"),
            l(vec![a(":extra"), a("opacity"), a("="), a("1"), l(vec![a("ignore"), a("popup"), a("All_Opaque"), a("Normal"), a("As_Premult")])]),
            l(vec![a(":modify"), a("mask_use"), l(vec![a("id_insert"), a("540")])]),
            l(vec![a(":modify"), a("blur_mask"), l(vec![a("id_insert"), a("541")])]),
            l(vec![a(":modify"), a("invert_mask"), l(vec![a("id_insert"), a("542")])]),
        ]),
    ]
}

fn layout_warp_perspective_autogen() -> Vec<Sx> {
    vec![
        a("layout_effect"),
        l(vec![a("Warp_Perspective"), a("Warp_Perspective_Autogen")]),
        l(vec![a("Source"), l(vec![a("Matte"), a("Mask")])]),
        l(vec![]),
        l(vec![
            l(vec![a(":modify"), a("mocha_project"), l(vec![a("id_insert"), a("520")])]),
            l(vec![a(":modify"), a("blur_mocha"), a("blur_mocha_mask"), l(vec![a("id_insert"), a("521"), a("group_start"), a("mocha")])]),
            l(vec![a(":modify"), a("mocha_opacity"), a("mocha_mask_opacity"), l(vec![a("id_insert"), a("522")])]),
            l(vec![a(":modify"), a("invert_mocha"), a("invert_mocha_mask"), l(vec![a("id_insert"), a("523")])]),
            l(vec![a(":modify"), a("resize_mocha"), a("resize_mocha_mask"), l(vec![a("id_insert"), a("524")])]),
            l(vec![a(":modify"), a("resize_rel_x"), a("resize_mocha_rel_x"), l(vec![a("id_insert"), a("525")])]),
            l(vec![a(":modify"), a("resize_rel_y"), a("resize_mocha_rel_y"), l(vec![a("id_insert"), a("526")])]),
            l(vec![a(":modify"), a("shift_mocha"), a("shift_mocha_mask"), l(vec![a("id_insert"), a("527")])]),
            l(vec![a(":modify"), a("bypass_mocha"), a("bypass_mocha_mask"), l(vec![a("id_insert"), a("531")])]),
            l(vec![a(":modify"), a("show_mocha_only"), a("show_mocha_mask_only"), l(vec![a("id_insert"), a("529")])]),
            l(vec![a(":modify"), a("combine_masks"), l(vec![a("id_insert"), a("530"), a("group_end"), a("mocha")])]),
            l(vec![a(":modify"), a("latitude"), l(vec![a("min"), a("-89"), a("max"), a("89")])]),
            a("swing"),
            a("roll"),
            l(vec![a(":modify"), a("z_dist"), l(vec![a("min"), a(".001"), a("big_slider_range"), a(".001"), a("20")])]),
            l(vec![a(":modify"), a("tele_lens_width"), l(vec![a("max"), a("3.0")])]),
            l(vec![a(":modify"), a("shift_orig"), a("shift"), l(vec![a("new_column"), a("normalize_scale"), a("1")])]),
            a("wrap"),
            l(vec![a(":modify"), a("wrap_above_horizon"), l(vec![a("id_insert"), a("100")])]),
            l(vec![a(":modify"), a("filter"), l(vec![a("shared")])]),
            l(vec![a(":modify"), a("blur_matte"), a("blur_mask"), l(vec![a("shared"), a("new_column")])]),
            l(vec![a(":modify"), a("invert_matte"), a("invert_mask"), l(vec![a("shared")])]),
            l(vec![a(":modify"), a("matte_use"), a("mask_use")]),
            l(vec![a(":extra"), a("opacity"), a("="), a("1"), l(vec![a("ignore"), a("popup"), a("All_Opaque"), a("Normal"), a("As_Premult")])]),
            l(vec![a(":hardcode"), a("comp_premult"), a("="), a("0")]),
        ]),
    ]
}

fn eff_flicker(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "amplitude", D::N(0.2), &["slider_max", "1", "auto_anim_floor0"]),
        param(a, "blue_amp", D::N(1.0), &["slider_max", "2", "big_slider_max", "5"]),
        param(a, "brightness", D::N(1.0), &[]),
        param(a, "green_amp", D::N(1.0), &["slider_max", "2", "big_slider_max", "5"]),
        param(a, "rand_color_amp", D::N(0.0), &["slider_max", "2", "big_slider_max", "5"]),
        param(a, "rand_freq", D::N(30.0), &["slider_max", "60"]),
        param(a, "rand_luma_amp", D::N(1.0), &["slider_max", "2", "big_slider_max", "5"]),
        param(a, "red_amp", D::N(1.0), &["slider_max", "2", "big_slider_max", "5"]),
        param(a, "seed", D::N(0.123), &[]),
        param(a, "wave_amp", D::N(0.0), &["slider_max", "2", "big_slider_max", "5"]),
        param(a, "wave_b_phase", D::N(0.0), &["min", "*", "slider_range", "-1", "1"]),
        param(a, "wave_freq", D::N(5.0), &["slider_max", "60"]),
        param(a, "wave_g_phase", D::N(0.0), &["min", "*", "slider_range", "-1", "1"]),
        param(a, "wave_r_phase", D::N(0.0), &["min", "*", "slider_range", "-1", "1"]),
    ];
    let body = lower(a, &lt(vec![
        (&["color_phase"], call("v3", vec![id("wave_r_phase"), id("wave_g_phase"), id("wave_b_phase")])),
        (&["amp3"], call("*", vec![id("amplitude"), call("v3", vec![id("red_amp"), id("green_amp"), id("blue_amp")])])),
        (&["t2"], call("v2", vec![call("get_time", vec![]), n(0.0)])),
        (&["rand"], call("*", vec![
            call("+", vec![
                call("*", vec![
                    id("rand_luma_amp"),
                    call("-", vec![
                        call("warp_noise", vec![id("t2"), id("rand_freq"), n(1.0), id("seed")]),
                        n(0.5),
                    ]),
                ]),
                call("*", vec![
                    id("rand_color_amp"),
                    call("-", vec![
                        call("warp_noise3", vec![
                            id("t2"),
                            id("rand_freq"),
                            n(1.0),
                            call("+", vec![id("seed"), n(0.3333)]),
                        ]),
                        n(0.5),
                    ]),
                ]),
            ]),
            n(2.0),
        ])),
        (&["wave"], call("*", vec![
            call("sin", vec![
                call("+", vec![
                    call("*", vec![call("get_time", vec![]), id("wave_freq")]),
                    call("-", vec![n(0.25), id("color_phase")]),
                ]),
            ]),
            id("wave_amp"),
        ])),
    ], call("*", vec![
        id("Source"),
        call("+", vec![
            id("brightness"),
            call("*", vec![id("amp3"), call("+", vec![id("rand"), id("wave")])]),
        ]),
    ])));
    EffectDef { name: "Flicker".into(), va: "0x13fc760".into(), inputs: vec![inp("Source", None)], flags: strv(&["generates_color", "skip_auto_mocha", "skip_auto_mask"]), params, body }
}

fn eff_glow(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "comp_premult", D::N(0.0), &["shared", "boolean"]),
        param(a, "affect_alpha", D::N(0.0), &["shared", "dont_save", "slider_max", "1", "default_to_1_if_postdiv"]),
        param(a, "brightness", D::N(2.0), &["auto_anim_floor0"]),
        param(a, "color", D::V(&[1.0, 1.0, 1.0]), &["color"]),
        param(a, "combine", D::N(2.0), &["popup", "Mult", "Add", "Screen", "Difference", "Overlay"]),
        param(a, "glow_from_alpha", D::N(0.0), &["shared", "max", "1"]),
        param(a, "glow_under_source", D::N(0.0), &["shared", "max", "1"]),
        param(a, "invert_mask", D::N(0.0), &["shared", "show_with_mask", "boolean"]),
        param(a, "light_bg", D::N(0.0), &["shared", "max", "1"]),
        param(a, "scale_background", D::N(1.0), &["shared", "slider_max", "1"]),
        param(a, "scale_source", D::N(1.0), &["shared", "max", "1"]),
        param(a, "show", D::N(0.0), &["popup", "Result", "Threshold"]),
        param(a, "subpixel", D::N(1.0), &["boolean"]),
        param(a, "threshold", D::N(0.5), &["slider_max", "1"]),
        param(a, "threshold_add_color", D::V(&[0.0, 0.0, 0.0]), &["color"]),
        param(a, "width", D::N(0.4), &["normalize_scale", "4", "slider_max", "2"]),
        param(a, "width_blue", D::N(1.4), &["widget", "glow.size_blue"]),
        param(a, "width_green", D::N(1.2), &["widget", "glow.size_green"]),
        param(a, "width_red", D::N(1.0), &["widget", "glow.size_red"]),
        param(a, "width_x", D::N(1.0), &["widget", "glow.size_x"]),
        param(a, "width_y", D::N(1.0), &["widget", "glow.size_y"]),
        param(a, "edge_mode", D::N(1.0), &["old_default", "v7", "0"]),
        param(a, "atmosphere_amp", D::N(0.0), &["slider_max", "5"]),
        param(a, "atmosphere_freq", D::N(1.0), &["min", ".1"]),
        param(a, "atmosphere_detail", D::N(0.6), &["max", "1"]),
        param(a, "atmosphere_seed", D::N(0.123), &[]),
        param(a, "atmosphere_speed", D::N(1.0), &["min", "*", "slider_range", "-5", "5"]),
        param(a, "eclipse_with_bg", D::N(0.0), &["max", "1"]),
        param(a, "mocha_project", D::N(0.0), &["optional", "mocha_data", "builder_disable", "shared"]),
        param(a, "blur_mocha_mask", D::N(0.0), &["optional", "disable_motion_blur", "normalize_scale", "4", "slider_max", "0.2", "builder_disable", "shared"]),
        param(a, "mocha_mask_opacity", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "1", "shared"]),
        param(a, "invert_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "resize_mocha_mask", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_x", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_y", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "shift_mocha_mask", D::V(&[0.0, 0.0]), &["optional", "disable_motion_blur", "builder_disable", "normalize_scale", "1", "min", "*", "slider_range", "-2", "2", "shared"]),
        param(a, "bypass_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "show_mocha_mask_only", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "combine_masks", D::N(0.0), &["optional", "popup", "Union", "Intersect", "Mocha_Only", "builder_disable", "shared"]),
    ];
    let body = lower(a, &lt(vec![
        (&["source"], call("maybe_add_alpha", vec![id("Source"), id("SparksMatte"), id("comp_premult")])),
        (&["mask", "mocha_mask"], call("PROCESS_COLOR_MASKS", vec![
            id("Mask"),
            id("invert_mask"),
            n(0.0),
            n(1.0),
            id("mocha_project"),
            id("bypass_mocha_mask"),
            id("invert_mocha_mask"),
            id("blur_mocha_mask"),
            id("combine_masks"),
            id("mocha_mask_opacity"),
            id("resize_mocha_mask"),
            id("resize_mocha_rel_x"),
            id("resize_mocha_rel_y"),
            id("shift_mocha_mask"),
        ])),
        (&["s"], call("interp", vec![id("source"), call("coerce_to_alpha", vec![id("source")]), id("glow_from_alpha")])),
        (&["atmosphere"], call("RAYS_ATMOSPHERE", vec![
            id("atmosphere_amp"),
            id("atmosphere_freq"),
            id("atmosphere_detail"),
            id("atmosphere_seed"),
            id("atmosphere_speed"),
        ])),
        (&["glow3"], call("*", vec![
            id("atmosphere"),
            call("glow", vec![
                call("*", vec![id("s"), id("mask")]),
                call("+", vec![id("threshold"), id("threshold_add_color")]),
                call("v2", vec![id("width_x"), id("width_y")]),
                call("*", vec![
                    id("width"),
                    call("v3", vec![id("width_red"), id("width_green"), id("width_blue")]),
                ]),
                call("*", vec![id("brightness"), id("color")]),
                id("subpixel"),
                id("edge_mode"),
                id("show"),
            ]),
        ])),
        (&["dest_threshold"], id("glow3")),
        (&["glow4"], call("glow_rgba_combine", vec![
            id("Background"),
            id("source"),
            id("affect_alpha"),
            id("combine"),
            id("dest_threshold"),
            id("glow3"),
            id("glow_under_source"),
            id("light_bg"),
            id("scale_background"),
            id("scale_source"),
            id("show"),
            id("eclipse_with_bg"),
        ])),
    ], call("if+", vec![id("show_mocha_mask_only"), id("mocha_mask"), id("glow4")])));
    EffectDef { name: "Glow".into(), va: "0x1406320".into(), inputs: vec![inp("Source", None), inp("Background", Some("0")), inp("SparksMatte", Some("1")), inp("Mask", Some("1"))], flags: strv(&["rgba_ok", "generates_color", "generates_alpha"]), params, body }
}

fn eff_swishpan(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "transition_dir", D::N(0.0), &["popup", "Wipe_Off_to_Bg", "Wipe_On_from_Bg"]),
        param(a, "auto_trans", D::N(0.0), &["popup", "\"No: use Wipe%\"", "\"Yes: ignore Wipe%\""]),
        param(a, "wipe_amt", D::N(0.0), &["shared", "autotrans", "auto_anim_ramp_up"]),
        param(a, "swish_auto_trans", D::N(0.0), &["hidden"]),
        param(a, "blur_amount", D::N(2.0), &["normalize_scale", "4", "slider_max", "2"]),
        param(a, "direction", D::N(0.0), &["popup", "Left", "Right", "Up", "Down"]),
        param(a, "overlap", D::N(0.0), &["min", "*", "slider_range", "-1", "1"]),
        param(a, "slow_in", D::N(0.5), &[]),
        param(a, "slow_out", D::N(0.5), &[]),
    ];
    let body = lower(a, &lt(vec![
        (&["amount"], id("wipe_amt")),
        (&["from", "to"], call("transition_swap_inputs", vec![id("From"), id("To"), id("transition_dir")])),
        (&["amount2"], call("transition_amount", vec![id("auto_trans"), id("amount")])),
        (&["time"], call("slow_in_out", vec![id("amount2"), id("slow_in"), id("slow_out"), n(0.0)])),
        (&["blur_use_auto_trans"], call("max", vec![id("auto_trans"), id("swish_auto_trans")])),
        (&["blur_width"], call("if+", vec![
            id("blur_use_auto_trans"),
            call("*", vec![
                id("blur_amount"),
                call("slow_in_out", vec![id("amount2"), id("slow_in"), id("slow_out"), n(1.0)]),
            ]),
            call("*", vec![
                id("blur_amount"),
                call("*", vec![
                    call("diff", vec![
                        call("slow_in_out", vec![
                            call("get_curve_val", vec![id("amount"), n(0.5)]),
                            id("slow_in"),
                            id("slow_out"),
                            n(0.0),
                        ]),
                        call("slow_in_out", vec![
                            call("get_curve_val", vec![id("amount"), n(-0.5)]),
                            id("slow_in"),
                            id("slow_out"),
                            n(0.0),
                        ]),
                    ]),
                    call("get_total_frames", vec![]),
                ]),
            ]),
        ])),
        (&["shift"], call("if>", vec![
            id("direction"),
            n(2.0),
            call("v2", vec![n(0.0), call("*", vec![n(2.0), call("get_aspect_ratio", vec![])])]),
            call("if>", vec![
                id("direction"),
                n(1.0),
                call("v2", vec![n(0.0), call("*", vec![n(-2.0), call("get_aspect_ratio", vec![])])]),
                call("if+", vec![id("direction"), v(&[-2.0, 0.0]), v(&[2.0, 0.0])]),
            ]),
        ])),
        (&["over"], call("*", vec![id("shift"), id("overlap")])),
        (&["shift2"], call("-", vec![id("shift"), id("over")])),
        (&["from_coords"], call("+", vec![id("xy"), call("*", vec![id("shift2"), id("time")])])),
        (&["to_coords"], call("-", vec![id("from_coords"), id("shift2")])),
        (&["blur_dir"], call("if>", vec![id("direction"), n(1.0), v(&[0.0, 1.0]), v(&[1.0, 0.0])])),
        (&["warp_from"], call("warp", vec![
            call("make_mipmap", vec![id("from"), v(&[0.0, 0.0]), n(1.0)]),
            id("from_coords"),
            v(&[0.0, 0.0]),
            n(1.0),
        ])),
        (&["warp_to"], call("warp", vec![
            call("make_mipmap", vec![id("to"), v(&[0.0, 0.0]), n(1.0)]),
            id("to_coords"),
            v(&[0.0, 0.0]),
            n(1.0),
        ])),
    ], call("blur", vec![
        call("screen_super_if_needed", vec![id("warp_from"), id("warp_to")]),
        call("*", vec![id("blur_dir"), id("blur_width")]),
        n(0.0),
        n(1.0),
        n(0.0),
    ])));
    EffectDef { name: "SwishPan".into(), va: "0x1481ba0".into(), inputs: vec![inp("From", Some("0")), inp("To", Some("0"))], flags: strv(&["rgba_ok", "transition"]), params, body }
}

fn defun_combine_color_masks(a: &mut Arena) -> DefunDef {
    let body = lower(a, &call("if_neq_const", vec![
        id("Mask"),
        n(1.0),
        call("if_neq_const", vec![
            id("m2"),
            n(1.0),
            call("switch", vec![
                id("combine_mode"),
                n(0.0),
                call("max", vec![id("m1"), id("m2")]),
                n(1.0),
                call("*", vec![id("m1"), id("m2")]),
                n(2.0),
                id("m2"),
                call("max", vec![id("m1"), id("m2")]),
            ]),
            id("m1"),
        ]),
        id("m2"),
    ]));
    DefunDef { name: "COMBINE_COLOR_MASKS".into(), va: "0x13bbb20".into(), args: strv(&["Mask", "m1", "m2", "combine_mode"]), body }
}

fn defun_process_color_masks(a: &mut Arena) -> DefunDef {
    let body = lower(a, &lt(vec![
        (&["mask1", "mask3", "mask_color_p"], call("color_mask", vec![id("Mask"), id("mask_type"), id("invert_mask"), id("blur_mask")])),
        (&["mocha_mask"], call("MOCHA_MASK", vec![
            id("mocha_project"),
            id("bypass_mocha_mask"),
            id("invert_mocha_mask"),
            id("blur_mocha_mask"),
            id("mocha_mask_opacity"),
            id("resize_mocha_mask"),
            id("resize_mocha_rel_x"),
            id("resize_mocha_rel_y"),
            id("shift_mocha_mask"),
        ])),
        (&["combined_mask1"], call("COMBINE_MASKS", vec![id("Mask"), id("mask1"), id("mocha_mask"), id("combine_mode")])),
        (&["combined_mask3"], call("COMBINE_COLOR_MASKS", vec![id("Mask"), id("mask3"), id("mocha_mask"), id("combine_mode")])),
        (&["mask"], call("if+", vec![id("mask_color_p"), id("combined_mask3"), id("combined_mask1")])),
    ], call("values", vec![id("mask"), id("mocha_mask")])));
    DefunDef { name: "PROCESS_COLOR_MASKS".into(), va: "0x14623e0".into(), args: strv(&["Mask", "invert_mask", "blur_mask", "mask_type", "mocha_project", "bypass_mocha_mask", "invert_mocha_mask", "blur_mocha_mask", "combine_mode", "mocha_mask_opacity", "resize_mocha_mask", "resize_mocha_rel_x", "resize_mocha_rel_y", "shift_mocha_mask"]), body }
}

fn defun_rays_atmosphere(a: &mut Arena) -> DefunDef {
    let body = lower(a, &call("-", vec![
        n(1.0),
        call("*", vec![
            call("simplex_noise", vec![
                call("append", vec![
                    call("*", vec![id("atmosphere_freq"), id("xy")]),
                    call("*", vec![
                        call("*", vec![
                            id("atmosphere_speed"),
                            call("+", vec![call("get_time", vec![]), n(1.0)]),
                        ]),
                        n(0.1),
                    ]),
                ]),
                n(8.0),
                id("atmosphere_turbulence"),
                id("atmosphere_seed"),
                n(0.0),
            ]),
            id("atmosphere_amp"),
        ]),
    ]));
    DefunDef { name: "RAYS_ATMOSPHERE".into(), va: "0x1469000".into(), args: strv(&["atmosphere_amp", "atmosphere_freq", "atmosphere_turbulence", "atmosphere_seed", "atmosphere_speed"]), body }
}

fn defun_color_mask(a: &mut Arena) -> DefunDef {
    let body = lower(a, &lt(vec![
        (&["mask1"], call("blur", vec![
            call("if+", vec![
                id("invert_mask"),
                call("one_minus", vec![
                    call("if>", vec![
                        id("mask_type"),
                        n(1.0),
                        call("coerce_to_alpha", vec![id("Mask")]),
                        call("monochrome", vec![id("Mask")]),
                    ]),
                ]),
                call("if>", vec![
                    id("mask_type"),
                    n(1.0),
                    call("coerce_to_alpha", vec![id("Mask")]),
                    call("monochrome", vec![id("Mask")]),
                ]),
            ]),
            id("blur_mask"),
            n(3.0),
            n(1.0),
            n(0.0),
        ])),
        (&["mask3"], call("blur", vec![
            call("if+", vec![
                id("invert_mask"),
                call("one_minus", vec![call("remove_alpha", vec![id("Mask")])]),
                call("remove_alpha", vec![id("Mask")]),
            ]),
            id("blur_mask"),
            n(3.0),
            n(1.0),
            n(0.0),
        ])),
        (&["mask_color_p"], call("mod", vec![id("mask_type"), n(2.0)])),
    ], call("values", vec![id("mask1"), id("mask3"), id("mask_color_p")])));
    DefunDef { name: "color_mask".into(), va: "0x14d1e40".into(), args: strv(&["Mask", "mask_type", "invert_mask", "blur_mask"]), body }
}

fn defun_glow_rgba_combine(a: &mut Arena) -> DefunDef {
    let body = lower(a, &lt(vec![
        (&["glowB"], call("*", vec![
            id("glow3"),
            call("interp", vec![
                n(1.0),
                call("*", vec![
                    n(2.0),
                    call("if_neq_const", vec![id("Background"), n(0.0), id("Background"), id("Source")]),
                ]),
                id("light_bg"),
            ]),
        ])),
        (&["glow"], call("alpha_from_rgb_max", vec![id("glowB"), id("affect_alpha"), id("Output")])),
        (&["source"], call("*", vec![id("Source"), id("scale_source")])),
        (&["back"], call("if_neq_const", vec![
            id("Background"),
            n(0.0),
            call("scale_rgb", vec![id("Background"), id("scale_background")]),
            call("coerce_up", vec![n(0.0), id("Output")]),
        ])),
        (&["source_on_back"], call("comp_rgba", vec![id("back"), id("source")])),
        (&["elnum_source_back"], call("coerce_up", vec![id("glow"), id("source_on_back")])),
        (&["elnum_back"], call("coerce_up", vec![id("glow"), id("back")])),
        (&["glow_on_src_back"], call("if>", vec![
            id("combine"),
            n(3.0),
            call("overlay_super_if_needed", vec![id("glow"), id("source_on_back")]),
            call("if>", vec![
                id("combine"),
                n(2.0),
                call("v3_to_v4_if4", vec![
                    call("diff", vec![
                        call("remove_alpha", vec![id("glow")]),
                        call("remove_alpha", vec![id("source_on_back")]),
                    ]),
                    call("screen", vec![
                        call("get_alpha", vec![id("glow")]),
                        call("get_alpha", vec![id("source_on_back")]),
                    ]),
                    id("elnum_source_back"),
                ]),
                call("if>", vec![
                    id("combine"),
                    n(1.0),
                    call("screen_super_if_needed", vec![id("glow"), id("source_on_back")]),
                    call("if+", vec![
                        id("combine"),
                        call("+", vec![id("glow"), id("source_on_back")]),
                        call("*", vec![id("glow"), id("source_on_back")]),
                    ]),
                ]),
            ]),
        ])),
        (&["glow_on_back"], call("if>", vec![
            id("combine"),
            n(3.0),
            call("overlay_super_if_needed", vec![id("glow"), id("back")]),
            call("if>", vec![
                id("combine"),
                n(2.0),
                call("v3_to_v4_if4", vec![
                    call("diff", vec![
                        call("remove_alpha", vec![id("glow")]),
                        call("remove_alpha", vec![id("back")]),
                    ]),
                    call("screen", vec![
                        call("get_alpha", vec![id("glow")]),
                        call("get_alpha", vec![id("back")]),
                    ]),
                    id("elnum_back"),
                ]),
                call("if>", vec![
                    id("combine"),
                    n(1.0),
                    call("screen_super_if_needed", vec![id("glow"), id("back")]),
                    call("if+", vec![
                        id("combine"),
                        call("+", vec![id("glow"), id("back")]),
                        call("*", vec![id("glow"), id("back")]),
                    ]),
                ]),
            ]),
        ])),
        (&["dest_normal"], call("interp", vec![
            call("interp", vec![
                id("glow_on_src_back"),
                call("+", vec![id("source_on_back"), id("glow")]),
                id("light_bg"),
            ]),
            call("comp_rgba", vec![
                call("interp", vec![
                    id("glow_on_back"),
                    call("+", vec![id("back"), id("glow")]),
                    id("light_bg"),
                ]),
                id("source"),
            ]),
            call("screen", vec![id("glow_under_source"), id("light_bg")]),
        ])),
        (&["dest_eclipse"], call("+", vec![
            call("*", vec![
                id("Source"),
                call("*", vec![call("one_minus", vec![id("eclipse_with_bg")]), id("scale_source")]),
            ]),
            call("-", vec![
                id("glow"),
                call("*", vec![id("glow"), call("*", vec![id("source"), id("scale_source")])]),
            ]),
        ])),
        (&["result"], call("if+", vec![
            id("show"),
            call("coerce_up_a1", vec![id("dest_threshold"), id("dest_normal")]),
            call("if+", vec![id("eclipse_with_bg"), id("dest_eclipse"), id("dest_normal")]),
        ])),
    ], id("result")));
    DefunDef { name: "glow_rgba_combine".into(), va: "0x14d43e0".into(), args: strv(&["Background", "Source", "affect_alpha", "combine", "dest_threshold", "glow3", "glow_under_source", "light_bg", "scale_background", "scale_source", "show", "eclipse_with_bg"]), body }
}

fn defun_transition_amount(a: &mut Arena) -> DefunDef {
    let body = lower(a, &call("if+", vec![id("auto_trans"), call("get_percent_time", vec![]), id("orig_amount")]));
    DefunDef { name: "transition_amount".into(), va: "0x14e3ac0".into(), args: strv(&["auto_trans", "orig_amount"]), body }
}

fn defun_transition_swap_inputs(a: &mut Arena) -> DefunDef {
    let body = lower(a, &call("values", vec![
        call("if+", vec![id("transition_dir"), id("bg"), id("fg")]),
        call("if+", vec![id("transition_dir"), id("fg"), id("bg")]),
    ]));
    DefunDef { name: "transition_swap_inputs".into(), va: "0x14e3b40".into(), args: strv(&["fg", "bg", "transition_dir"]), body }
}

fn layout_flicker() -> Vec<Sx> {
    vec![
        a("layout_effect"),
        a("Flicker"),
        l(vec![a("Source")]),
        l(vec![]),
        l(vec![
            a("amplitude"),
            a("rand_luma_amp"),
            a("rand_color_amp"),
            a("rand_freq"),
            l(vec![a(":modify"), a("wave_amp"), l(vec![a("group_start_open"), a("wave"), a("new_column")])]),
            a("wave_freq"),
            a("wave_r_phase"),
            a("wave_g_phase"),
            l(vec![a(":modify"), a("wave_b_phase"), l(vec![a("group_end"), a("wave")])]),
            l(vec![a(":modify"), a("red_amp"), l(vec![a("new_column")])]),
            a("green_amp"),
            a("blue_amp"),
            a("brightness"),
            a("seed"),
        ]),
    ]
}

fn layout_glow() -> Vec<Sx> {
    vec![
        a("layout_effect"),
        a("Glow"),
        l(vec![a("Source"), a("Background"), l(vec![a("Matte"), a("Mask")])]),
        l(vec![a("mask_color")]),
        l(vec![
            l(vec![a(":modify"), a("mocha_project"), l(vec![a("id_insert"), a("520")])]),
            l(vec![a(":modify"), a("blur_mocha"), a("blur_mocha_mask"), l(vec![a("id_insert"), a("521"), a("group_start"), a("mocha")])]),
            l(vec![a(":modify"), a("mocha_opacity"), a("mocha_mask_opacity"), l(vec![a("id_insert"), a("522")])]),
            l(vec![a(":modify"), a("invert_mocha"), a("invert_mocha_mask"), l(vec![a("id_insert"), a("523")])]),
            l(vec![a(":modify"), a("resize_mocha"), a("resize_mocha_mask"), l(vec![a("id_insert"), a("524")])]),
            l(vec![a(":modify"), a("resize_rel_x"), a("resize_mocha_rel_x"), l(vec![a("id_insert"), a("525")])]),
            l(vec![a(":modify"), a("resize_rel_y"), a("resize_mocha_rel_y"), l(vec![a("id_insert"), a("526")])]),
            l(vec![a(":modify"), a("shift_mocha"), a("shift_mocha_mask"), l(vec![a("id_insert"), a("527")])]),
            l(vec![a(":modify"), a("bypass_mocha"), a("bypass_mocha_mask"), l(vec![a("id_insert"), a("531")])]),
            l(vec![a(":modify"), a("show_mocha_only"), a("show_mocha_mask_only"), l(vec![a("id_insert"), a("529")])]),
            l(vec![a(":modify"), a("combine_masks"), l(vec![a("id_insert"), a("530"), a("group_end"), a("mocha")])]),
            a("brightness"),
            a("color"),
            a("threshold"),
            l(vec![a(":modify"), a("threshold_add_color"), l(vec![a("shared")])]),
            l(vec![a(":modify"), a("glow_width"), a("width"), l(vec![a("new_column"), a("widget"), a("glow.size")])]),
            a("width_x"),
            a("width_y"),
            a("width_red"),
            a("width_green"),
            a("width_blue"),
            l(vec![a(":modify"), a("subpixel"), l(vec![a("shared")])]),
            l(vec![a(":modify"), a("show"), l(vec![a("id_insert"), a("100")])]),
            l(vec![a(":modify"), a("combine"), l(vec![a("id_insert"), a("101")])]),
            l(vec![a(":modify"), a("edge_mode"), l(vec![a("id_insert"), a("102"), a("popup"), a("Transparent"), a("Reflect")])]),
            a("affect_alpha"),
            a("glow_from_alpha"),
            a("glow_under_source"),
            l(vec![a(":modify"), a("light_background"), a("light_bg")]),
            l(vec![a(":modify"), a("source_opacity"), a("scale_source")]),
            l(vec![a(":modify"), a("bg_brightness"), a("scale_background"), l(vec![a("ignore")])]),
            l(vec![a(":modify"), a("invert_matte"), a("invert_mask"), l(vec![a("new_column")])]),
            l(vec![a(":extra"), a("expand_borders"), a("="), a("0"), l(vec![a("ignore"), a("boolean")])]),
            l(vec![a(":modify"), a("atmosphere_amp"), l(vec![a("group_start"), a("atmosphere"), a("id_insert"), a("103")])]),
            l(vec![a(":modify"), a("atmosphere_freq"), l(vec![a("max"), a("20"), a("id_insert"), a("104")])]),
            l(vec![a(":modify"), a("atmosphere_detail"), l(vec![a("id_insert"), a("105")])]),
            l(vec![a(":modify"), a("atmosphere_seed"), l(vec![a("id_insert"), a("106")])]),
            l(vec![a(":modify"), a("atmosphere_speed"), l(vec![a("group_end"), a("atmosphere"), a("id_insert"), a("107")])]),
            l(vec![a(":extra"), a("opacity"), a("="), a("1"), l(vec![a("ignore"), a("popup"), a("All_Opaque"), a("Normal"), a("As_Premult")])]),
            l(vec![a(":hardcode"), a("eclipse_with_bg"), a("="), a("0")]),
            l(vec![a(":hardcode"), a("comp_premult"), a("="), a("0")]),
        ]),
    ]
}

fn layout_swishpan() -> Vec<Sx> {
    vec![
        a("layout_effect"),
        a("SwishPan"),
        l(vec![l(vec![a("Foreground"), a("From"), a("required")]), l(vec![a("Background"), a("To"), a("required")])]),
        l(vec![]),
        l(vec![
            l(vec![a(":modify"), a("transition_dir"), l(vec![a("dont_save"), a("shared")])]),
            l(vec![a(":modify"), a("auto_trans"), l(vec![a("dont_save"), a("shared")])]),
            l(vec![a(":modify"), a("wipe_percent"), a("wipe_amt"), l(vec![a("percent"), a("shared")])]),
            l(vec![a(":modify"), a("direction"), l(vec![a("new_column")])]),
            a("blur_amount"),
            a("overlap"),
            l(vec![a(":modify"), a("slow_in"), l(vec![a("max"), a("1")])]),
            l(vec![a(":modify"), a("slow_out"), l(vec![a("max"), a("1")])]),
            l(vec![a(":extra"), a("opacity"), a("="), a("1"), l(vec![a("ignore"), a("popup"), a("All_Opaque"), a("Normal"), a("As_Premult")])]),
            a("swish_auto_trans"),
        ]),
    ]
}

fn eff_dissolve_shake(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "transition_dir", D::N(0.0), &["popup", "Dissolve_Off_to_Bg", "Dissolve_On_from_Bg"]),
        param(a, "auto_trans", D::N(0.0), &["popup", "\"No: use Dissolve%\"", "\"Yes: ignore Dissolve%\""]),
        param(a, "dissolve_amt", D::N(0.0), &["autotrans", "auto_anim_ramp_up"]),
        param(a, "dissolve_speed", D::N(3.0), &["min", "1", "slider_max", "10", "big_slider_max", "10"]),
        param(a, "amplitude", D::N(3.0), &[]),
        param(a, "frequency", D::N(10.0), &["slider_range", ".1", "30"]),
        param(a, "motion_blur", D::N(1.0), &["boolean"]),
        param(a, "mo_blur_length", D::N(0.5), &[]),
        param(a, "phase", D::N(0.0), &[]),
        param(a, "seed", D::N(0.0), &[]),
        param(a, "tilt_phase", D::N(0.0), &["min", "*"]),
        param(a, "tilt_rand_amp", D::N(0.0), &["slider_max", "90"]),
        param(a, "tilt_rand_freq", D::N(1.0), &[]),
        param(a, "tilt_wave_amp", D::N(0.0), &["slider_max", "90"]),
        param(a, "tilt_wave_freq", D::N(0.5), &[]),
        param(a, "wrap", D::V(&[2.0, 2.0]), &["popup", "No", "Tile", "Reflect"]),
        param(a, "x_phase", D::N(0.0), &["min", "*"]),
        param(a, "x_rand_amp", D::N(0.2), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "x_rand_freq", D::N(1.0), &[]),
        param(a, "x_wave_amp", D::N(0.0), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "x_wave_freq", D::N(0.5), &[]),
        param(a, "y_phase", D::N(0.0), &["min", "*"]),
        param(a, "y_rand_amp", D::N(0.1), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "y_rand_freq", D::N(1.0), &[]),
        param(a, "y_wave_amp", D::N(0.0), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "y_wave_freq", D::N(0.5), &[]),
        param(a, "z_dist", D::N(0.0), &[]),
        param(a, "z_phase", D::N(0.0), &["min", "*"]),
        param(a, "z_rand_amp", D::N(0.0), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "z_rand_freq", D::N(1.0), &[]),
        param(a, "z_wave_amp", D::N(0.0), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "z_wave_freq", D::N(0.5), &[]),
        param(a, "red_amplitude", D::N(1.0), &[]),
        param(a, "blue_amplitude", D::N(1.0), &[]),
        param(a, "green_amplitude", D::N(1.0), &[]),
        param(a, "red_phase", D::N(0.0), &["min", "*", "slider_range", "-0.5", ".5"]),
        param(a, "blue_phase", D::N(0.0), &["min", "*", "slider_range", "-0.5", ".5"]),
        param(a, "green_phase", D::N(0.0), &["min", "*", "slider_range", "-0.5", ".5"]),
        param(a, "RGB_randomness", D::N(0.0), &[]),
        param(a, "RGB_frequency", D::N(2.0), &["slider_range", ".1", "30"]),
    ];
    let body = lower(a, &lt(vec![
        (&["from", "to"], call("transition_swap_inputs", vec![id("From"), id("To"), id("transition_dir")])),
        (&["amount"], call("transition_amount", vec![id("auto_trans"), id("dissolve_amt")])),
        (&["scaled_amp"], call("*", vec![
            id("amplitude"),
            call("if>", vec![
                id("amount"),
                n(0.5),
                call("-", vec![n(2.0), call("*", vec![n(2.0), id("amount")])]),
                call("*", vec![n(2.0), id("amount")]),
            ]),
        ])),
        (&["d"], call("DELAY_DISSOLVE", vec![id("amount"), id("dissolve_speed")])),
        (&["from1"], call("if_neq_const", vec![id("from"), n(0.0), id("from"), v(&[0.0, 0.0, 0.0, 0.0])])),
        (&["to1"], call("if_neq_const", vec![id("to"), n(0.0), id("to"), v(&[0.0, 0.0, 0.0, 0.0])])),
        (&["src"], call("dissolve", vec![
            call("coerce_up", vec![id("from1"), id("Output")]),
            call("coerce_up", vec![id("to1"), id("Output")]),
            id("d"),
        ])),
    ], call("Shake_Normal", vec![
        id("src"),
        id("scaled_amp"),
        id("frequency"),
        id("motion_blur"),
        id("mo_blur_length"),
        id("phase"),
        id("seed"),
        id("tilt_phase"),
        id("tilt_rand_amp"),
        id("tilt_rand_freq"),
        id("tilt_wave_amp"),
        id("tilt_wave_freq"),
        id("wrap"),
        id("x_phase"),
        id("x_rand_amp"),
        id("x_rand_freq"),
        id("x_wave_amp"),
        id("x_wave_freq"),
        id("y_phase"),
        id("y_rand_amp"),
        id("y_rand_freq"),
        id("y_wave_amp"),
        id("y_wave_freq"),
        id("z_dist"),
        id("z_phase"),
        id("z_rand_amp"),
        id("z_rand_freq"),
        id("z_wave_amp"),
        id("z_wave_freq"),
        id("red_amplitude"),
        id("green_amplitude"),
        id("blue_amplitude"),
        id("red_phase"),
        id("green_phase"),
        id("blue_phase"),
        id("RGB_randomness"),
        id("RGB_frequency"),
    ])));
    EffectDef { name: "Dissolve_Shake".into(), va: "0x13dfd60".into(), inputs: vec![inp("From", Some("0")), inp("To", Some("0"))], flags: strv(&["rgba_ok", "transition", "generates_alpha"]), params, body }
}

fn eff_shake_jumpy(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "amplitude", D::N(1.0), &["auto_anim_floor0"]),
        param(a, "frequency", D::N(8.0), &["slider_range", ".1", "30"]),
        param(a, "motion_blur", D::N(0.0), &[]),
        param(a, "motion_blur_length", D::N(1.0), &[]),
        param(a, "phase", D::N(0.0), &["min", "*"]),
        param(a, "seed", D::N(0.0), &[]),
        param(a, "tilt_phase", D::N(0.0), &["min", "*"]),
        param(a, "tilt_rand_amp", D::N(0.0), &["slider_max", "90"]),
        param(a, "tilt_rand_freq", D::N(1.0), &[]),
        param(a, "tilt_wave_amp", D::N(0.0), &["slider_max", "90"]),
        param(a, "tilt_wave_freq", D::N(0.5), &[]),
        param(a, "wrap", D::V(&[2.0, 2.0]), &["popup", "No", "Tile", "Reflect"]),
        param(a, "x_phase", D::N(0.0), &["min", "*"]),
        param(a, "x_rand_amp", D::N(0.2), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "x_rand_freq", D::N(1.0), &[]),
        param(a, "x_wave_amp", D::N(0.0), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "x_wave_freq", D::N(0.5), &[]),
        param(a, "y_phase", D::N(0.0), &["min", "*"]),
        param(a, "y_rand_amp", D::N(0.1), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "y_rand_freq", D::N(1.0), &[]),
        param(a, "y_wave_amp", D::N(0.0), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "y_wave_freq", D::N(0.5), &[]),
        param(a, "z_dist", D::N(1.0), &["slider_range", ".1", "2"]),
        param(a, "z_phase", D::N(0.0), &["min", "*"]),
        param(a, "z_rand_amp", D::N(0.0), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "z_rand_freq", D::N(1.0), &[]),
        param(a, "z_wave_amp", D::N(0.0), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "z_wave_freq", D::N(0.5), &[]),
        param(a, "red_amplitude", D::N(1.0), &[]),
        param(a, "green_amplitude", D::N(1.0), &[]),
        param(a, "blue_amplitude", D::N(1.0), &[]),
        param(a, "red_phase", D::N(0.0), &["min", "*", "slider_range", "-0.1", ".1"]),
        param(a, "green_phase", D::N(0.0), &["min", "*", "slider_range", "-0.1", ".1"]),
        param(a, "blue_phase", D::N(0.0), &["min", "*", "slider_range", "-0.1", ".1"]),
        param(a, "RGB_randomness", D::N(0.0), &[]),
        param(a, "RGB_frequency", D::N(2.0), &["slider_range", ".1", "30"]),
        param(a, "drift", D::N(0.3), &["not_shared", "max", "1"]),
        param(a, "center_bias", D::N(0.0), &["not_shared", "slider_max", "1"]),
    ];
    let body = lower(a, &call("shake", vec![
        id("Source"),
        id("amplitude"),
        id("frequency"),
        id("motion_blur"),
        id("motion_blur_length"),
        id("phase"),
        id("seed"),
        id("tilt_phase"),
        id("tilt_rand_amp"),
        id("tilt_rand_freq"),
        id("tilt_wave_amp"),
        id("tilt_wave_freq"),
        id("wrap"),
        id("x_phase"),
        id("x_rand_amp"),
        id("x_rand_freq"),
        id("x_wave_amp"),
        id("x_wave_freq"),
        id("y_phase"),
        id("y_rand_amp"),
        id("y_rand_freq"),
        id("y_wave_amp"),
        id("y_wave_freq"),
        id("z_dist"),
        id("z_phase"),
        id("z_rand_amp"),
        id("z_rand_freq"),
        id("z_wave_amp"),
        id("z_wave_freq"),
        id("red_amplitude"),
        id("green_amplitude"),
        id("blue_amplitude"),
        id("red_phase"),
        id("green_phase"),
        id("blue_phase"),
        id("RGB_randomness"),
        id("RGB_frequency"),
        n(1.0),
        id("drift"),
        n(1.0),
        id("center_bias"),
        call("*", vec![n(4.0), id("frequency")]),
    ]));
    EffectDef { name: "Shake_Jumpy".into(), va: "0x1472560".into(), inputs: vec![inp("Source", None)], flags: strv(&["rgba_ok", "warp_crop", "share_all", "effect_popup_name", "Style"]), params, body }
}

fn eff_shake_jumpy_autogen(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "amplitude", D::N(1.0), &["auto_anim_floor0"]),
        param(a, "frequency", D::N(8.0), &["slider_range", ".1", "30"]),
        param(a, "motion_blur", D::N(0.0), &[]),
        param(a, "motion_blur_length", D::N(1.0), &[]),
        param(a, "phase", D::N(0.0), &["min", "*"]),
        param(a, "seed", D::N(0.0), &[]),
        param(a, "tilt_phase", D::N(0.0), &["min", "*"]),
        param(a, "tilt_rand_amp", D::N(0.0), &["slider_max", "90"]),
        param(a, "tilt_rand_freq", D::N(1.0), &[]),
        param(a, "tilt_wave_amp", D::N(0.0), &["slider_max", "90"]),
        param(a, "tilt_wave_freq", D::N(0.5), &[]),
        param(a, "wrap", D::V(&[2.0, 2.0]), &["popup", "No", "Tile", "Reflect"]),
        param(a, "x_phase", D::N(0.0), &["min", "*"]),
        param(a, "x_rand_amp", D::N(0.2), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "x_rand_freq", D::N(1.0), &[]),
        param(a, "x_wave_amp", D::N(0.0), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "x_wave_freq", D::N(0.5), &[]),
        param(a, "y_phase", D::N(0.0), &["min", "*"]),
        param(a, "y_rand_amp", D::N(0.1), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "y_rand_freq", D::N(1.0), &[]),
        param(a, "y_wave_amp", D::N(0.0), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "y_wave_freq", D::N(0.5), &[]),
        param(a, "z_dist", D::N(1.0), &["slider_range", ".1", "2"]),
        param(a, "z_phase", D::N(0.0), &["min", "*"]),
        param(a, "z_rand_amp", D::N(0.0), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "z_rand_freq", D::N(1.0), &[]),
        param(a, "z_wave_amp", D::N(0.0), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "z_wave_freq", D::N(0.5), &[]),
        param(a, "red_amplitude", D::N(1.0), &[]),
        param(a, "green_amplitude", D::N(1.0), &[]),
        param(a, "blue_amplitude", D::N(1.0), &[]),
        param(a, "red_phase", D::N(0.0), &["min", "*", "slider_range", "-0.1", ".1"]),
        param(a, "green_phase", D::N(0.0), &["min", "*", "slider_range", "-0.1", ".1"]),
        param(a, "blue_phase", D::N(0.0), &["min", "*", "slider_range", "-0.1", ".1"]),
        param(a, "RGB_randomness", D::N(0.0), &[]),
        param(a, "RGB_frequency", D::N(2.0), &["slider_range", ".1", "30"]),
        param(a, "drift", D::N(0.3), &["not_shared", "max", "1"]),
        param(a, "center_bias", D::N(0.0), &["not_shared", "slider_max", "1"]),
        param(a, "mocha_project", D::N(0.0), &["optional", "mocha_data", "builder_disable", "shared"]),
        param(a, "blur_mocha_mask", D::N(0.0), &["optional", "disable_motion_blur", "normalize_scale", "4", "slider_max", "0.2", "builder_disable", "shared"]),
        param(a, "mocha_mask_opacity", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "1", "shared"]),
        param(a, "invert_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "resize_mocha_mask", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_x", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_y", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "shift_mocha_mask", D::V(&[0.0, 0.0]), &["optional", "disable_motion_blur", "builder_disable", "normalize_scale", "1", "min", "*", "slider_range", "-2", "2", "shared"]),
        param(a, "bypass_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "show_mocha_mask_only", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "combine_masks", D::N(0.0), &["optional", "popup", "Union", "Intersect", "Mocha_Only", "builder_disable", "shared"]),
        param(a, "mask_use", D::N(0.0), &["show_with_mask", "optional", "shared", "popup", "Luma", "Alpha"]),
        param(a, "blur_mask", D::N(0.050000004), &["show_with_mask", "optional", "shared", "show_with_mask", "normalize_scale", "4", "slider_max", "2"]),
        param(a, "invert_mask", D::N(0.0), &["show_with_mask", "optional", "shared", "boolean"]),
    ];
    let body = lower(a, &lt(vec![
        (&["combined_mask", "mocha_mask"], call("PROCESS_MASKS", vec![
            id("Mask"),
            id("invert_mask"),
            id("blur_mask"),
            id("mask_use"),
            id("mocha_project"),
            id("bypass_mocha_mask"),
            id("invert_mocha_mask"),
            id("blur_mocha_mask"),
            id("combine_masks"),
            id("mocha_mask_opacity"),
            id("resize_mocha_mask"),
            id("resize_mocha_rel_x"),
            id("resize_mocha_rel_y"),
            id("shift_mocha_mask"),
        ])),
        (&["result"], call("Shake_Jumpy", vec![
            id("Source"),
            id("amplitude"),
            id("frequency"),
            id("motion_blur"),
            id("motion_blur_length"),
            id("phase"),
            id("seed"),
            id("tilt_phase"),
            id("tilt_rand_amp"),
            id("tilt_rand_freq"),
            id("tilt_wave_amp"),
            id("tilt_wave_freq"),
            id("wrap"),
            id("x_phase"),
            id("x_rand_amp"),
            id("x_rand_freq"),
            id("x_wave_amp"),
            id("x_wave_freq"),
            id("y_phase"),
            id("y_rand_amp"),
            id("y_rand_freq"),
            id("y_wave_amp"),
            id("y_wave_freq"),
            id("z_dist"),
            id("z_phase"),
            id("z_rand_amp"),
            id("z_rand_freq"),
            id("z_wave_amp"),
            id("z_wave_freq"),
            id("red_amplitude"),
            id("green_amplitude"),
            id("blue_amplitude"),
            id("red_phase"),
            id("green_phase"),
            id("blue_phase"),
            id("RGB_randomness"),
            id("RGB_frequency"),
            id("drift"),
            id("center_bias"),
        ])),
    ], call("if+", vec![
        id("show_mocha_mask_only"),
        id("mocha_mask"),
        call("dissolve", vec![
            id("Source"),
            call("coerce_up", vec![id("result"), id("Source")]),
            id("combined_mask"),
        ]),
    ])));
    EffectDef { name: "Shake_Jumpy_Autogen".into(), va: "0x1472c40".into(), inputs: vec![inp("Source", None), inp("Mask", Some("1"))], flags: strv(&["rgba_ok", "warp_crop", "share_all", "effect_popup_name", "Style", "rgba_ok_for_input", "1"]), params, body }
}

fn eff_shake_normal(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "amplitude", D::N(1.0), &["auto_anim_floor0"]),
        param(a, "frequency", D::N(8.0), &["slider_range", ".1", "30"]),
        param(a, "motion_blur", D::N(0.0), &[]),
        param(a, "motion_blur_length", D::N(1.0), &[]),
        param(a, "phase", D::N(0.0), &["min", "*"]),
        param(a, "seed", D::N(0.0), &[]),
        param(a, "tilt_phase", D::N(0.0), &["min", "*"]),
        param(a, "tilt_rand_amp", D::N(0.0), &["slider_max", "90"]),
        param(a, "tilt_rand_freq", D::N(1.0), &[]),
        param(a, "tilt_wave_amp", D::N(0.0), &["slider_max", "90"]),
        param(a, "tilt_wave_freq", D::N(0.5), &[]),
        param(a, "wrap", D::V(&[2.0, 2.0]), &["popup", "No", "Tile", "Reflect"]),
        param(a, "x_phase", D::N(0.0), &["min", "*"]),
        param(a, "x_rand_amp", D::N(0.2), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "x_rand_freq", D::N(1.0), &[]),
        param(a, "x_wave_amp", D::N(0.0), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "x_wave_freq", D::N(0.5), &[]),
        param(a, "y_phase", D::N(0.0), &["min", "*"]),
        param(a, "y_rand_amp", D::N(0.1), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "y_rand_freq", D::N(1.0), &[]),
        param(a, "y_wave_amp", D::N(0.0), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "y_wave_freq", D::N(0.5), &[]),
        param(a, "z_dist", D::N(1.0), &["slider_range", ".1", "2"]),
        param(a, "z_phase", D::N(0.0), &["min", "*"]),
        param(a, "z_rand_amp", D::N(0.0), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "z_rand_freq", D::N(1.0), &[]),
        param(a, "z_wave_amp", D::N(0.0), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "z_wave_freq", D::N(0.5), &[]),
        param(a, "red_amplitude", D::N(1.0), &[]),
        param(a, "green_amplitude", D::N(1.0), &[]),
        param(a, "blue_amplitude", D::N(1.0), &[]),
        param(a, "red_phase", D::N(0.0), &["min", "*", "slider_range", "-0.1", ".1"]),
        param(a, "green_phase", D::N(0.0), &["min", "*", "slider_range", "-0.1", ".1"]),
        param(a, "blue_phase", D::N(0.0), &["min", "*", "slider_range", "-0.1", ".1"]),
        param(a, "RGB_randomness", D::N(0.0), &[]),
        param(a, "RGB_frequency", D::N(2.0), &["slider_range", ".1", "30"]),
    ];
    let body = lower(a, &call("shake", vec![
        id("Source"),
        id("amplitude"),
        id("frequency"),
        id("motion_blur"),
        id("motion_blur_length"),
        id("phase"),
        id("seed"),
        id("tilt_phase"),
        id("tilt_rand_amp"),
        id("tilt_rand_freq"),
        id("tilt_wave_amp"),
        id("tilt_wave_freq"),
        id("wrap"),
        id("x_phase"),
        id("x_rand_amp"),
        id("x_rand_freq"),
        id("x_wave_amp"),
        id("x_wave_freq"),
        id("y_phase"),
        id("y_rand_amp"),
        id("y_rand_freq"),
        id("y_wave_amp"),
        id("y_wave_freq"),
        id("z_dist"),
        id("z_phase"),
        id("z_rand_amp"),
        id("z_rand_freq"),
        id("z_wave_amp"),
        id("z_wave_freq"),
        id("red_amplitude"),
        id("green_amplitude"),
        id("blue_amplitude"),
        id("red_phase"),
        id("green_phase"),
        id("blue_phase"),
        id("RGB_randomness"),
        id("RGB_frequency"),
        n(0.0),
        n(0.0),
        n(0.0),
        n(0.0),
        n(0.0),
    ]));
    EffectDef { name: "Shake_Normal".into(), va: "0x1473920".into(), inputs: vec![inp("Source", None)], flags: strv(&["rgba_ok", "warp_crop", "share_all", "effect_popup_name", "Style"]), params, body }
}

fn eff_shake_normal_autogen(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "amplitude", D::N(1.0), &["auto_anim_floor0"]),
        param(a, "frequency", D::N(8.0), &["slider_range", ".1", "30"]),
        param(a, "motion_blur", D::N(0.0), &[]),
        param(a, "motion_blur_length", D::N(1.0), &[]),
        param(a, "phase", D::N(0.0), &["min", "*"]),
        param(a, "seed", D::N(0.0), &[]),
        param(a, "tilt_phase", D::N(0.0), &["min", "*"]),
        param(a, "tilt_rand_amp", D::N(0.0), &["slider_max", "90"]),
        param(a, "tilt_rand_freq", D::N(1.0), &[]),
        param(a, "tilt_wave_amp", D::N(0.0), &["slider_max", "90"]),
        param(a, "tilt_wave_freq", D::N(0.5), &[]),
        param(a, "wrap", D::V(&[2.0, 2.0]), &["popup", "No", "Tile", "Reflect"]),
        param(a, "x_phase", D::N(0.0), &["min", "*"]),
        param(a, "x_rand_amp", D::N(0.2), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "x_rand_freq", D::N(1.0), &[]),
        param(a, "x_wave_amp", D::N(0.0), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "x_wave_freq", D::N(0.5), &[]),
        param(a, "y_phase", D::N(0.0), &["min", "*"]),
        param(a, "y_rand_amp", D::N(0.1), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "y_rand_freq", D::N(1.0), &[]),
        param(a, "y_wave_amp", D::N(0.0), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "y_wave_freq", D::N(0.5), &[]),
        param(a, "z_dist", D::N(1.0), &["slider_range", ".1", "2"]),
        param(a, "z_phase", D::N(0.0), &["min", "*"]),
        param(a, "z_rand_amp", D::N(0.0), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "z_rand_freq", D::N(1.0), &[]),
        param(a, "z_wave_amp", D::N(0.0), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "z_wave_freq", D::N(0.5), &[]),
        param(a, "red_amplitude", D::N(1.0), &[]),
        param(a, "green_amplitude", D::N(1.0), &[]),
        param(a, "blue_amplitude", D::N(1.0), &[]),
        param(a, "red_phase", D::N(0.0), &["min", "*", "slider_range", "-0.1", ".1"]),
        param(a, "green_phase", D::N(0.0), &["min", "*", "slider_range", "-0.1", ".1"]),
        param(a, "blue_phase", D::N(0.0), &["min", "*", "slider_range", "-0.1", ".1"]),
        param(a, "RGB_randomness", D::N(0.0), &[]),
        param(a, "RGB_frequency", D::N(2.0), &["slider_range", ".1", "30"]),
        param(a, "mocha_project", D::N(0.0), &["optional", "mocha_data", "builder_disable", "shared"]),
        param(a, "blur_mocha_mask", D::N(0.0), &["optional", "disable_motion_blur", "normalize_scale", "4", "slider_max", "0.2", "builder_disable", "shared"]),
        param(a, "mocha_mask_opacity", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "1", "shared"]),
        param(a, "invert_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "resize_mocha_mask", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_x", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_y", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "shift_mocha_mask", D::V(&[0.0, 0.0]), &["optional", "disable_motion_blur", "builder_disable", "normalize_scale", "1", "min", "*", "slider_range", "-2", "2", "shared"]),
        param(a, "bypass_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "show_mocha_mask_only", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "combine_masks", D::N(0.0), &["optional", "popup", "Union", "Intersect", "Mocha_Only", "builder_disable", "shared"]),
        param(a, "mask_use", D::N(0.0), &["show_with_mask", "optional", "shared", "popup", "Luma", "Alpha"]),
        param(a, "blur_mask", D::N(0.050000004), &["show_with_mask", "optional", "shared", "show_with_mask", "normalize_scale", "4", "slider_max", "2"]),
        param(a, "invert_mask", D::N(0.0), &["show_with_mask", "optional", "shared", "boolean"]),
    ];
    let body = lower(a, &lt(vec![
        (&["combined_mask", "mocha_mask"], call("PROCESS_MASKS", vec![
            id("Mask"),
            id("invert_mask"),
            id("blur_mask"),
            id("mask_use"),
            id("mocha_project"),
            id("bypass_mocha_mask"),
            id("invert_mocha_mask"),
            id("blur_mocha_mask"),
            id("combine_masks"),
            id("mocha_mask_opacity"),
            id("resize_mocha_mask"),
            id("resize_mocha_rel_x"),
            id("resize_mocha_rel_y"),
            id("shift_mocha_mask"),
        ])),
        (&["result"], call("Shake_Normal", vec![
            id("Source"),
            id("amplitude"),
            id("frequency"),
            id("motion_blur"),
            id("motion_blur_length"),
            id("phase"),
            id("seed"),
            id("tilt_phase"),
            id("tilt_rand_amp"),
            id("tilt_rand_freq"),
            id("tilt_wave_amp"),
            id("tilt_wave_freq"),
            id("wrap"),
            id("x_phase"),
            id("x_rand_amp"),
            id("x_rand_freq"),
            id("x_wave_amp"),
            id("x_wave_freq"),
            id("y_phase"),
            id("y_rand_amp"),
            id("y_rand_freq"),
            id("y_wave_amp"),
            id("y_wave_freq"),
            id("z_dist"),
            id("z_phase"),
            id("z_rand_amp"),
            id("z_rand_freq"),
            id("z_wave_amp"),
            id("z_wave_freq"),
            id("red_amplitude"),
            id("green_amplitude"),
            id("blue_amplitude"),
            id("red_phase"),
            id("green_phase"),
            id("blue_phase"),
            id("RGB_randomness"),
            id("RGB_frequency"),
        ])),
    ], call("if+", vec![
        id("show_mocha_mask_only"),
        id("mocha_mask"),
        call("dissolve", vec![
            id("Source"),
            call("coerce_up", vec![id("result"), id("Source")]),
            id("combined_mask"),
        ]),
    ])));
    EffectDef { name: "Shake_Normal_Autogen".into(), va: "0x1473f80".into(), inputs: vec![inp("Source", None), inp("Mask", Some("1"))], flags: strv(&["rgba_ok", "warp_crop", "share_all", "effect_popup_name", "Style", "rgba_ok_for_input", "1"]), params, body }
}

fn eff_shake_twitchy(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "amplitude", D::N(1.0), &["auto_anim_floor0"]),
        param(a, "frequency", D::N(8.0), &["slider_range", ".1", "30"]),
        param(a, "motion_blur", D::N(0.0), &[]),
        param(a, "motion_blur_length", D::N(1.0), &[]),
        param(a, "phase", D::N(0.0), &["min", "*"]),
        param(a, "seed", D::N(0.0), &[]),
        param(a, "tilt_phase", D::N(0.0), &["min", "*"]),
        param(a, "tilt_rand_amp", D::N(0.0), &["slider_max", "90"]),
        param(a, "tilt_rand_freq", D::N(1.0), &[]),
        param(a, "tilt_wave_amp", D::N(0.0), &["slider_max", "90"]),
        param(a, "tilt_wave_freq", D::N(0.5), &[]),
        param(a, "wrap", D::V(&[2.0, 2.0]), &["popup", "No", "Tile", "Reflect"]),
        param(a, "x_phase", D::N(0.0), &["min", "*"]),
        param(a, "x_rand_amp", D::N(0.2), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "x_rand_freq", D::N(1.0), &[]),
        param(a, "x_wave_amp", D::N(0.0), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "x_wave_freq", D::N(0.5), &[]),
        param(a, "y_phase", D::N(0.0), &["min", "*"]),
        param(a, "y_rand_amp", D::N(0.1), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "y_rand_freq", D::N(1.0), &[]),
        param(a, "y_wave_amp", D::N(0.0), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "y_wave_freq", D::N(0.5), &[]),
        param(a, "z_dist", D::N(1.0), &["slider_range", ".1", "2"]),
        param(a, "z_phase", D::N(0.0), &["min", "*"]),
        param(a, "z_rand_amp", D::N(0.0), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "z_rand_freq", D::N(1.0), &[]),
        param(a, "z_wave_amp", D::N(0.0), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "z_wave_freq", D::N(0.5), &[]),
        param(a, "red_amplitude", D::N(1.0), &[]),
        param(a, "green_amplitude", D::N(1.0), &[]),
        param(a, "blue_amplitude", D::N(1.0), &[]),
        param(a, "red_phase", D::N(0.0), &["min", "*", "slider_range", "-0.1", ".1"]),
        param(a, "green_phase", D::N(0.0), &["min", "*", "slider_range", "-0.1", ".1"]),
        param(a, "blue_phase", D::N(0.0), &["min", "*", "slider_range", "-0.1", ".1"]),
        param(a, "RGB_randomness", D::N(0.0), &[]),
        param(a, "RGB_frequency", D::N(2.0), &["slider_range", ".1", "30"]),
        param(a, "stillness", D::N(0.7), &["not_shared", "max", "1"]),
        param(a, "twitch_frequency", D::N(2.0), &["not_shared", "slider_range", ".1", "30"]),
    ];
    let body = lower(a, &call("shake", vec![
        id("Source"),
        id("amplitude"),
        call("*", vec![n(10.0), id("frequency")]),
        id("motion_blur"),
        id("motion_blur_length"),
        id("phase"),
        id("seed"),
        id("tilt_phase"),
        id("tilt_rand_amp"),
        id("tilt_rand_freq"),
        id("tilt_wave_amp"),
        id("tilt_wave_freq"),
        id("wrap"),
        id("x_phase"),
        id("x_rand_amp"),
        id("x_rand_freq"),
        id("x_wave_amp"),
        id("x_wave_freq"),
        id("y_phase"),
        id("y_rand_amp"),
        id("y_rand_freq"),
        id("y_wave_amp"),
        id("y_wave_freq"),
        id("z_dist"),
        id("z_phase"),
        id("z_rand_amp"),
        id("z_rand_freq"),
        id("z_wave_amp"),
        id("z_wave_freq"),
        id("red_amplitude"),
        id("green_amplitude"),
        id("blue_amplitude"),
        id("red_phase"),
        id("green_phase"),
        id("blue_phase"),
        id("RGB_randomness"),
        id("RGB_frequency"),
        n(0.0),
        n(0.0),
        n(1.0),
        id("stillness"),
        id("twitch_frequency"),
    ]));
    EffectDef { name: "Shake_Twitchy".into(), va: "0x1474c00".into(), inputs: vec![inp("Source", None)], flags: strv(&["rgba_ok", "warp_crop", "share_all", "effect_popup_name", "Style"]), params, body }
}

fn eff_shake_twitchy_autogen(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "amplitude", D::N(1.0), &["auto_anim_floor0"]),
        param(a, "frequency", D::N(8.0), &["slider_range", ".1", "30"]),
        param(a, "motion_blur", D::N(0.0), &[]),
        param(a, "motion_blur_length", D::N(1.0), &[]),
        param(a, "phase", D::N(0.0), &["min", "*"]),
        param(a, "seed", D::N(0.0), &[]),
        param(a, "tilt_phase", D::N(0.0), &["min", "*"]),
        param(a, "tilt_rand_amp", D::N(0.0), &["slider_max", "90"]),
        param(a, "tilt_rand_freq", D::N(1.0), &[]),
        param(a, "tilt_wave_amp", D::N(0.0), &["slider_max", "90"]),
        param(a, "tilt_wave_freq", D::N(0.5), &[]),
        param(a, "wrap", D::V(&[2.0, 2.0]), &["popup", "No", "Tile", "Reflect"]),
        param(a, "x_phase", D::N(0.0), &["min", "*"]),
        param(a, "x_rand_amp", D::N(0.2), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "x_rand_freq", D::N(1.0), &[]),
        param(a, "x_wave_amp", D::N(0.0), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "x_wave_freq", D::N(0.5), &[]),
        param(a, "y_phase", D::N(0.0), &["min", "*"]),
        param(a, "y_rand_amp", D::N(0.1), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "y_rand_freq", D::N(1.0), &[]),
        param(a, "y_wave_amp", D::N(0.0), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "y_wave_freq", D::N(0.5), &[]),
        param(a, "z_dist", D::N(1.0), &["slider_range", ".1", "2"]),
        param(a, "z_phase", D::N(0.0), &["min", "*"]),
        param(a, "z_rand_amp", D::N(0.0), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "z_rand_freq", D::N(1.0), &[]),
        param(a, "z_wave_amp", D::N(0.0), &["normalize_scale", "1", "slider_max", "0.5"]),
        param(a, "z_wave_freq", D::N(0.5), &[]),
        param(a, "red_amplitude", D::N(1.0), &[]),
        param(a, "green_amplitude", D::N(1.0), &[]),
        param(a, "blue_amplitude", D::N(1.0), &[]),
        param(a, "red_phase", D::N(0.0), &["min", "*", "slider_range", "-0.1", ".1"]),
        param(a, "green_phase", D::N(0.0), &["min", "*", "slider_range", "-0.1", ".1"]),
        param(a, "blue_phase", D::N(0.0), &["min", "*", "slider_range", "-0.1", ".1"]),
        param(a, "RGB_randomness", D::N(0.0), &[]),
        param(a, "RGB_frequency", D::N(2.0), &["slider_range", ".1", "30"]),
        param(a, "stillness", D::N(0.7), &["not_shared", "max", "1"]),
        param(a, "twitch_frequency", D::N(2.0), &["not_shared", "slider_range", ".1", "30"]),
        param(a, "mocha_project", D::N(0.0), &["optional", "mocha_data", "builder_disable", "shared"]),
        param(a, "blur_mocha_mask", D::N(0.0), &["optional", "disable_motion_blur", "normalize_scale", "4", "slider_max", "0.2", "builder_disable", "shared"]),
        param(a, "mocha_mask_opacity", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "1", "shared"]),
        param(a, "invert_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "resize_mocha_mask", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_x", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_y", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "shift_mocha_mask", D::V(&[0.0, 0.0]), &["optional", "disable_motion_blur", "builder_disable", "normalize_scale", "1", "min", "*", "slider_range", "-2", "2", "shared"]),
        param(a, "bypass_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "show_mocha_mask_only", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "combine_masks", D::N(0.0), &["optional", "popup", "Union", "Intersect", "Mocha_Only", "builder_disable", "shared"]),
        param(a, "mask_use", D::N(0.0), &["show_with_mask", "optional", "shared", "popup", "Luma", "Alpha"]),
        param(a, "blur_mask", D::N(0.050000004), &["show_with_mask", "optional", "shared", "show_with_mask", "normalize_scale", "4", "slider_max", "2"]),
        param(a, "invert_mask", D::N(0.0), &["show_with_mask", "optional", "shared", "boolean"]),
    ];
    let body = lower(a, &lt(vec![
        (&["combined_mask", "mocha_mask"], call("PROCESS_MASKS", vec![
            id("Mask"),
            id("invert_mask"),
            id("blur_mask"),
            id("mask_use"),
            id("mocha_project"),
            id("bypass_mocha_mask"),
            id("invert_mocha_mask"),
            id("blur_mocha_mask"),
            id("combine_masks"),
            id("mocha_mask_opacity"),
            id("resize_mocha_mask"),
            id("resize_mocha_rel_x"),
            id("resize_mocha_rel_y"),
            id("shift_mocha_mask"),
        ])),
        (&["result"], call("Shake_Twitchy", vec![
            id("Source"),
            id("amplitude"),
            id("frequency"),
            id("motion_blur"),
            id("motion_blur_length"),
            id("phase"),
            id("seed"),
            id("tilt_phase"),
            id("tilt_rand_amp"),
            id("tilt_rand_freq"),
            id("tilt_wave_amp"),
            id("tilt_wave_freq"),
            id("wrap"),
            id("x_phase"),
            id("x_rand_amp"),
            id("x_rand_freq"),
            id("x_wave_amp"),
            id("x_wave_freq"),
            id("y_phase"),
            id("y_rand_amp"),
            id("y_rand_freq"),
            id("y_wave_amp"),
            id("y_wave_freq"),
            id("z_dist"),
            id("z_phase"),
            id("z_rand_amp"),
            id("z_rand_freq"),
            id("z_wave_amp"),
            id("z_wave_freq"),
            id("red_amplitude"),
            id("green_amplitude"),
            id("blue_amplitude"),
            id("red_phase"),
            id("green_phase"),
            id("blue_phase"),
            id("RGB_randomness"),
            id("RGB_frequency"),
            id("stillness"),
            id("twitch_frequency"),
        ])),
    ], call("if+", vec![
        id("show_mocha_mask_only"),
        id("mocha_mask"),
        call("dissolve", vec![
            id("Source"),
            call("coerce_up", vec![id("result"), id("Source")]),
            id("combined_mask"),
        ]),
    ])));
    EffectDef { name: "Shake_Twitchy_Autogen".into(), va: "0x14752e0".into(), inputs: vec![inp("Source", None), inp("Mask", Some("1"))], flags: strv(&["rgba_ok", "warp_crop", "share_all", "effect_popup_name", "Style", "rgba_ok_for_input", "1"]), params, body }
}

fn defun_delay_dissolve(a: &mut Arena) -> DefunDef {
    let body = lower(a, &call("+", vec![n(0.5), call("*", vec![id("speed"), call("-", vec![id("amount"), n(0.5)])])]));
    DefunDef { name: "DELAY_DISSOLVE".into(), va: "0x13cea00".into(), args: strv(&["amount", "speed"]), body }
}

fn defun_channel_with_alpha(a: &mut Arena) -> DefunDef {
    let body = lower(a, &call("copy_alpha", vec![call("element_of", vec![id("source"), id("channel")]), id("source")]));
    DefunDef { name: "channel_with_alpha".into(), va: "0x14d0980".into(), args: strv(&["source", "channel"]), body }
}

fn defun_shake(a: &mut Arena) -> DefunDef {
    let body = lower(a, &lt(vec![
        (&["shake_rgb"], call("shake_channels", vec![
            id("red_amplitude"),
            id("green_amplitude"),
            id("blue_amplitude"),
            id("red_phase"),
            id("green_phase"),
            id("blue_phase"),
            id("Source"),
            id("amplitude"),
            id("frequency"),
            id("motion_blur"),
            id("motion_blur_length"),
            id("phase"),
            id("seed"),
            id("tilt_phase"),
            id("tilt_rand_amp"),
            id("tilt_rand_freq"),
            id("tilt_wave_amp"),
            id("tilt_wave_freq"),
            id("wrap"),
            id("x_phase"),
            id("x_rand_amp"),
            id("x_rand_freq"),
            id("x_wave_amp"),
            id("x_wave_freq"),
            id("y_phase"),
            id("y_rand_amp"),
            id("y_rand_freq"),
            id("y_wave_amp"),
            id("y_wave_freq"),
            id("z_dist"),
            id("z_phase"),
            id("z_rand_amp"),
            id("z_rand_freq"),
            id("z_wave_amp"),
            id("z_wave_freq"),
            id("jumpy"),
            id("drift"),
            id("twitchy"),
            id("stillness"),
            id("twitch_frequency"),
            id("rgb_randomness"),
            id("rgb_freq"),
        ])),
        (&["shake_normal"], call("shake_internal", vec![
            id("Source"),
            id("amplitude"),
            id("frequency"),
            id("motion_blur"),
            id("motion_blur_length"),
            id("phase"),
            id("seed"),
            id("tilt_phase"),
            id("tilt_rand_amp"),
            id("tilt_rand_freq"),
            id("tilt_wave_amp"),
            id("tilt_wave_freq"),
            id("wrap"),
            id("x_phase"),
            id("x_rand_amp"),
            id("x_rand_freq"),
            id("x_wave_amp"),
            id("x_wave_freq"),
            id("y_phase"),
            id("y_rand_amp"),
            id("y_rand_freq"),
            id("y_wave_amp"),
            id("y_wave_freq"),
            id("z_dist"),
            id("z_phase"),
            id("z_rand_amp"),
            id("z_rand_freq"),
            id("z_wave_amp"),
            id("z_wave_freq"),
            id("jumpy"),
            id("drift"),
            id("twitchy"),
            id("stillness"),
            id("twitch_frequency"),
            n(0.0),
            n(0.0),
            n(0.0),
        ])),
    ], call("if+", vec![
        call("min", vec![
            call("min", vec![
                call("min", vec![
                    call("eq", vec![id("red_amplitude"), n(1.0)]),
                    call("min", vec![
                        call("eq", vec![id("blue_amplitude"), n(1.0)]),
                        call("eq", vec![id("green_amplitude"), n(1.0)]),
                    ]),
                ]),
                call("min", vec![
                    call("eq", vec![id("red_phase"), n(0.0)]),
                    call("min", vec![
                        call("eq", vec![id("blue_phase"), n(0.0)]),
                        call("eq", vec![id("green_phase"), n(0.0)]),
                    ]),
                ]),
            ]),
            call("eq", vec![id("rgb_randomness"), n(0.0)]),
        ]),
        id("shake_normal"),
        id("shake_rgb"),
    ])));
    DefunDef { name: "shake".into(), va: "0x14dfee0".into(), args: strv(&["Source", "amplitude", "frequency", "motion_blur", "motion_blur_length", "phase", "seed", "tilt_phase", "tilt_rand_amp", "tilt_rand_freq", "tilt_wave_amp", "tilt_wave_freq", "wrap", "x_phase", "x_rand_amp", "x_rand_freq", "x_wave_amp", "x_wave_freq", "y_phase", "y_rand_amp", "y_rand_freq", "y_wave_amp", "y_wave_freq", "z_dist", "z_phase", "z_rand_amp", "z_rand_freq", "z_wave_amp", "z_wave_freq", "red_amplitude", "green_amplitude", "blue_amplitude", "red_phase", "green_phase", "blue_phase", "rgb_randomness", "rgb_freq", "jumpy", "drift", "twitchy", "stillness", "twitch_frequency"]), body }
}

fn defun_shake_channels(a: &mut Arena) -> DefunDef {
    let body = lower(a, &lt(vec![
        (&["red"], call("channel_with_alpha", vec![id("Source"), n(0.0)])),
        (&["green"], call("channel_with_alpha", vec![id("Source"), n(1.0)])),
        (&["blue"], call("channel_with_alpha", vec![id("Source"), n(2.0)])),
        (&["shake_red"], call("shake_internal", vec![
            id("red"),
            call("*", vec![id("red_amp"), id("amplitude")]),
            id("frequency"),
            id("motion_blur"),
            id("motion_blur_length"),
            call("+", vec![id("red_phase"), id("phase")]),
            id("seed"),
            id("tilt_phase"),
            id("tilt_rand_amp"),
            id("tilt_rand_freq"),
            id("tilt_wave_amp"),
            id("tilt_wave_freq"),
            id("wrap"),
            id("x_phase"),
            id("x_rand_amp"),
            id("x_rand_freq"),
            id("x_wave_amp"),
            id("x_wave_freq"),
            id("y_phase"),
            id("y_rand_amp"),
            id("y_rand_freq"),
            id("y_wave_amp"),
            id("y_wave_freq"),
            id("z_dist"),
            id("z_phase"),
            id("z_rand_amp"),
            id("z_rand_freq"),
            id("z_wave_amp"),
            id("z_wave_freq"),
            id("jumpy"),
            id("drift"),
            id("twitchy"),
            id("stillness"),
            id("twitch_frequency"),
            id("rgb_randomness"),
            n(1.0),
            id("rgb_freq"),
        ])),
        (&["shake_green"], call("shake_internal", vec![
            id("green"),
            call("*", vec![id("green_amp"), id("amplitude")]),
            id("frequency"),
            id("motion_blur"),
            id("motion_blur_length"),
            call("+", vec![id("green_phase"), id("phase")]),
            id("seed"),
            id("tilt_phase"),
            id("tilt_rand_amp"),
            id("tilt_rand_freq"),
            id("tilt_wave_amp"),
            id("tilt_wave_freq"),
            id("wrap"),
            id("x_phase"),
            id("x_rand_amp"),
            id("x_rand_freq"),
            id("x_wave_amp"),
            id("x_wave_freq"),
            id("y_phase"),
            id("y_rand_amp"),
            id("y_rand_freq"),
            id("y_wave_amp"),
            id("y_wave_freq"),
            id("z_dist"),
            id("z_phase"),
            id("z_rand_amp"),
            id("z_rand_freq"),
            id("z_wave_amp"),
            id("z_wave_freq"),
            id("jumpy"),
            id("drift"),
            id("twitchy"),
            id("stillness"),
            id("twitch_frequency"),
            id("rgb_randomness"),
            n(2.0),
            id("rgb_freq"),
        ])),
        (&["shake_blue"], call("shake_internal", vec![
            id("blue"),
            call("*", vec![id("blue_amp"), id("amplitude")]),
            id("frequency"),
            id("motion_blur"),
            id("motion_blur_length"),
            call("+", vec![id("blue_phase"), id("phase")]),
            id("seed"),
            id("tilt_phase"),
            id("tilt_rand_amp"),
            id("tilt_rand_freq"),
            id("tilt_wave_amp"),
            id("tilt_wave_freq"),
            id("wrap"),
            id("x_phase"),
            id("x_rand_amp"),
            id("x_rand_freq"),
            id("x_wave_amp"),
            id("x_wave_freq"),
            id("y_phase"),
            id("y_rand_amp"),
            id("y_rand_freq"),
            id("y_wave_amp"),
            id("y_wave_freq"),
            id("z_dist"),
            id("z_phase"),
            id("z_rand_amp"),
            id("z_rand_freq"),
            id("z_wave_amp"),
            id("z_wave_freq"),
            id("jumpy"),
            id("drift"),
            id("twitchy"),
            id("stillness"),
            id("twitch_frequency"),
            id("rgb_randomness"),
            n(3.0),
            id("rgb_freq"),
        ])),
    ], call("v3_to_v4_if4_a", vec![
        call("v3", vec![
            call("element_of", vec![id("shake_red"), n(0.0)]),
            call("element_of", vec![id("shake_green"), n(0.0)]),
            call("element_of", vec![id("shake_blue"), n(0.0)]),
        ]),
        call("*", vec![
            n(0.33),
            call("+", vec![
                call("get_alpha", vec![id("shake_red")]),
                call("+", vec![
                    call("get_alpha", vec![id("shake_green")]),
                    call("get_alpha", vec![id("shake_blue")]),
                ]),
            ]),
        ]),
        id("Source"),
    ])));
    DefunDef { name: "shake_channels".into(), va: "0x14e0540".into(), args: strv(&["red_amp", "green_amp", "blue_amp", "red_phase", "green_phase", "blue_phase", "Source", "amplitude", "frequency", "motion_blur", "motion_blur_length", "phase", "seed", "tilt_phase", "tilt_rand_amp", "tilt_rand_freq", "tilt_wave_amp", "tilt_wave_freq", "wrap", "x_phase", "x_rand_amp", "x_rand_freq", "x_wave_amp", "x_wave_freq", "y_phase", "y_rand_amp", "y_rand_freq", "y_wave_amp", "y_wave_freq", "z_dist", "z_phase", "z_rand_amp", "z_rand_freq", "z_wave_amp", "z_wave_freq", "jumpy", "drift", "twitchy", "stillness", "twitch_frequency", "rgb_randomness", "rgb_freq"]), body }
}

fn defun_shake_internal(a: &mut Arena) -> DefunDef {
    let body = lower(a, &lt(vec![
        (&["time"], call("get_time", vec![])),
        (&["xtime"], call("*", vec![
            call("-", vec![id("time"), call("+", vec![id("x_phase"), id("phase")])]),
            id("frequency"),
        ])),
        (&["ytime"], call("*", vec![
            call("-", vec![id("time"), call("+", vec![id("y_phase"), id("phase")])]),
            id("frequency"),
        ])),
        (&["ztime"], call("*", vec![
            call("-", vec![id("time"), call("+", vec![id("z_phase"), id("phase")])]),
            id("frequency"),
        ])),
        (&["ttime"], call("*", vec![
            call("-", vec![id("time"), call("+", vec![id("tilt_phase"), id("phase")])]),
            id("frequency"),
        ])),
        (&["time2"], call("+", vec![
            call("get_time", vec![]),
            call("/", vec![n(1.0), call("get_frames_per_second", vec![])]),
        ])),
        (&["xtime2"], call("*", vec![
            call("-", vec![id("time2"), call("+", vec![id("x_phase"), id("phase")])]),
            id("frequency"),
        ])),
        (&["ytime2"], call("*", vec![
            call("-", vec![id("time2"), call("+", vec![id("y_phase"), id("phase")])]),
            id("frequency"),
        ])),
        (&["ztime2"], call("*", vec![
            call("-", vec![id("time2"), call("+", vec![id("z_phase"), id("phase")])]),
            id("frequency"),
        ])),
        (&["ttime2"], call("*", vec![
            call("-", vec![id("time2"), call("+", vec![id("tilt_phase"), id("phase")])]),
            id("frequency"),
        ])),
        (&["xseed"], call("+", vec![id("seed"), n(0.1)])),
        (&["yseed"], call("+", vec![id("seed"), n(0.2)])),
        (&["zseed"], call("+", vec![id("seed"), n(0.3)])),
        (&["tseed"], call("+", vec![id("seed"), n(0.4)])),
        (&["twitch_scale"], call("if+", vec![
            id("twitchy"),
            call("smooth_twitch", vec![id("seed"), id("time"), id("twitch_frequency"), id("stillness")]),
            n(1.0),
        ])),
        (&["xshift"], call("shake_shift", vec![
            id("xseed"),
            id("xtime"),
            call("*", vec![id("amplitude"), id("x_rand_amp")]),
            id("x_rand_freq"),
            call("*", vec![id("amplitude"), id("x_wave_amp")]),
            id("x_wave_freq"),
            call("*", vec![id("extra_rand_amp"), id("x_rand_amp")]),
            id("extra_rand_seed"),
            id("extra_rand_freq"),
            id("jumpy"),
            id("drift"),
            id("twitch_scale"),
        ])),
        (&["yshift"], call("shake_shift", vec![
            id("yseed"),
            id("ytime"),
            call("*", vec![id("amplitude"), id("y_rand_amp")]),
            id("y_rand_freq"),
            call("*", vec![id("amplitude"), id("y_wave_amp")]),
            id("y_wave_freq"),
            call("*", vec![id("extra_rand_amp"), id("y_rand_amp")]),
            id("extra_rand_seed"),
            id("extra_rand_freq"),
            id("jumpy"),
            id("drift"),
            id("twitch_scale"),
        ])),
        (&["zshift"], call("shake_shift", vec![
            id("zseed"),
            id("ztime"),
            call("*", vec![id("amplitude"), id("z_rand_amp")]),
            id("z_rand_freq"),
            call("*", vec![id("amplitude"), id("z_wave_amp")]),
            id("z_wave_freq"),
            call("*", vec![id("extra_rand_amp"), id("z_rand_amp")]),
            id("extra_rand_seed"),
            id("extra_rand_freq"),
            id("jumpy"),
            id("drift"),
            id("twitch_scale"),
        ])),
        (&["tshift"], call("shake_shift", vec![
            id("tseed"),
            id("ttime"),
            call("*", vec![id("amplitude"), id("tilt_rand_amp")]),
            id("tilt_rand_freq"),
            call("*", vec![id("amplitude"), id("tilt_wave_amp")]),
            id("tilt_wave_freq"),
            call("*", vec![id("extra_rand_amp"), id("tilt_rand_amp")]),
            id("extra_rand_seed"),
            id("extra_rand_freq"),
            id("jumpy"),
            id("drift"),
            id("twitch_scale"),
        ])),
        (&["twitch_scale2"], call("if+", vec![
            id("twitchy"),
            call("smooth_twitch", vec![id("seed"), id("time2"), id("twitch_frequency"), id("stillness")]),
            n(1.0),
        ])),
        (&["xshift2"], call("shake_shift", vec![
            id("xseed"),
            id("xtime2"),
            call("*", vec![id("amplitude"), id("x_rand_amp")]),
            id("x_rand_freq"),
            call("*", vec![id("amplitude"), id("x_wave_amp")]),
            id("x_wave_freq"),
            call("*", vec![id("extra_rand_amp"), id("x_rand_amp")]),
            id("extra_rand_seed"),
            id("extra_rand_freq"),
            id("jumpy"),
            id("drift"),
            id("twitch_scale2"),
        ])),
        (&["yshift2"], call("shake_shift", vec![
            id("yseed"),
            id("ytime2"),
            call("*", vec![id("amplitude"), id("y_rand_amp")]),
            id("y_rand_freq"),
            call("*", vec![id("amplitude"), id("y_wave_amp")]),
            id("y_wave_freq"),
            call("*", vec![id("extra_rand_amp"), id("y_rand_amp")]),
            id("extra_rand_seed"),
            id("extra_rand_freq"),
            id("jumpy"),
            id("drift"),
            id("twitch_scale2"),
        ])),
        (&["zshift2"], call("shake_shift", vec![
            id("zseed"),
            id("ztime2"),
            call("*", vec![id("amplitude"), id("z_rand_amp")]),
            id("z_rand_freq"),
            call("*", vec![id("amplitude"), id("z_wave_amp")]),
            id("z_wave_freq"),
            call("*", vec![id("extra_rand_amp"), id("z_rand_amp")]),
            id("extra_rand_seed"),
            id("extra_rand_freq"),
            id("jumpy"),
            id("drift"),
            id("twitch_scale2"),
        ])),
        (&["tshift2"], call("shake_shift", vec![
            id("tseed"),
            id("ttime2"),
            call("*", vec![id("amplitude"), id("tilt_rand_amp")]),
            id("tilt_rand_freq"),
            call("*", vec![id("amplitude"), id("tilt_wave_amp")]),
            id("tilt_wave_freq"),
            call("*", vec![id("extra_rand_amp"), id("tilt_rand_amp")]),
            id("extra_rand_seed"),
            id("extra_rand_freq"),
            id("jumpy"),
            id("drift"),
            id("twitch_scale2"),
        ])),
        (&["coords"], call("*", vec![
            call("rotate", vec![
                call("+", vec![id("xy"), call("v2", vec![id("xshift"), id("yshift")])]),
                call("/", vec![id("tshift"), n(360.0)]),
                v(&[0.0, 0.0]),
            ]),
            call("+", vec![id("z_dist"), id("zshift")]),
        ])),
        (&["coords2"], call("*", vec![
            call("rotate", vec![
                call("+", vec![id("xy"), call("v2", vec![id("xshift2"), id("yshift2")])]),
                call("/", vec![id("tshift2"), n(360.0)]),
                v(&[0.0, 0.0]),
            ]),
            call("+", vec![id("z_dist"), id("zshift2")]),
        ])),
    ], call("if+", vec![
        id("motion_blur"),
        call("blur_warp", vec![
            call("make_mipmap", vec![id("Source"), id("wrap"), n(0.0)]),
            id("coords"),
            call("interp", vec![id("coords"), id("coords2"), id("motion_blur_length")]),
            v(&[1.0, 1.0]),
            n(1.0),
            id("wrap"),
            n(1.0),
            n(0.0),
        ]),
        call("warp", vec![
            call("make_mipmap", vec![id("Source"), id("wrap"), n(0.0)]),
            id("coords"),
            id("wrap"),
            n(0.0),
        ]),
    ])));
    DefunDef { name: "shake_internal".into(), va: "0x14e0dc0".into(), args: strv(&["Source", "amplitude", "frequency", "motion_blur", "motion_blur_length", "phase", "seed", "tilt_phase", "tilt_rand_amp", "tilt_rand_freq", "tilt_wave_amp", "tilt_wave_freq", "wrap", "x_phase", "x_rand_amp", "x_rand_freq", "x_wave_amp", "x_wave_freq", "y_phase", "y_rand_amp", "y_rand_freq", "y_wave_amp", "y_wave_freq", "z_dist", "z_phase", "z_rand_amp", "z_rand_freq", "z_wave_amp", "z_wave_freq", "jumpy", "drift", "twitchy", "stillness", "twitch_frequency", "extra_rand_amp", "extra_rand_seed", "extra_rand_freq"]), body }
}

fn defun_smooth_twitch(a: &mut Arena) -> DefunDef {
    let body = lower(a, &lt(vec![
        (&["time_step"], call("/", vec![n(1.0), call("get_frames_per_second", vec![])])),
        (&["v0"], call("twitch", vec![
            id("seed"),
            call("-", vec![id("time"), id("time_step")]),
            id("freq"),
            id("threshold"),
        ])),
        (&["v1"], call("twitch", vec![id("seed"), id("time"), id("freq"), id("threshold")])),
        (&["v2"], call("twitch", vec![
            id("seed"),
            call("+", vec![id("time"), id("time_step")]),
            id("freq"),
            id("threshold"),
        ])),
    ], call("/", vec![call("+", vec![id("v0"), call("+", vec![id("v1"), id("v2")])]), n(3.0)])));
    DefunDef { name: "smooth_twitch".into(), va: "0x14e1e40".into(), args: strv(&["seed", "time", "freq", "threshold"]), body }
}

fn defun_twitch(a: &mut Arena) -> DefunDef {
    let body = lower(a, &call("if>", vec![
        call("random_sawtooth", vec![
            id("seed"),
            call("*", vec![id("time"), id("freq")]),
            n(0.0),
            n(1.0),
            n(0.0),
            n(0.0),
            n(0.0),
        ]),
        id("threshold"),
        n(1.0),
        n(0.0),
    ]));
    DefunDef { name: "twitch".into(), va: "0x14e40a0".into(), args: strv(&["seed", "time", "freq", "threshold"]), body }
}

fn defun_eq(a: &mut Arena) -> DefunDef {
    let body = lower(a, &call("if_eq", vec![id("a"), id("b"), n(1.0), n(0.0)]));
    DefunDef { name: "eq".into(), va: "0x14e8694".into(), args: strv(&["a", "b"]), body }
}

fn layout_dissolve_shake() -> Vec<Sx> {
    vec![
        a("layout_effect"),
        a("Dissolve_Shake"),
        l(vec![l(vec![a("Foreground"), a("From"), a("required")]), l(vec![a("Background"), a("To"), a("required")])]),
        l(vec![]),
        l(vec![
            l(vec![a(":modify"), a("transition_dir"), l(vec![a("dont_save")])]),
            l(vec![a(":modify"), a("auto_trans"), l(vec![a("dont_save")])]),
            l(vec![a(":modify"), a("dissolve_percent"), a("dissolve_amt"), l(vec![a("shared"), a("percent")])]),
            a("dissolve_speed"),
            a("amplitude"),
            a("frequency"),
            a("motion_blur"),
            a("mo_blur_length"),
            a("seed"),
            a("wrap"),
            l(vec![a(":modify"), a("x_rand_amp"), l(vec![a("group_start"), a("x_shake"), a("doc_group")])]),
            a("x_rand_freq"),
            a("x_wave_amp"),
            a("x_wave_freq"),
            l(vec![a(":modify"), a("x_phase"), l(vec![a("group_end"), a("x_shake")])]),
            l(vec![a(":modify"), a("y_rand_amp"), l(vec![a("group_start"), a("y_shake"), a("doc_group")])]),
            a("y_rand_freq"),
            a("y_wave_amp"),
            a("y_wave_freq"),
            l(vec![a(":modify"), a("y_phase"), l(vec![a("group_end"), a("y_shake")])]),
            l(vec![a(":modify"), a("z_rand_amp"), l(vec![a("group_start"), a("z_shake"), a("new_column"), a("doc_group")])]),
            a("z_rand_freq"),
            a("z_wave_amp"),
            a("z_wave_freq"),
            l(vec![a(":modify"), a("z_phase"), l(vec![a("group_end"), a("z_shake")])]),
            l(vec![a(":modify"), a("tilt_rand_amp"), l(vec![a("group_start"), a("tilt_shake"), a("new_column"), a("doc_group")])]),
            a("tilt_rand_freq"),
            a("tilt_wave_amp"),
            a("tilt_wave_freq"),
            l(vec![a(":modify"), a("tilt_phase"), l(vec![a("group_end"), a("tilt_shake")])]),
            l(vec![a(":modify"), a("red_amplitude"), l(vec![a("group_start"), a("channels"), a("doc_group"), a("id_insert"), a("100")])]),
            l(vec![a(":modify"), a("green_amplitude"), l(vec![a("id_insert"), a("101")])]),
            l(vec![a(":modify"), a("blue_amplitude"), l(vec![a("id_insert"), a("102")])]),
            l(vec![a(":modify"), a("red_phase"), l(vec![a("id_insert"), a("103")])]),
            l(vec![a(":modify"), a("green_phase"), l(vec![a("id_insert"), a("104")])]),
            l(vec![a(":modify"), a("blue_phase"), l(vec![a("id_insert"), a("105")])]),
            l(vec![a(":modify"), a("RGB_randomness"), l(vec![a("id_insert"), a("106")])]),
            l(vec![a(":modify"), a("RGB_frequency"), l(vec![a("group_end"), a("channels"), a("id_insert"), a("107")])]),
            l(vec![a(":extra"), a("opacity"), a("="), a("1"), l(vec![a("doc_group"), a("ignore"), a("popup"), a("All_Opaque"), a("Normal"), a("As_Premult")])]),
            l(vec![a(":hardcode"), a("z_dist"), a("="), a("1")]),
            l(vec![a(":hardcode"), a("phase"), a("="), a("0")]),
        ]),
    ]
}

fn layout_shake_normal_autogen() -> Vec<Sx> {
    vec![
        a("layout_effect"),
        l(vec![a("Normal"), a("Shake_Normal_Autogen")]),
        l(vec![a("Source"), a("Mask")]),
        l(vec![]),
        l(vec![
            l(vec![a(":modify"), a("mocha_project"), l(vec![a("id_insert"), a("520")])]),
            l(vec![a(":modify"), a("blur_mocha"), a("blur_mocha_mask"), l(vec![a("id_insert"), a("521"), a("group_start"), a("mocha")])]),
            l(vec![a(":modify"), a("mocha_opacity"), a("mocha_mask_opacity"), l(vec![a("id_insert"), a("522")])]),
            l(vec![a(":modify"), a("invert_mocha"), a("invert_mocha_mask"), l(vec![a("id_insert"), a("523")])]),
            l(vec![a(":modify"), a("resize_mocha"), a("resize_mocha_mask"), l(vec![a("id_insert"), a("524")])]),
            l(vec![a(":modify"), a("resize_rel_x"), a("resize_mocha_rel_x"), l(vec![a("id_insert"), a("525")])]),
            l(vec![a(":modify"), a("resize_rel_y"), a("resize_mocha_rel_y"), l(vec![a("id_insert"), a("526")])]),
            l(vec![a(":modify"), a("shift_mocha"), a("shift_mocha_mask"), l(vec![a("id_insert"), a("527")])]),
            l(vec![a(":modify"), a("bypass_mocha"), a("bypass_mocha_mask"), l(vec![a("id_insert"), a("531")])]),
            l(vec![a(":modify"), a("show_mocha_only"), a("show_mocha_mask_only"), l(vec![a("id_insert"), a("529")])]),
            l(vec![a(":modify"), a("combine_masks"), l(vec![a("id_insert"), a("530"), a("group_end"), a("mocha")])]),
            a("amplitude"),
            a("frequency"),
            a("phase"),
            l(vec![a(":modify"), a("z_dist"), l(vec![a("min"), a(".001")])]),
            l(vec![a(":modify"), a("motion_blur"), l(vec![a("boolean")])]),
            l(vec![a(":modify"), a("mo_blur_length"), a("motion_blur_length")]),
            a("seed"),
            a("wrap"),
            l(vec![a(":modify"), a("x_rand_amp"), l(vec![a("group_start"), a("x_shake"), a("doc_group")])]),
            a("x_rand_freq"),
            a("x_wave_amp"),
            a("x_wave_freq"),
            l(vec![a(":modify"), a("x_phase"), l(vec![a("group_end"), a("x_shake")])]),
            l(vec![a(":modify"), a("y_rand_amp"), l(vec![a("group_start"), a("y_shake"), a("doc_group")])]),
            a("y_rand_freq"),
            a("y_wave_amp"),
            a("y_wave_freq"),
            l(vec![a(":modify"), a("y_phase"), l(vec![a("group_end"), a("y_shake")])]),
            l(vec![a(":modify"), a("z_rand_amp"), l(vec![a("group_start"), a("z_shake"), a("new_column"), a("doc_group")])]),
            a("z_rand_freq"),
            a("z_wave_amp"),
            a("z_wave_freq"),
            l(vec![a(":modify"), a("z_phase"), l(vec![a("group_end"), a("z_shake")])]),
            l(vec![a(":modify"), a("tilt_rand_amp"), l(vec![a("group_start"), a("tilt_shake"), a("new_column"), a("doc_group")])]),
            a("tilt_rand_freq"),
            a("tilt_wave_amp"),
            a("tilt_wave_freq"),
            l(vec![a(":modify"), a("tilt_phase"), l(vec![a("group_end"), a("tilt_shake")])]),
            l(vec![a(":modify"), a("red_amplitude"), l(vec![a("group_start"), a("channels"), a("doc_group"), a("id_insert"), a("100")])]),
            l(vec![a(":modify"), a("green_amplitude"), l(vec![a("id_insert"), a("101")])]),
            l(vec![a(":modify"), a("blue_amplitude"), l(vec![a("id_insert"), a("102")])]),
            l(vec![a(":modify"), a("red_phase"), l(vec![a("id_insert"), a("103")])]),
            l(vec![a(":modify"), a("green_phase"), l(vec![a("id_insert"), a("104")])]),
            l(vec![a(":modify"), a("blue_phase"), l(vec![a("id_insert"), a("105")])]),
            l(vec![a(":modify"), a("RGB_randomness"), l(vec![a("id_insert"), a("106")])]),
            l(vec![a(":modify"), a("RGB_frequency"), l(vec![a("group_end"), a("channels"), a("id_insert"), a("107")])]),
            l(vec![a(":extra"), a("opacity"), a("="), a("1"), l(vec![a("doc_group"), a("ignore"), a("popup"), a("All_Opaque"), a("Normal"), a("As_Premult")])]),
            l(vec![a(":modify"), a("mask_use"), l(vec![a("id_insert"), a("540")])]),
            l(vec![a(":modify"), a("blur_mask"), l(vec![a("id_insert"), a("541")])]),
            l(vec![a(":modify"), a("invert_mask"), l(vec![a("id_insert"), a("542")])]),
        ]),
    ]
}

fn layout_shake_twitchy_autogen() -> Vec<Sx> {
    vec![
        a("layout_effect"),
        l(vec![a("Twitchy"), a("Shake_Twitchy_Autogen")]),
        l(vec![a("Source"), a("Mask")]),
        l(vec![]),
        l(vec![
            l(vec![a(":modify"), a("mocha_project"), l(vec![a("id_insert"), a("520")])]),
            l(vec![a(":modify"), a("blur_mocha"), a("blur_mocha_mask"), l(vec![a("id_insert"), a("521"), a("group_start"), a("mocha")])]),
            l(vec![a(":modify"), a("mocha_opacity"), a("mocha_mask_opacity"), l(vec![a("id_insert"), a("522")])]),
            l(vec![a(":modify"), a("invert_mocha"), a("invert_mocha_mask"), l(vec![a("id_insert"), a("523")])]),
            l(vec![a(":modify"), a("resize_mocha"), a("resize_mocha_mask"), l(vec![a("id_insert"), a("524")])]),
            l(vec![a(":modify"), a("resize_rel_x"), a("resize_mocha_rel_x"), l(vec![a("id_insert"), a("525")])]),
            l(vec![a(":modify"), a("resize_rel_y"), a("resize_mocha_rel_y"), l(vec![a("id_insert"), a("526")])]),
            l(vec![a(":modify"), a("shift_mocha"), a("shift_mocha_mask"), l(vec![a("id_insert"), a("527")])]),
            l(vec![a(":modify"), a("bypass_mocha"), a("bypass_mocha_mask"), l(vec![a("id_insert"), a("531")])]),
            l(vec![a(":modify"), a("show_mocha_only"), a("show_mocha_mask_only"), l(vec![a("id_insert"), a("529")])]),
            l(vec![a(":modify"), a("combine_masks"), l(vec![a("id_insert"), a("530"), a("group_end"), a("mocha")])]),
            a("amplitude"),
            a("frequency"),
            a("phase"),
            l(vec![a(":modify"), a("stillness"), l(vec![a("id_insert"), a("110")])]),
            l(vec![a(":modify"), a("twitch_frequency"), l(vec![a("id_insert"), a("111")])]),
            l(vec![a(":modify"), a("z_dist"), l(vec![a("min"), a(".001")])]),
            l(vec![a(":modify"), a("motion_blur"), l(vec![a("boolean")])]),
            l(vec![a(":modify"), a("mo_blur_length"), a("motion_blur_length")]),
            a("seed"),
            a("wrap"),
            l(vec![a(":modify"), a("x_rand_amp"), l(vec![a("group_start"), a("x_shake"), a("doc_group")])]),
            a("x_rand_freq"),
            a("x_wave_amp"),
            a("x_wave_freq"),
            l(vec![a(":modify"), a("x_phase"), l(vec![a("group_end"), a("x_shake")])]),
            l(vec![a(":modify"), a("y_rand_amp"), l(vec![a("group_start"), a("y_shake"), a("doc_group")])]),
            a("y_rand_freq"),
            a("y_wave_amp"),
            a("y_wave_freq"),
            l(vec![a(":modify"), a("y_phase"), l(vec![a("group_end"), a("y_shake")])]),
            l(vec![a(":modify"), a("z_rand_amp"), l(vec![a("group_start"), a("z_shake"), a("new_column"), a("doc_group")])]),
            a("z_rand_freq"),
            a("z_wave_amp"),
            a("z_wave_freq"),
            l(vec![a(":modify"), a("z_phase"), l(vec![a("group_end"), a("z_shake")])]),
            l(vec![a(":modify"), a("tilt_rand_amp"), l(vec![a("group_start"), a("tilt_shake"), a("new_column"), a("doc_group")])]),
            a("tilt_rand_freq"),
            a("tilt_wave_amp"),
            a("tilt_wave_freq"),
            l(vec![a(":modify"), a("tilt_phase"), l(vec![a("group_end"), a("tilt_shake")])]),
            l(vec![a(":modify"), a("red_amplitude"), l(vec![a("group_start"), a("channels"), a("doc_group"), a("id_insert"), a("100")])]),
            l(vec![a(":modify"), a("green_amplitude"), l(vec![a("id_insert"), a("101")])]),
            l(vec![a(":modify"), a("blue_amplitude"), l(vec![a("id_insert"), a("102")])]),
            l(vec![a(":modify"), a("red_phase"), l(vec![a("id_insert"), a("103")])]),
            l(vec![a(":modify"), a("green_phase"), l(vec![a("id_insert"), a("104")])]),
            l(vec![a(":modify"), a("blue_phase"), l(vec![a("id_insert"), a("105")])]),
            l(vec![a(":modify"), a("RGB_randomness"), l(vec![a("id_insert"), a("106")])]),
            l(vec![a(":modify"), a("RGB_frequency"), l(vec![a("group_end"), a("channels"), a("id_insert"), a("107")])]),
            l(vec![a(":extra"), a("opacity"), a("="), a("1"), l(vec![a("doc_group"), a("ignore"), a("popup"), a("All_Opaque"), a("Normal"), a("As_Premult")])]),
            l(vec![a(":modify"), a("mask_use"), l(vec![a("id_insert"), a("540")])]),
            l(vec![a(":modify"), a("blur_mask"), l(vec![a("id_insert"), a("541")])]),
            l(vec![a(":modify"), a("invert_mask"), l(vec![a("id_insert"), a("542")])]),
        ]),
    ]
}

fn layout_shake_jumpy_autogen() -> Vec<Sx> {
    vec![
        a("layout_effect"),
        l(vec![a("Jumpy"), a("Shake_Jumpy_Autogen")]),
        l(vec![a("Source"), a("Mask")]),
        l(vec![]),
        l(vec![
            l(vec![a(":modify"), a("mocha_project"), l(vec![a("id_insert"), a("520")])]),
            l(vec![a(":modify"), a("blur_mocha"), a("blur_mocha_mask"), l(vec![a("id_insert"), a("521"), a("group_start"), a("mocha")])]),
            l(vec![a(":modify"), a("mocha_opacity"), a("mocha_mask_opacity"), l(vec![a("id_insert"), a("522")])]),
            l(vec![a(":modify"), a("invert_mocha"), a("invert_mocha_mask"), l(vec![a("id_insert"), a("523")])]),
            l(vec![a(":modify"), a("resize_mocha"), a("resize_mocha_mask"), l(vec![a("id_insert"), a("524")])]),
            l(vec![a(":modify"), a("resize_rel_x"), a("resize_mocha_rel_x"), l(vec![a("id_insert"), a("525")])]),
            l(vec![a(":modify"), a("resize_rel_y"), a("resize_mocha_rel_y"), l(vec![a("id_insert"), a("526")])]),
            l(vec![a(":modify"), a("shift_mocha"), a("shift_mocha_mask"), l(vec![a("id_insert"), a("527")])]),
            l(vec![a(":modify"), a("bypass_mocha"), a("bypass_mocha_mask"), l(vec![a("id_insert"), a("531")])]),
            l(vec![a(":modify"), a("show_mocha_only"), a("show_mocha_mask_only"), l(vec![a("id_insert"), a("529")])]),
            l(vec![a(":modify"), a("combine_masks"), l(vec![a("id_insert"), a("530"), a("group_end"), a("mocha")])]),
            a("amplitude"),
            a("frequency"),
            a("phase"),
            l(vec![a(":modify"), a("drift"), l(vec![a("id_insert"), a("113")])]),
            l(vec![a(":modify"), a("center_bias"), l(vec![a("id_insert"), a("114")])]),
            l(vec![a(":modify"), a("z_dist"), l(vec![a("min"), a(".001")])]),
            l(vec![a(":modify"), a("motion_blur"), l(vec![a("boolean")])]),
            l(vec![a(":modify"), a("mo_blur_length"), a("motion_blur_length")]),
            a("seed"),
            a("wrap"),
            l(vec![a(":modify"), a("x_rand_amp"), l(vec![a("group_start"), a("x_shake"), a("doc_group")])]),
            a("x_rand_freq"),
            a("x_wave_amp"),
            a("x_wave_freq"),
            l(vec![a(":modify"), a("x_phase"), l(vec![a("group_end"), a("x_shake")])]),
            l(vec![a(":modify"), a("y_rand_amp"), l(vec![a("group_start"), a("y_shake"), a("doc_group")])]),
            a("y_rand_freq"),
            a("y_wave_amp"),
            a("y_wave_freq"),
            l(vec![a(":modify"), a("y_phase"), l(vec![a("group_end"), a("y_shake")])]),
            l(vec![a(":modify"), a("z_rand_amp"), l(vec![a("group_start"), a("z_shake"), a("new_column"), a("doc_group")])]),
            a("z_rand_freq"),
            a("z_wave_amp"),
            a("z_wave_freq"),
            l(vec![a(":modify"), a("z_phase"), l(vec![a("group_end"), a("z_shake")])]),
            l(vec![a(":modify"), a("tilt_rand_amp"), l(vec![a("group_start"), a("tilt_shake"), a("new_column"), a("doc_group")])]),
            a("tilt_rand_freq"),
            a("tilt_wave_amp"),
            a("tilt_wave_freq"),
            l(vec![a(":modify"), a("tilt_phase"), l(vec![a("group_end"), a("tilt_shake")])]),
            l(vec![a(":modify"), a("red_amplitude"), l(vec![a("group_start"), a("channels"), a("doc_group"), a("id_insert"), a("100")])]),
            l(vec![a(":modify"), a("green_amplitude"), l(vec![a("id_insert"), a("101")])]),
            l(vec![a(":modify"), a("blue_amplitude"), l(vec![a("id_insert"), a("102")])]),
            l(vec![a(":modify"), a("red_phase"), l(vec![a("id_insert"), a("103")])]),
            l(vec![a(":modify"), a("green_phase"), l(vec![a("id_insert"), a("104")])]),
            l(vec![a(":modify"), a("blue_phase"), l(vec![a("id_insert"), a("105")])]),
            l(vec![a(":modify"), a("RGB_randomness"), l(vec![a("id_insert"), a("106")])]),
            l(vec![a(":modify"), a("RGB_frequency"), l(vec![a("group_end"), a("channels"), a("id_insert"), a("107")])]),
            l(vec![a(":extra"), a("opacity"), a("="), a("1"), l(vec![a("doc_group"), a("ignore"), a("popup"), a("All_Opaque"), a("Normal"), a("As_Premult")])]),
            l(vec![a(":modify"), a("mask_use"), l(vec![a("id_insert"), a("540")])]),
            l(vec![a(":modify"), a("blur_mask"), l(vec![a("id_insert"), a("541")])]),
            l(vec![a(":modify"), a("invert_mask"), l(vec![a("id_insert"), a("542")])]),
        ]),
    ]
}

fn eff_blur_motion(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "blur_mask", D::N(0.0), &[":blur_mask"]),
        param(a, "blur_res", D::N(0.0), &["popup", "Full", "Half", "Quarter"]),
        param(a, "center", D::V(&[0.0, 0.0]), &[]),
        param(a, "from_z_dist", D::N(1.0), &["min", ".001"]),
        param(a, "from_rotate", D::N(0.0), &["min", "*", "slider_range", "-90", "90", "big_slider_range", "-180", "180", "widget", "rectshift.rotate", "instance", "3", "angle"]),
        param(a, "from_shift", D::V(&[0.0, 0.0]), &[]),
        param(a, "to_z_dist", D::N(0.8), &["min", ".001", "auto_anim_floor1"]),
        param(a, "to_rotate", D::N(0.0), &["min", "*", "slider_range", "-90", "90", "big_slider_range", "-180", "180", "widget", "rectshift_green.rotate", "instance", "4", "angle"]),
        param(a, "to_shift", D::V(&[0.0, 0.0]), &[]),
        param(a, "exposure_bias", D::N(0.5), &["max", "1"]),
        param(a, "invert_mask", D::N(0.0), &["show_with_mask", "boolean"]),
        param(a, "mask_use", D::N(0.0), &["show_with_mask", "popup", "Luma", "Alpha"]),
        param(a, "scale_result", D::N(1.0), &[]),
        param(a, "subpixel", D::N(1.0), &["boolean"]),
        param(a, "wrap", D::V(&[0.0, 0.0]), &["popup", "No", "Tile", "Reflect"]),
    ];
    let body = lower(a, &lt(vec![
        (&["mask"], call("WARP_MASK", vec![id("Mask"), id("invert_mask"), id("blur_mask"), id("mask_use")])),
        (&["from_xy"], call("+", vec![
            call("*", vec![
                call("rotate", vec![
                    call("-", vec![id("xy"), id("center")]),
                    call("/", vec![id("from_rotate"), n(-360.0)]),
                    v(&[0.0, 0.0]),
                ]),
                id("from_z_dist"),
            ]),
            call("-", vec![id("center"), id("from_shift")]),
        ])),
        (&["to_xy"], call("+", vec![
            call("*", vec![
                call("rotate", vec![
                    call("-", vec![id("xy"), id("center")]),
                    call("/", vec![id("to_rotate"), n(-360.0)]),
                    v(&[0.0, 0.0]),
                ]),
                id("to_z_dist"),
            ]),
            call("-", vec![id("center"), id("to_shift")]),
        ])),
        (&["from_rel_bright"], call("one_minus", vec![id("exposure_bias")])),
        (&["to_rel_bright"], id("exposure_bias")),
    ], call("blur_warp", vec![
        call("make_mipmap", vec![id("Source"), id("wrap"), n(0.0)]),
        call("if_neq_const", vec![
            id("Mask"),
            n(1.0),
            call("interp", vec![id("xy"), id("from_xy"), id("mask")]),
            id("from_xy"),
        ]),
        call("if_neq_const", vec![
            id("Mask"),
            n(1.0),
            call("interp", vec![id("xy"), id("to_xy"), id("mask")]),
            id("to_xy"),
        ]),
        call("v2", vec![id("from_rel_bright"), id("to_rel_bright")]),
        id("scale_result"),
        id("wrap"),
        id("subpixel"),
        id("blur_res"),
    ])));
    EffectDef { name: "Blur_Motion".into(), va: "0x13b71c0".into(), inputs: vec![inp("Source", None), inp("Mask", Some("1"))], flags: strv(&["rgba_ok", "share_all", "warp_crop"]), params, body }
}

fn eff_blur_motion_autogen(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "blur_mask", D::N(0.0), &["show_with_mask", "normalize_scale", "4", "slider_max", "2"]),
        param(a, "blur_res", D::N(0.0), &["popup", "Full", "Half", "Quarter"]),
        param(a, "center", D::V(&[0.0, 0.0]), &[]),
        param(a, "from_z_dist", D::N(1.0), &["min", ".001"]),
        param(a, "from_rotate", D::N(0.0), &["min", "*", "slider_range", "-90", "90", "big_slider_range", "-180", "180", "widget", "rectshift.rotate", "instance", "3", "angle"]),
        param(a, "from_shift", D::V(&[0.0, 0.0]), &[]),
        param(a, "to_z_dist", D::N(0.8), &["min", ".001", "auto_anim_floor1"]),
        param(a, "to_rotate", D::N(0.0), &["min", "*", "slider_range", "-90", "90", "big_slider_range", "-180", "180", "widget", "rectshift_green.rotate", "instance", "4", "angle"]),
        param(a, "to_shift", D::V(&[0.0, 0.0]), &[]),
        param(a, "exposure_bias", D::N(0.5), &["max", "1"]),
        param(a, "invert_mask", D::N(0.0), &["show_with_mask", "boolean"]),
        param(a, "mask_use", D::N(0.0), &["show_with_mask", "popup", "Luma", "Alpha"]),
        param(a, "scale_result", D::N(1.0), &[]),
        param(a, "subpixel", D::N(1.0), &["boolean"]),
        param(a, "wrap", D::V(&[0.0, 0.0]), &["popup", "No", "Tile", "Reflect"]),
        param(a, "mocha_project", D::N(0.0), &["optional", "mocha_data", "builder_disable", "shared"]),
        param(a, "blur_mocha_mask", D::N(0.0), &["optional", "disable_motion_blur", "normalize_scale", "4", "slider_max", "0.2", "builder_disable", "shared"]),
        param(a, "mocha_mask_opacity", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "1", "shared"]),
        param(a, "invert_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "resize_mocha_mask", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_x", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_y", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "shift_mocha_mask", D::V(&[0.0, 0.0]), &["optional", "disable_motion_blur", "builder_disable", "normalize_scale", "1", "min", "*", "slider_range", "-2", "2", "shared"]),
        param(a, "bypass_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "show_mocha_mask_only", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "combine_masks", D::N(0.0), &["optional", "popup", "Union", "Intersect", "Mocha_Only", "builder_disable", "shared"]),
    ];
    let body = lower(a, &lt(vec![
        (&["combined_mask", "mocha_mask"], call("PROCESS_MASKS", vec![
            id("Mask"),
            id("invert_mask"),
            id("blur_mask"),
            id("mask_use"),
            id("mocha_project"),
            id("bypass_mocha_mask"),
            id("invert_mocha_mask"),
            id("blur_mocha_mask"),
            id("combine_masks"),
            id("mocha_mask_opacity"),
            id("resize_mocha_mask"),
            id("resize_mocha_rel_x"),
            id("resize_mocha_rel_y"),
            id("shift_mocha_mask"),
        ])),
        (&["result"], call("Blur_Motion", vec![
            id("Source"),
            id("combined_mask"),
            n(0.0),
            id("blur_res"),
            id("center"),
            id("from_z_dist"),
            id("from_rotate"),
            id("from_shift"),
            id("to_z_dist"),
            id("to_rotate"),
            id("to_shift"),
            id("exposure_bias"),
            n(0.0),
            n(0.0),
            id("scale_result"),
            id("subpixel"),
            id("wrap"),
        ])),
    ], call("if+", vec![id("show_mocha_mask_only"), id("mocha_mask"), id("result")])));
    EffectDef { name: "Blur_Motion_Autogen".into(), va: "0x13b7700".into(), inputs: vec![inp("Source", None), inp("Mask", Some("1"))], flags: strv(&["rgba_ok", "share_all", "warp_crop"]), params, body }
}

fn eff_blur_motion_mono(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "blur_mask", D::N(0.0), &[":blur_mask"]),
        param(a, "blur_res", D::N(0.0), &["popup", "Full", "Half", "Quarter"]),
        param(a, "center", D::V(&[0.0, 0.0]), &[]),
        param(a, "from_z_dist", D::N(1.0), &["min", ".001"]),
        param(a, "from_rotate", D::N(0.0), &["min", "*", "slider_range", "-90", "90", "big_slider_range", "-180", "180", "widget", "rectshift.rotate", "instance", "3", "angle"]),
        param(a, "from_shift", D::V(&[0.0, 0.0]), &[]),
        param(a, "to_z_dist", D::N(0.8), &["min", ".001", "auto_anim_floor1"]),
        param(a, "to_rotate", D::N(0.0), &["min", "*", "slider_range", "-90", "90", "big_slider_range", "-180", "180", "widget", "rectshift_green.rotate", "instance", "4", "angle"]),
        param(a, "to_shift", D::V(&[0.0, 0.0]), &[]),
        param(a, "exposure_bias", D::N(0.5), &["max", "1"]),
        param(a, "invert_mask", D::N(0.0), &["show_with_mask", "boolean"]),
        param(a, "mask_use", D::N(0.0), &["show_with_mask", "popup", "Luma", "Alpha"]),
        param(a, "scale_result", D::N(1.0), &[]),
        param(a, "subpixel", D::N(1.0), &["boolean"]),
        param(a, "wrap", D::V(&[0.0, 0.0]), &["popup", "No", "Tile", "Reflect"]),
    ];
    let body = lower(a, &lt(vec![
        (&["bw"], call("monochrome_a", vec![id("Source")])),
        (&["m"], call("if+", vec![
            id("mask_use"),
            call("coerce_to_alpha", vec![id("Mask")]),
            call("monochrome", vec![id("Mask")]),
        ])),
        (&["mask"], call("blur", vec![
            call("if+", vec![id("invert_mask"), call("one_minus", vec![id("m")]), id("m")]),
            id("blur_mask"),
            n(2.0),
            n(1.0),
            n(0.0),
        ])),
        (&["from_xy"], call("+", vec![
            call("*", vec![
                call("rotate", vec![
                    call("-", vec![id("xy"), id("center")]),
                    call("/", vec![id("from_rotate"), n(-360.0)]),
                    v(&[0.0, 0.0]),
                ]),
                id("from_z_dist"),
            ]),
            call("-", vec![id("center"), id("from_shift")]),
        ])),
        (&["to_xy"], call("+", vec![
            call("*", vec![
                call("rotate", vec![
                    call("-", vec![id("xy"), id("center")]),
                    call("/", vec![id("to_rotate"), n(-360.0)]),
                    v(&[0.0, 0.0]),
                ]),
                id("to_z_dist"),
            ]),
            call("-", vec![id("center"), id("to_shift")]),
        ])),
        (&["from_rel_bright"], call("one_minus", vec![id("exposure_bias")])),
        (&["to_rel_bright"], id("exposure_bias")),
    ], call("blur_warp", vec![
        call("make_mipmap", vec![id("bw"), id("wrap"), n(0.0)]),
        call("if_neq_const", vec![
            id("Mask"),
            n(1.0),
            call("interp", vec![id("xy"), id("from_xy"), id("mask")]),
            id("from_xy"),
        ]),
        call("if_neq_const", vec![
            id("Mask"),
            n(1.0),
            call("interp", vec![id("xy"), id("to_xy"), id("mask")]),
            id("to_xy"),
        ]),
        call("v2", vec![id("from_rel_bright"), id("to_rel_bright")]),
        id("scale_result"),
        id("wrap"),
        id("subpixel"),
        id("blur_res"),
    ])));
    EffectDef { name: "Blur_Motion_Mono".into(), va: "0x13b8e00".into(), inputs: vec![inp("Source", None), inp("Mask", Some("1"))], flags: strv(&["rgba_ok", "share_all", "warp_crop"]), params, body }
}

fn eff_blur_motion_mono_autogen(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "blur_mask", D::N(0.0), &["show_with_mask", "normalize_scale", "4", "slider_max", "2"]),
        param(a, "blur_res", D::N(0.0), &["popup", "Full", "Half", "Quarter"]),
        param(a, "center", D::V(&[0.0, 0.0]), &[]),
        param(a, "from_z_dist", D::N(1.0), &["min", ".001"]),
        param(a, "from_rotate", D::N(0.0), &["min", "*", "slider_range", "-90", "90", "big_slider_range", "-180", "180", "widget", "rectshift.rotate", "instance", "3", "angle"]),
        param(a, "from_shift", D::V(&[0.0, 0.0]), &[]),
        param(a, "to_z_dist", D::N(0.8), &["min", ".001", "auto_anim_floor1"]),
        param(a, "to_rotate", D::N(0.0), &["min", "*", "slider_range", "-90", "90", "big_slider_range", "-180", "180", "widget", "rectshift_green.rotate", "instance", "4", "angle"]),
        param(a, "to_shift", D::V(&[0.0, 0.0]), &[]),
        param(a, "exposure_bias", D::N(0.5), &["max", "1"]),
        param(a, "invert_mask", D::N(0.0), &["show_with_mask", "boolean"]),
        param(a, "mask_use", D::N(0.0), &["show_with_mask", "popup", "Luma", "Alpha"]),
        param(a, "scale_result", D::N(1.0), &[]),
        param(a, "subpixel", D::N(1.0), &["boolean"]),
        param(a, "wrap", D::V(&[0.0, 0.0]), &["popup", "No", "Tile", "Reflect"]),
        param(a, "mocha_project", D::N(0.0), &["optional", "mocha_data", "builder_disable", "shared"]),
        param(a, "blur_mocha_mask", D::N(0.0), &["optional", "disable_motion_blur", "normalize_scale", "4", "slider_max", "0.2", "builder_disable", "shared"]),
        param(a, "mocha_mask_opacity", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "1", "shared"]),
        param(a, "invert_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "resize_mocha_mask", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_x", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_y", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "shift_mocha_mask", D::V(&[0.0, 0.0]), &["optional", "disable_motion_blur", "builder_disable", "normalize_scale", "1", "min", "*", "slider_range", "-2", "2", "shared"]),
        param(a, "bypass_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "show_mocha_mask_only", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "combine_masks", D::N(0.0), &["optional", "popup", "Union", "Intersect", "Mocha_Only", "builder_disable", "shared"]),
    ];
    let body = lower(a, &lt(vec![
        (&["combined_mask", "mocha_mask"], call("PROCESS_MASKS", vec![
            id("Mask"),
            id("invert_mask"),
            id("blur_mask"),
            id("mask_use"),
            id("mocha_project"),
            id("bypass_mocha_mask"),
            id("invert_mocha_mask"),
            id("blur_mocha_mask"),
            id("combine_masks"),
            id("mocha_mask_opacity"),
            id("resize_mocha_mask"),
            id("resize_mocha_rel_x"),
            id("resize_mocha_rel_y"),
            id("shift_mocha_mask"),
        ])),
        (&["result"], call("Blur_Motion_Mono", vec![
            id("Source"),
            id("combined_mask"),
            n(0.0),
            id("blur_res"),
            id("center"),
            id("from_z_dist"),
            id("from_rotate"),
            id("from_shift"),
            id("to_z_dist"),
            id("to_rotate"),
            id("to_shift"),
            id("exposure_bias"),
            n(0.0),
            n(0.0),
            id("scale_result"),
            id("subpixel"),
            id("wrap"),
        ])),
    ], call("if+", vec![id("show_mocha_mask_only"), id("mocha_mask"), id("result")])));
    EffectDef { name: "Blur_Motion_Mono_Autogen".into(), va: "0x13b93a0".into(), inputs: vec![inp("Source", None), inp("Mask", Some("1"))], flags: strv(&["rgba_ok", "share_all", "warp_crop"]), params, body }
}

fn eff_glow_edges(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "comp_premult", D::N(0.0), &["shared", "boolean"]),
        param(a, "affect_alpha", D::N(0.0), &["shared", "dont_save", "slider_max", "1", "default_to_1_if_postdiv"]),
        param(a, "color", D::V(&[1.0, 1.0, 1.0]), &["color"]),
        param(a, "combine", D::N(2.0), &["popup", "Mult", "Add", "Screen", "Difference", "Overlay"]),
        param(a, "edges_brightness", D::N(1.0), &["slider_max", "10"]),
        param(a, "edges_smooth", D::N(0.0), &["normalize_scale", "4", "slider_max", "0.2"]),
        param(a, "edges_threshold", D::N(0.5), &["slider_max", "1"]),
        param(a, "glow_brightness", D::N(2.0), &["auto_anim_floor0"]),
        param(a, "glow_from_alpha", D::N(0.0), &["shared", "max", "1"]),
        param(a, "glow_under_source", D::N(0.0), &["shared", "max", "1"]),
        param(a, "invert_mask", D::N(0.0), &["shared", "show_with_mask", "boolean"]),
        param(a, "light_bg", D::N(0.0), &["shared", "max", "1"]),
        param(a, "scale_background", D::N(1.0), &["shared", "slider_max", "1"]),
        param(a, "scale_source", D::N(1.0), &["shared", "max", "1"]),
        param(a, "show", D::N(0.0), &["popup", "Result", "Edges"]),
        param(a, "subpixel", D::N(1.0), &["boolean"]),
        param(a, "width", D::N(0.22399999), &["normalize_scale", "4", "slider_max", "2"]),
        param(a, "width_blue", D::N(1.4), &["widget", "glow.size_blue"]),
        param(a, "width_green", D::N(1.2), &["widget", "glow.size_green"]),
        param(a, "width_red", D::N(1.0), &["widget", "glow.size_red"]),
        param(a, "width_x", D::N(1.0), &["widget", "glow.size_x"]),
        param(a, "width_y", D::N(1.0), &["widget", "glow.size_y"]),
        param(a, "eclipse_with_bg", D::N(0.0), &["max", "1"]),
        param(a, "mocha_project", D::N(0.0), &["optional", "mocha_data", "builder_disable", "shared"]),
        param(a, "blur_mocha_mask", D::N(0.0), &["optional", "disable_motion_blur", "normalize_scale", "4", "slider_max", "0.2", "builder_disable", "shared"]),
        param(a, "mocha_mask_opacity", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "1", "shared"]),
        param(a, "invert_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "resize_mocha_mask", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_x", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_y", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "shift_mocha_mask", D::V(&[0.0, 0.0]), &["optional", "disable_motion_blur", "builder_disable", "normalize_scale", "1", "min", "*", "slider_range", "-2", "2", "shared"]),
        param(a, "bypass_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "show_mocha_mask_only", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "combine_masks", D::N(0.0), &["optional", "popup", "Union", "Intersect", "Mocha_Only", "builder_disable", "shared"]),
    ];
    let body = lower(a, &lt(vec![
        (&["source"], call("maybe_add_alpha", vec![id("Source"), id("SparksMatte"), id("comp_premult")])),
        (&["mask", "mocha_mask"], call("PROCESS_COLOR_MASKS", vec![
            id("Mask"),
            id("invert_mask"),
            n(0.0),
            n(1.0),
            id("mocha_project"),
            id("bypass_mocha_mask"),
            id("invert_mocha_mask"),
            id("blur_mocha_mask"),
            id("combine_masks"),
            id("mocha_mask_opacity"),
            id("resize_mocha_mask"),
            id("resize_mocha_rel_x"),
            id("resize_mocha_rel_y"),
            id("shift_mocha_mask"),
        ])),
        (&["s"], call("interp", vec![id("source"), call("coerce_to_alpha", vec![id("source")]), id("glow_from_alpha")])),
        (&["e_scale"], call("*", vec![
            id("edges_brightness"),
            call("*", vec![n(4.0), call("+", vec![id("edges_smooth"), n(0.1)])]),
        ])),
        (&["edges"], call("*", vec![
            call("clamp", vec![
                call("-", vec![
                    call("edge_detect", vec![
                        call("monochrome", vec![id("s")]),
                        id("edges_smooth"),
                        id("e_scale"),
                        id("subpixel"),
                    ]),
                    id("edges_threshold"),
                ]),
                n(0.0),
                n(1.0),
            ]),
            id("mask"),
        ])),
        (&["glow3"], call("blur_channels", vec![
            id("edges"),
            call("*", vec![id("width"), call("v2", vec![id("width_x"), id("width_y")])]),
            call("v3", vec![id("width_red"), id("width_green"), id("width_blue")]),
            n(3.0),
            call("*", vec![id("glow_brightness"), id("color")]),
            id("subpixel"),
        ])),
        (&["dest_threshold"], id("edges")),
        (&["glow4"], call("glow_rgba_combine", vec![
            id("Background"),
            id("source"),
            id("affect_alpha"),
            id("combine"),
            id("dest_threshold"),
            id("glow3"),
            id("glow_under_source"),
            id("light_bg"),
            id("scale_background"),
            id("scale_source"),
            id("show"),
            id("eclipse_with_bg"),
        ])),
    ], call("if+", vec![id("show_mocha_mask_only"), id("mocha_mask"), id("glow4")])));
    EffectDef { name: "Glow_Edges".into(), va: "0x140bca0".into(), inputs: vec![inp("Source", None), inp("Background", Some("0")), inp("SparksMatte", Some("1")), inp("Mask", Some("1"))], flags: strv(&["generates_color", "generates_alpha", "rgba_ok", "mask_color"]), params, body }
}

fn eff_warp_transform(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "comp_premult", D::N(0.0), &["shared", "boolean"]),
        param(a, "blur_mask", D::N(0.0), &[":blur_mask"]),
        param(a, "filter", D::N(1.0), &["boolean"]),
        param(a, "invert_mask", D::N(0.0), &["show_with_mask", "boolean"]),
        param(a, "mask_use", D::N(0.0), &["show_with_mask", "popup", "Luma", "Alpha"]),
        param(a, "perspective_amount", D::N(1.0), &["min", ".25", "max", "4.0"]),
        param(a, "rotate", D::N(0.0), &["angle"]),
        param(a, "scale", D::V(&[1.0, 1.0]), &["min", "*", "slider_range", "0", "2"]),
        param(a, "shear", D::V(&[0.0, 0.0]), &["min", "*", "slider_range", "-2", "2"]),
        param(a, "shift", D::V(&[0.0, 0.0]), &["slider_range", "-2", "2"]),
        param(a, "swivel", D::N(0.0), &["angle"]),
        param(a, "tilt", D::N(0.0), &["angle"]),
        param(a, "wrap", D::V(&[0.0, 0.0]), &["popup", "No", "Tile", "Reflect"]),
        param(a, "z_dist", D::N(1.0), &["min", ".001", "slider_min", ".1", "big_slider_min", ".001"]),
    ];
    let body = lower(a, &lt(vec![
        (&["source"], call("maybe_add_alpha", vec![id("Source"), id("SparksMatte"), id("comp_premult")])),
        (&["mask"], call("WARP_MASK", vec![id("Mask"), id("invert_mask"), id("blur_mask"), id("mask_use")])),
        (&["tilt2"], call("v2", vec![id("swivel"), id("tilt")])),
        (&["scale_safe"], call("if+", vec![
            call("negate", vec![id("scale")]),
            call("min", vec![n(-0.001), id("scale")]),
            call("max", vec![n(0.001), id("scale")]),
        ])),
        (&["coords"], call("interp", vec![
            id("xy"),
            call("tilt", vec![
                call("force_no_cache", vec![id("xy")]),
                id("tilt2"),
                id("perspective_amount"),
                call("negate", vec![id("shift")]),
                call("negate", vec![id("rotate")]),
                call("/", vec![id("z_dist"), id("scale_safe")]),
                call("negate", vec![id("shear")]),
                id("wrap"),
            ]),
            id("mask"),
        ])),
        (&["alpha"], call("if>", vec![
            call("max_element", vec![call("abs", vec![id("tilt2")])]),
            n(10.0),
            call("interp", vec![
                n(1.0),
                call("tilt_alpha", vec![id("xy"), id("tilt2"), id("perspective_amount")]),
                id("mask"),
            ]),
            n(1.0),
        ])),
        (&["warped"], call("motion_blur", vec![
            call("*", vec![
                call("warp", vec![
                    call("make_mipmap", vec![id("source"), id("wrap"), id("filter")]),
                    id("coords"),
                    id("wrap"),
                    id("filter"),
                ]),
                id("alpha"),
            ]),
        ])),
        (&["mask_for_const_test"], call("if_neq_const", vec![id("Mask"), n(1.0), n(1.0), n(0.0)])),
        (&["d0"], call("if_neq_const", vec![
            id("mask_for_const_test"),
            n(1.0),
            call("if+", vec![
                call("min_element", vec![call("+", vec![call("abs", vec![id("scale")]), id("wrap")])]),
                id("warped"),
                call("coerce_to", vec![n(0.0), id("source")]),
            ]),
            id("warped"),
        ])),
    ], call("warp_combine", vec![id("d0"), id("back")])));
    EffectDef { name: "Warp_Transform".into(), va: "0x14afd60".into(), inputs: vec![inp("Mask", Some("1")), inp("Source", None), inp("SparksMatte", Some("1")), inp("back", Some("0"))], flags: strv(&["rgba_ok"]), params, body }
}

fn eff_warp_transform_autogen(a: &mut Arena) -> EffectDef {
    let params = vec![
        param(a, "comp_premult", D::N(0.0), &["shared", "boolean"]),
        param(a, "blur_mask", D::N(0.0), &["show_with_mask", "normalize_scale", "4", "slider_max", "2"]),
        param(a, "filter", D::N(1.0), &["boolean"]),
        param(a, "invert_mask", D::N(0.0), &["show_with_mask", "boolean"]),
        param(a, "mask_use", D::N(0.0), &["show_with_mask", "popup", "Luma", "Alpha"]),
        param(a, "perspective_amount", D::N(1.0), &["min", ".25", "max", "4.0"]),
        param(a, "rotate", D::N(0.0), &["angle"]),
        param(a, "scale", D::V(&[1.0, 1.0]), &["min", "*", "slider_range", "0", "2"]),
        param(a, "shear", D::V(&[0.0, 0.0]), &["min", "*", "slider_range", "-2", "2"]),
        param(a, "shift", D::V(&[0.0, 0.0]), &["slider_range", "-2", "2"]),
        param(a, "swivel", D::N(0.0), &["angle"]),
        param(a, "tilt", D::N(0.0), &["angle"]),
        param(a, "wrap", D::V(&[0.0, 0.0]), &["popup", "No", "Tile", "Reflect"]),
        param(a, "z_dist", D::N(1.0), &["min", ".001", "slider_min", ".1", "big_slider_min", ".001"]),
        param(a, "mocha_project", D::N(0.0), &["optional", "mocha_data", "builder_disable", "shared"]),
        param(a, "blur_mocha_mask", D::N(0.0), &["optional", "disable_motion_blur", "normalize_scale", "4", "slider_max", "0.2", "builder_disable", "shared"]),
        param(a, "mocha_mask_opacity", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "1", "shared"]),
        param(a, "invert_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "resize_mocha_mask", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_x", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "resize_mocha_rel_y", D::N(1.0), &["optional", "disable_motion_blur", "builder_disable_1", "max", "2", "shared"]),
        param(a, "shift_mocha_mask", D::V(&[0.0, 0.0]), &["optional", "disable_motion_blur", "builder_disable", "normalize_scale", "1", "min", "*", "slider_range", "-2", "2", "shared"]),
        param(a, "bypass_mocha_mask", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "show_mocha_mask_only", D::N(0.0), &["optional", "boolean", "builder_disable", "shared"]),
        param(a, "combine_masks", D::N(0.0), &["optional", "popup", "Union", "Intersect", "Mocha_Only", "builder_disable", "shared"]),
    ];
    let body = lower(a, &lt(vec![
        (&["combined_mask", "mocha_mask"], call("PROCESS_MASKS", vec![
            id("Mask"),
            id("invert_mask"),
            id("blur_mask"),
            id("mask_use"),
            id("mocha_project"),
            id("bypass_mocha_mask"),
            id("invert_mocha_mask"),
            id("blur_mocha_mask"),
            id("combine_masks"),
            id("mocha_mask_opacity"),
            id("resize_mocha_mask"),
            id("resize_mocha_rel_x"),
            id("resize_mocha_rel_y"),
            id("shift_mocha_mask"),
        ])),
        (&["result"], call("Warp_Transform", vec![
            id("combined_mask"),
            id("Source"),
            id("SparksMatte"),
            id("back"),
            id("comp_premult"),
            n(0.0),
            id("filter"),
            n(0.0),
            n(0.0),
            id("perspective_amount"),
            id("rotate"),
            id("scale"),
            id("shear"),
            id("shift"),
            id("swivel"),
            id("tilt"),
            id("wrap"),
            id("z_dist"),
        ])),
    ], call("if+", vec![id("show_mocha_mask_only"), id("mocha_mask"), id("result")])));
    EffectDef { name: "Warp_Transform_Autogen".into(), va: "0x14b0300".into(), inputs: vec![inp("Mask", Some("1")), inp("Source", None), inp("SparksMatte", Some("1")), inp("back", Some("0"))], flags: strv(&["rgba_ok"]), params, body }
}

fn layout_blur_motion_autogen() -> Vec<Sx> {
    vec![
        a("layout_effect"),
        l(vec![a("Blur_Color"), a("Blur_Motion_Autogen")]),
        l(vec![a("Source"), l(vec![a("Matte"), a("Mask")])]),
        l(vec![]),
        l(vec![
            l(vec![a(":modify"), a("mocha_project"), l(vec![a("id_insert"), a("520")])]),
            l(vec![a(":modify"), a("blur_mocha"), a("blur_mocha_mask"), l(vec![a("id_insert"), a("521"), a("group_start"), a("mocha")])]),
            l(vec![a(":modify"), a("mocha_opacity"), a("mocha_mask_opacity"), l(vec![a("id_insert"), a("522")])]),
            l(vec![a(":modify"), a("invert_mocha"), a("invert_mocha_mask"), l(vec![a("id_insert"), a("523")])]),
            l(vec![a(":modify"), a("resize_mocha"), a("resize_mocha_mask"), l(vec![a("id_insert"), a("524")])]),
            l(vec![a(":modify"), a("resize_rel_x"), a("resize_mocha_rel_x"), l(vec![a("id_insert"), a("525")])]),
            l(vec![a(":modify"), a("resize_rel_y"), a("resize_mocha_rel_y"), l(vec![a("id_insert"), a("526")])]),
            l(vec![a(":modify"), a("shift_mocha"), a("shift_mocha_mask"), l(vec![a("id_insert"), a("527")])]),
            l(vec![a(":modify"), a("bypass_mocha"), a("bypass_mocha_mask"), l(vec![a("id_insert"), a("531")])]),
            l(vec![a(":modify"), a("show_mocha_only"), a("show_mocha_mask_only"), l(vec![a("id_insert"), a("529")])]),
            l(vec![a(":modify"), a("combine_masks"), l(vec![a("id_insert"), a("530"), a("group_end"), a("mocha")])]),
            l(vec![a(":modify"), a("center"), l(vec![a("normalize"), a("widget"), a("cross_circle"), a("name"), a("center"), a("instance"), a("0"), a("widget"), a("rect"), a("name"), a("from_transfm"), a("instance"), a("1"), a("widget"), a("rect_green"), a("name"), a("to_transform"), a("instance"), a("2"), a("widget"), a("rectshift"), a("name"), a("from_shift"), a("instance"), a("3"), a("widget"), a("rectshift_green"), a("name"), a("to_shift"), a("instance"), a("4")])]),
            l(vec![a(":modify"), a("from_z_dist"), l(vec![a("slider_range"), a(".1"), a("10"), a("widget"), a("rect.z_dist"), a("instance"), a("1"), a("widget"), a("rectshift.z_dist"), a("instance"), a("3")])]),
            l(vec![a(":modify"), a("from_rotate"), l(vec![a("angle_no_dial"), a("slider_range"), a("-90"), a("90"), a("widget"), a("rect.rotate"), a("instance"), a("1")])]),
            l(vec![a(":modify"), a("from_shift"), l(vec![a("min"), a("*"), a("normalize_scale"), a("1"), a("slider_range"), a("-1"), a("1"), a("big_slider_range"), a("-2"), a("2"), a("widget"), a("rect.shift"), a("instance"), a("1"), a("widget"), a("rectshift.shift"), a("default_off"), a("instance"), a("3")])]),
            l(vec![a(":modify"), a("to_z_dist"), l(vec![a("slider_range"), a(".1"), a("10"), a("new_column"), a("widget"), a("rect_green.z_dist"), a("instance"), a("2"), a("widget"), a("rectshift_green.z_dist"), a("instance"), a("4")])]),
            l(vec![a(":modify"), a("to_rotate"), l(vec![a("angle_no_dial"), a("slider_range"), a("-90"), a("90"), a("widget"), a("rect_green.rotate"), a("instance"), a("2")])]),
            l(vec![a(":modify"), a("to_shift"), l(vec![a("min"), a("*"), a("normalize_scale"), a("1"), a("slider_range"), a("-1"), a("1"), a("big_slider_range"), a("-2"), a("2"), a("widget"), a("rect_green.shift"), a("instance"), a("2"), a("widget"), a("rectshift_green.shift"), a("default_off"), a("instance"), a("4")])]),
            a("exposure_bias"),
            l(vec![a(":modify"), a("brightness"), a("scale_result")]),
            a("wrap"),
            a("blur_res"),
            a("subpixel"),
            l(vec![a(":modify"), a("blur_matte"), a("blur_mask")]),
            l(vec![a(":modify"), a("invert_matte"), a("invert_mask")]),
            l(vec![a(":modify"), a("matte_use"), a("mask_use")]),
            l(vec![a(":extra"), a("opacity"), a("="), a("1"), l(vec![a("ignore"), a("popup"), a("All_Opaque"), a("Normal"), a("As_Premult")])]),
        ]),
    ]
}

fn layout_blur_motion_mono_autogen() -> Vec<Sx> {
    vec![
        a("layout_effect"),
        l(vec![a("Blur_Mono"), a("Blur_Motion_Mono_Autogen")]),
        l(vec![a("Source"), l(vec![a("Matte"), a("Mask")])]),
        l(vec![]),
        l(vec![
            l(vec![a(":modify"), a("mocha_project"), l(vec![a("id_insert"), a("520")])]),
            l(vec![a(":modify"), a("blur_mocha"), a("blur_mocha_mask"), l(vec![a("id_insert"), a("521"), a("group_start"), a("mocha")])]),
            l(vec![a(":modify"), a("mocha_opacity"), a("mocha_mask_opacity"), l(vec![a("id_insert"), a("522")])]),
            l(vec![a(":modify"), a("invert_mocha"), a("invert_mocha_mask"), l(vec![a("id_insert"), a("523")])]),
            l(vec![a(":modify"), a("resize_mocha"), a("resize_mocha_mask"), l(vec![a("id_insert"), a("524")])]),
            l(vec![a(":modify"), a("resize_rel_x"), a("resize_mocha_rel_x"), l(vec![a("id_insert"), a("525")])]),
            l(vec![a(":modify"), a("resize_rel_y"), a("resize_mocha_rel_y"), l(vec![a("id_insert"), a("526")])]),
            l(vec![a(":modify"), a("shift_mocha"), a("shift_mocha_mask"), l(vec![a("id_insert"), a("527")])]),
            l(vec![a(":modify"), a("bypass_mocha"), a("bypass_mocha_mask"), l(vec![a("id_insert"), a("531")])]),
            l(vec![a(":modify"), a("show_mocha_only"), a("show_mocha_mask_only"), l(vec![a("id_insert"), a("529")])]),
            l(vec![a(":modify"), a("combine_masks"), l(vec![a("id_insert"), a("530"), a("group_end"), a("mocha")])]),
            l(vec![a(":modify"), a("center"), l(vec![a("normalize"), a("widget"), a("cross_circle"), a("name"), a("center"), a("instance"), a("0"), a("widget"), a("rect"), a("name"), a("from_transfm"), a("instance"), a("1"), a("widget"), a("rect_green"), a("name"), a("to_transform"), a("instance"), a("2"), a("widget"), a("rectshift"), a("name"), a("from_shift"), a("instance"), a("3"), a("widget"), a("rectshift_green"), a("name"), a("to_shift"), a("instance"), a("4")])]),
            l(vec![a(":modify"), a("from_z_dist"), l(vec![a("slider_range"), a(".1"), a("10"), a("widget"), a("rect.z_dist"), a("instance"), a("1"), a("widget"), a("rectshift.z_dist"), a("instance"), a("3")])]),
            l(vec![a(":modify"), a("from_rotate"), l(vec![a("angle_no_dial"), a("slider_range"), a("-90"), a("90"), a("widget"), a("rect.rotate"), a("instance"), a("1")])]),
            l(vec![a(":modify"), a("from_shift"), l(vec![a("min"), a("*"), a("normalize_scale"), a("1"), a("slider_range"), a("-1"), a("1"), a("big_slider_range"), a("-2"), a("2"), a("widget"), a("rect.shift"), a("instance"), a("1"), a("widget"), a("rectshift.shift"), a("default_off"), a("instance"), a("3")])]),
            l(vec![a(":modify"), a("to_z_dist"), l(vec![a("slider_range"), a(".1"), a("10"), a("new_column"), a("widget"), a("rect_green.z_dist"), a("instance"), a("2"), a("widget"), a("rectshift_green.z_dist"), a("instance"), a("4")])]),
            l(vec![a(":modify"), a("to_rotate"), l(vec![a("angle_no_dial"), a("slider_range"), a("-90"), a("90"), a("widget"), a("rect_green.rotate"), a("instance"), a("2")])]),
            l(vec![a(":modify"), a("to_shift"), l(vec![a("min"), a("*"), a("normalize_scale"), a("1"), a("slider_range"), a("-1"), a("1"), a("big_slider_range"), a("-2"), a("2"), a("widget"), a("rect_green.shift"), a("instance"), a("2"), a("widget"), a("rectshift_green.shift"), a("default_off"), a("instance"), a("4")])]),
            a("exposure_bias"),
            l(vec![a(":modify"), a("brightness"), a("scale_result")]),
            a("wrap"),
            a("blur_res"),
            a("subpixel"),
            l(vec![a(":modify"), a("blur_matte"), a("blur_mask")]),
            l(vec![a(":modify"), a("invert_matte"), a("invert_mask")]),
            l(vec![a(":modify"), a("matte_use"), a("mask_use")]),
            l(vec![a(":extra"), a("opacity"), a("="), a("1"), l(vec![a("ignore"), a("popup"), a("All_Opaque"), a("Normal"), a("As_Premult")])]),
        ]),
    ]
}

fn layout_glow_edges() -> Vec<Sx> {
    vec![
        a("layout_effect"),
        a("Glow_Edges"),
        l(vec![a("Source"), a("Background"), l(vec![a("Matte"), a("Mask")])]),
        l(vec![]),
        l(vec![
            l(vec![a(":modify"), a("mocha_project"), l(vec![a("id_insert"), a("520")])]),
            l(vec![a(":modify"), a("blur_mocha"), a("blur_mocha_mask"), l(vec![a("id_insert"), a("521"), a("group_start"), a("mocha")])]),
            l(vec![a(":modify"), a("mocha_opacity"), a("mocha_mask_opacity"), l(vec![a("id_insert"), a("522")])]),
            l(vec![a(":modify"), a("invert_mocha"), a("invert_mocha_mask"), l(vec![a("id_insert"), a("523")])]),
            l(vec![a(":modify"), a("resize_mocha"), a("resize_mocha_mask"), l(vec![a("id_insert"), a("524")])]),
            l(vec![a(":modify"), a("resize_rel_x"), a("resize_mocha_rel_x"), l(vec![a("id_insert"), a("525")])]),
            l(vec![a(":modify"), a("resize_rel_y"), a("resize_mocha_rel_y"), l(vec![a("id_insert"), a("526")])]),
            l(vec![a(":modify"), a("shift_mocha"), a("shift_mocha_mask"), l(vec![a("id_insert"), a("527")])]),
            l(vec![a(":modify"), a("bypass_mocha"), a("bypass_mocha_mask"), l(vec![a("id_insert"), a("531")])]),
            l(vec![a(":modify"), a("show_mocha_only"), a("show_mocha_mask_only"), l(vec![a("id_insert"), a("529")])]),
            l(vec![a(":modify"), a("combine_masks"), l(vec![a("id_insert"), a("530"), a("group_end"), a("mocha")])]),
            a("glow_brightness"),
            a("color"),
            l(vec![a(":modify"), a("glow_width"), a("width"), l(vec![a("new_column"), a("widget"), a("glow.size")])]),
            a("width_x"),
            a("width_y"),
            a("width_red"),
            a("width_green"),
            a("width_blue"),
            l(vec![a(":modify"), a("subpixel"), l(vec![a("shared")])]),
            l(vec![a(":modify"), a("edges_smooth"), l(vec![a("new_column")])]),
            a("edges_brightness"),
            a("edges_threshold"),
            a("show"),
            l(vec![a(":modify"), a("combine"), l(vec![a("id_insert"), a("100")])]),
            a("affect_alpha"),
            a("glow_from_alpha"),
            a("glow_under_source"),
            l(vec![a(":modify"), a("light_background"), a("light_bg")]),
            l(vec![a(":modify"), a("source_opacity"), a("scale_source")]),
            l(vec![a(":modify"), a("bg_brightness"), a("scale_background"), l(vec![a("ignore")])]),
            l(vec![a(":modify"), a("invert_matte"), a("invert_mask"), l(vec![a("new_column")])]),
            l(vec![a(":extra"), a("expand_borders"), a("="), a("0"), l(vec![a("ignore"), a("boolean")])]),
            l(vec![a(":extra"), a("opacity"), a("="), a("1"), l(vec![a("ignore"), a("popup"), a("All_Opaque"), a("Normal"), a("As_Premult")])]),
            l(vec![a(":hardcode"), a("eclipse_with_bg"), a("="), a("0")]),
            l(vec![a(":hardcode"), a("comp_premult"), a("="), a("0")]),
        ]),
    ]
}

fn layout_warp_transform_autogen() -> Vec<Sx> {
    vec![
        a("layout_effect"),
        l(vec![a("Warp_Transform"), a("Warp_Transform_Autogen")]),
        l(vec![a("Source"), l(vec![a("Matte"), a("Mask")])]),
        l(vec![a("warp_crop")]),
        l(vec![
            l(vec![a(":modify"), a("mocha_project"), l(vec![a("id_insert"), a("520")])]),
            l(vec![a(":modify"), a("blur_mocha"), a("blur_mocha_mask"), l(vec![a("id_insert"), a("521"), a("group_start"), a("mocha")])]),
            l(vec![a(":modify"), a("mocha_opacity"), a("mocha_mask_opacity"), l(vec![a("id_insert"), a("522")])]),
            l(vec![a(":modify"), a("invert_mocha"), a("invert_mocha_mask"), l(vec![a("id_insert"), a("523")])]),
            l(vec![a(":modify"), a("resize_mocha"), a("resize_mocha_mask"), l(vec![a("id_insert"), a("524")])]),
            l(vec![a(":modify"), a("resize_rel_x"), a("resize_mocha_rel_x"), l(vec![a("id_insert"), a("525")])]),
            l(vec![a(":modify"), a("resize_rel_y"), a("resize_mocha_rel_y"), l(vec![a("id_insert"), a("526")])]),
            l(vec![a(":modify"), a("shift_mocha"), a("shift_mocha_mask"), l(vec![a("id_insert"), a("527")])]),
            l(vec![a(":modify"), a("bypass_mocha"), a("bypass_mocha_mask"), l(vec![a("id_insert"), a("531")])]),
            l(vec![a(":modify"), a("show_mocha_only"), a("show_mocha_mask_only"), l(vec![a("id_insert"), a("529")])]),
            l(vec![a(":modify"), a("combine_masks"), l(vec![a("id_insert"), a("530"), a("group_end"), a("mocha")])]),
            a("scale"),
            l(vec![a(":modify"), a("shift"), l(vec![a("normalize")])]),
            a("z_dist"),
            l(vec![a(":modify"), a("rotate"), l(vec![a("new_column")])]),
            l(vec![a(":modify"), a("swivel"), l(vec![a("id_insert"), a("100")])]),
            l(vec![a(":modify"), a("tilt"), l(vec![a("id_insert"), a("101")])]),
            l(vec![a(":modify"), a("perspective_amount"), l(vec![a("id_insert"), a("102")])]),
            a("shear"),
            l(vec![a(":modify"), a("wrap"), l(vec![a("new_column")])]),
            l(vec![a(":modify"), a("filter"), l(vec![a("shared")])]),
            l(vec![a(":modify"), a("blur_matte"), a("blur_mask"), l(vec![a("shared")])]),
            l(vec![a(":modify"), a("invert_matte"), a("invert_mask"), l(vec![a("shared")])]),
            l(vec![a(":modify"), a("matte_use"), a("mask_use")]),
            l(vec![a(":extra"), a("opacity"), a("="), a("1"), l(vec![a("ignore"), a("popup"), a("All_Opaque"), a("Normal"), a("As_Premult")])]),
            l(vec![a(":hardcode"), a("comp_premult"), a("="), a("0")]),
        ]),
    ]
}

pub(crate) fn effects(a: &mut Arena) -> Vec<EffectDef> {
    vec![
        eff_blur_color(a),
        eff_blur_color_autogen(a),
        eff_blur_mono(a),
        eff_blur_mono_autogen(a),
        eff_blur_motion_curves_no_transform(a),
        eff_blur_motion_curves_no_transform_autogen(a),
        eff_diffuse(a),
        eff_diffuse_autogen(a),
        eff_distort_rgb(a),
        eff_distort_rgb_autogen(a),
        eff_drop_shadow(a),
        eff_hue_sat_bright(a),
        eff_hue_sat_bright_autogen(a),
        eff_sharpen_best_quality(a),
        eff_sharpen_best_quality_autogen(a),
        eff_sharpen_fast(a),
        eff_sharpen_fast_autogen(a),
        eff_time_warp_rgb(a),
        eff_time_warp_rgb_autogen(a),
        eff_trails(a),
        eff_trails_autogen(a),
        eff_transform_and_blur(a),
        eff_transform_and_blur_autogen(a),
        eff_warp_perspective(a),
        eff_warp_perspective_autogen(a),
        eff_flicker(a),
        eff_glow(a),
        eff_swishpan(a),
        eff_dissolve_shake(a),
        eff_shake_jumpy(a),
        eff_shake_jumpy_autogen(a),
        eff_shake_normal(a),
        eff_shake_normal_autogen(a),
        eff_shake_twitchy(a),
        eff_shake_twitchy_autogen(a),
        eff_blur_motion(a),
        eff_blur_motion_autogen(a),
        eff_blur_motion_mono(a),
        eff_blur_motion_mono_autogen(a),
        eff_glow_edges(a),
        eff_warp_transform(a),
        eff_warp_transform_autogen(a),
    ]
}

pub(crate) fn defuns(a: &mut Arena) -> Vec<DefunDef> {
    vec![
        defun_combine_masks(a),
        defun_mocha_mask(a),
        defun_process_masks(a),
        defun_scale_lights(a),
        defun_scale_lights_offset_darks(a),
        defun_warp_mask(a),
        defun_blur_mo_curves_get(a),
        defun_blur_motion_curves_from_to(a),
        defun_drop_shadow_combine(a),
        defun_drop_shadow_shadow(a),
        defun_feedback_combine(a),
        defun_feedback_init_prev(a),
        defun_maybe_add_alpha(a),
        defun_warp_combine(a),
        defun_combine_color_masks(a),
        defun_process_color_masks(a),
        defun_rays_atmosphere(a),
        defun_color_mask(a),
        defun_glow_rgba_combine(a),
        defun_transition_amount(a),
        defun_transition_swap_inputs(a),
        defun_delay_dissolve(a),
        defun_channel_with_alpha(a),
        defun_shake(a),
        defun_shake_channels(a),
        defun_shake_internal(a),
        defun_smooth_twitch(a),
        defun_twitch(a),
        defun_eq(a),
    ]
}

pub(crate) fn layouts() -> Vec<Vec<Sx>> {
    vec![
        layout_blur_color_autogen(),
        layout_blur_mono_autogen(),
        layout_transform_and_blur_autogen(),
        layout_blur_motion_curves_no_transform_autogen(),
        layout_diffuse_autogen(),
        layout_distort_rgb_autogen(),
        layout_drop_shadow(),
        layout_hue_sat_bright_autogen(),
        layout_sharpen_best_quality_autogen(),
        layout_sharpen_fast_autogen(),
        layout_time_warp_rgb_autogen(),
        layout_trails_autogen(),
        layout_warp_perspective_autogen(),
        layout_flicker(),
        layout_glow(),
        layout_swishpan(),
        layout_dissolve_shake(),
        layout_shake_normal_autogen(),
        layout_shake_twitchy_autogen(),
        layout_shake_jumpy_autogen(),
        layout_blur_motion_autogen(),
        layout_blur_motion_mono_autogen(),
        layout_glow_edges(),
        layout_warp_transform_autogen(),
    ]
}
