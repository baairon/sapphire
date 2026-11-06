//! fx_run: the NV-9 command line. Takes a program, a binding, the input planes and host frames, writes the output planes.
//!
//! fx_run [<programs.tsv> | -] <program> --size WxH --out PREFIX [--set name=v1,v2 ...] [--hc] [--mask-image]
//!   [--in Name=path[:C]] [--frame OFF=path[:C]] [--prev path[:C]] [--xy path] [--synth SEED]
//!   [--geom left,right,aspect,assumption_e,assumption_h,roi_x,roi_y] [--geom-old (NV9-GEOM-OLD)] [--aspect a] [--static2-seed N] [--pad v] [--blur-edge 0|1] [--blur-scale N]
//!
//! Files are raw little-endian f32, planar (C planes of W*H floats, plane 0 first). `Name` is an input of the plan (Source, Front,
//! Back, Mask, ...); an unbound Mask is the constant 1 unless --mask-image. --hc is the unkeyed HOST-CURVES binding (W116).
//! Writes PREFIX.dest.f32, PREFIX.dest.meta.txt (size, channels, the run list with combo tags) and PREFIX.prev.f32 (what
//! save_prev_dest stored). --synth SEED fills every input not given with the deterministic generator `synth_img`.
use fx_eval::{build_effect, Corpus};
use fx_exec::{Binding, Geom, HostFrames, Image, Inputs};
use std::collections::BTreeMap;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let h = std::thread::Builder::new().stack_size(256 << 20).spawn(move || run(&a)).unwrap();
    std::process::exit(h.join().unwrap());
}

fn read_f32(path: &str, c: usize, n: usize) -> Result<Image, String> {
    let b = std::fs::read(path).map_err(|e| format!("cannot read {}: {}", path, e))?;
    if b.len() != c * n * 4 {
        return Err(format!("{}: {} bytes, expected {} ({} channels x {} floats)", path, b.len(), c * n * 4, c, n));
    }
    let f: Vec<f32> = b.chunks_exact(4).map(|x| f32::from_le_bytes([x[0], x[1], x[2], x[3]])).collect();
    Ok(Image { n, planes: f.chunks(n).map(|p| p.to_vec()).collect() })
}

fn write_f32(path: &str, im: &Image) -> std::io::Result<()> {
    let mut b = Vec::with_capacity(im.planes.len() * im.n * 4);
    for p in &im.planes {
        for x in p {
            b.extend_from_slice(&x.to_le_bytes());
        }
    }
    std::fs::write(path, b)
}

/// deterministic synthetic planes in [0.05, 1.05] (LCG 6364136223846793005 / 1442695040888963407, state = seed)
pub fn synth_img(c: usize, n: usize, seed: u64) -> Image {
    let mut s = seed;
    let planes = (0..c)
        .map(|_| {
            (0..n)
                .map(|_| {
                    s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
                    0.05 + ((s >> 33) as u32 as f32 / 2147483648.0)
                })
                .collect()
        })
        .collect();
    Image { n, planes }
}

fn split(p: &str, dflt: usize) -> (String, usize) {
    match p.rsplit_once(':') {
        Some((a, cc)) if cc.parse::<usize>().is_ok() => (a.to_string(), cc.parse().unwrap()),
        _ => (p.to_string(), dflt),
    }
}

fn run(a0: &[String]) -> i32 {
    // NATIVE PROGRAMS: the thirteen styles are built by fx_eval::programs_native; no programs table is read. The first argument is then absent
    // (`fx_run <program> --size ...`) or `-`. A path in that place is an optional override (tests): the text route.
    let a0n: Vec<String> = native_args(a0);
    let a0: &[String] = &a0n;
    if a0.iter().any(|x| x == "--host-times") {
        return run_history(a0);
    }
    if a0.len() < 3 {
        eprintln!("usage: fx_run [<programs.tsv> | -] <program> --size WxH --out PREFIX [options], see the source header");
        return 2;
    }
    // INT17 step 5 (Batch 49 step 5, Batch 47 rule 3): STYLE-SELECT. --effect-answer N --effect-count K declare the host popup "Effect" answer and
    // the program count [owner]; the selected program must be the one run, else STYLE-SELECT-MISMATCH. Absent: check-only (INT16-A3).
    let (a_style, style_label) = match style_select_args(a0) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("{}", e);
            return 2;
        }
    };
    let a: &[String] = &a_style;
    let path = a[1].clone();
    let prog = a[2].clone();
    let (mut w, mut h) = (0usize, 0usize);
    let mut out = String::new();
    let mut b = Binding::defaults();
    // INT6 (Batch 36 rule 1): no harness default Mask = 1; the bound-input mask below decides, a clear bit takes the program-text default.
    let mut bound_mask: Option<u32> = None;
    let mut mask_bad = false;
    let mut bind_supplied = false;
    let mut legacy_seed_w = false;
    let mut seed_missing = false;
    let mut mask_image = false;
    let (mut ins, mut frames): (Vec<(String, String)>, Vec<(String, String)>) = (Vec::new(), Vec::new());
    let (mut prev, mut xy, mut synth): (Option<String>, Option<String>, Option<u64>) = (None, None, None);
    let rest: Vec<&String> = a[3..].iter().collect();
    let mut round_old = false;
    // INT5: DIF1 (W267 d) host count and frame of Diffuse's seed [ctx] = [ctx] * cvttss2si32([ctx]); DIFM1 source bitmask; CELL-GT1 band cell and margin
    let mut host_count: Option<i32> = None;
    let mut host_frame: Option<f32> = None;
    // INT29 step TW (W559 (a)): the request inputs of the get_frame route (--tw-route) and the host frames keyed by request (--frame-req)
    let mut tw_route_s: Option<String> = None;
    let mut frame_reqs: Vec<(String, String)> = Vec::new();
    // step FH (W607 (c)): host frames on the 'ae32' route as the harness's ae32 words, converted by the head and class of their input
    let mut frame_words: Vec<(String, String)> = Vec::new();
    let (mut cell, mut margin): (usize, usize) = (1, 0);
    // INT1 CONVERT wiring: None = CONVERT-STANDIN (planes go straight in and out, the NV-9 route); Some(mode byte) = native convert pair on host memory
    let mut convert: Option<i8> = std::env::var("INT1_CONVERT").ok().and_then(|v| v.parse().ok());
    let mut seed_given = false;
    let mut ds_phase_opt: Option<u32> = None;
    // INT11: the 'ae32' interface (format 14 in, code 14 out). Every input is required in this mode; nothing has a default.
    let mut ae: Ae32Args = Ae32Args::default();
    let mut i = 0;
    while i < rest.len() {
        let k = rest[i].as_str();
        let val = rest.get(i + 1).map(|s| s.to_string()).unwrap_or_default();
        match k {
            "--size" => {
                let (x, y) = val.split_once('x').unwrap_or(("0", "0"));
                w = x.parse().unwrap_or(0);
                h = y.parse().unwrap_or(0);
                i += 1;
            }
            "--out" => {
                out = val;
                i += 1;
            }
            "--set" => {
                if let Some((n, l)) = val.split_once('=') {
                    b.overrides.insert(n.to_string(), parse_set_lanes(l));
                }
                i += 1;
            }
            "--no-layout-merge" => {} // LYM1: A/B gate only, read again below
            "--hc" => b.host_curves_unkeyed = true,
            "--hc-zero-legacy" => b.hc_zero_legacy = true, // INT8 HC-ZERO-LEGACY (named state A option, Batch 40 rule 4)
            "--hc-keyed" => {
                // HCK1: keyed host curve answers, TSV rows: curve name, time f32 bits (hex), lanes (hex words, comma separated)
                let mut m: std::collections::BTreeMap<(String, u32), Vec<f32>> = Default::default();
                for l in std::fs::read_to_string(&val).unwrap_or_default().lines().filter(|l| !l.is_empty() && !l.starts_with("curve\t")) {
                    let f: Vec<&str> = l.split('\t').collect();
                    if f.len() >= 3 {
                        let tb = u32::from_str_radix(f[1], 16).unwrap_or(0);
                        m.insert((f[0].to_string(), tb), f[2].split(',').filter_map(|x| u32::from_str_radix(x, 16).ok()).map(f32::from_bits).collect());
                    }
                }
                b.host_keyed = Some(m);
                i += 1;
            }
            "--host-curve-values" => {
                // W479/W490 step T: the CLAIMED get_curve_val input (a host interface like the source pixels); file form in fx_eval/src/hostcurve.rs
                match std::fs::read_to_string(&val).map_err(|e| e.to_string()).and_then(|t| fx_eval::hostcurve::HostCurveValues::parse(&t)) {
                    Ok(mut h) => {
                        h.source = val.clone();
                        b.host_curve_values = Some(h)
                    }
                    Err(e) => {
                        eprintln!("--host-curve-values: {}", e);
                        return 2;
                    }
                }
                i += 1;
            }
            "--ds-phase" => {
                // W490 (c) ALIGN-HOST-PER-RUN: the small-plane base phase mod 32 of exec-downsampled is a claimed per-run host input (W159 (a), W431 (d)); accepted on the 'ae32' route
                match val.parse::<u32>() {
                    Ok(p) if p < 32 => ds_phase_opt = Some(p),
                    _ => {
                        eprintln!("--ds-phase takes 0 to 31 (mod 32)");
                        return 2;
                    }
                }
                b.kp.ds_dst_phase = ds_phase_opt.unwrap();
                i += 1;
            }
            "--src-chan" => {
                // SHF1: HOST-CONTEXT-MINIMAL (W112): Source bound with N channels (3 for no_alpha_only programs: Sharpen_Fast, Sharpen_Best_Quality, Z_Glow; the original rejects 4)
                b.input_elnum.insert("Source".to_string(), val.parse().unwrap_or(4));
                b.input_elnum.insert("Foreground".to_string(), val.parse().unwrap_or(4));
                i += 1;
            }
            "--const-in" => {
                // DSW6: bind an input to a constant, Name=v (an unbound input with a numeric default, e.g. Drop_Shadow Background 0)
                if let Some((n, v)) = val.split_once('=') {
                    b.const_inputs.insert(n.into(), v.split(',').map(|x| x.parse::<f32>().unwrap_or(0.0)).collect());
                }
                i += 1;
            }
            "--bound-mask" => {
                // INT6 rule 1: the ONE bound-input mask (hex), required, bit k = declared input k
                match u32::from_str_radix(val.trim_start_matches("0x"), 16) {
                    Ok(m) if bound_mask.is_none() || bound_mask == Some(m) => bound_mask = Some(m),
                    _ => mask_bad = true,
                }
                i += 1;
            }
            "--host-count" => {
                // DIF1: [ctx]. LABELLED STATE A TEST INTERFACE (W267 d): the claim route derives it as W x H; this option exists only to run state A (count W)
                host_count = val.parse().ok();
                i += 1;
            }
            "--host-frame" => {
                // DIF1: [ctx] as a float, truncated with cvttss2si32 (W267 d, W269 c)
                host_frame = val.parse().ok();
                i += 1;
            }
            "--tw-route" => {
                // INT29 step TW: dur_frames,limit_cell,local_time_step,host_id,f4,r8,r9 from the run's record (no field has a default)
                tw_route_s = Some(val);
                i += 1;
            }
            "--frame-words" => {
                if let Some((n, p)) = val.split_once('=') {
                    frame_words.push((n.into(), p.into()));
                }
                i += 1;
            }
            "--frame-req" => {
                // INT29 step TW: FRAME,STORED=path, the host frame answering the request (frame after the limit, stored time)
                if let Some((n, p)) = val.split_once('=') {
                    frame_reqs.push((n.into(), p.into()));
                }
                i += 1;
            }
            "--mask-byte" => {
                // STATE-A-MASK (Batch 36 rule 1): general test interface naming the same mask as --bound-mask, decimal ([owner]); not Diffuse-only
                match val.parse::<u32>() {
                    Ok(m) if bound_mask.is_none() || bound_mask == Some(m) => bound_mask = Some(m),
                    _ => mask_bad = true,
                }
                i += 1;
            }
            "--cell" => {
                cell = val.parse().unwrap_or(0);
                i += 1;
            }
            "--margin" => {
                margin = val.parse().unwrap_or(0);
                i += 1;
            }
            "--ranges-off" => b.ranges_off = true,
            "--mask-image" => mask_image = true, // legacy (only with --bind-supplied): the explicit Mask image is bound although its mask bit is clear
            "--ctx-time" | "--ctx-word2" | fx_exec::judge_keys::OPT_CTX_OWNER_MODE | "--ctx-has-data" => i += 1,
            "--xy-base-list" | "--xy-render" => i += 1, // XWP2 (W746): parsed beside XY-HEAD-MODE // parsed by CompileContext::from_args
            "--word2-cert" | "--cert-job" | "--cert-sidecar" | "--sidecar-record" => i += 1, // INT13 (Batch 49 rule 14), INT15 (Batch 51 rule 1): parsed by word2_certified
            "--ranges" => {
                // Batch 36 rule 5: REAL default; REC-HARNESS is the named state A option
                match fx_eval::ranges::range_set_from_args(&a) {
                    Some(Ok(s)) => {
                        let _ = fx_eval::ranges::set_range_set(s);
                    }
                    _ => {
                        eprintln!("--ranges takes REAL or REC-HARNESS");
                        return 2;
                    }
                }
                i += 1;
            }
            "--legacy-seed-w" => legacy_seed_w = true, // LEGACY-SEED-W (named option)
            "--bind-supplied" => bind_supplied = true, // STATE-A-BIND-SUPPLIED: legacy gate semantics, see the rule block after the graph is built
            "--in" => {
                if let Some((n, p)) = val.split_once('=') {
                    ins.push((n.into(), p.into()));
                }
                i += 1;
            }
            "--frame" => {
                if let Some((n, p)) = val.split_once('=') {
                    frames.push((n.into(), p.into()));
                }
                i += 1;
            }
            "--prev" => {
                prev = Some(val);
                i += 1;
            }
            "--xy" => {
                xy = Some(val);
                i += 1;
            }
            "--synth" => {
                synth = val.parse().ok();
                i += 1;
            }
            "--geom" => {
                let v: Vec<f32> = val.split(',').filter_map(|x| x.parse().ok()).collect();
                if v.len() == 7 {
                    b.kp.geom = Some(Geom { left: v[0], right: v[1], aspect: v[2], assumption_e: v[3], assumption_h: v[4], roi_x: v[5] as i32, roi_y: v[6] as i32 });
                }
                i += 1;
            }
            "--geom-old" => b.kp.geom_mode = fx_exec::GeomMode::Nv9Old,
            "--aspect" => {
                b.kp.host_aspect = val.parse().unwrap_or(1.0);
                i += 1;
            }
            "--static2-seed" => {
                seed_given = true;
                b.kp.static2_seed_a = val.parse().unwrap_or(0);
                i += 1;
            }
            "--convert" => {
                // alpha mode byte of the native convert-to-dest (0 none, 1 clamp-keep, 2 zero, 3 one)
                convert = val.parse().ok();
                i += 1;
            }
            "--convert-derive" => {
                // alpha mode from the dump rule with the harness host values (ctx = 0, no owner override, 4 planes); the value is dword [program]
                let pm: i32 = val.parse().unwrap_or(0);
                convert = Some(convert_pair::derive_alpha_mode(0, pm, 0, 0, 4));
                i += 1;
            }
            "--convert-standin" => convert = None,
            "--frame-round-old" => {
                round_old = true;
            }
            "--pad" => {
                let v: f32 = val.parse().unwrap_or(0.0);
                b.kp.pad_pixel = [v; 4];
                i += 1;
            }
            "--blur-edge" => {
                b.kp.blur_edge = val.parse().unwrap_or(1);
                i += 1;
            }
            "--xy-head" => {
                b.kp.xy_head = val.parse().unwrap_or(-1);
                i += 1;
            }
            "--blur-aspect-ctx" => {
                // INT10 (BCX1 PATCH 1): the recorded lw ctx cells FLAG (decimal) and BITS (hex f32 bits): Some(f32) iff FLAG != 0
                let p: Vec<&str> = val.split(',').collect();
                let fl: Option<i64> = p.first().and_then(|s| s.trim().parse().ok());
                let bt: Option<u32> = p.get(1).and_then(|s| u32::from_str_radix(s.trim().trim_start_matches("0x"), 16).ok());
                match (p.len(), fl, bt) {
                    (2, Some(f), Some(v)) => b.kp.blur_aspect = if f != 0 { Some(f32::from_bits(v)) } else { None },
                    _ => {
                        eprintln!("--blur-aspect-ctx takes FLAG,BITS (ctx decimal, ctx hex f32 bits)");
                        return 2;
                    }
                }
                i += 1;
            }
            "--do-postdiv" => {
                // INT10 (BCX1 PATCH 2): the compiler's __do_postdiv__ cell from the SMART context (0 or 1); absent = the state A derivation
                b.do_postdiv = match val.as_str() {
                    "0" => Some(false),
                    "1" => Some(true),
                    _ => {
                        eprintln!("--do-postdiv takes 0 or 1");
                        return 2;
                    }
                };
                i += 1;
            }
            "--blur-scale" => {
                b.kp.blur_render_scale = val.parse().unwrap_or(1);
                i += 1;
            }
            "--ae32-src" | "--src-planes" | "--owner-arm" | "--owner-opacity" | fx_exec::judge_keys::OPT_SRC_ROW_GATE | "--src-head" | "--aspect-triple" | "--ae32-dest" | "--ae32-prior" | "--ae32-copier" | "--ae32-out"
            | "--host-param" | "--units-ratio" | "--dest-stack-word" | "--prerender-late" | "--mode-inputs" | "--host-inplace" | "--class-cells" | "--class-record" | "--class3-record"
            | "--cset" | fx_exec::judge_keys::OPT_OPACITY_ARM | "--persp-base" | "--palpha-base" | "--palpha-cells" | "--xy-base" => {
                if let Err(e) = ae.take(k, &val) {
                    eprintln!("{}", e);
                    return 2;
                }
                i += 1;
            }
            other => {
                eprintln!("unknown option {}", other);
                return 2;
            }
        }
        i += 1;
    }
    if cell != 1 {
        eprintln!("{}", fx_exec::ExecError::CellGt1NotClaimed(cell));
        return 1;
    }
    if margin > 0 {
        eprintln!("convert: margin count {} above 0 is outside the claim (W211 e)", margin);
        return 1;
    }
    if ds_phase_opt.is_some() && std::env::var("DS2_PHASE").is_ok() {
        eprintln!("--ds-phase and DS2_PHASE given together (DS2_PHASE is a refused switch on the 'ae32' route, an error with the option on a state A route)");
        return 2;
    }
    if let Some(h) = b.host_curve_values.as_mut() {
        if h.time_frames.is_none() {
            h.time_frames = host_frame;
        }
    }
    if !seed_given && w > 0 {
        if let Some(fr) = host_frame {
            // DIF1 (W267 d): : [ctx] = [ctx] * cvttss2si32([ctx]) (32 bit imul wrap, ctx = 0); static2 adds it to its seed .
            // The count is W x H unless the state A test option --host-count gives it.
            let cnt = host_count.unwrap_or((w * h) as i32);
            b.kp.static2_seed_a = cnt.wrapping_mul(fx_common::x86::cvttss2si32(fr));
        } else if host_count.is_some() {
            eprintln!("--host-frame is required with --host-count (no default frame, W267 d)");
            return 2;
        } else if legacy_seed_w {
            // LEGACY-SEED-W (Batch 38 rule 5, W293 (c)): the NV10-P3 seed [ctx] = W of the W172 state A suites. A named option, never a default, never in a counted run.
            b.kp.static2_seed_a = w as i32;
        } else {
            seed_missing = true; // an error only if the compiled plan runs static2 (checked after the compile)
        }
    }
    // INT13 (Batch 49 rule 14, W405): --ctx-word2 1 is accepted; the native compiles it by its word 2 = 0 path only for a (program, size,
    // binding) row of the certificate file (--word2-cert FILE), else STACK-WORD2-NOT-IMPLEMENTED.
    let a_w2: Vec<String>;
    let mut w2_label: Option<String> = None;
    let a: &[String] = match word2_certified(a, &prog, w, h) {
        Ok(None) => a,
        Ok(Some((v, l))) => {
            a_w2 = v;
            w2_label = Some(l);
            &a_w2
        }
        Err(e) => {
            eprintln!("{}", e);
            return 2;
        }
    };
    match fx_eval::CompileContext::from_args(a) {
        Ok(c) => b.ctx = Some(c),
        Err(e) => {
            eprintln!("{}", e);
            return 2;
        }
    }
    if mask_bad || bound_mask.is_none() {
        eprintln!("the bound-input mask is required and must be one hex value: --bound-mask HEX (or --mask-byte N, STATE-A-MASK); missing, unparsable or conflicting masks are an error (Batch 36 rule 1)");
        return 2;
    }
    let bmask = bound_mask.unwrap();
    b.bound_input_mask = bmask;
    if w == 0 || h == 0 || out.is_empty() {
        eprintln!("need --size WxH and --out PREFIX");
        return 2;
    }
    let n = w * h;
    // LYM1 (W225 (b), LAY1): the registered program as the original's parser sees it: the layout_effect row ALONE (its `:extra` parameters
    // join the program's parameter array). --no-layout-merge reproduces the def_effect-only program of INT16 (A/B gate only).
    let layout_merge = !a.iter().any(|x| x == "--no-layout-merge");
    let lay_info: Option<fx_eval::layoutmerge::LayoutInfo> = if layout_merge {
        match lym_resolve(&path, &prog) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("{}", e);
                return 2;
            }
        }
    } else {
        None
    };
    // INT26 step OP (W558 (a), W531 (b) 5): the host answer form. --cset Opacity=V (the checkout display name, V the host's checkout integer)
    // gives the loop value by OPC1's conversion; a given --owner-opacity is then a check (OWNER-OPACITY-LOOP-MISMATCH, W531 (b) 3).
    // step L2b (W616 (c), W547): each --cset display name maps to opacity, input_opacity or output_opacity by the program's registered
    // display names; any other name is not a host answer on this route
    for (n, v) in ae.csets.clone() {
        // W630 (b): a popup answer outside 1..N (N the registered choice count) refuses; the name resolution below keeps its own refusal
        if let Err(e) = popup_answer_check(&prog, &n, v) {
            eprintln!("{}", e);
            return 2;
        }
        match cset_param(&prog, &n) {
            Some(p) if p == "output_opacity" => ae.host_out_opacity = Some(v),
            Some(p) => {
                if ae.host_opacity.is_some() {
                    eprintln!("--cset {}: a second opacity host answer (step L2b)", n);
                    return 2;
                }
                ae.host_opacity = Some(v);
                ae.host_opacity_param = Some(p);
            }
            None => {
                eprintln!("CSET-NAME-NOT-ON-ROUTE: --cset {}: not the display name of {}'s opacity, input_opacity or output_opacity popup (STYLE_PARAM_INDEX.tsv, W547, step L2b)", n, prog);
                return 2;
            }
        }
    }
    if let Some(v) = ae.host_opacity {
        let conv = match fx_exec::opacity_conv::checkout_to_loop_opacity(ae.arm_cell_a.unwrap_or(0), v, None, None) {
            Ok(x) => x.to_bits(),
            Err(e) => {
                eprintln!("{}", e);
                return 2;
            }
        };
        match ae.opacity {
            Some(Some(ob)) if ob != conv => {
                eprintln!(
                    "OWNER-OPACITY-LOOP-MISMATCH (host answer: checkout integer {}: the conversion gives opacity {}, --owner-opacity gives {}; OWN1, W531)",
                    v,
                    f32::from_bits(conv),
                    f32::from_bits(ob)
                );
                return 2;
            }
            Some(None) => {
                eprintln!("OWNER-OPACITY-LOOP-MISMATCH (host answer: checkout integer {} given with --owner-opacity none; OWN1, W531)", v);
                return 2;
            }
            _ => ae.opacity = Some(Some(conv)),
        }
    }
    if let (Some(li), true) = (lay_info.as_ref(), ae.active()) {
        // LYM3 OWNER-OPACITY-DERIVED (W449 (b)), corrected by INT31 step L2 (W592 (c), W595): the owner arm's opacity, input_opacity and output_opacity are
        // f32(max(v - 1, 0)) for v the one-based popup checkout integer: the bound answer (host-answer form), or for an unbound row the host's registered
        // default from data/STYLE_PARAM_INDEX.tsv (never the layout text default). --owner-opacity stays as a cross-check that must equal the derived value.
        let setv = |n: &str| b.overrides.get(n).and_then(|v| v.first().copied());
        let mut derived_op: Option<u32> = None;
        let mut derived_name = String::new();
        let mut out_op_used = false;
        let mut in_op_used = false;
        for ex in &li.extras {
            let dv = match &ex.default {
                fx_eval::DefaultVal::Scalar(x) => *x as f32,
                _ => 0.0,
            };
            if ex.name == "opacity" || ex.name == "input_opacity" {
                derived_name = ex.name.clone();
                if ae.host_opacity_param.as_deref() == Some(ex.name.as_str()) {
                    in_op_used = true;
                }
                if let (Some(hv), Some(Some(cb)), true) = (ae.host_opacity, ae.opacity, ae.host_opacity_param.as_deref() == Some(ex.name.as_str())) {
                    // INT28 step L2 with step OP: under the host answer form (--cset Opacity=V) the record value is OPC1's conversion of the checkout
                    // integer (max(v - 1, 0), set above); a --set of the same parameter must equal it (the earlier OPACITY-SET-MISMATCH text kept)
                    if let Some(sv) = setv(&ex.name) {
                        if sv.to_bits() != cb {
                            eprintln!("OPACITY-SET-MISMATCH: --set {}={} differs from the host answer's loop value {} (--cset Opacity, step OP)", ex.name, sv, f32::from_bits(cb));
                            return 2;
                        }
                    }
                    derived_op = Some(cb);
                    ae.opacity_notes.push((ex.name.clone(), hv, None));
                } else {
                    let (v, reg) = match owner_opacity_v(&prog, &ex.name, None, dv) {
                        Ok(x) => x,
                        Err(e) => {
                            eprintln!("{}", e);
                            return 2;
                        }
                    };
                    if let Err(e) = fx_exec::opacity_conv::checkout_to_loop_opacity(ae.arm_cell_a.unwrap_or(0), v, None, None) {
                        eprintln!("{}", e);
                        return 2;
                    }
                    let db = owner_loop_value(v).to_bits();
                    if let Some(sv) = setv(&ex.name) {
                        if sv.to_bits() != db {
                            eprintln!("OPACITY-SET-MISMATCH: --set {}={} differs from the registered default's loop value {} (STYLE_PARAM_INDEX.tsv, W595)", ex.name, sv, f32::from_bits(db));
                            return 2;
                        }
                    }
                    derived_op = Some(db);
                    ae.opacity_notes.push((ex.name.clone(), v, reg));
                }
            } else if ex.name == "output_opacity" {
                // step L2b (W616 (c)): a --set of output_opacity is checked against the registered default's loop value; none is taken as given
                let sv = setv("output_opacity");
                out_op_used = true;
                match output_opacity_checked(&prog, sv, ae.host_out_opacity, dv) {
                    Ok((bits, v, reg)) => {
                        ae.out_opacity = Some(bits);
                        // W630 (d): the note is printed on every row, an unbound row with a --set included
                        if !cfg!(feature = "owner_note_set_mut") || sv.is_none() || ae.host_out_opacity.is_some() {
                            ae.opacity_notes.push(("output_opacity".into(), v, reg));
                        }
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                        return 2;
                    }
                }
            }
        }
        if let (Some(hv), false) = (ae.host_out_opacity, out_op_used) {
            eprintln!("CSET-NAME-NOT-ON-ROUTE: --cset Output Opacity={}: {} has no output_opacity parameter (step L2b, W616)", hv, prog);
            return 2;
        }
        if let (Some(p), false) = (ae.host_opacity_param.as_ref(), in_op_used) {
            eprintln!("CSET-NAME-NOT-ON-ROUTE: --cset of {}: {} has no {} parameter in its layout (step L2b, W616)", p, prog, p);
            return 2;
        }
        match (ae.opacity, derived_op) {
            (Some(Some(ob)), Some(db)) if ob != db => {
                eprintln!(
                    "OWNER-OPACITY-MISMATCH: --owner-opacity {} differs from the merged record's {} = {} (the bound value or the layout default); the option is only a cross-check (LYM3, W449 (b))",
                    f32::from_bits(ob),
                    derived_name,
                    f32::from_bits(db)
                );
                return 2;
            }
            (Some(Some(_)), None) | (Some(None), Some(_)) => {
                // the existing OWNER-INPUT-MISMATCH guard (owner_inputs_derived) names this case
            }
            (None, d) => {
                ae.opacity = Some(d);
                ae.opacity_derived = true;
            }
            _ => {}
        }
    }
    // INT11: the 'ae32' interface. Validated here: every input present, no option of another route, no environment switch.
    let mut ae32: Option<Ae32Ready> = if ae.active() {
        let given: Vec<(&str, &str)> = (0..rest.len())
            .filter(|&j| rest[j].starts_with("--"))
            .map(|j| (rest[j].as_str(), rest.get(j + 1).map(|s| s.as_str()).unwrap_or("")))
            .collect();
        match ae.ready(&prog, w, h, &given, &mut b, host_frame.is_some()) {
            Ok(r) => Some(r),
            Err(e) => {
                eprintln!("{}", e);
                return 2;
            }
        }
    } else {
        None
    };
    let c = match corpus_for(&path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            return 2;
        }
    };
    let g = match c.find_effect(&prog, None).map(|ei| build_effect(&c, ei)) {
        Some(Ok(mut g)) => {
            if let Some(li) = lay_info.as_ref() {
                // LYM2: the layout-row parse as genotype-parser.lua builds it (merge_layout): layout order, extras at their position, `:hardcode` parameters
                // removed with the constant in their value node
                match fx_eval::layoutmerge::merge_layout(&mut g, li) {
                    Ok(rep) => {
                        if std::env::var_os("LYM2_REPORT").is_some() { eprintln!("LAYOUT-ROW-MERGED ({}: params in layout order {}; :extra {:?}; :hardcode {:?}; :modify {}; LYM2)", prog, rep.order.len(), rep.extras, rep.hardcoded, rep.modify_count); }
                        // INT21 step L (W473 (b)): the vector :hardcode label, on the same report switch (the survey reads the first output line)
                        if std::env::var_os("LYM2_REPORT").is_some() && !rep.hardcoded_vec.is_empty() { eprintln!("label {} ({}: {:?})", fx_eval::layoutmerge::LAYOUT_HARDCODE_VECTOR_LABEL, prog, rep.hardcoded_vec); }
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                        return 2;
                    }
                }
            }
            g
        }
        _ => {
            eprintln!("effect {} not found or not buildable", prog);
            return 1;
        }
    };
    // INT19 S1 (W444 (b), W452 (c)): XY-HEAD-MODE derived from the plan's xy shape (kernels2::xy_head_mode_of; PerRow for Warp_Perspective plans)
    b.kp.xy_head_mode = fx_exec::kernels2::xy_head_mode_of(&g);
    // XWP2 (W746): --xy-base-list B[r1],B[r1],... one entry per default-xy writer of the placement model in execution order (base phase in bytes,
    // 0 to 31; r1 = a preserve writer's render 1 image reused), --xy-render 1|2 (default 1). The run's own record gives the bases (XY-BASE-FED-AT-REPREDICT).
    let xy_list_text: Option<String> = { let v: Vec<String> = std::env::args().collect(); v.iter().position(|x| x == "--xy-base-list").and_then(|p| v.get(p + 1).cloned()) };
    let mut xy_list_lab: Option<String> = None;
    if let Some(t) = xy_list_text {
        let v: Vec<String> = std::env::args().collect();
        let render: u32 = v.iter().position(|x| x == "--xy-render").and_then(|p| v.get(p + 1)).and_then(|s| s.parse().ok()).unwrap_or(1);
        let mut list = Vec::new();
        for e in t.split(',') {
            let (n, r1) = match e.strip_suffix("r1") { Some(x) => (x, true), None => (e, false) };
            match n.parse::<i32>() {
                Ok(b) if (0..=31).contains(&b) => list.push((b, r1)),
                _ => {
                    eprintln!("XY-BASE-LIST-BAD ({}: entry {:?}; each entry is 0 to 31, optionally followed by r1; W746)", prog, e);
                    return 2;
                }
            }
        }
        if list.is_empty() || !(1..=2).contains(&render) {
            eprintln!("XY-BASE-LIST-BAD ({}: empty list or render {} not 1 or 2; W746)", prog, render);
            return 2;
        }
        if ae.xy_base.is_none() {
            ae.xy_base = Some(list[0].0);
        }
        xy_list_lab = Some(format!("label XY-BASE-FED-AT-REPREDICT ({}: --xy-base-list {} render {}: one entry per default-xy writer of the placement model, head mode from the placement (plane PerRow, row buffer OneHead), preserve writers reused from render 1; XWP2, W746)", prog, t, render));
        *fx_exec::exec::XY_LIST.lock().unwrap() = Some((list, render));
    }
    // WPH (W634 (b)): the per-run plane base phases and perspective_alpha host cells are options on the 'ae32' route
    if ae.active() {
        if let Some(n) = ae.persp_base { b.kp.persp_base = n; }
        if let Some(n) = ae.palpha_base { b.kp.palpha_base = n; }
        if let Some(n) = ae.xy_base { b.kp.xy_base = n; }
        b.kp.palpha_cells = ae.palpha_cells;
        // WPH2 v2 (W682): every plan that reads the default xy (the xy special node, or perspective) needs --xy-base; no 'ae32' line reaches xy_grid
        if ae.xy_base.is_none() && g.nodes.iter().any(|n| (n.kind == fx_eval::build::NodeKind::Special && n.name == "xy") || (n.kind == fx_eval::build::NodeKind::Function && n.name.strip_suffix("_if_needed").unwrap_or(&n.name) == "perspective")) {
            eprintln!("XY-BASE-UNGIVEN ({}: the plan reads the default xy; the base phase of its x writer's destination is a per-run host input (ALIGN-HOST-PER-RUN); W665, W682)", prog);
            return 2;
        }
        for (kern, given) in [("perspective", ae.persp_base.is_some()), ("perspective_alpha", ae.palpha_base.is_some())] {
            let reaches = g.nodes.iter().any(|n| n.kind == fx_eval::build::NodeKind::Function && n.name.strip_suffix("_if_needed").unwrap_or(&n.name) == kern);
            if reaches && !given {
                eprintln!("PERSP-BASE-UNGIVEN ({}: the plan reaches {}; its output plane base phase is a per-run host input (ALIGN-HOST-PER-RUN); W634)", prog, kern);
                return 2;
            }
        }
    }
    // INT14 (SMR10 finding, coordinator 2026-11-03): an unknown parameter name fails closed. Every --set NAME (and every --host-param NAME,
    // which the 'ae32' route has already placed in the overrides) must be a parameter of the built program; a misspelt name used to give the
    // default plan with no error. Label: UNKNOWN-PARAM-NAME (Batch 50 rule 2; applied on every route, a superset of the claim routes).
    {
        let known: Vec<&str> = g.params.iter().map(|p| p.name.as_str()).collect();
        let bad: Vec<&String> = b.overrides.keys().filter(|n| !known.contains(&n.as_str())).collect();
        if !bad.is_empty() {
            eprintln!("UNKNOWN-PARAM-NAME: {:?} is not a parameter of {} (its parameters: {:?}); --set and --host-param names must be record names (INT14)", bad, prog, known);
            return 2;
        }
        // INT14 (Batch 50 rule 2, W414 (c) P3): MOCHA-OFF-SCOPE on the claim route ('ae32'): mocha_project != 0 or show_mocha_mask_only != 0 is
        // outside the claimed region (rule 1, MOCHA-OFF) and is refused by name; the text defaults of both are 0.
        if ae32.is_some() {
            for nm in ["mocha_project", "show_mocha_mask_only"] {
                if let Some(v) = b.overrides.get(nm) {
                    if v.iter().any(|x| *x != 0.0) {
                        eprintln!("MOCHA-OFF-SCOPE: {} = {:?} on the claim route is outside the claimed region (mocha_project != 0 or show_mocha_mask_only != 0; Batch 50 rule 2, INT14)", nm, v);
                        return 2;
                    }
                }
            }
        }
    }
    // INT13 (Batch 48 rules 1, 3; INT12 NEXT 1 (a), (c), (e)): the owner arm's inputs derived from the program definition; the recorded
    // --owner-arm and --owner-opacity fields are checks
    if let Some(h) = b.host_curve_values.as_mut() {
        h.ae32 = ae32.is_some(); // W490 (e) 2: on the claim route the global is HOST-APP's cell, 1
    }
    // step TPU2 (W708 (e) 1): a host point parameter is a host input. On the claim route its lanes come from the run's recorded point
    // lines through the ported point conversion (hostcurve::fold, the point route); no native default and no --set.
    if ae32.is_some() {
        let mut pts: Vec<(String, Vec<f32>)> = Vec::new();
        for p in &g.params {
            let row = fx_eval::hostcurve::record_of(&g.name, &p.name);
            let is_pt = row.as_ref().map(|f| f.kind_byte == 2 && matches!((f.subtype_byte as u32) & 0xf, 1 | 2 | 4 | 6 | 7)).unwrap_or(false);
            if !is_pt {
                continue;
            }
            if b.overrides.contains_key(&p.name) {
                eprintln!("TICKMAP-POINT-SET-REFUSED ({}: {}: a host point parameter is a host input from its point lines, never --set; W708)", g.name, p.name);
                return 2;
            }
            let h = match b.host_curve_values.as_ref() {
                Some(h) => h,
                None => {
                    eprintln!("TICKMAP-POINT-HOST-INPUT-UNSOURCED ({}: {}: the host point parameter needs the run's recorded point lines in --host-curve-values; W708)", g.name, p.name);
                    return 2;
                }
            };
            let mut d = Vec::new();
            match fx_eval::hostcurve::fold(&g.name, &p.name, 0.0, &[0.0, 0.0], 2, h, &mut d) {
                Some(l) => pts.push((p.name.clone(), l)),
                None => {
                    eprintln!("TICKMAP-POINT-HOST-INPUT-UNSOURCED ({}: {}: {}; W708)", g.name, p.name, d.join(" | "));
                    return 2;
                }
            }
        }
        for (n, l) in pts {
            if let Some(r) = ae32.as_mut() {
                r.labels.push(format!("label POINT-PARAM-FROM-HOST ({}: {}: lanes {} from the run's point lines through the ported point route; W708 (e))", g.name, n,
                    l.iter().map(|v| format!("{:08x}", v.to_bits())).collect::<Vec<_>>().join(",")));
            }
            b.overrides.insert(n, l);
        }
    }
    // step TPU2b (W746 (b), CURVE-PARAM-DIRECT-READ): a parameter that is a host curve (a non-point TICKMAP_PARAM_RECORDS row of the program)
    // also feeds the program's direct reads of it. Its lanes are the curve's value at the current time from the host-curve input, through the
    // tick map (hostcurve::fold at t = 0: the 16.16 conversion, the unit nibble and the record's negate flag), as the original's parameter
    // record shows it (rotate = -host Rotate, z_dist = host Z Dist; BMR1 FEED_RULES.tsv, 30 of 30 records). Never --set.
    if ae32.is_some() {
        if let Some(h) = b.host_curve_values.as_ref() {
            let mut cps: Vec<(String, Vec<f32>)> = Vec::new();
            for p in &g.params {
                let row = fx_eval::hostcurve::record_of(&g.name, &p.name);
                let f = match row {
                    Some(f) => f,
                    None => continue,
                };
                let is_pt = f.kind_byte == 2 && matches!((f.subtype_byte as u32) & 0xf, 1 | 2 | 4 | 6 | 7);
                if is_pt {
                    continue;
                }
                if b.overrides.contains_key(&p.name) {
                    eprintln!("TICKMAP-CURVE-SET-REFUSED ({}: {}: a host curve parameter's direct reads take the curve at the current time, never --set; W746)", g.name, p.name);
                    return 2;
                }
                let n: usize = f.nlanes as usize;
                let mut d = Vec::new();
                match fx_eval::hostcurve::fold(&g.name, &p.name, 0.0, &vec![0.0f32; n], n, h, &mut d) {
                    Some(l) => cps.push((p.name.clone(), l)),
                    None => {
                        eprintln!("TICKMAP-CURVE-DIRECT-READ-UNSOURCED ({}: {}: {}; W746)", g.name, p.name, d.join(" | "));
                        return 2;
                    }
                }
            }
            for (n, l) in cps {
                if let Some(r) = ae32.as_mut() {
                    r.labels.push(format!("label CURVE-PARAM-DIRECT-READ ({}: {}: lanes {} from the host curve at the current time; W746)", g.name, n,
                        l.iter().map(|v| format!("{:08x}", v.to_bits())).collect::<Vec<_>>().join(",")));
                }
                b.overrides.insert(n, l);
            }
        }
    }
    if let (Some(r), Some(l)) = (ae32.as_mut(), w2_label.as_ref()) {
        r.labels.push(l.clone());
    }
    if let (Some(r), Some(l)) = (ae32.as_mut(), style_label.as_ref()) {
        r.labels.push(l.clone());
    }
    if let Some(r) = ae32.as_mut() {
        match owner_inputs_derived(&g, &r.arm) {
            Ok(mut ls) => {
                if lay_info.is_some() {
                    // LYM3: with the layout row merged the opacity values are the record's, not a recorded input
                    ls.retain(|l| !l.starts_with("label OWNER-ARM-RECORDED"));
                    let tag = std::path::Path::new(&r.out).file_name().map(|x| x.to_string_lossy().trim_end_matches(".ae32.u32").to_string()).unwrap_or_default();
                    for (nm, v, reg) in &ae.opacity_notes {
                        ls.push(owner_opacity_label(&prog, &tag, nm, *v, reg.as_ref()));
                    }
                }
                r.labels.extend(ls)
            }
            Err(e) => {
                eprintln!("{}", e);
                return 2;
            }
        }
        // INT18 (Batch 52 rule 11, W434 (b) 2, W435 (b)): THE CHECKOUT RULE ON THE ROUTE. A --host-param whose record the fetch gate 
        // leaves unfetched at this run's [owner] (the bound mask) is not consumed: the record keeps its parse-time value, taken as the
        // text default's record (fx_eval::units::unbound_default_record, INFERRED INT16-A5; 0 for every frozen row's unfetched parameter).
        // The fetch test is fx_eval::checkout::try_refresh (owner non-NULL on the route). A decision that would rest on [owner]
        // (ba >= 0, bit clear, show_with_mocha) is refused by name: the cell is not an input of this route.
        {
            use fx_eval::checkout::{bind_bytes, try_refresh, Arms, CheckoutError};
            let mut ins: Vec<(usize, &str)> = g.inputs.iter().map(|i| (i.pos, i.name.as_str())).collect();
            ins.sort();
            let in_names: Vec<&str> = ins.iter().map(|x| x.1).collect();
            if bmask > 0xff {
                eprintln!("CHECKOUT-MASK-WIDE: the bound mask 0x{:x} does not fit the byte [owner] the fetch gate reads (INT18)", bmask);
                return 2;
            }
            for (n, _) in r.host_params.clone() {
                let p = match g.params.iter().find(|p| p.name == n) {
                    Some(p) => p,
                    None => continue, // UNKNOWN-PARAM-NAME has refused it above
                };
                let bytes = bind_bytes(&p.flags, &in_names, Arms::READ);
                let fetched = match try_refresh(bytes, bmask as u8, 0, true) {
                    Ok(f) => f,
                    Err(CheckoutError::BaOutOfRange(ba)) => {
                        eprintln!("CHECKOUT-BA-OUT-OF-RANGE: {} of {} has ba {} (>= 32; the listing's bt takes it modulo 32)", n, prog, ba);
                        return 2;
                    }
                    Err(CheckoutError::OwnerNull) => {
                        eprintln!("CHECKOUT-OWNER-NULL: {} of {}", n, prog);
                        return 2;
                    }
                };
                if !fetched && bytes.ba >= 0 && bytes.bc & 1 == 1 {
                    eprintln!("CHECKOUT-124B-UNRECORDED: {} of {} (ba {}, presence bit clear, show_with_mocha): the fetch decision rests on bit 0x10 of [owner], which is not an input of this route (INT18 stand-in INT18-A2)", n, prog, bytes.ba);
                    return 2;
                }
                if fetched {
                    continue;
                }
                let (td, ns) = match fx_eval::units::ns_entry(&prog, &n) {
                    Some(x) => x,
                    None => {
                        eprintln!("CHECKOUT-NOT-FETCHED-DEFAULT-UNREAD: {} of {} is not fetched and its text default is not a scalar of the normalize_scale table (INT18)", n, prog);
                        return 2;
                    }
                };
                if td != 0.0 {
                    // INT20 step G (Batch 55 rule 8, W463 (a) 1, INT18-A1): the cached lane at [rec] on the unfetched arm is not recorded
                    eprintln!("CHECKOUT-NOT-FETCHED-NONZERO-DEFAULT ({}: {}): not fetched and its text default {} is not 0; refused until a record shows the lane at [rec] on the unfetched arm", prog, n, td);
                    return 2;
                }
                let rec = fx_eval::units::unbound_default_record(td, ns, r.uw, false);
                b.overrides.insert(n.clone(), vec![rec]);
                let hostlbl = format!("param {} host ", n);
                r.labels.retain(|l| !l.starts_with(&hostlbl));
                r.labels.push(format!(
                    "label CHECKOUT-NOT-FETCHED ({}: {}, host value ignored as by the original's fetch gate; UNT5 model; ba {} bc {} at [owner] 0x{:02x})",
                    prog, n, bytes.ba, bytes.bc, bmask
                ));
                r.labels.push(format!("param {} record [\"0x{:08x}\"] PARAM-BY-RECORD (not fetched: the text default {} as the record, ns {} width {}; INFERRED INT16-A5)", n, rec.to_bits(), td, ns, r.uw));
            }
        }
        let ae32_ready_a = b.ctx.as_ref().map(|c| c.owner_mode_byte).unwrap_or(0);
        let rec_mode = r.route.dest.as_ref().map(|d| d.mode).unwrap_or(-1);
        let bound_in: Vec<(usize, String)> = g.inputs.iter().enumerate().filter(|(k, _)| *k < 32 && (bmask >> *k) & 1 == 1).map(|(k, i)| (k, i.name.clone())).collect();
        // DCP1 (W603 DX2): the special node Output carries the destination's plane count on the 'ae32' route (the donor of v3_to_v4_if4 in
        // Drop_Shadow is Output: the destination has 4 planes in the record, the minimal-context default of 3 would fold the node at 3 lanes).
        // step DX2 v2 (W652 (d)): the bound-count rule of v3_to_v4_if4 runs on this route only
        b.v3_bound_count = true;
        if let Some(d) = r.route.dest.as_ref() {
            if !cfg!(feature = "dx2_output_default_mut") {
                b.input_elnum.entry("Output".to_string()).or_insert(d.planes as i32);
            }
        }
        let g3in = G3In {
            class3_rec: r.class3_rec,
            rec_head: r.rec_head,
            sources: r.names.iter().zip(r.route.sources.iter()).map(|(n, s)| (n.clone(), s.reader_head, s.planes)).collect(),
            dest_planes: r.route.dest.as_ref().map(|d| d.planes).unwrap_or(0),
            copier: r.route.dest.as_ref().and_then(|d| d.copier_source),
        };
        let mut class3_inputs: Vec<String> = Vec::new();
        match mode_byte_derived(&g, &r.arm, ae32_ready_a, r.late.as_ref(), r.modein, r.host_inplace.unwrap_or(0) == 1, r.wh, rec_mode, &bound_in, r.class_cells, &r.class_record, &g3in, &mut class3_inputs) {
            Ok(ls) => {
                r.labels.extend(ls);
                // INT20 step G3 (RDR10 PATCH 1, Batch 55 rule 3): a class-3 input is compiled as an input node of elnum 3
                for n in &class3_inputs {
                    b.input_elnum.insert(n.clone(), 3);
                }
            }
            Err(e) => {
                eprintln!("{}", e);
                return 2;
            }
        }
    }
    // INT6 rule 1: bit k = declared input k. Bound: the run's image (--in, or --synth), converted like Source. Clear: the program-text default (a number) as a constant input.
    let mut alias_bound: Vec<String> = Vec::new();
    // INT31 step AL (W589 (b), W600 (a)): on the 'ae32' route the alias binds the target's node (resolved against the 'ae32' sources and `ins`)
    let ae_alias: Vec<(String, String, String, String)> = match ae32.as_ref() {
        Some(r) => {
            let tag = std::path::Path::new(&r.out).file_name().map(|x| x.to_string_lossy().trim_end_matches(".ae32.u32").to_string()).unwrap_or_default();
            let ins_names: Vec<String> = ins.iter().map(|x| x.0.to_string()).collect();
            match ae32_input_aliases(&prog, &tag, &g.inputs, bmask, &r.names, &ins_names) {
                Ok(v) => {
                    let ls: Vec<String> = v.iter().map(|a| input_alias_label(&prog, &tag, a)).collect();
                    if let Some(r) = ae32.as_mut() {
                        r.labels.extend(ls);
                    }
                    v
                }
                Err(e) => {
                    eprintln!("{}", e);
                    return 2;
                }
            }
        }
        None => Vec::new(),
    };
    for (k, inp) in g.inputs.iter().enumerate() {
        if ae_alias.iter().any(|a| a.0 == inp.name) {
            continue;
        }
        let bit = k < 32 && (bmask >> k) & 1 == 1;
        let explicit = ins.iter().any(|(nm, _)| *nm == inp.name) || (mask_image && inp.name == "Mask");
        if bit {
            continue;
        }
        if explicit && !bind_supplied {
            eprintln!("input {} (declared input {}) has its mask bit clear but an image was given: bind the bit or drop the image (rule 1)", inp.name, k);
            return 2;
        }
        if explicit {
            continue; // STATE-A-BIND-SUPPLIED
        }
        if b.const_inputs.contains_key(&inp.name) {
            continue; // --const-in (labelled state A test interface)
        }
        if bind_supplied && inp.name != "Mask" {
            continue; // STATE-A-BIND-SUPPLIED keeps the legacy harness: only Mask had a text-default constant
        }
        // ALS1 INPUT-DEFAULT-ALIAS (W353 (e), W414 (d)): an unbound input whose text default `( X Y )` names another declared input Y
        // binds Y's planes (converted like Y below). No default, or a default that is neither a number nor an input name, is unchanged.
        // Resolved through a chain of aliases (bounded by the input count); a target that is itself unbound without any image refuses by name.
        if let Some(dn) = inp.default.as_deref().filter(|_| !(cfg!(feature = "al_const_mut") && ae32.is_some())) {
            if fx_eval::orig_number_pub(dn).is_none() && g.inputs.iter().any(|o| o.name == dn) {
                let mut cur = dn.to_string();
                let mut hops = 0usize;
                let found = loop {
                    if let Some((_, p)) = ins.iter().find(|(nm, _)| *nm == cur) {
                        break Some(p.clone());
                    }
                    let nxt = g.inputs.iter().find(|o| o.name == cur).and_then(|o| o.default.clone());
                    match nxt {
                        Some(n2) if hops < g.inputs.len() && fx_eval::orig_number_pub(&n2).is_none() && g.inputs.iter().any(|o| o.name == n2) => {
                            cur = n2;
                            hops += 1;
                        }
                        _ => break None,
                    }
                };
                match found {
                    Some(p) => {
                        ins.push((inp.name.clone(), p));
                        alias_bound.push(inp.name.clone());
                        continue;
                    }
                    None => {
                        eprintln!("INPUT-DEFAULT-ALIAS: input {} defaults to input {} which has no image bound; refused", inp.name, dn);
                        return 2;
                    }
                }
            }
        }
        match inp.default.as_deref().and_then(fx_eval::orig_number_pub) {
            Some(v) => {
                b.const_inputs.insert(inp.name.clone(), vec![v as f32]);
            }
            None => {} // no text default: a plan that reads it is an error (the executor refuses a missing input)
        }
    }
    let g = {
        let mut g = g;
        alias_input_nodes(&mut g, &ae_alias);
        g
    };
    let _ = fx_eval::compile::eq_const_scan_take();
    let _ = fx_eval::compile::ifneq_arg1_nonconst_take();
    let _ = fx_eval::compile::ifneq_trace_take();
    let _ = fx_eval::fold::v3_take();
    // INT29 step TW (TWF1 PATCH 1, W559 (a)): on the 'ae32' route F (--host-frame, required there) is the explicit get_frame_number input
    // (FRAME-INDEX-REQUIRED), and a plan that reads get_frame needs the request inputs of the run's record.
    let tw_route = match tw_route_s.as_deref().map(parse_tw_route) {
        None => None,
        Some(Ok(mut q)) => match host_frame {
            Some(f) => {
                q.big_f = f;
                Some(q)
            }
            None => {
                eprintln!("FRAME-REQUEST-INPUT-UNRECORDED (--tw-route needs F from --host-frame; W559)");
                return 2;
            }
        },
        Some(Err(e)) => {
            eprintln!("{}", e);
            return 2;
        }
    };
    if ae32.is_some() || tw_route.is_some() {
        b.frame_index_required = true;
        if let Some(f) = host_frame {
            b.host.insert("get_frame_number".to_string(), vec![f]);
        }
        if ae32.is_some() && tw_route.is_none() && g.nodes.iter().any(|x| x.name == "get_frame") {
            eprintln!("FRAME-REQUEST-INPUT-UNRECORDED ({}: the plan reads get_frame; the request inputs (--tw-route: duration in frames, the limit cell, local_time_step, the host id, the step flag, r8d, r9d) are not given from the run's record; TWF2, W559)", prog);
            return 2;
        }
    }
    let cp = match fx_exec::compile_for(&g, &b) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            return 1;
        }
    };
    // INT32 step DF (W605 (c) 6, (d)): the fed 16.16 value of every unbound fixed-slider (host type 2, non-normalize_scale) parameter, one
    // diagnostic line each, for the per-run check against the run's own checkout bytes (PARAM-DEFAULT-BITS-MISMATCH is the checker's label).
    if let Some(r) = ae32.as_mut() {
        for p in &g.params {
            let ln = if p.layout_name.is_empty() { p.name.clone() } else { p.layout_name.clone() };
            if p.node.is_none() || b.overrides.get(&p.name).map(|o| !o.is_empty()).unwrap_or(false) || !fx_eval::units::registered_fixed_slider(&g.name, &ln) {
                continue;
            }
            if let Some((_, l)) = cp.params.iter().find(|x| x.0 == p.name) {
                let disp = fx_eval::units::style_param_row(&g.name, &ln).map(|x| x.display_name).unwrap_or_default();
                let fed: Vec<String> = l.iter().map(|w| format!("{:08x} ({})", w, f32::from_bits(*w) as f64 * 65536.0)).collect();
                r.labels.push(format!("diagnostic PARAM-DEFAULT-FED-16-16\t{}\t'{}'\tunbound\tfed {}", p.name, disp, fed.join(",")));
            }
        }
    }
    // INT21 step Q (W491, W487 (e)): the precondition of is ported (fx_eval prune_arm's count conjunct, EQC1); step G's refusal is
    // removed. A non-constant second argument of if_neq_const (EQC1's c7 form, not probed) is refused on the claim route.
    // INT22 step B (W509 (b) 2, W515 (d), W521 (a)): v3_to_v4_if4 folds of an unmeasured form are refused on the claim route; every fold is
    // printed as a census label (form M4, M3Z, M2S or UNMEASURED-FORM). The survey route keeps the patched result.
    {
        let v3 = fx_eval::fold::v3_take();
        if ae32.is_some() {
            if let Some(x) = v3.iter().find(|x| x.5 == "UNMEASURED-FORM") {
                let m = format!("V3TOV4IF4-NARROW-UNMEASURED ({}: node {}: n {}, argument 0 lanes {} ({}), argument 1 {}; the original's folded result for this form is not measured; measured forms: n = 4; n = 3 with a scalar argument 0 and argument 1 = 0; n = 3 with a 3-lane argument 0 and argument 1 = 1.0, argument 1 not placed; n = 3 with a one-lane argument 0 and a one-lane argument 1, argument 0 broadcast to lanes 0 to 2 and argument 1 in lane 3 (FLD3); n = 1 with one-lane arguments, the pair [argument 0, argument 1] (FLD3M); n = 2 with scalar arguments; BRD1, BRD2, W509, W515, W523)", prog, x.0, x.1, x.2, x.6.iter().map(|w| format!("{:08x}", w)).collect::<Vec<_>>().join(","), x.3.iter().map(|w| format!("{:08x}", w)).collect::<Vec<_>>().join(","));
                eprintln!("{}", m);
                // the whole line also in PREFIX.refusal.txt (no meta is written for a refused run)
                let _ = std::fs::write(format!("{}.refusal.txt", out), format!("{}\n", m));
                return 2;
            }
        }
        if let Some(r) = ae32.as_mut() {
            for x in &v3 {
                // INT24 (W533 (d)): the node's program-text path beside its id
                r.labels.push(format!("diagnostic V3TOV4IF4-FOLD\tnode {}\tn {}\targ0_lanes {}\targ1 {}\tresult {}\tform {}\targ0 {}\tpath {}", x.0, x.1, x.2, x.3.iter().map(|w| format!("{:08x}", w)).collect::<Vec<_>>().join(","), x.4.iter().map(|w| format!("{:08x}", w)).collect::<Vec<_>>().join(","), x.5, x.6.iter().map(|w| format!("{:08x}", w)).collect::<Vec<_>>().join(","), fx_exec::ops_census::node_path_fold(&cp, &g, x.0)));
            }
        }
    }
    // INT24 (W526 (a)): every run-primitive of +, *, -, / and dissolve printed as an OPS-ARITH-RUN census label on the claim route, and the
    // tripwire OPS-ARITH-SCALAR-SCALAR-UNJUDGED: a +, *, - or / run-primitive with both operands non-plane is refused there.
    if ae32.is_some() {
        let rows = fx_exec::ops_census::ops_rows(&cp);
        if let Some(m) = fx_exec::ops_census::scalar_scalar_tripwire(&prog, &rows) {
            eprintln!("{}", m);
            let _ = std::fs::write(format!("{}.refusal.txt", out), format!("{}\n", m));
            return 2;
        }
        if let Some(r) = ae32.as_mut() {
            for x in &rows {
                r.labels.push(fx_exec::ops_census::census_line(x));
            }
        }
    }
    {
        let ids = fx_eval::compile::ifneq_arg1_nonconst_take();
        if ae32.is_some() && !ids.is_empty() {
            let mut d = ids.clone();
            d.dedup();
            eprintln!("IFNEQ-CONST-ARG1-NONCONST ({}: node {})", prog, d.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", node "));
            return 2;
        }
        if let Some(r) = ae32.as_mut() {
            r.labels.push("label IFNEQ-CONST-FIELDS-MEASURED ([this] = argument 0's constant flag, 1 at c1 to c5 and 0 at c6 with argument 0 = x; [this] and [this] = the lane counts of arguments 0 and 1 for constant operands (1 and 3 at c1, 3 and 3 at c2, 4 and 4 at c5) and 4 for a non-constant operand (c6); [this] = argument 1's constant flag, measured set at c1 to c6; writers (flag) and (count) for a constant operand, for a non-constant one, each storing through the operand's [op] and [op], the cells reads; the original's arm equals the patched native's frozen arm at c1 to c6 and differs from the unpatched native's at c1; probe vehicle Blend (the routine is shared by every program); a non-constant argument 1 is refused natively (IFNEQ-CONST-ARG1-NONCONST) until EQC1's c7 probe; EQC1, W480, W487)".to_string());
        }
    }
    // INT25 (W503 (b), INT21-A2): every if_neq_const decision of this compile as a census label on the claim route (observe only)
    {
        let tr = fx_eval::compile::ifneq_trace_take();
        if let Some(r) = ae32.as_mut() {
            for x in &tr {
                r.labels.push(format!("diagnostic IFNEQ-CONST-ARM\tnode {}\tflags {} {}\tcounts {} {}\tarm {}\tpath {}", x.0, x.1 as u8, x.2 as u8, x.3, x.4, x.5.map(|a| a.to_string()).unwrap_or_else(|| "-".into()), fx_exec::ops_census::node_path_fold(&cp, &g, x.0)));
            }
        }
    }
    // INT18 (W422 (c)): the lane-count scan of this compile, printed on the claim route (observe only)
    {
        let (calls, diff) = fx_eval::compile::eq_const_scan_take();
        if let Some(r) = ae32.as_mut() {
            r.labels.push(format!("label EQ-CONST-SCAN ({}: {} eq_const calls with two constant operands, {} with differing operand counts; W422 (c), INT18)", prog, calls, diff));
        }
    }
    // WPH: the fed per-run inputs, printed in the meta
    if let Some(r) = ae32.as_mut() {
        if ae.persp_base.is_some() || ae.palpha_base.is_some() || ae.palpha_cells.is_some() {
            r.labels.push(format!("label WPH-HOST-INPUTS (persp_base {:?}, palpha_base {:?}, palpha_cells {:?}, xy_base {:?}; per-run host inputs ALIGN-HOST-PER-RUN and [[rec]]; W634)", ae.persp_base, ae.palpha_base, ae.palpha_cells, ae.xy_base));
        }
    }
    // INT19 S1: the XY-HEAD-MODE label on the claim route
    if let Some(r) = ae32.as_mut() {
        if let Some(l) = &xy_list_lab {
            r.labels.push(l.clone());
        }
        r.labels.push(format!("label XY-HEAD-MODE ({}: {:?}; OneHead for programs whose plan writes one x row buffer, PerRow for programs whose plan allocates an x plane indexed by row: MEASURED by XYB1 part 2 hooks; WPE3, XYB1; derived from the plan's perspective primitives, INT19)", prog, b.kp.xy_head_mode));
    }
    // INT18 (W301, PROGRAM-FLAG-READ): [prog] bit 4 derived from the class table (report only on this tree; no compile input reads it)
    if let Some(r) = ae32.as_mut() {
        r.labels.push(format!("label PROGRAM-FLAG-READ ({}: [prog] bit 4 = {} derived from the class flag word 0x40000 of the program's function nodes; motion_blur is the only class with it; W301, INT18)", prog, fx_eval::layout::program_flag_bit4(&g) as u8));
    }
    // INT13 (Batch 49 rule 9): on the claim route ('ae32') a plan constant with count below 1 is refused by name (CONST-COUNT-OFF-CONTRACT);
    // the count -1 binding of exec.rs const_val (CONST-COUNT-NEGATIVE-OFF-CLAIM) is for the survey only.
    {
        let below: Vec<(usize, i32)> = cp
            .nodes
            .iter()
            .filter(|nd| nd.kind == "const" || ((nd.kind == "func" || nd.kind == "auto") && (nd.b160 || nd.folded)))
            .filter(|nd| nd.elnum < 1)
            .map(|nd| (nd.idx, nd.elnum))
            .collect();
        if ae32.is_some() && !below.is_empty() {
            eprintln!("CONST-COUNT-OFF-CONTRACT: the claim route refuses plan constants with count below 1: {:?} (Batch 49 rule 9)", below);
            return 2;
        }
    }
    if seed_missing && cp.nodes.iter().any(|nd| nd.name.contains("static2")) {
        eprintln!("the plan runs static2: --host-frame (with the count W x H) is required; LEGACY-SEED-W (--legacy-seed-w) is the named option for the W172 state A seed [ctx] = W");
        return 2;
    }
    let mut images = BTreeMap::new();
    for nd in &cp.nodes {
        if nd.kind == "input" {
            let ch = if nd.elnum > 0 { nd.elnum as usize } else { 4 };
            if let Some(r) = &ae32 {
                if let Some(k) = r.names.iter().position(|x| *x == nd.name) {
                    // the 'ae32' source reader fills this input (execute_ae32); its plane count is the record's
                    // SVB1: a class-3 input (elnum 3) on the variant-B 4-plane head is read as 4 planes and handed over as 3 lanes
                    let vb4_h = r.route.sources[k].reader_head == fx_exec::hostconv14::VB4_HEAD && r.route.sources[k].planes == 4 && ch == 3;
                    if r.route.sources[k].planes != ch && !vb4_h {
                        eprintln!("'ae32' source {} has {} planes recorded, the plan's input node has {}", nd.name, r.route.sources[k].planes, ch);
                        return 1;
                    }
                    continue;
                }
                if synth.is_some() {
                    eprintln!("--synth is refused on the 'ae32' route: every input is given");
                    return 2;
                }
            }
            let im = if let Some((_, p)) = ins.iter().find(|(nm, _)| *nm == nd.name) {
                let (pp, cc) = split(p, ch);
                read_f32(&pp, cc, n)
            } else if let Some(sd) = synth {
                Ok(synth_img(ch, n, sd + nd.name.len() as u64))
            } else {
                Err(format!("input {} not given (use --in {}=path or --synth SEED)", nd.name, nd.name))
            };
            match im {
                Ok(im) => {
                    images.insert(nd.name.clone(), im);
                }
                Err(e) => {
                    eprintln!("{}", e);
                    return 1;
                }
            }
        }
    }
    if let Some(p) = &xy {
        match read_f32(p, 2, n) {
            Ok(im) => {
                images.insert("xy".into(), im);
            }
            Err(e) => {
                eprintln!("{}", e);
                return 1;
            }
        }
    }
    let mut host = HostFrames::default();
    host.frame_round_old = round_old;
    host.tw_route = tw_route;
    if let Err(m) = frame_content_check(&prog, ae32.is_some(), frame_reqs.len()) {
        eprintln!("{}", m);
        return 2;
    }
    for (k, p) in &frame_words {
        let kv: Vec<&str> = k.split(',').collect();
        let key = match (kv.first().and_then(|x| x.parse::<i32>().ok()), kv.get(1).and_then(|x| x.parse::<f32>().ok())) {
            (Some(fr), Some(st)) if kv.len() == 2 => (fr, st.to_bits()),
            _ => {
                eprintln!("--frame-words {}: expected FRAME,STORED=path", k);
                return 2;
            }
        };
        let conv = match ae32.as_ref().and_then(|r| r.route.sources.first()) {
            Some(src0) => read_words(p).map_err(|e| e.to_string()).and_then(|words| {
                let fs = fx_exec::hostconv14::Ae32Source { words, reader_head: src0.reader_head, planes: src0.planes };
                if option_env!("FH_MUT") == Some("1") {
                    // mutant "raw frames": the words are taken as straight colour planes (never in a shipped build)
                    let n = fs.words.len() / 4;
                    Ok(Image { n, planes: (1..=fs.planes).map(|c| fs.words.iter().skip(c).step_by(4).map(|x| f32::from_bits(*x)).collect()).collect() })
                } else {
                    fx_exec::hostconv14::ae32_to_image(&fs, w, h).map_err(|e| format!("{:?}", e))
                }
            }),
            None => Err("--frame-words needs the 'ae32' route".to_string()),
        };
        match conv {
            Ok(im) => {
                host.by_request.insert(key, im);
            }
            Err(e) => {
                eprintln!("{}", e);
                return 1;
            }
        }
    }
    for (k, p) in &frame_reqs {
        let kv: Vec<&str> = k.split(',').collect();
        let key = match (kv.first().and_then(|x| x.parse::<i32>().ok()), kv.get(1).and_then(|x| x.parse::<f32>().ok())) {
            (Some(fr), Some(st)) if kv.len() == 2 => (fr, st.to_bits()),
            _ => {
                eprintln!("--frame-req {}: expected FRAME,STORED=path", k);
                return 2;
            }
        };
        let (pp, cc) = split(p, 4);
        match read_f32(&pp, cc, n) {
            Ok(im) => {
                host.by_request.insert(key, im);
            }
            Err(e) => {
                eprintln!("{}", e);
                return 1;
            }
        }
    }
    for (off, p) in &frames {
        let (pp, cc) = split(p, 4);
        match read_f32(&pp, cc, n) {
            Ok(im) => {
                host.frames.insert(off.parse().unwrap_or(0), im);
            }
            Err(e) => {
                eprintln!("{}", e);
                return 1;
            }
        }
    }
    if let Some(p) = &prev {
        let (pp, cc) = split(p, 4);
        match read_f32(&pp, cc, n) {
            Ok(im) => host.prev_dest = Some(im),
            Err(e) => {
                eprintln!("{}", e);
                return 1;
            }
        }
    }
    // INT1: native convert-from-source on the Source input (planes are re-interleaved to host memory B,G,R,A first)
    if convert.is_some() {
        let bit_of = |nm: &str| g.inputs.iter().position(|i| i.name == nm).map(|k| k < 32 && (bmask >> k) & 1 == 1).unwrap_or(false);
        let mut names: Vec<&str> = ["Source", "Foreground", "Matte", "Mask"].into_iter().filter(|x| *x != "Mask" || bit_of("Mask")).collect();
        for a in &alias_bound {
            if !names.contains(&a.as_str()) {
                names.push(a.as_str());
            }
        }
        for nm in names {
        if let Some(src) = images.get(nm).cloned() {
            if src.channels() == 4 {
                let mut mem = vec![0f32; 4 * n];
                for k in 0..n {
                    mem[4 * k] = src.planes[2][k];
                    mem[4 * k + 1] = src.planes[1][k];
                    mem[4 * k + 2] = src.planes[0][k];
                    mem[4 * k + 3] = src.planes[3][k];
                }
                match fx_exec::host_to_image(&mem, w, h) {
                    Ok(im) => {
                        images.insert(nm.into(), im);
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                        return 1;
                    }
                }
            }
        }
        }
    }
    if let Some(r) = &ae32 {
        // INT27 step SI (SIP1, W552 (b), W561): a plan without Source that is one folded constant node renders through the destination
        // convert only at Source class 3 with 3 planes; every other missing input keeps the refusal by name (fx_exec::srcindep).
        let si_names: Vec<(String, usize)> = r.names.iter().cloned().zip(r.route.sources.iter().map(|s| s.planes)).collect();
        let si_style = if prog.starts_with("Hue_Sat_Bright") { "S_HueSatBright".to_string() } else { prog.to_string() };
        let si_tag = std::path::Path::new(&out).file_name().map(|x| x.to_string_lossy().to_string()).unwrap_or_default();
        let si = match fx_exec::srcindep::route_check(&cp, &si_names, b.input_elnum.get("Source").copied(), &si_style, &si_tag) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("{}", e);
                return 1;
            }
        };
        let si_label = if si { fx_exec::srcindep::single_const3(&cp).map(|l| fx_exec::srcindep::label(&si_style, &si_tag, &l)) } else { None };
        let res = if si { fx_exec::execute_ae32_const(&cp, &b, &r.route, &images, w, h, &host) } else { fx_exec::execute_ae32(&cp, &b, &r.route, &r.names, &images, w, h, &host) };
        return match res {
            Ok(o) => {
                let mut meta = format!("program {}\nsize {}x{}\nchannels {}\nroute ae32\n", prog, w, h, o.exec.dest.channels());
                if let Some(l) = &si_label {
                    meta.push_str(l);
                    meta.push('\n');
                }
                if let Some(p) = ds_phase_opt {
                    meta.push_str(&format!("ds-phase {}\n", p));
                }
                for l in &r.labels {
                    meta.push_str(l);
                    meta.push('\n');
                }
                for rr in &o.exec.trace {
                    meta.push_str(&format!("run node {} {} operands {:?} tag {} out_channels {}\n", rr.node, rr.class, rr.operands, rr.tag, rr.out_channels));
                }
                for d in &o.exec.diagnostics {
                    meta.push_str(&format!("diagnostic {}\n", d));
                }
                for d in cp.diagnostics.iter().filter(|d| d.starts_with("HCV-CHECKOUT\t")) {
                    meta.push_str(&format!("diagnostic {}\n", d));
                }
                let wb: Vec<u8> = o.dest_words.iter().flat_map(|x| x.to_le_bytes()).collect();
                if let Err(e) = std::fs::write(&r.out, wb) {
                    eprintln!("cannot write {}: {}", r.out, e);
                    return 1;
                }
                let _ = write_f32(&format!("{}.dest.f32", out), &o.exec.dest);
                for (k, sp) in o.source_planes.iter().enumerate() {
                    let _ = write_f32(&format!("{}.src{}.f32", out, k), sp);
                }
                let _ = std::fs::write(format!("{}.dest.meta.txt", out), meta);
                if let Some(p) = &o.exec.saved_prev_dest {
                    let _ = write_f32(&format!("{}.prev.f32", out), p);
                }
                0
            }
            Err(e) => {
                eprintln!("ERR {}", e);
                1
            }
        };
    }
    let inputs = Inputs { width: w, height: h, images };
    match fx_exec::execute_compiled(&cp, &b, &inputs, &host) {
        Ok(mut o) => {
            if let Some(mb) = convert {
                let mode = match convert_pair::AlphaMode::from_byte(mb) {
                    Some(m) => m,
                    None => {
                        eprintln!("bad alpha mode {}", mb);
                        return 2;
                    }
                };
                let prefill = vec![f32::from_bits(0xaaaa_aaaa); 4 * n];
                let (dest_in, mode) = match fx_exec::dest_for_host(&o.dest, mode) {
                    Ok(x) => x,
                    Err(e) => {
                        eprintln!("{}", e);
                        return 1;
                    }
                };
                match fx_exec::image_to_host(&dest_in, w, h, mode, &prefill) {
                    Ok((hd, _)) => {
                        let mut pl = vec![vec![0f32; n]; 4];
                        for k in 0..n {
                            pl[0][k] = hd[4 * k + 2];
                            pl[1][k] = hd[4 * k + 1];
                            pl[2][k] = hd[4 * k];
                            pl[3][k] = hd[4 * k + 3];
                        }
                        o.dest = Image { n, planes: pl };
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                        return 1;
                    }
                }
            }
            let mut meta = format!("program {}\nsize {}x{}\nchannels {}\n", prog, w, h, o.dest.channels());
            if let Some(p) = ds_phase_opt {
                meta.push_str(&format!("ds-phase {}\n", p));
            }
            if let Some(l) = &style_label {
                meta.push_str(l);
                meta.push('\n');
            }
            for r in &o.trace {
                meta.push_str(&format!("run node {} {} operands {:?} tag {} out_channels {}\n", r.node, r.class, r.operands, r.tag, r.out_channels));
            }
            for d in &o.diagnostics {
                meta.push_str(&format!("diagnostic {}\n", d));
            }
            // W490: the checkout sequence of the wired get_curve_val (compile diagnostics HCV-CHECKOUT), in call order
            for d in cp.diagnostics.iter().filter(|d| d.starts_with("HCV-CHECKOUT\t")) {
                meta.push_str(&format!("diagnostic {}\n", d));
            }
            let _ = write_f32(&format!("{}.dest.f32", out), &o.dest);
            let _ = std::fs::write(format!("{}.dest.meta.txt", out), meta);
            if let Some(p) = &o.saved_prev_dest {
                let _ = write_f32(&format!("{}.prev.f32", out), p);
            }
            0
        }
        Err(e) => {
            eprintln!("ERR {}", e);
            1
        }
    }
}

/// INT11 and ENVFENCE: environment variables any binary of the tree reads (grep of `env::var` over fx_exec, fx_eval and crates); the 'ae32' route
/// refuses to run while any of them is set (no environment switch on the claimed route).
const ENV_SWITCHES: [&str; 40] = [
    "DSW5_CLONE", "DSW5_CLONE_SCOPE", "NV10_BW_WHOLE", "DS1_OFF", "NV9_BLUR_SCALE_OLD", "CV_MUT", "COV_MUT", "DS2_PHASE", "DS_MUT", "FX_SETS_ORIG_NUMBER",
    "LG_TRACE", "NV3B_HC", "NV3D_MODE", "TW2_LOGFRAMES", "DS2_XY", "FX1_COERCE_FOLD_OLD", "FX1_COERCE_EXTRA_OFF", "SHE4_OFF", "DSW3_TRACE", "WPG1_PERSP_WHOLE",
    "INT1_CONVERT", "FX1_TRACE", "CV_MUTANT",
    // ENVFENCE (W618): every other runtime environment read of code linked into fx_run (fixed names, then the NV3E_<RULE> family).
    "WPG1_PBASE", "WPG1_ABASE", "WPG1_S8", "WPG1_S48", "WPG1_S30", "WPG12_OLDFMT", "DSW3_UNBOUND_OFF", "FLD1_NO_COPIES", "FLD1_NO_INTTYPE",
    "NEG2_RANGES_OFF", "LYM2_REPORT", "NV3E_DONOR", "NV3E_DS", "NV3E_FLAG", "NV3E_INPLACE", "NV3E_LASTC", "NV3E_SLOT",
];
/// Options of the state A routes that the 'ae32' route refuses (each would replace a recorded input by a harness value).
const OTHER_ROUTE: [&str; 17] = [
    "--convert", "--convert-derive", "--convert-standin", "--synth", "--blur-aspect-ctx", "--legacy-seed-w", "--host-count", "--bind-supplied", "--geom-old",
    "--hc", "--hc-zero-legacy", "--hc-keyed", "--ranges-off", "--mask-image", "--frame-round-old", "--aspect", "--mask-byte",
];
/// Options the 'ae32' route requires (the context cells, the seed frame, the row geometry, the compile cell, the range set).
const REQUIRED: [&str; 20] = [
    "--ae32-src", "--src-planes", "--owner-arm", "--owner-opacity", fx_exec::judge_keys::OPT_SRC_ROW_GATE, "--src-head", "--aspect-triple", "--ae32-dest", "--ae32-prior", "--ae32-copier", "--ae32-out",
    "--units-ratio", "--dest-stack-word", "--do-postdiv", "--bound-mask", "--ctx-time", "--ctx-word2", fx_exec::judge_keys::OPT_CTX_OWNER_MODE, "--ctx-has-data", "--geom",
];

#[derive(Default)]
struct Ae32Args {
    srcs: Vec<(String, String)>,
    planes: Option<usize>,
    gate: Option<(bool, bool, bool)>,
    /// INT12 (Batch 47 rule 4): the checked-out record value of the kind-4 "opacity" / "input_opacity" parameter (f32 bits), or none
    opacity: Option<Option<u32>>,
    /// LYM1: record value of the layout `output_opacity` parameter (Time_Warp_RGB), f32 bits; None = the program has none
    out_opacity: Option<u32>,
    /// LYM3: the owner arm's opacity was derived from the merged record (no --owner-opacity given)
    opacity_derived: bool,
    /// INT31 step L2 (W595): per derived extra (name, the checkout integer v, None for the bound answer or Some((display name, record, style)) for the registered default)
    opacity_notes: Vec<(String, i32, Option<(String, String, String)>)>,
    row_gate_cell: Option<u32>,
    head: Option<Option<u64>>,
    triple: Option<(i64, u32, u32)>,
    dest: Option<(usize, i32)>,
    prior: Option<String>,
    copier: Option<Option<usize>>,
    out: Option<String>,
    host_params: Vec<(String, f32)>,
    /// NSS1 (W726 (d)): the lane-1 host value of a two-lane normalize_scale parameter given as --host-param NAME=X,Y
    host_lane1: Vec<(String, f32)>,
    ratio: Option<(f64, f64)>,
    /// WPH (W634 (b))
    persp_base: Option<i32>,
    palpha_base: Option<i32>,
    xy_base: Option<i32>,
    palpha_cells: Option<(i32, i32, f64)>,
    /// INT17 (PDV3 PATCH 1): the later pre-render stage's index [prog] and its arrays (rect flags, checkout bytes) on the claim host model
    /// INT20 step G (W450 (b) 1): [prog], [prog], * from the style's input-class record; the recorded class bytes (per-run check)
    /// INT20 step G3: the class-3 record (plane-count builder hits, allocate command images and elnum, [prog])
    class3_rec: Option<(u32, u32, u32, u8)>,
    class_cells: Option<(Option<u8>, Option<u8>, Option<u8>)>,
    class_record: Vec<(String, u8)>,
    late: Option<(i8, Vec<u8>, Vec<u32>)>,
    /// INT17: [ctx] and [[ctx]], read by only when the mode byte is 2
    modein: Option<(u32, u32)>,
    /// M3P (W496 (d)): the host datum, 1 = the output world is the input world's buffer (in place); None = 0
    host_inplace: Option<u32>,
    dest_sw: Option<u32>,
    /// INT26 step OP: the host's Opacity checkout integer (--cset Opacity=V), converted by OPC1's port
    host_opacity: Option<i32>,
    host_opacity_param: Option<String>,
    host_out_opacity: Option<i32>,
    csets: Vec<(String, i32)>,
    /// INT26 step OP: the recorded [prog] (kind-4 arm selector); absent means the integer arm (the per-style cell of W558 (c))
    arm_cell_a: Option<u32>,
}

struct Ae32Ready {
    route: fx_exec::hostconv14::Ae32Route,
    names: Vec<String>,
    out: String,
    labels: Vec<String>,
    /// INT13: the owner arm inputs the route used (checked against the program definition after the build)
    arm: fx_exec::hostconv14::OwnerArm,
    /// INT17: the later stage's inputs and the mode-byte inputs (PDV3 PATCH 1)
    class3_rec: Option<(u32, u32, u32, u8)>,
    rec_head: Option<u64>,
    class_cells: Option<(Option<u8>, Option<u8>, Option<u8>)>,
    class_record: Vec<(String, u8)>,
    late: Option<(i8, Vec<u8>, Vec<u32>)>,
    modein: Option<(u32, u32)>,
    host_inplace: Option<u32>,
    wh: (usize, usize),
    /// INT18 (Batch 52 rule 11, W434 (b) 2): the --host-param values as given and the units width, for the checkout rule after the build
    host_params: Vec<(String, f32)>,
    uw: f32,
}

fn hex32(s: &str) -> Result<u32, String> {
    u32::from_str_radix(s.trim().trim_start_matches("0x"), 16).map_err(|_| format!("not a hex word: {}", s))
}

fn read_words(path: &str) -> Result<Vec<u32>, String> {
    let b = std::fs::read(path).map_err(|e| format!("cannot read {}: {}", path, e))?;
    if b.len() % 4 != 0 {
        return Err(format!("{}: {} bytes is not a whole number of u32 words", path, b.len()));
    }
    Ok(b.chunks_exact(4).map(|x| u32::from_le_bytes([x[0], x[1], x[2], x[3]])).collect())
}

impl Ae32Args {
    fn active(&self) -> bool {
        !self.srcs.is_empty() || self.planes.is_some() || self.gate.is_some() || self.opacity.is_some() || self.row_gate_cell.is_some() || self.head.is_some() || self.triple.is_some() || self.dest.is_some()
            || self.prior.is_some() || self.copier.is_some() || self.out.is_some() || !self.host_params.is_empty() || self.ratio.is_some() || self.dest_sw.is_some()
            || self.host_opacity.is_some() || self.host_inplace.is_some() || self.persp_base.is_some() || self.palpha_base.is_some() || self.palpha_cells.is_some() || self.xy_base.is_some()
    }

    fn take(&mut self, k: &str, v: &str) -> Result<(), String> {
        let once = |set: bool| if set { Err(format!("{} given twice", k)) } else { Ok(()) };
        match k {
            "--prerender-late" => {
                once(self.late.is_some())?;
                let p: Vec<&str> = v.split(',').collect();
                if p.len() != 3 {
                    return Err("--prerender-late takes LATE_INPUTS_A,R0.R1..,A0.A1.. ([prog] signed, the rectangle flags, the checkout bytes)".into());
                }
                let k: i8 = p[0].parse().map_err(|_| "--prerender-late LATE_INPUTS_A (signed byte)")?;
                let r: Result<Vec<u8>, _> = p[1].split('.').map(|x| x.parse::<u8>()).collect();
                let c: Result<Vec<u32>, _> = p[2].split('.').map(|x| x.parse::<u32>()).collect();
                self.late = Some((k, r.map_err(|_| "--prerender-late rectangle flags")?, c.map_err(|_| "--prerender-late checkout bytes")?));
            }
            "--class-cells" => {
                once(self.class_cells.is_some())?;
                let p: Vec<&str> = v.split(',').collect();
                if p.len() != 3 {
                    return Err("--class-cells takes FX_RUN_CELL_C,FX_RUN_CELL_D,F1B2F810 (hex bytes from the style's input-class record, '-' for a cell not in the record)".into());
                }
                let c = |x: &str| fx_exec::input_class::parse_cell(x);
                self.class_cells = Some((c(p[0])?, c(p[1])?, c(p[2])?));
            }
            "--class3-record" => {
                once(self.class3_rec.is_some())?;
                let p: Vec<&str> = v.split(',').collect();
                if p.len() != 4 {
                    return Err("--class3-record takes HITS,IMAGES,ELNUM,FX_RUN_CELL_E (plane-count builder hits, the recorded allocate command's image count and elnum, [prog] hex)".into());
                }
                let u = |x: &str| x.trim().parse::<u32>().map_err(|_| format!("--class3-record: not a count: {}", x));
                self.class3_rec = Some((u(p[0])?, u(p[1])?, u(p[2])?, u8::from_str_radix(p[3].trim().trim_start_matches("0x"), 16).map_err(|_| "--class3-record FX_RUN_CELL_E (hex byte)")?));
            }
            "--class-record" => {
                for it in v.split(',') {
                    let (n, k) = it.split_once('=').ok_or("--class-record takes NAME=CLASS[,NAME=CLASS] (the recorded class byte of each bound input)")?;
                    self.class_record.push((n.to_string(), k.parse().map_err(|_| "--class-record CLASS")?));
                }
            }
            "--host-inplace" => {
                once(self.host_inplace.is_some())?;
                self.host_inplace = Some(match v { "0" => 0, "1" => 1, _ => return Err("--host-inplace takes 0 or 1".into()) });
            }
            "--mode-inputs" => {
                once(self.modein.is_some())?;
                let (a, b) = v.split_once(',').ok_or("--mode-inputs takes CTX_MODE_CELL,DEST_DESC_MODE")?;
                self.modein = Some((a.parse().map_err(|_| "--mode-inputs CTX_MODE_CELL")?, b.parse().map_err(|_| "--mode-inputs DEST_DESC_MODE")?));
            }
            "--ae32-src" => {
                let (n, p) = v.split_once('=').ok_or("--ae32-src takes NAME=FILE (raw u32 words A,R,G,B per pixel)")?;
                self.srcs.push((n.to_string(), p.to_string()));
            }
            "--src-planes" => {
                once(self.planes.is_some())?;
                self.planes = Some(v.parse().map_err(|_| "--src-planes takes the recorded plane count")?);
            }
            "--owner-arm" => {
                // INT11 (Batch 46 rule 5): OWNER-ARM-RECORDED inputs: [owner] (decimal, non-zero or 0), bit 2 of [owner] (0 or 1),
                // the mode-byte-2 exit (0 or 1: the constructor defaults stand)
                once(self.gate.is_some())?;
                let p: Vec<&str> = v.split(',').collect();
                if p.len() != 3 {
                    return Err("--owner-arm takes OWNER13E0,BIT2,EXIT2 ([owner], bit 2 of [owner], the mode-byte-2 exit; decimal)".into());
                }
                let o: u64 = p[0].parse().map_err(|_| "--owner-arm OWNER13E0")?;
                let b2: u64 = p[1].parse().map_err(|_| "--owner-arm BIT2")?;
                let x2: u64 = p[2].parse().map_err(|_| "--owner-arm EXIT2")?;
                if b2 > 1 || x2 > 1 {
                    return Err("--owner-arm BIT2 and EXIT2 are 0 or 1".into());
                }
                self.gate = Some((o != 0, b2 == 1, x2 == 1));
            }
            "--owner-opacity" => {
                // INT12 (Batch 47 rule 4, W380): the Opacity loop's input, OWNER-ARM-RECORDED: the record value of the program's kind-4
                // parameter "opacity" or "input_opacity" (the popup answer minus 1 by POPUP-ONE-BASED), or "none" if the program registers none
                once(self.opacity.is_some())?;
                self.opacity = Some(if v == "none" {
                    None
                } else {
                    let x: f32 = v.parse().map_err(|_| "--owner-opacity takes none or the recorded value (f32)")?;
                    Some(x.to_bits())
                });
            }
            fx_exec::judge_keys::OPT_SRC_ROW_GATE => {
                once(self.row_gate_cell.is_some())?;
                self.row_gate_cell = Some(v.parse().map_err(|_| fx_exec::judge_keys::MSG_SRC_ROW_GATE)?);
            }
            "--src-head" => {
                once(self.head.is_some())?;
                self.head = Some(if v == "unrecorded" { None } else { Some(hex32(v)? as u64) });
            }
            "--aspect-triple" => {
                once(self.triple.is_some())?;
                let p: Vec<&str> = v.split(',').collect();
                if p.len() != 3 {
                    return Err("--aspect-triple takes FLAG,B_CELL_A,B_CELL_B (ctx decimal, ctx and ctx hex f32 bits)".into());
                }
                self.triple = Some((p[0].trim().parse().map_err(|_| "--aspect-triple FLAG")?, hex32(p[1])?, hex32(p[2])?));
            }
            "--ae32-dest" => {
                once(self.dest.is_some())?;
                let (a, m) = v.split_once(',').ok_or("--ae32-dest takes PLANES,MODE (the recorded destination plane count and r8d)")?;
                self.dest = Some((a.parse().map_err(|_| "--ae32-dest PLANES")?, m.parse().map_err(|_| "--ae32-dest MODE")?));
            }
            "--ae32-prior" => {
                once(self.prior.is_some())?;
                self.prior = Some(v.to_string());
            }
            "--ae32-copier" => {
                once(self.copier.is_some())?;
                self.copier = Some(if v == "none" { None } else { Some(v.parse().map_err(|_| "--ae32-copier takes none or the source index")?) });
            }
            "--ae32-out" => {
                once(self.out.is_some())?;
                self.out = Some(v.to_string());
            }
            "--host-param" => {
                let (n, x) = v.split_once('=').ok_or("--host-param takes NAME=HOST (a normalize_scale parameter's host value, lane 0) or NAME=X,Y (two lanes)")?;
                let (x, y) = match x.split_once(',') {
                    Some((x0, y0)) => (x0, Some(y0)),
                    None => (x, None),
                };
                if let Some(y) = y {
                    self.host_lane1.push((n.to_string(), y.parse().map_err(|_| format!("--host-param {}: lane 1 is not a number", n))?));
                }
                if self.host_params.iter().any(|(m, _)| m == n) {
                    return Err(format!("--host-param {} given twice", n));
                }
                self.host_params.push((n.to_string(), x.parse().map_err(|_| format!("--host-param {}: not a number", n))?));
            }
            "--cset" => {
                // INT26 step OP, step L2b (W616 (c), W547): the host answer by the checkout DISPLAY name; the name is resolved against the
                // program's own parameter-to-display-name table (STYLE_PARAM_INDEX.tsv) once the program is known (cset_param)
                let (n, x) = v.split_once('=').ok_or("--cset takes DISPLAYNAME=V (the host checkout integer)")?;
                if self.csets.iter().any(|(m, _)| m == n) {
                    return Err(format!("--cset {} given twice", n));
                }
                self.csets.push((n.to_string(), x.parse().map_err(|_| format!("--cset {} takes the host checkout integer", n))?));
            }
            fx_exec::judge_keys::OPT_OPACITY_ARM => {
                once(self.arm_cell_a.is_some())?;
                self.arm_cell_a = Some(v.parse().map_err(|_| fx_exec::judge_keys::MSG_OPACITY_ARM)?);
            }
            "--persp-base" | "--palpha-base" | "--xy-base" => {
                let n: i32 = v.parse().map_err(|_| format!("{} takes 0 to 31", k))?;
                if !(0..=31).contains(&n) {
                    return Err(format!("{} takes 0 to 31", k));
                }
                let slot = if k == "--persp-base" { &mut self.persp_base } else if k == "--xy-base" { &mut self.xy_base } else { &mut self.palpha_base };
                once(slot.is_some())?;
                *slot = Some(n);
            }
            "--palpha-cells" => {
                once(self.palpha_cells.is_some())?;
                let f: Vec<&str> = v.split(',').collect();
                if f.len() != 3 {
                    return Err("--palpha-cells takes S8,PORT_INFO_B,PORT_INFO_A ([[rec]] i32, i32, f64)".into());
                }
                self.palpha_cells = Some((
                    f[0].parse().map_err(|_| "--palpha-cells S8 (i32)")?,
                    f[1].parse().map_err(|_| "--palpha-cells PORT_INFO_B (i32)")?,
                    f[2].parse().map_err(|_| "--palpha-cells PORT_INFO_A (f64)")?,
                ));
            }
            "--units-ratio" => {
                once(self.ratio.is_some())?;
                let (a, c) = v.split_once(',').ok_or("--units-ratio takes FX_RUN_CELL_B,FX_RUN_CELL_A ([[ctx]], [[ctx]])")?;
                self.ratio = Some((a.parse().map_err(|_| "--units-ratio FX_RUN_CELL_B")?, c.parse().map_err(|_| "--units-ratio FX_RUN_CELL_A")?));
            }
            "--dest-stack-word" => {
                once(self.dest_sw.is_some())?;
                self.dest_sw = Some(v.parse().map_err(|_| "--dest-stack-word takes the recorded stack word (decimal)")?);
            }
            _ => return Err(format!("not an 'ae32' option: {}", k)),
        }
        Ok(())
    }

    fn ready(&self, prog: &str, w: usize, h: usize, given: &[(&str, &str)], b: &mut Binding, frame_given: bool) -> Result<Ae32Ready, String> {
        for (k, e) in ENV_SWITCHES.iter().enumerate() {
            // G2 mutants (each must fail t_envfence): the W630 (h) 5 drop-name mutant, the added tail dropped, the NV3E family dropped
            if cfg!(feature = "envfence_dropname_mut") && *e == "WPG1_PBASE" {
                continue;
            }
            if cfg!(feature = "envfence_tail_mut") && k >= 23 {
                continue;
            }
            if cfg!(feature = "envfence_nv3e_mut") && e.starts_with("NV3E_") {
                continue;
            }
            if std::env::var_os(e).is_some() {
                return Err(format!("ENV-SWITCH-REFUSED: {} is set; the 'ae32' route runs with no environment switch", e));
            }
        }
        for (o, _) in given {
            if OTHER_ROUTE.contains(o) {
                return Err(format!("{} is a state A option, refused on the 'ae32' route (every input comes from the record)", o));
            }
        }
        for r in REQUIRED {
            if r == "--owner-opacity" && self.host_opacity.is_some() {
                continue; // INT26 step OP: the host answer gives the loop value
            }
            if r == "--owner-opacity" && self.opacity.is_some() {
                continue; // LYM3 OWNER-OPACITY-DERIVED (step L2): taken from the merged record
            }
            if !given.iter().any(|(o, _)| *o == r) {
                return Err(format!("the 'ae32' route requires {} (no default)", r));
            }
        }
        if !frame_given {
            return Err("the 'ae32' route requires --host-frame (ctx; the count is W x H)".into());
        }
        match given.iter().find(|(o, _)| *o == "--ranges") {
            Some((_, "REAL")) => {}
            _ => return Err("the 'ae32' route requires --ranges REAL (the range set is stated, never defaulted)".into()),
        }
        let mut labels = Vec::new();
        if let Some(v) = self.host_opacity {
            labels.push(format!("label {} (host checkout integer {}, loop value {})", fx_exec::opacity_conv::OPACITY_CONVERT_PORTED, v, f32::from_bits(self.opacity.unwrap().unwrap())));
        }
        // the aspect triple (Batch 45 rule 1): blur kernel aspect Some iff the flag is set; warp_rel's aspect_block_d is the geometry's aspect field
        let (fl, b_cell_a, b_cell_b) = self.triple.unwrap();
        let claim = fl == 0 && b_cell_a == 0x3f80_0000 && b_cell_b == 0x3f80_0000;
        b.kp.blur_aspect = if fl != 0 { Some(f32::from_bits(b_cell_b)) } else { None };
        b.kp.host_aspect = f32::from_bits(b_cell_a);
        match &b.kp.geom {
            Some(g) if g.aspect.to_bits() == b_cell_b => {}
            Some(g) => return Err(format!("ASPECT-INCONSISTENT: the geometry's aspect field 0x{:08x} differs from the triple's ctx 0x{:08x}", g.aspect.to_bits(), b_cell_b)),
            None => return Err("the 'ae32' route requires --geom".into()),
        }
        labels.push(if claim {
            "label HOST-ASPECT-SQUARE (triple 0, 1.0, 1.0; blur aspect None; warp_rel aspect_block_d 1.0)".to_string()
        } else {
            format!("label ASPECT-OFF-CLAIM (attribution; triple {}, 0x{:08x}, 0x{:08x})", fl, b_cell_a, b_cell_b)
        });
        // POSTDIV-DERIVED (Batch 46 rule 5): do_postdiv = ctx by the owner arm; the recorded cell (--do-postdiv) is a check
        let (late_out_b, bit2, exit2) = self.gate.unwrap();
        let arm = fx_exec::hostconv14::OwnerArm { owner_arm_nonzero: late_out_b, owner_arm_bit2: bit2, mode_byte_exit: exit2, opacity_bits: self.opacity.unwrap(), output_opacity_bits: self.out_opacity };
        let cells = fx_exec::hostconv14::try_owner_arm(&arm).map_err(|e| format!("{:?}", e))?;
        let dpd = cells.ctx_postdiv_cell != 0;
        match b.do_postdiv {
            Some(d) if d == dpd => labels.push(format!(
                "label POSTDIV-DERIVED ({}: do_postdiv {} = ctx by the owner arm {:?}, cells {:?}; the recorded cell {} checked equal; OWNER-ARM-RECORDED)",
                prog, dpd as u8, arm, cells, d as u8
            )),
            Some(d) => return Err(format!("POSTDIV-MISMATCH: the owner arm {:?} derives ctx = {}, the recorded cell is {}", arm, cells.ctx_postdiv_cell, d as u8)),
            None => return Err("the 'ae32' route requires --do-postdiv 0|1 (the recorded cell ctx, a check)".into()),
        }
        b.do_postdiv = Some(dpd);
        // PARAM-UNITS (Batch 44 rule 3): every normalize_scale parameter of the program is bound by its host value (converted here, lane 0,
        // ns 2 or 4) or by --set as its record value (PARAM-BY-RECORD); never both, never neither
        let (fx_run_cell_b, fx_run_cell_a) = self.ratio.unwrap();
        let uw = fx_eval::units::units_width(w as i32, fx_run_cell_b, fx_run_cell_a);
        let nsrows: Vec<(String, f32)> = fx_eval::units::ns_rows_of(&prog);
        for (n, x) in &self.host_params {
            if let Some(lane) = fx_eval::units::angle_host_lane(prog, n, *x) {
                // WSR1 (W729 (b) 2): host angle -> record lane = -host (MEASURED in the S_WarpPerspective records)
                b.overrides.insert(n.clone(), vec![lane]);
                labels.push(format!("param {} host 0x{:08x} as bound angle lane 0x{:08x} (negated host value) ANGLE-LANE-NEGATED (WSR1; W729)", n, x.to_bits(), lane.to_bits()));
                continue;
            }
            fx_eval::ranges::host_value_index_check(prog, n)?; // INT28 step PI (W563 (b))
            let ns = fx_eval::units::ns_of(prog, n).ok_or_else(|| format!("PARAM-UNITS: {} is not a normalize_scale parameter of {}; bind it with --set", n, prog))?;
            let lane1 = self.host_lane1.iter().find(|(m, _)| m == n).map(|(_, y)| *y);
            if let Some(y) = lane1 {
                // NSS1 (W726 (d)): a two-lane ns 1 parameter (Drop_Shadow shift): lane 0 = x * ((2 / W) * 1), lane 1 = -(y * ((2 / W) * 1)); the
                // original's lane 1 is lane 0 x (conv_in_b x A / -B) with conv_in_b = A = B = 1 on the claim state (UNT2 conv_record)
                if ns != 1.0 {
                    return Err(format!("PARAM-UNITS-NOT-JUDGED: {} has normalize_scale {} (two host lanes are judged for ns 1 only)", n, ns));
                }
                if b.overrides.contains_key(n) {
                    return Err(format!("{} given both as --host-param and --set", n));
                }
                for hv in [*x, y] {
                    if let Some(k) = fx_eval::units::host_value_off_grid(prog, n, hv) {
                        return Err(format!("HOST-VALUE-OFF-GRID ({} {}: --host-param {} is {} / 65536 with k not an integer; W663)", prog, n, hv, k));
                    }
                }
                let uw2 = if cfg!(feature = "nss1_w64_mut") { 64.0f32 } else { uw };
                let r0 = fx_eval::units::normalize_scale_record(*x, uw2, ns, false);
                let r1 = -fx_eval::units::normalize_scale_record(y, uw2, ns, false);
                b.overrides.insert(n.clone(), vec![r0, r1]);
                labels.push(format!("param {} host 0x{:08x},0x{:08x} as bound ns {} width {} compiled {:.9},{:.9} (0x{:08x},0x{:08x}) PARAM-UNITS-TWO-LANE (NSS1)", n, x.to_bits(), y.to_bits(), ns, uw, r0, r1, r0.to_bits(), r1.to_bits()));
                continue;
            }
            if ns != 2.0 && ns != 4.0 {
                return Err(format!("PARAM-UNITS-NOT-JUDGED: {} has normalize_scale {} (the label covers 2 and 4, lane 0)", n, ns));
            }
            if let Some(k) = fx_eval::units::host_value_off_grid(prog, n, *x) {
                return Err(format!("HOST-VALUE-OFF-GRID ({} {}: --host-param {} is {} / 65536 with k not an integer; a host type 2 slider's checkout is a 16.16 value; W663)", prog, n, x, k));
            }
            if b.overrides.contains_key(n) {
                return Err(format!("{} given both as --host-param and --set", n));
            }
            // Batch 46 rule 1: the host value as the runner bound it (never re-snapped)
            let rec = fx_eval::units::normalize_scale_record(*x, uw, ns, false);
            b.overrides.insert(n.clone(), vec![rec]);
            labels.push(format!("param {} host 0x{:08x} as bound ns {} width {} compiled {:.9} (0x{:08x}) PARAM-UNITS", n, x.to_bits(), ns, uw, rec, rec.to_bits()));
        }
        for (n, _) in &nsrows {
            if self.host_params.iter().any(|(m, _)| m == n) {
                continue;
            }
            match b.overrides.get(n) {
                Some(v) => labels.push(param_by_record_label(n, v)),
                None => {
                    // SHJ5 SDS widened (W663 (e) 1): an unbound NS host type 2 slider takes its registered bits / 65536 before the width scaling
                    if let Some(h) = fx_eval::units::unbound_ns_host(prog, n) {
                        let ns = fx_eval::units::ns_of(prog, n).unwrap_or(4.0);
                        let rec = fx_eval::units::normalize_scale_record(h, uw, ns, false);
                        b.overrides.insert(n.clone(), vec![rec]);
                        labels.push(format!("label PARAM-DEFAULT-REGISTERED-16-16 ({} {}: unbound normalize_scale host type 2 slider; host {} / 65536 = {}, before the width scaling; record 0x{:08x}; W605, W663)", prog, n, (h as f64 * 65536.0) as i64, h as f64, rec.to_bits()));
                    } else {
                        return Err(format!("PARAM-UNITS: the normalize_scale parameter {} needs --host-param {}=HOST or --set {}=RECORD (no default)", n, n, n));
                    }
                }
            }
        }
        // sources: the derived gate gives the reader head; the recorded head is a check (SOURCE-VARIANT-MISMATCH)
        let g = fx_exec::hostconv14::SourceGate::from_arm(&arm, self.row_gate_cell.unwrap());
        let planes = self.planes.unwrap();
        let mut sources = Vec::new();
        let mut names = Vec::new();
        for (nm, p) in &self.srcs {
            if names.contains(nm) {
                return Err(format!("'ae32' source {} given twice", nm));
            }
            let words = read_words(p)?;
            if words.len() != 4 * w * h {
                return Err(format!("'ae32' source {}: {} words, {}x{}x4 needed", nm, words.len(), w, h));
            }
            let s = fx_exec::hostconv14::derived_source(words, planes, &g, self.head.unwrap()).map_err(|e| e.to_string())?;
            labels.push(format!("source {} head {} derived from {:?}; record {}", nm, fx_exec::hostconv14::head_name(s.reader_head), g, match self.head.unwrap() { Some(r) => format!("{} (check equal)", fx_exec::hostconv14::head_name(r)), None => "unrecorded".into() }));
            sources.push(s);
            names.push(nm.clone());
        }
        let (dp, dm) = self.dest.unwrap();
        let prior = read_words(self.prior.as_ref().unwrap())?;
        let copier = self.copier.unwrap();
        labels.push(format!("label DEST-ROUTE (code 14, stack word {}, {} planes, mode {}, copier {:?}; R-PARITY (FLOOR-PENDING))", self.dest_sw.unwrap(), dp, dm, copier));
        let route = fx_exec::hostconv14::Ae32Route {
            sources,
            dest_stack_word: self.dest_sw.unwrap(),
            dest: Some(fx_exec::hostconv14::Ae32Dest { planes: dp, mode: dm, prior, copier_source: copier }),
        };
        Ok(Ae32Ready { route, names, out: self.out.clone().unwrap(), labels, arm, late: self.late.clone(), modein: self.modein, host_inplace: self.host_inplace, wh: (w, h), class_cells: self.class_cells, class_record: self.class_record.clone(), class3_rec: self.class3_rec, rec_head: self.head.unwrap(), host_params: self.host_params.clone(), uw })
    }
}

/// INT13 (Batch 48 rules 1 and 3): derive the owner arm's inputs from the built program and check the recorded fields against them.
/// def_dynamic: the token "dynamic" in the definition's flags ([owner] = 0 for the 13 styles, MEASURED-DYLIB, PDV2).
/// opacity parameter: a parameter named "opacity" or "input_opacity" sets bit 0x40 of [prog] (PDV2); its record value stays
/// OWNER-ARM-RECORDED (--owner-opacity) until PDV3's array read is ruled. bit 2 of [owner]: owner_loop::negotiate at the stated host
/// model: gate closed without the opacity parameter (NEGOTIATION-GATE-CLOSED), else HOST-NEGOTIATION-ZERO (zero answer record, ret 0),
///  = 1 (HOST-APP), cell = 1 (MEASURED-DYLIB); [owner] enters as 0.
fn owner_inputs_derived(g: &fx_eval::build::Genotype, arm: &fx_exec::hostconv14::OwnerArm) -> Result<Vec<String>, String> {
    use fx_exec::owner_loop::{negotiate, Negotiation};
    let mut labels = Vec::new();
    let def_dynamic = g.flags.iter().any(|f| f == "dynamic");
    if def_dynamic != arm.owner_arm_nonzero {
        return Err(format!("OWNER-INPUT-MISMATCH: the definition gives [owner] != 0 = {}, the recorded field is {}", def_dynamic, arm.owner_arm_nonzero));
    }
    let opac: Vec<&str> = g.params.iter().map(|p| p.name.as_str()).filter(|n| *n == "opacity" || *n == "input_opacity").collect();
    let bit40 = !opac.is_empty();
    if bit40 != arm.opacity_bits.is_some() {
        return Err(format!(
            "OWNER-INPUT-MISMATCH: the program's opacity parameters {:?} (bit 0x40 of [prog] = {}), --owner-opacity {}",
            opac,
            bit40 as u8,
            if arm.opacity_bits.is_some() { "gives a value" } else { "is none" }
        ));
    }
    let zero = Negotiation { kind: 1, gate_cell_a: 1, acquire_present: true, suite_acquired: true, ret1: 0, flag: 0, fmt: Vec::new(), mode: 0, ret2: 0 };
    let fx_run_cell_h = negotiate(0, bit40, Some(&zero));
    let bit2 = fx_run_cell_h & 0x4 != 0;
    if bit2 != arm.owner_arm_bit2 {
        return Err(format!("OWNER-INPUT-MISMATCH: the negotiation model gives bit 2 of [owner] = {}, the recorded field is {}", bit2 as u8, arm.owner_arm_bit2 as u8));
    }
    labels.push(format!("label OWNER-INPUTS-DERIVED (def_dynamic {} from the definition flags; opacity parameters {:?}; bit 2 = {} by owner_loop::negotiate; the recorded --owner-arm fields checked equal; INT13)", def_dynamic as u8, opac, bit2 as u8));
    labels.push(if bit40 {
        "label HOST-NEGOTIATION-ZERO (the host model offers the Image Format Negotiation Suite (version 1) and answers [suite] with return 0 and a 0x20-byte answer record written as zeros: flag byte 0, empty format string, mode word 0; the test then takes ecx = 0 and exits with bits 0x4 and 0x8 of [owner] unchanged; the twins' test ported (negotiate); PDV2, PDV3)".to_string()
    } else {
        "label NEGOTIATION-GATE-CLOSED (no opacity parameter: bit 0x40 of [prog] clear; no acquire of the suite; the per-run suite-slot list is the record's check)".to_string()
    });
    labels.push(if bit40 {
        format!("label OWNER-ARM-RECORDED ({}: the Opacity loop's record value from --owner-opacity until PDV3's array read is ruled (Batch 48 rule 4))", g.name)
    } else {
        "label OWNER-PARAMS-DERIVED (no kind-4 opacity or input_opacity parameter in the program: the loop's opacity branch is not taken; v = 0)".to_string()
    });
    labels.push("label POSTDIV-DERIVED (ctx = [owner] by the ported owner arm: setup function (SMART_PRE_RENDER, its entry hit recorded per run), the arm for the 13 styles ([owner] = 0: no definition carries the token \"dynamic\", MEASURED-DYLIB), then the parameter loop with the opacity branch : m = cvttss2si(record + 0.5f); v = 1 iff a kind-4 parameter named opacity or input_opacity in the program's parameter array has m = 1 and bit 2 of [owner] is clear; bit 2 from the ported negotiation test at the claim host's answer (W390); writes the ctx cells on every route; checked against the recorded owner and ctx cells at every compile; PDV1, PDV2, W371, W380)".to_string());
    Ok(labels)
}

/// INT13 (Batch 49 rule 14): the word 2 certificate gate. Ok(None): word 2 is not 1 (the arguments stand). Ok(Some((args, label))): a certified
/// job, the arguments with --ctx-word2 0 and the label.
/// INT15 (Batch 51 rule 1, W418): A CERTIFICATE ROW CERTIFIES ONE JOB. fx_run takes --word2-cert FILE --cert-job "<job line>" --cert-sidecar FILE.
///   The job key is the job identity as run, from the job line PROG|TAG|W|H|SETS|EXTRA: program, tag, WxH, the full sorted binding (every H: host
///   name and value and every record name), curtime, pattern, worldfmt, smem and now (presteps excluded: the counted and certificate jobs may differ).
///   Certificate rows (TSV, '#' comments): program, job key, counted run tag, certificate job tag, counted presteps, certificate presteps,
///   SHA-256 of the original's plan at word 2 = 0 (certificate job), SHA-256 of the recorded plan at word 2 = 1 (counted run), W113 verdict (EQUAL), source.
///   The sidecar (written beside the frozen command) holds `job<TAB>LINE` and `native<TAB>KEY`; KEY is this command's converted arguments
///   bind=<sorted --set and host:--host-param>|time=<--ctx-time>|frame=<--host-frame>|src=<sorted Name:SHA-256 of each --ae32-src / --in file>.
///   Refused with CERT-KEY-MISMATCH unless: the sidecar's job line equals --cert-job; the job's W, H equal --size; its curtime equals --ctx-time
///   (and --host-frame when given); the sidecar's native key equals this command's; and a row exists for exactly that program and job key.
fn word2_certified(a: &[String], prog: &str, w: usize, h: usize) -> Result<Option<(Vec<String>, String)>, String> {
    let pos = match a.iter().position(|x| x == "--ctx-word2") {
        Some(p) => p,
        None => return Ok(None),
    };
    if a.get(pos + 1).map(|s| s.as_str()) != Some("1") {
        return Ok(None);
    }
    let refuse = |why: String| -> String {
        format!("STACK-WORD2-NOT-IMPLEMENTED: stack word 2 = 1 enables the node-grouping pass (in the planner) which is not ported; {} (Batch 49 rule 14)", why)
    };
    let mism = |why: String| -> String { format!("CERT-KEY-MISMATCH: {} (Batch 51 rule 1, W418: a certificate row certifies one job; no borrowed rows)", why) };
    let opt = |name: &str| -> Option<String> { a.iter().position(|x| x == name).and_then(|p| a.get(p + 1)).cloned() };
    let cf = match opt("--word2-cert") {
        Some(c) => c,
        None => return Err(refuse("no --word2-cert certificate file given".into())),
    };
    let job = opt("--cert-job").ok_or_else(|| mism("no --cert-job job line given".into()))?;
    let sc = opt("--cert-sidecar").ok_or_else(|| mism("no --cert-sidecar file given".into()))?;
    let jk = job_key(&job).map_err(|e| mism(format!("--cert-job [{}]: {}", job, e)))?;
    let f: Vec<&str> = job.split('|').collect();
    if f[0] != prog {
        return Err(mism(format!("the job's program {} is not {}", f[0], prog)));
    }
    if f[2].trim().parse::<usize>().ok() != Some(w) || f[3].trim().parse::<usize>().ok() != Some(h) {
        return Err(mism(format!("the job's size {}x{} is not --size {}x{}", f[2], f[3], w, h)));
    }
    let time = opt("--ctx-time").unwrap_or_else(|| "-".into());
    let frame = opt("--host-frame").unwrap_or_else(|| "-".into());
    let ct = extra_field(f[5], "--curtime").unwrap_or_default();
    let num = |s: &str| s.parse::<f64>().ok();
    if num(&ct).is_none() || num(&ct) != num(&time) || (frame != "-" && num(&frame) != num(&ct)) {
        return Err(mism(format!("the job's curtime {} is not --ctx-time {} / --host-frame {}", ct, time, frame)));
    }
    let mut bind: Vec<String> = Vec::new();
    let mut srcs: Vec<String> = Vec::new();
    for j in 0..a.len() {
        let v = match a.get(j + 1) {
            Some(v) => v,
            None => continue,
        };
        match a[j].as_str() {
            "--set" => bind.push(v.clone()),
            "--host-param" => bind.push(format!("host:{}", v)),
            "--ae32-src" | "--in" => {
                let (n, p) = v.split_once('=').unwrap_or(("", v.as_str()));
                let bytes = std::fs::read(p).map_err(|e| mism(format!("cannot read the source {} for the native key: {}", p, e)))?;
                srcs.push(format!("{}:{}", n, sha256_hex(&bytes)));
            }
            _ => {}
        }
    }
    bind.sort();
    srcs.sort();
    let nkey = format!("bind={}|time={}|frame={}|src={}", bind.join(";"), time, frame, srcs.join(","));
    let st = std::fs::read_to_string(&sc).map_err(|e| mism(format!("cannot read the sidecar {}: {}", sc, e)))?;
    let sget = |k: &str| st.lines().find_map(|l| l.strip_prefix(&format!("{}\t", k))).map(|s| s.to_string());
    if sget("job").as_deref() != Some(job.as_str()) {
        return Err(mism(format!("the sidecar {} job line is not --cert-job", sc)));
    }
    if sget("native").as_deref() != Some(nkey.as_str()) {
        return Err(mism(format!("this command's converted arguments [{}] are not the sidecar's [{}]", nkey, sget("native").unwrap_or_default())));
    }
    // INT16 (Batch 52 rule 5, W434): the sidecar must be the one the frozen set's record names, by path and SHA-256 (SIDECAR-NOT-OF-RECORD)
    let sonr = |why: String| -> String { format!("SIDECAR-NOT-OF-RECORD: {} (Batch 52 rule 5, W434)", why) };
    let recf = opt("--sidecar-record").ok_or_else(|| sonr("no --sidecar-record file given".into()))?;
    let rect = std::fs::read_to_string(&recf).map_err(|e| sonr(format!("cannot read the sidecar record {}: {}", recf, e)))?;
    let scsha = sha256_hex(st.as_bytes());
    let of_record = rect.lines().filter(|l| !l.starts_with('#')).any(|l| {
        let c: Vec<&str> = l.split('\t').collect();
        c.len() >= 3 && c[0] == sc && c[1] == scsha && c[2] == jk
    });
    if !of_record {
        return Err(sonr(format!("the sidecar {} (sha256 {}) is not named for job [{}] by the record {}", sc, scsha, jk, recf)));
    }
    let text = std::fs::read_to_string(&cf).map_err(|e| refuse(format!("cannot read the certificate {}: {}", cf, e)))?;
    for l in text.lines() {
        if l.starts_with('#') || l.trim().is_empty() {
            continue;
        }
        let c: Vec<&str> = l.split('\t').collect();
        if c.len() < 9 || c[0] != prog || c[1] != jk {
            continue;
        }
        let hex = |s: &str| s.len() == 64 && s.chars().all(|ch| ch.is_ascii_hexdigit());
        if !hex(c[6]) || !hex(c[7]) || c[8] != "EQUAL" {
            return Err(mism(format!("the certificate row for [{}] is not a W113 EQUAL pair with two plan SHA-256 values", jk)));
        }
        let mut v = a.to_vec();
        v[pos + 1] = "0".to_string();
        let w113 = if c[6] == c[7] { format!("W113 EQUAL (identical recorded plan text, SHA-256 {})", c[6]) } else { "W113 EQUAL (plancmp)".to_string() };
        let label = format!(
            "label STACK-WORD2-INERT-ORIG ({}: stack word 2 = 1 in the compile by the ported formula (HOST-FIELD, W315); the grouping pass is not ported; for this job the original's plan compiled at word 2 = 0 by certificate job {} (presteps {}, sha256 {}) equals the plan recorded by counted run {} at word 2 = 1 (presteps {}, sha256 {}) under W113, and the native plan is taken at the word 2 = 0 input; certificate {}; job key [{}]; native key [{}] from sidecar {} (sidecar of record, sha256 {}, record {}); {}; Batch 51 rule 1 (W418), Batch 52 rule 5 (W430, W434), Batch 50 rule 8; PCX2, WPG12, SMR9, SMR10)",
            prog, c[3], c[5], c[6], c[2], c[4], c[7], cf, jk, nkey, sc, scsha, recf, w113
        );
        return Ok(Some((v, label)));
    }
    Err(mism(format!("no certificate row for {} job [{}] in {}", prog, jk, cf)))
}

/// INT15 (Batch 51 rule 1): one token after `name` in a job line's EXTRA field.
fn extra_field(extra: &str, name: &str) -> Option<String> {
    let t: Vec<&str> = extra.split_whitespace().collect();
    t.iter().position(|x| *x == name).and_then(|p| t.get(p + 1)).map(|s| s.to_string())
}

/// INT15 (Batch 51 rule 1): the job key of a job line PROG|TAG|W|H|SETS|EXTRA[|...]: program, tag, WxH, the sorted binding, curtime, pattern,
/// worldfmt, smem and now. Every field is required.
fn job_key(job: &str) -> Result<String, String> {
    let f: Vec<&str> = job.split('|').collect();
    if f.len() < 6 {
        return Err("not a PROG|TAG|W|H|SETS|EXTRA job line".into());
    }
    let mut b: Vec<&str> = f[4].split(';').filter(|x| !x.is_empty()).collect();
    b.sort();
    let mut k = format!("{}|{}|{}x{}|{}", f[0], f[1], f[2], f[3], b.join(";"));
    for (n, o) in [("curtime", "--curtime"), ("pat", "--pat"), ("worldfmt", "--worldfmt"), ("smem", "--smem"), ("now", "--now")] {
        let v = extra_field(f[5], o).ok_or_else(|| format!("the job line has no {}", o))?;
        k += &format!("|{}={}", n, v);
    }
    Ok(k)
}

/// INT15: SHA-256 (FIPS 180-4) of a byte string as lowercase hex, for the certificate key's source field (no crate dependency).
fn sha256_hex(m: &[u8]) -> String {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
        0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
        0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
        0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];
    let mut hs: [u32; 8] = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19];
    let mut d = m.to_vec();
    let bits = (m.len() as u64).wrapping_mul(8);
    d.push(0x80);
    while d.len() % 64 != 56 {
        d.push(0);
    }
    d.extend_from_slice(&bits.to_be_bytes());
    for blk in d.chunks(64) {
        let mut wv = [0u32; 64];
        for t in 0..16 {
            wv[t] = u32::from_be_bytes([blk[4 * t], blk[4 * t + 1], blk[4 * t + 2], blk[4 * t + 3]]);
        }
        for t in 16..64 {
            let s0 = wv[t - 15].rotate_right(7) ^ wv[t - 15].rotate_right(18) ^ (wv[t - 15] >> 3);
            let s1 = wv[t - 2].rotate_right(17) ^ wv[t - 2].rotate_right(19) ^ (wv[t - 2] >> 10);
            wv[t] = wv[t - 16].wrapping_add(s0).wrapping_add(wv[t - 7]).wrapping_add(s1);
        }
        let mut v = hs;
        for t in 0..64 {
            let s1 = v[4].rotate_right(6) ^ v[4].rotate_right(11) ^ v[4].rotate_right(25);
            let ch = (v[4] & v[5]) ^ (!v[4] & v[6]);
            let t1 = v[7].wrapping_add(s1).wrapping_add(ch).wrapping_add(K[t]).wrapping_add(wv[t]);
            let s0 = v[0].rotate_right(2) ^ v[0].rotate_right(13) ^ v[0].rotate_right(22);
            let mj = (v[0] & v[1]) ^ (v[0] & v[2]) ^ (v[1] & v[2]);
            let t2 = s0.wrapping_add(mj);
            v = [t1.wrapping_add(t2), v[0], v[1], v[2], v[3].wrapping_add(t1), v[4], v[5], v[6]];
        }
        for i in 0..8 {
            hs[i] = hs[i].wrapping_add(v[i]);
        }
    }
    hs.iter().map(|x| format!("{:08x}", x)).collect()
}

/// INT17 step 5: strip --effect-answer N and --effect-count K from the arguments and judge them by fx_exec::style (BCJ3's render-path rule,
/// ). Both or neither. Declared: the selected program must equal the run's core program, else STYLE-SELECT-MISMATCH.
/// Undeclared for a program of a measured multi-style table: check-only stand-in label (INT16-A3). Other programs: no label.
fn style_select_args(a: &[String]) -> Result<(Vec<String>, Option<String>), String> {
    let mut v = Vec::with_capacity(a.len());
    let (mut ans, mut cnt): (Option<i32>, Option<i16>) = (None, None);
    let mut i = 0;
    while i < a.len() {
        match a[i].as_str() {
            "--effect-answer" | "--effect-count" if i >= 3 => {
                let x = a.get(i + 1).ok_or_else(|| format!("{} needs a value", a[i]))?;
                if a[i] == "--effect-answer" {
                    if ans.is_some() {
                        return Err("--effect-answer given twice".into());
                    }
                    ans = Some(x.parse().map_err(|_| "--effect-answer takes the popup answer (integer)")?);
                } else {
                    if cnt.is_some() {
                        return Err("--effect-count given twice".into());
                    }
                    cnt = Some(x.parse().map_err(|_| "--effect-count takes [owner] (integer)")?);
                }
                i += 2;
            }
            _ => {
                v.push(a[i].clone());
                i += 1;
            }
        }
    }
    let prog = a[2].as_str();
    let core = fx_exec::style::core_name(prog);
    let tabled = fx_exec::style::STYLE_TABLE.iter().find(|(_, t)| t.contains(&core)).map(|(e, _)| *e);
    match (ans, cnt) {
        (Some(n), Some(k)) => {
            // INT32 step BOPS (BOP1, W570 (e)): a def that a style runs by style_program (Blur_Only's def) is judged by that style's table
            let by_style = if tabled.is_none() {
                fx_exec::style::STYLE_TABLE.iter().flat_map(|(_, t)| t.iter()).find(|st| fx_exec::style::style_program(st) == Some(prog)).copied()
            } else {
                None
            };
            let (eff, idx, sel) = fx_exec::style::select(by_style.unwrap_or(prog), k, n)?;
            if by_style.is_some() || (sel != core && fx_exec::style::style_program(sel) == Some(prog)) {
                if fx_exec::style::style_program(sel) != Some(prog) {
                    return Err(format!("STYLE-SELECT-MISMATCH: {} runs {} but the Effect answer {} selects style index {} ({}); INT32", eff, prog, n, idx, sel));
                }
                return Ok((v, Some(format!("label STYLE-PROGRAM ({}: Effect answer {} of {} gives style index {} = {}, which runs the def {}; BOP1, W570; INT32)", eff, n, k, idx, sel, prog))));
            }
            if sel != core {
                return Err(format!("STYLE-SELECT-MISMATCH: {} runs {} but [owner] = {} with Effect answer {} selects style index {} ({})", eff, prog, k, n, idx, sel));
            }
            Ok((v, Some(format!("label STYLE-SELECT ({}: [owner] = {}, Effect answer {} gives style index {} = {}, the program run; render path, BCJ3 MEASURED; INT17)", eff, k, n, idx, sel))))
        }
        (None, None) => Ok((v, tabled.map(|e| format!("label STYLE-SELECT-UNDECLARED ({}: {} is in the measured program table; no Effect answer declared, check-only stand-in, INT16-A3; INT17)", e, prog)))),
        _ => Err("STYLE-SELECT: --effect-answer and --effect-count are given together or not at all".into()),
    }
}

/// INT17 step s11_prerender (PDV3 PATCH 1, W409 (b); Batch 51 rule 10): [prog] by PROG_MODE_WORD-RULE-READ, the later pre-render stage
///  (owner_loop::later_stage) between the two calls, the mode byte by owner_loop::ctx_init, and the destination
/// body selected by it. The recorded --ae32-dest mode is a check.
fn mode_byte_derived(
    g: &fx_eval::build::Genotype,
    arm: &fx_exec::hostconv14::OwnerArm,
    ae32_ready_a: u8,
    late: Option<&(i8, Vec<u8>, Vec<u32>)>,
    modein: Option<(u32, u32)>,
    host_inplace: bool,
    wh: (usize, usize),
    rec_mode: i32,
    bound_in: &[(usize, String)],
    class_cells: Option<(Option<u8>, Option<u8>, Option<u8>)>,
    class_record: &[(String, u8)],
    g3: &G3In,
    class3_out: &mut Vec<String>,
) -> Result<Vec<String>, String> {
    use fx_exec::owner_loop::{ctx_init, later_stage, setup_arm, LateInputs, ParamView};
    let mut labels = Vec::new();
    // PROG_MODE_WORD-RULE-READ: the creator stores 2; the flags loop stores 1 for rgba_ok and 3 for set_alpha_to_one 
    let mut g3_in_a = 2u32;
    for f in &g.flags {
        match f.as_str() {
            "rgba_ok" => g3_in_a = 1,
            "set_alpha_to_one" => g3_in_a = 3,
            _ => {}
        }
    }
    labels.push(format!(
        "label PROG_MODE_WORD-RULE-READ ({}: [prog] = {} derived from the definition flags (creator 2, rgba_ok 1, set_alpha_to_one 3; 2 becomes 3 when [prog] == 0, premise [prog] != 0, recorded per run by PDV3 PATCH 4); the recorded value is a per-run check; PDV3, W409)",
        g.name, g3_in_a
    ));
    let mut params: Vec<ParamView> = match arm.opacity_bits {
        Some(b) => vec![ParamView { name: "opacity".to_string(), kind: 4, value: f32::from_bits(b) }],
        None => vec![],
    };
    // TWR1 (W722, W725): the program's output_opacity loop value follows opacity in registered order (arm), the same list
    // hostconv14::try_owner_arm builds; without it [owner] stayed 0 where the original holds 3.
    if let Some(b) = arm.output_opacity_bits {
        params.push(ParamView { name: "output_opacity".to_string(), kind: 4, value: f32::from_bits(b) });
    }
    let mut o = setup_arm(arm.owner_arm_nonzero, arm.owner_arm_bit2, &params).map_err(|e| format!("OWNER-ARM-VALUE-OUT-OF-RANGE: {:?}", e))?;
    let (setup_cell_a, setup_cell_b) = (o.ae32_ready_c, o.ae32_ready_d);
    let (late_inputs_a, rect, a8) = match late {
        Some((k, r, c)) => (*k, r.clone(), c.clone()),
        None if g3_in_a == 1 && o.ae32_ready_d != 3 => (0i8, vec![0u8], vec![0u32]), // [prog] 1: no branch of the stage reads the index or arrays
        None if o.ae32_ready_d == 3 => (0i8, vec![0u8], vec![0u32]),            // [owner] 3: the first test stores 3 without reading them
        None => return Err(format!("PRERENDER-LATE-INPUT-REQUIRED: {} has [prog] = {}; the later stage reads [prog] and its arrays (--prerender-late)", g.name, g3_in_a)),
    };
    let li = LateInputs { ae32_ready_b: o.ae32_ready_d, ae32_ready_a, prog_mode_word: g3_in_a, late_inputs_a, rect_flag: rect, a8 };
    let lo = later_stage(&li).map_err(|e| format!("PRERENDER-LATE-{:?}: [prog] = {} outside the stage's arrays", e, late_inputs_a))?;
    o.ae32_ready_d = lo.ae32_ready_d;
    labels.push(format!(
        "label PRERENDER-LATE-STAGE (ported, owner_loop::later_stage: inputs [owner] {}, [owner] {}, [prog] {}, [prog] {}; out [owner] {}, flag {}; {}; PDV3)",
        li.ae32_ready_b, ae32_ready_a, g3_in_a, late_inputs_a, lo.ae32_ready_d, lo.rbp_cell_a,
        if lo.rbp_cell_a == 0 { "no-store exit" } else { "store branch (PRERENDER-STORE-BRANCH-UNOBSERVED: tested by constructed inputs only)" }
    ));
    let r8 = if o.ae32_ready_d != 0 { o.ae32_ready_d } else { g3_in_a };
    // M3P (W496 (d), W511): ctx and desc derived from the host datum (--host-inplace, default 0) by RDR12's rule when the mode
    // byte inputs are read (r8 == 2); --mode-inputs is only a recorded check, refused on mismatch.
    let (fx_run_cell_f, d_cell_a) = if (r8 & 0xff) == 2 {
        let d = fx_exec::mode_select::derive(r8 as u32, host_inplace, wh.0, wh.1)?;
        if let Some(x) = modein {
            if x != d {
                return Err(format!("MODE-INPUT-MISMATCH ({}: derived ({}, {}), recorded ({}, {}))", g.name, d.0, d.1, x.0, x.1));
            }
        }
        labels.push(format!(
            "label HOST-INPLACE-DERIVED ({}: --host-inplace {}; ctx = {} and desc = {} by (RDR12 line 7: 590 value {}, desc 2 by writer, in[idx] 2 and desc 0 as RJ1 claim-route constants, in[idx] the host world {}x{}, data pointers equal exactly when in place); the recorded --mode-inputs {}; M3P, W496)",
            g.name, host_inplace as u32, d.0, d.1, r8, wh.0, wh.1, match modein { Some(x) => format!("({}, {}) checked equal", x.0, x.1), None => "not given".into() }
        ));
        // INT27 (W524 (c)): only S_HueSatBright (HSR2's RJ1 record) has its own record of the five terms; every other style books nothing
        // until its own record shows them.
        if let Some(t) = fx_exec::mode_select::terms_unrecorded_label(&g.name) {
            labels.push(t);
        }
        d
    } else {
        modein.unwrap_or((1, 0))
    };
    let cc = ctx_init(&o, g3_in_a, fx_run_cell_f, d_cell_a);
    let m = cc.ctx_cells_a as i32;
    if !(1..=3).contains(&m) {
        return Err(format!("MODE-BODY-NOT-PORTED: the derived mode byte {} selects no ported destination body (code 14 bodies for modes 1, 2, 3)", m));
    }
    if m != rec_mode {
        return Err(format!("MODE-BYTE-MISMATCH: the derived mode byte {} ([owner] {}, [prog] {}) differs from the recorded destination mode {} (a finding)", m, o.ae32_ready_d, g3_in_a, rec_mode));
    }
    labels.push(format!(
        "label MODE-BYTE-DERIVED (ctx = {} by from [owner] {} and [prog] {}; the destination body is selected by it; the recorded mode {} checked equal; PDV3, Batch 51 rule 10)",
        m, o.ae32_ready_d, g3_in_a, rec_mode
    ));
    // INT20 step G: INPUT-CLASS-GUARD (RDR8 PATCH 2 and 4, W450 (b)): every bound input's class derived before any kernel is chosen
    if setup_cell_b != o.ae32_ready_d {
        return Err(format!("INPUT-CLASS-1358-ORDER-UNRECORDED: the later stage changed [owner] from {} to {}; the order of and the later stage is not recorded (INT20-A2)", setup_cell_b, o.ae32_ready_d));
    }
    {
        use fx_exec::input_class::{guard_one, input_class_guard, ClassCells, RowCells, CLASS_IMPLEMENTED};
        let (fx_run_cell_c, fx_run_cell_d, fx_run_cell_g) = class_cells.unwrap_or((None, None, None));
        let cells = ClassCells { ae32_ready_c: setup_cell_a, ae32_ready_d: setup_cell_b, prog_mode_word: g3_in_a, prog_class_byte: fx_run_cell_c, prog_flag_byte: fx_run_cell_d, flag_2: fx_run_cell_g };
        let rows = RowCells { fmt_cell_a: 0, b_cell_o: 0, word_cell_a: 0 };
        let bi: Vec<(usize, &str)> = bound_in.iter().map(|(k, n)| (*k, n.as_str())).collect();
        let derived: Vec<(usize, String, u8)> = {
            let mut v = Vec::new();
            for &(k, n) in &bi {
                v.push((k, n.to_string(), guard_one(&g.name, k, n, &cells, &rows)?));
            }
            v
        };
        for (n, k) in class_record {
            match derived.iter().find(|d| d.1 == *n) {
                Some(d) if d.2 == *k => {}
                Some(d) => return Err(format!("INPUT-CLASS-RECORD-MISMATCH ({} input {}: derived class {}, recorded class byte {}; a finding that stops this style's bookings, W450 (b) 3)", g.name, n, d.2, k)),
                None => return Err(format!("INPUT-CLASS-RECORD-MISMATCH ({} input {}: a recorded class byte for an input that is not bound)", g.name, n)),
            }
        }
        // INT20 step G3: class 4 as at step G; class 3 admitted only on the ported path (variant B head, body at mode 2,
        // alpha slot from the host source row); class 1 and 2 refused; every other case refused by name, never a fallback.
        let _ = input_class_guard;
        for (k, n, c) in &derived {
            match *c {
                CLASS_IMPLEMENTED => {}
                3 => {
                    labels.extend(class3_admit(&g.name, *k, n, g3_in_a, m, g3)?);
                    class3_out.push(n.clone());
                }
                other => return Err(format!("INPUT-CLASS-NOT-PORTED ({} input {}: derived class {}; consumers of a non-uniform class not ported)", g.name, n, other)),
            }
        }
        if !class3_out.is_empty() {
            let (_, ri, re, _) = g3.class3_rec.unwrap();
            let (pi, pe) = (class3_out.len() as u32, convert_pair::class3_planes() as u32);
            if (pi, pe) != (ri, re) {
                return Err(format!("INPUT-CLASS-ALLOC-MISMATCH ({}: planned allocate command {} image(s), elnum {}; record {} image(s), elnum {})", g.name, pi, pe, ri, re));
            }
            labels.push(format!("label ALLOCATE-PLANNED ({}: {} image(s), elnum {}; the record's allocate command checked equal; RDR10)", g.name, pi, pe));
        }
        labels.push(format!(
            "label INPUT-CLASS-GUARD ({}: bound inputs {:?} derive classes {:?} by the rule and the caller's class-1 test from [owner] {}, [owner] {}, [prog] {}, record cells [prog] {}, [prog] {}, * {}; claim routes refused unless every bound input derives the class the kernels implement; per-run record check {}; RDR7, RDR8, W450 (b))",
            g.name, derived.iter().map(|d| d.1.as_str()).collect::<Vec<_>>(), derived.iter().map(|d| d.2).collect::<Vec<_>>(), setup_cell_a, setup_cell_b, g3_in_a,
            fx_run_cell_c.map(|x| format!("0x{:02x}", x)).unwrap_or("not read or unrecorded".into()), fx_run_cell_d.map(|x| format!("0x{:02x}", x)).unwrap_or("not read or unrecorded".into()),
            fx_run_cell_g.map(|x| format!("0x{:02x}", x)).unwrap_or("not read or unrecorded".into()),
            if class_record.is_empty() { "not given (no input-class record on this job line)".to_string() } else { format!("{} of {} equal", class_record.len(), class_record.len()) }
        ));
        labels.push("label INPUT-CLASS-CTX-ROWS-ZERO (ctx rows and [owner+2i] are 0 on the 'ae32' host model: MEASURED in HSR1's hsrD record and CAP2's record; INFERRED for other styles; INT20-A3)".to_string());
    }
    if m == 3 {
        labels.push("label MODE3-BODY (the destination body for mode byte 3 named by a hook record at the dispatcher's body entry; INFERRED routine P; floor as the 3-plane code 14 rows if it is P; recorded in the first answer 1 counted run)".to_string());
    }
    Ok(labels)
}

/// INT29 step TW: --tw-route dur_frames,limit_cell,local_time_step,host_id,f4,r8,r9 (seven fields, each required; F comes from --host-frame).
fn parse_tw_route(v: &str) -> Result<fx_exec::twframe::TwRoute, String> {
    let f: Vec<&str> = v.split(',').collect();
    let bad = || format!("FRAME-REQUEST-INPUT-UNRECORDED (--tw-route {}: seven fields dur_frames,limit_cell,local_time_step,host_id,f4,r8,r9 from the run's record; W559)", v);
    if f.len() != 7 {
        return Err(bad());
    }
    let i = |k: usize| f[k].parse::<i32>().map_err(|_| bad());
    Ok(fx_exec::twframe::TwRoute { big_f: 0.0, dur_frames: f[0].parse::<f32>().map_err(|_| bad())?, limit_cell: i(1)?, step: i(2)?, kind: i(3)?, f4: i(4)?, r8: i(5)?, r9: i(6)? })
}

/// INT29 (W580 (e), Batch 69 rule 13): the head label of a class-3 input, the same at modes 2 and 3: R3 names
/// SOURCE-HEAD-R3-ON-CLASS3, variant B names CONVERT-VARIANT-B-UNWEIGHTED.
fn class3_head_label(style: &str, name: &str, r3: bool, vb4: bool) -> String {
    if vb4 {
        format!("label SOURCE-HEAD-VB4-ON-CLASS3 ({} input {}: the class-3 Foreground is read by the variant-B 4-plane head (read4b, FMT3e, no alpha weight), derived from the gate cells and checked against the run's head hook; the three colour lanes are handed to the class-3 body; SVB1, W744)", style, name)
    } else if r3 {
        format!("label SOURCE-HEAD-R3-ON-CLASS3 ({}: the class-3 Source is read by head (R3, FMT3a, R-PARITY and FLOOR-MET, W341), derived from planes 3 and the owner-arm gate and executed per the head hooks (DRR2 dr2h: 2 entries; : 0 hits); weighted rows MEASURED 64 of 64 per plane on 2 calls; DRR2, W476; input {})", style, name)
    } else {
        format!("label CONVERT-VARIANT-B-UNWEIGHTED ({} input {}: head, the second and third dwords, through the finite test, no alpha read, no weight; MEASURED RDR10 r1b)", style, name)
    }
}

/// INT20 step G3: what the class-3 admission reads from the route (the recorded head, the sources' derived heads and plane counts, the destination).
struct G3In {
    class3_rec: Option<(u32, u32, u32, u8)>,
    rec_head: Option<u64>,
    sources: Vec<(String, u64, usize)>,
    dest_planes: usize,
    copier: Option<usize>,
}

/// INT20 step G3 (RDR10 PATCH 1 and 2, Batch 55 rule 3, coordinator note on DRR2): admit a class-3 bound input only on the ported path.
fn class3_admit(style: &str, _k: usize, name: &str, g3_in_a: u32, mode: i32, g3: &G3In) -> Result<Vec<String>, String> {
    let (hits, _, _, fx_run_cell_e) = g3.class3_rec.ok_or_else(|| format!("INPUT-CLASS-CELL-UNRECORDED ({}: the class-3 record (builder hits, allocate command, [prog]; --class3-record))", style))?;
    if hits != 0 {
        return Err(format!("CLASS3-BUILDER-ON-ROUTE ({}: the record shows {} plane-count builder hits; the plane form is not ported)", style, hits));
    }
    if fx_run_cell_e == 0 {
        return Err(format!("PROG_MODE_WORD-PREMISE-5A2-ZERO ({}: [prog] = 0 turns [prog] 2 into 3; the premise of PROG_MODE_WORD-RULE-READ fails)", style));
    }
    let k = g3.sources.iter().position(|s| s.0 == name).ok_or_else(|| format!("INPUT-CLASS-NOT-PORTED ({} input {}: class 3 for an input that is not an 'ae32' source)", style, name))?;
    let (_, head, planes) = &g3.sources[k];
    // INT21 step G3R (W476 (d), W481 (a)): R3 (readers14 read3, the W341 body) on the class-3 path, admitted only when the record's
    // head hooks show (the record's head is the check, never the input; hostconv14::ae32_to_image reads R3 by readers14 for 3 planes).
    let r3 = *head == fx_exec::hostconv14::R3_HEAD;
    // SVB1 (W744 (b)): the variant-B 4-plane head on the class-3 path, admitted only when the run's record head hook shows it
    let vb4 = *head == fx_exec::hostconv14::VB4_HEAD;
    if *head != fx_exec::hostconv14::VB3_HEAD && !r3 && !vb4 {
        return Err(format!("SOURCE-VARIANT-UNPORTED ({} input {}: derived head 0x{:x}; only variant B, variant B 4-plane (by record) and R3 are ported for class 3)", style, name, head));
    }
    if vb4 && g3.rec_head != Some(fx_exec::hostconv14::VB4_HEAD) {
        return Err(format!("SOURCE-HEAD-UNRECORDED ({}: class 3 derived, source head selected, the run's record carries no source-head hook showing it; the head is not taken on the claim route without its record; SVB1, W744)", style));
    }
    if r3 && g3.rec_head != Some(fx_exec::hostconv14::R3_HEAD) {
        // INT24 (W515 (h)): the refusal text as ruled
        return Err(format!("SOURCE-HEAD-UNRECORDED ({}: class 3 derived, source head selected, the run's record carries no source-head hook to check it against; the head is not taken on the claim route without its record; G3R, W476, W515)", style));
    }
    // Batch 57 (W481 (a)): no SOURCE-HEAD-MISMATCH refusal; a recorded head that differs is refused by derived_source (SOURCE-VARIANT-MISMATCH)
    let _ = g3.rec_head;
    let want = convert_pair::class3_planes();
    // SVB1: the 4-plane head hands over its three colour lanes (hostconv14::class3_vb4_lanes); the record's plane count is 4
    if vb4 && !(mode == 3 && g3.dest_planes == 3) {
        return Err(format!("MODE-BODY-NOT-PORTED ({}: class 3 on the variant-B 4-plane head is admitted at mode byte 3 with a 3-plane destination only; got mode {} and {} destination planes; SVB1)", style, mode, g3.dest_planes));
    }
    if vb4 && *planes != 4 {
        return Err(format!("INPUT-CLASS-PLANES-MISMATCH ({}: {}: variant B reads 4 planes, the record gives {})", style, name, planes));
    }
    if !vb4 && *planes != want {
        return Err(format!("INPUT-CLASS-PLANES-MISMATCH ({}: {}: derived {} planes for class 3, the record gives {})", style, name, want, planes));
    }
    // M3P2 (W666 (d) 3): at mode byte 3 a 1-plane destination runs the ported code-14 one-plane zero-arm body (FMT3c)
    let one_plane_m3 = mode == 3 && g3.dest_planes == 1 && !cfg!(feature = "m3p2_refusal_kept_mut");
    if !(mode == 2 || mode == 3) || !(g3.dest_planes == 3 || one_plane_m3) {
        return Err(format!("MODE-BODY-NOT-PORTED ({}: class 3 with mode byte {} and {} destination planes; ported: body at mode 2 and body at mode 3, 3 planes, and the one-plane zero-arm body at mode 3, 1 plane)", style, mode, g3.dest_planes));
    }
    if mode == 3 && g3.dest_planes == 1 {
        if g3.copier.is_some() {
            return Err(format!("COPIER-ORDER-UNRECORDED ({}: a row copier record at mode 3; the one-plane body writes word 0 itself)", style));
        }
        return Ok(vec![
            format!("label INPUT-CLASS-PLANES-DERIVED ({} input {}: class 3 by the caller rule from [prog] {} and the record cells; planes {} set through the binding's input elnum before compile; the recorded --src-planes checked equal; RDR10)", style, name, g3_in_a, want),
            class3_head_label(style, name, r3, vb4),
            format!("label MODE3-CLASS3-ROUTED-1P ({}: class 3 at mode byte 3 with a 1-plane destination runs the code-14 one-plane zero-arm body (FMT3c's port, R-PARITY cited: FMT3c's T2, plane count 1, tier A, mode 3, 639 of 639 cases PASS), MEASURED by the OP1 record of BCJ5 (oM1c: the one-plane body 64 entries, the three-plane body 0); words 1 to 3 = plane 0, word 0 = 1.0, no row copier; FLOOR-OPEN under W433 (b) until the reader pair; W626, W666)", style),
            "label FLOOR-OPEN (convert_pair c14 one-plane tier A body: FLOOR-PARTIAL in FMT3c, 122 of 651 mutants unproven over the four plain bodies; W433 (b); the reader pair of W626 (h) 3 reads the UNCLASSIFIED tier A survivors before the Mono OP1 count)".to_string(),
        ]);
    }
    if mode == 3 {
        // M3P: body (RJ1 rj1M3F: 64 hits, 0 hits of) stores 1.0 in slot 0; no row copier follows it
        if g3.copier.is_some() {
            return Err(format!("COPIER-ORDER-UNRECORDED ({}: a row copier record at mode 3; the RJ1 mode-3 record shows 0 hits of and the body writes slot 0 itself)", style));
        }
        return Ok(vec![
            format!("label INPUT-CLASS-PLANES-DERIVED ({} input {}: class 3 by the caller rule from [prog] {} and the record cells; planes {} set through the binding's input elnum before compile; the recorded --src-planes checked equal; RDR10)", style, name, g3_in_a, want),
            // INT29 (W580 (e)): the mode-3 label follows the head as the mode-2 branch does (the read follows it: hostconv14.rs ae32_to_image)
            class3_head_label(style, name, r3, vb4),
            format!("label MODE3-CLASS3-ROUTED ({}: an in-place host (output world data pointer equal to the input world's) makes ctx = 0 at render (RDR12), so derives mode byte 3; when [owner] is already 3 (the Output Opacity answer's loop value 0 through, or the input arm) takes the mode byte from the owner cell, the arm is not entered and the host datum is not read (Time_Warp_RGB_Autogen, W722); the class-3 path then runs the code-14 three-plane body (routed to the existing FMT3c code-14 body, slot 0 = 1.0, not re-ported), MEASURED by the mode-3 record of HSR2 RJ1 (rj1M3F: 64 hits, 0, 0); the host datum is fx_run's --host-inplace; HOST-INPLACE-DERIVED; M3P, W496)", style),
            "label FLOOR-MET (convert_pair: code-14 three-plane tier A body, 45 of 45 mutants killed or read equivalent by two readers, MUT1 and RDR5, W433 (a); route cited, not re-ported)".to_string(),
        ]);
    }
    // INT27: the mode-2 copier requirement, dropped by M3P's PATCH (its PATCH.md says it is kept for mode 2); restored with the s26_op text
    // (the G3 control copier_none ran rc 0 on the first s27_g3m chain instead of refusing).
    if g3.copier != Some(k) {
        return Err(format!("DEST-ALPHA-FROM-SOURCE-ROW-UNRECORDED ({}: the class-3 body leaves slot 0; the row copier from source {} is required, the record gives {:?})", style, k, g3.copier));
    }
    Ok(vec![
        format!("label INPUT-CLASS-PLANES-DERIVED ({} input {}: class 3 by the caller rule from [prog] {} and the record cells; planes {} set through the binding's input elnum before compile; the recorded --src-planes checked equal; RDR10)", style, name, g3_in_a, want),
        class3_head_label(style, name, r3, vb4),
        format!("label DEST-ALPHA-FROM-SOURCE-ROW ({}: mode 2, body leaves slot 0 unwritten; copies the host source row's slot 0; MEASURED RDR10 r1b and r1c at the identity binding and RDR11 h25 at Hue Shift 0.25, alpha 0.25 to 1.0, and alx, alpha -0.5 to 2.5)", style),
        format!("label CLASS3-BUILDERS-NOT-ON-ROUTE ({}: 0 plane-count builder hits in the record)", style),
        "label FLOOR-MET (convert_pair: variant B and the mode-2 three-plane body; t_class3 with special values, 5 of 5 mutants killed with binary SHAs, INT20 runs/g3_floor; route pieces equal by t33_class3_route)".to_string(),
    ])
}

/// The corpus source argument that selects the native builders.
const NATIVE_PROGRAMS: &str = "-";

/// `fx_run <program> --size ...` (no table argument) when the first argument is a natively built program: insert the native marker.
fn native_args(a: &[String]) -> Vec<String> {
    let mut v = a.to_vec();
    if v.len() >= 2 && fx_eval::programs_native::has_program(&v[1]) && (v.len() == 2 || v[2].starts_with("--")) {
        v.insert(1, NATIVE_PROGRAMS.to_string());
    }
    v
}

/// The program corpus: the native builders for `-`, else the programs table at `path` (optional override, text route).
fn corpus_for(path: &str) -> Result<Corpus, String> {
    if path == NATIVE_PROGRAMS {
        return Ok(Corpus::native());
    }
    let text = std::fs::read_to_string(path).map_err(|e| format!("cannot read {}: {}", path, e))?;
    Ok(Corpus::from_programs_tsv(&text))
}

/// LYM1: the layout row registered for `prog` (header's last name = the program), parsed; None when the corpus has no such row.
fn lym_resolve(path: &str, prog: &str) -> Result<Option<fx_eval::layoutmerge::LayoutInfo>, String> {
    if path == NATIVE_PROGRAMS {
        return fx_eval::programs_native::layout_info(prog);
    }
    let text = std::fs::read_to_string(path).map_err(|e| format!("cannot read {}: {}", path, e))?;
    let c = Corpus::from_programs_tsv(&text);
    let rows = fx_eval::layoutmerge::find_layout_text(c.layout_texts.iter().map(|x| x.1.as_str()), prog);
    match rows.len() {
        0 => Ok(None),
        1 => fx_eval::layoutmerge::read_layout(rows[0]).map(Some).map_err(|e| format!("LAYOUT-ROW-UNREADABLE {}: {}", prog, e)),
        n => Err(format!("LAYOUT-ROW-AMBIGUOUS: {} layout rows are registered for {}", n, prog)),
    }
}

#[cfg(test)]
mod int29_tests {
    use super::*;
    fn g3(head: u64) -> G3In {
        G3In { class3_rec: Some((0, 0, 0, 1)), rec_head: Some(head), sources: vec![("Source".to_string(), head, convert_pair::class3_planes())], dest_planes: 3, copier: None }
    }
    /// W580 (e): at mode 3 the label names the head the run reads; the R3 head and the variant B head give different labels
    /// (before INT29 both printed CONVERT-VARIANT-B-UNWEIGHTED, so this test fails on s27_si's text).
    #[test]
    fn mode3_label_follows_head() {
        let r3 = class3_admit("S", 0, "Source", 2, 3, &g3(fx_exec::hostconv14::R3_HEAD)).unwrap();
        let vb = class3_admit("S", 0, "Source", 2, 3, &g3(fx_exec::hostconv14::VB3_HEAD)).unwrap();
        assert!(r3.iter().any(|l| l.starts_with("label SOURCE-HEAD-R3-ON-CLASS3")));
        assert!(!r3.iter().any(|l| l.starts_with("label CONVERT-VARIANT-B-UNWEIGHTED")));
        assert!(vb.iter().any(|l| l.starts_with("label CONVERT-VARIANT-B-UNWEIGHTED")));
        assert!(!vb.iter().any(|l| l.starts_with("label SOURCE-HEAD-R3-ON-CLASS3")));
    }
    /// SVB1 (W744 (b)): VB4 is admitted on class 3 only by the record head at mode 3 / 3 planes; every other unported head still refuses by name.
    #[test]
    fn svb1_vb4_admitted_by_record_only() {
        let ok = class3_admit("S", 0, "Source", 2, 3, &{ let mut a = g3(fx_exec::hostconv14::VB4_HEAD); a.sources[0].2 = 4; a }).unwrap();
        assert!(ok.iter().any(|l| l.starts_with("label SOURCE-HEAD-VB4-ON-CLASS3")));
        assert!(ok.iter().any(|l| l.starts_with("label MODE3-CLASS3-ROUTED (")));
        let mut a = g3(fx_exec::hostconv14::VB4_HEAD);
        a.sources[0].2 = 4;
        a.rec_head = None;
        assert!(class3_admit("S", 0, "Source", 2, 3, &a).unwrap_err().starts_with("SOURCE-HEAD-UNRECORDED"));
        a.rec_head = Some(fx_exec::hostconv14::R3_HEAD);
        assert!(class3_admit("S", 0, "Source", 2, 3, &a).unwrap_err().starts_with("SOURCE-HEAD-UNRECORDED"));
        a.rec_head = Some(fx_exec::hostconv14::VB4_HEAD);
        assert!(class3_admit("S", 0, "Source", 2, 2, &a).unwrap_err().starts_with("MODE-BODY-NOT-PORTED"));
        a.sources[0].2 = 3;
        assert!(class3_admit("S", 0, "Source", 2, 3, &a).unwrap_err().starts_with("INPUT-CLASS-PLANES-MISMATCH"));
    }
    #[test]
    fn svb1_other_heads_still_refuse() {
        use fx_exec::hostconv14 as h;
        for head in [h::R4_HEAD, h::R1_HEAD, h::VB1_HEAD].into_iter().chain(h::UNPORTED_HEADS).chain([h::R4_HEAD]) {
            let mut a = g3(head);
            a.rec_head = Some(head);
            let e = class3_admit("S", 0, "Source", 2, 3, &a).unwrap_err();
            assert!(e.starts_with("SOURCE-VARIANT-UNPORTED"), "0x{:x}: {}", head, e);
        }
    }
    /// M3P2 (W666 (d) 3): class 3 at mode 3 with a 1-plane destination reaches the one-plane body; with 2 planes (or 4) it still refuses; at mode 2
    /// a 1-plane destination still refuses; a copier record at mode 3 is refused.
    #[test]
    fn m3p2_mode3_one_plane_routed() {
        let mut a = g3(fx_exec::hostconv14::R3_HEAD);
        a.dest_planes = 1;
        let ls = class3_admit("S", 0, "Source", 2, 3, &a).unwrap();
        assert!(ls.iter().any(|l| l.starts_with("label MODE3-CLASS3-ROUTED-1P")));
        assert!(ls.iter().any(|l| l.starts_with("label FLOOR-OPEN")));
        assert!(!ls.iter().any(|l| l.starts_with("label MODE3-CLASS3-ROUTED (")));
        for np in [0usize, 2, 4] {
            a.dest_planes = np;
            assert!(class3_admit("S", 0, "Source", 2, 3, &a).unwrap_err().starts_with("MODE-BODY-NOT-PORTED"), "{} planes", np);
        }
        a.dest_planes = 1;
        assert!(class3_admit("S", 0, "Source", 2, 2, &a).unwrap_err().starts_with("MODE-BODY-NOT-PORTED"));
        assert!(class3_admit("S", 0, "Source", 2, 1, &a).unwrap_err().starts_with("MODE-BODY-NOT-PORTED"));
        a.copier = Some(0);
        assert!(class3_admit("S", 0, "Source", 2, 3, &a).unwrap_err().starts_with("COPIER-ORDER-UNRECORDED"));
        // the 3-plane mode 3 route is unchanged
        let mut b = g3(fx_exec::hostconv14::R3_HEAD);
        b.dest_planes = 3;
        assert!(class3_admit("S", 0, "Source", 2, 3, &b).unwrap().iter().any(|l| l.starts_with("label MODE3-CLASS3-ROUTED (")));
    }
    /// Mode 2 keeps its head labels (the copier from source 0 is given).
    #[test]
    fn mode2_label_follows_head() {
        let mut a = g3(fx_exec::hostconv14::R3_HEAD);
        a.copier = Some(0);
        let mut b = g3(fx_exec::hostconv14::VB3_HEAD);
        b.copier = Some(0);
        assert!(class3_admit("S", 0, "Source", 2, 2, &a).unwrap().iter().any(|l| l.starts_with("label SOURCE-HEAD-R3-ON-CLASS3")));
        assert!(class3_admit("S", 0, "Source", 2, 2, &b).unwrap().iter().any(|l| l.starts_with("label CONVERT-VARIANT-B-UNWEIGHTED")));
    }
    /// INT29 step TW: every --tw-route field is required.
    #[test]
    fn tw_route_fields_required() {
        assert!(parse_tw_route("240,0,2,0,0,0,1").is_ok());
        assert!(parse_tw_route("240,0,2,0,0,0").unwrap_err().starts_with("FRAME-REQUEST-INPUT-UNRECORDED"));
        assert!(parse_tw_route("240,x,2,0,0,0,1").is_err());
    }
}

/// HPT1: the render-history driver. `--host-times t1,t2,..` runs the renders in order; per render the record decides KEEP or RESTART
/// (fx_exec::history). Options, each from the run's record with no default (INT29 step H, W576 (c) to (f), Batch 69 rules 1 and 8):
/// `--host-step S` (local_time_step, in_data; never time_step), `OPT_PROG_FLAG V` (the program flag byte; bit value 4 is the
/// history flag), `--host-alloc V` (the allocator cell; nonzero is present, 0 the free path), `--host-depth N` (the depth n), `--host-id H`
/// (the host id; the claim route is ported for id 1). `--host-sizes WxH,..` (default: `--size`). Any `--in`, `--frame` or `--prev` value
/// may contain `%R`, replaced by the render number (1-based). A KEEP render reads the previous render's saved destination; a given
/// `--prev` is only a check (HISTORY-INPUT-MISMATCH). Outputs go to PREFIX.rK.* and PREFIX.history.tsv.
fn run_history(a0: &[String]) -> i32 {
    use fx_exec::history::{Decision, FrameRecord};
    let mut times: Vec<i32> = Vec::new();
    let (mut step, mut sizes, mut flag, mut alloc, mut depth, mut host_id): (Option<i32>, Vec<(usize, usize)>, Option<u32>, Option<bool>, Option<i32>, Option<i64>) =
        (None, Vec::new(), None, None, None, None);
    let (mut out, mut size0, mut prev_arg): (String, Option<(usize, usize)>, Option<String>) = (String::new(), None, None);
    let parse_size = |s: &str| -> Option<(usize, usize)> {
        let (x, y) = s.split_once('x')?;
        Some((x.parse().ok()?, y.parse().ok()?))
    };
    let mut base: Vec<String> = a0[..3.min(a0.len())].to_vec();
    let mut i = 3;
    while i < a0.len() {
        let k = a0[i].as_str();
        let v = a0.get(i + 1).cloned().unwrap_or_default();
        match k {
            "--host-times" | "--host-step" | "--host-sizes" | fx_exec::judge_keys::OPT_PROG_FLAG | "--host-alloc" | "--host-depth" | "--host-id" | "--size" | "--out" | "--prev" => {
                let ok = match k {
                    "--host-times" => v.split(',').map(|x| x.trim().parse::<i32>()).collect::<Result<Vec<_>, _>>().map(|t| times = t).is_ok(),
                    "--host-step" => v.parse::<i32>().map(|x| step = Some(x)).is_ok(),
                    "--host-sizes" => v.split(',').map(parse_size).collect::<Option<Vec<_>>>().map(|s| sizes = s).is_some(),
                    fx_exec::judge_keys::OPT_PROG_FLAG => v.parse::<u32>().map(|x| flag = Some(x)).is_ok(),
                    "--host-alloc" => parse_cell(&v).map(|x| alloc = Some(x != 0)).is_some(),
                    "--host-depth" => v.parse::<i32>().map(|x| depth = Some(x)).is_ok(),
                    "--host-id" => parse_cell(&v).map(|x| host_id = Some(x as i64)).is_some(),
                    "--size" => parse_size(&v).map(|s| size0 = Some(s)).is_some(),
                    "--out" => {
                        out = v.clone();
                        true
                    }
                    _ => {
                        prev_arg = Some(v.clone());
                        true
                    }
                };
                if !ok {
                    eprintln!("{} takes a valid value, got '{}'", k, v);
                    return 2;
                }
                i += 2;
            }
            _ => {
                base.push(a0[i].clone());
                i += 1;
            }
        }
    }
    let tag = std::path::Path::new(&out).file_name().map(|x| x.to_string_lossy().to_string()).unwrap_or_default();
    let (step, flag, depth, alloc) = match history_required(&tag, step, flag, depth, alloc, host_id) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            return 2;
        }
    };
    if times.is_empty() || out.is_empty() || (sizes.is_empty() && size0.is_none()) || (!sizes.is_empty() && sizes.len() != times.len()) {
        eprintln!("--host-times needs --out and --size or one --host-sizes entry per render");
        return 2;
    }
    let mut rec = FrameRecord::fresh();
    let mut log = String::from("render\tt\tstep\twidth\theight\tdecision\tlast_t_after\trestart_t\tw4\tw6\tprev_input\tcheck\n");
    let mut prev_file: Option<(String, usize)> = None;
    for (k, &t) in times.iter().enumerate() {
        let (w, h) = if sizes.is_empty() { size0.unwrap() } else { sizes[k] };
        let d = rec.begin(flag & 4 != 0, t, step, (w as i32, h as i32));
        let pre = format!("{}.r{}", out, k + 1);
        let mut args = base.clone();
        for x in args.iter_mut().skip(3) {
            if x.contains("%R") {
                *x = x.replace("%R", &(k + 1).to_string());
            }
        }
        args.extend(["--size".into(), format!("{}x{}", w, h), "--out".into(), pre.clone()]);
        let mut input = "none".to_string();
        let mut check = "none".to_string();
        if d == Decision::Keep {
            if let Some((pf, c)) = &prev_file {
                args.extend(["--prev".into(), format!("{}:{}", pf, c)]);
                input = pf.clone();
                if let Some(pa) = &prev_arg {
                    let given = pa.replace("%R", &(k + 1).to_string());
                    let (gp, _) = split(&given, 4);
                    let same = std::fs::read(&gp).ok() == std::fs::read(pf).ok();
                    check = if same { "equal".into() } else { "HISTORY-INPUT-MISMATCH".into() };
                    if !same {
                        eprintln!("HISTORY-INPUT-MISMATCH (render {}: the given --prev {} differs from the native's own previous destination {}; W516 (d))", k + 1, gp, pf);
                        return 1;
                    }
                }
            }
        }
        let rc = run(&args);
        if rc != 0 {
            eprintln!("render {} failed with rc {}", k + 1, rc);
            return rc;
        }
        let pf = format!("{}.prev.f32", pre);
        let chans = std::fs::read_to_string(format!("{}.dest.meta.txt", pre))
            .ok()
            .and_then(|m| m.lines().find_map(|l| l.strip_prefix("channels ").and_then(|c| c.trim().parse::<usize>().ok())))
            .unwrap_or(4);
        prev_file = if std::path::Path::new(&pf).exists() { Some((pf, chans)) } else { None };
        if flag & 4 != 0 {
            rec.save(alloc, (w as i32, h as i32), depth);
        }
        log.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\t{:?}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            k + 1, t, step, w, h, d, rec.last_t, rec.restart_t, rec.w4, rec.w6, input, check
        ));
    }
    let _ = std::fs::write(format!("{}.history.tsv", out), log);
    // RENDER-HISTORY-PORTED (W516 (d) with W532 (c)'s clause, K and VARIANT-INERT-ON-ROUTE; W576 (f)); the text with its cited
    // addresses is data (fx_exec/data/RENDER_HISTORY_PORTED.txt), not code.
    let _ = std::fs::write(format!("{}.history.labels.txt", out), RENDER_HISTORY_PORTED);
    0
}

/// INT29 step H: the label texts with their cited addresses, kept as data.
const RENDER_HISTORY_PORTED: &str = include_str!("../../data/RENDER_HISTORY_PORTED.txt");
const HOST_DETECT_ID_UNEXPECTED: &str = include_str!("../../data/HOST_DETECT_ID_UNEXPECTED.txt");

/// A record cell value, decimal or 0x hex.
fn parse_cell(v: &str) -> Option<u64> {
    match v.strip_prefix("0x") {
        Some(h) => u64::from_str_radix(h, 16).ok(),
        None => v.parse::<u64>().ok(),
    }
}

/// INT29 step H (W576 (c) to (f)): every history input from the run's record, refused by name when absent; the host id must be 1.
fn history_required(tag: &str, step: Option<i32>, flag: Option<u32>, depth: Option<i32>, alloc: Option<bool>, host_id: Option<i64>) -> Result<(i32, u32, i32, bool), String> {
    let step = step.ok_or_else(|| format!("HISTORY-HOST-DATA-MISSING (S_Trails {}: --host-step, local_time_step, is required with --host-times; W576)", tag))?;
    let flag = flag.ok_or_else(|| format!("HISTORY-FLAG-UNRECORDED (S_Trails {}: [prog] is not given from the run's record; W576)", tag))?;
    let depth = depth.ok_or_else(|| format!("HISTORY-HOST-CELL-UNRECORDED (S_Trails {}: the depth n is not given from the run's record; W576)", tag))?;
    let alloc = alloc.ok_or_else(|| format!("HISTORY-HOST-CELL-UNRECORDED (S_Trails {}: the allocator cell is not given from the run's record; W576)", tag))?;
    match host_id {
        Some(1) => Ok((step, flag, depth, alloc)),
        Some(v) => Err(HOST_DETECT_ID_UNEXPECTED.trim().replace("<v>", &v.to_string())),
        None => Err(format!("HISTORY-HOST-CELL-UNRECORDED (S_Trails {}: the host id is not given from the run's record; W576, W527)", tag)),
    }
}

#[cfg(test)]
mod int29_history_tests {
    use super::*;
    #[test]
    fn history_inputs_required() {
        let ok = history_required("t", Some(1), Some(13), Some(3), Some(true), Some(1));
        assert_eq!(ok, Ok((1, 13, 3, true)));
        assert!(history_required("t", Some(1), None, Some(3), Some(true), Some(1)).unwrap_err().starts_with("HISTORY-FLAG-UNRECORDED (S_Trails t:"));
        let e = history_required("t", Some(1), Some(13), None, Some(true), Some(1)).unwrap_err();
        assert!(e.starts_with("HISTORY-HOST-CELL-UNRECORDED") && e.contains("the depth n"));
        let e = history_required("t", Some(1), Some(13), Some(3), None, Some(1)).unwrap_err();
        assert!(e.starts_with("HISTORY-HOST-CELL-UNRECORDED") && e.contains("the allocator cell"));
        let e = history_required("t", Some(1), Some(13), Some(3), Some(true), Some(4)).unwrap_err();
        assert!(e.starts_with("HOST-DETECT-ID-UNEXPECTED") && e.contains("= 4;"));
        assert!(history_required("t", Some(1), Some(13), Some(3), Some(true), None).unwrap_err().starts_with("HISTORY-HOST-CELL-UNRECORDED"));
        assert!(history_required("t", None, Some(13), Some(3), Some(true), Some(1)).unwrap_err().starts_with("HISTORY-HOST-DATA-MISSING"));
        // the allocator cell 0 is a recorded value (the free path), not an absent input
        assert_eq!(history_required("t", Some(1), Some(13), Some(3), Some(false), Some(1)), Ok((1, 13, 3, false)));
    }
}

/// INT31 step AL (W589 (b), W600 (a)): the input alias rule on the 'ae32' route, for every program. An unbound input whose def-text default
/// names another declared input binds that input's node (the same planes, class, head and conversion); no image is constructed for it.
/// The chain of defaults is resolved against the inputs the 'ae32' host world binds (`ae_names`) and `ins_names`. Returns per aliased input
/// (input, the default it names, the final target, the record evidence). A record that shows the input's own layer (its bound-mask bit set,
/// or a source given for it) refuses INPUT-ALIAS-RECORD-MISMATCH; a target that nothing binds refuses INPUT-DEFAULT-ALIAS as before.
fn ae32_input_aliases(prog: &str, tag: &str, inputs: &[fx_eval::build::InputRec], bmask: u32, ae_names: &[String], ins_names: &[String]) -> Result<Vec<(String, String, String, String)>, String> {
    let declared = |n: &str| inputs.iter().any(|o| o.name == n);
    let alias_of = |d: Option<&str>| d.filter(|t| fx_eval::orig_number_pub(t).is_none() && declared(t)).map(|t| t.to_string());
    let given = |n: &str| ae_names.iter().any(|x| x == n) || ins_names.iter().any(|x| x == n);
    let mut out = Vec::new();
    if cfg!(feature = "al_const_mut") {
        return Ok(out); // mutant: the alias falls to the constant default path
    }
    for (k, inp) in inputs.iter().enumerate() {
        let dn = match alias_of(inp.default.as_deref()) {
            Some(d) => d,
            None => continue,
        };
        let bit = k < 32 && (bmask >> k) & 1 == 1;
        if bit && given(&inp.name) {
            continue; // bound by its record and its line: the alias does not apply
        }
        if bit || given(&inp.name) {
            return Err(format!("INPUT-ALIAS-RECORD-MISMATCH ({} {}: the record shows a {} layer bound; the alias does not apply; W589)", prog, tag, inp.name));
        }
        let mut cur = dn.clone();
        let mut hops = 0usize;
        let target = loop {
            if given(&cur) {
                break Some(cur.clone());
            }
            match alias_of(inputs.iter().find(|o| o.name == cur).and_then(|o| o.default.as_deref())) {
                Some(n2) if hops < inputs.len() => {
                    cur = n2;
                    hops += 1;
                }
                _ => break None,
            }
        };
        match target {
            Some(t) => out.push((inp.name.clone(), dn, t, format!("bound mask 0x{:x} bit {} clear and no source given for {}", bmask, k, inp.name))),
            None => return Err(format!("INPUT-DEFAULT-ALIAS: input {} defaults to input {} which has no image bound; refused", inp.name, dn)),
        }
    }
    Ok(out)
}

/// INT31 step AL: every use of an aliased input's node reads the target's input node instead (the original's plan reads IN:<target> at the use);
/// the aliased input node is left unreferenced, so the reachable compile has no node for it.
fn alias_input_nodes(g: &mut fx_eval::build::Genotype, al: &[(String, String, String, String)]) {
    for (inp, _, tgt, _) in al {
        let ai = g.inputs.iter().find(|x| x.name == *inp).map(|x| x.node);
        let ti = g.inputs.iter().find(|x| x.name == *tgt).map(|x| x.node);
        if let (Some(ai), Some(ti)) = (ai, ti) {
            for n in g.nodes.iter_mut() {
                for c in n.children.iter_mut() {
                    if *c == ai {
                        *c = ti;
                    }
                }
            }
            for d in g.dest.iter_mut() {
                if *d == ai {
                    *d = ti;
                }
            }
        }
    }
}

/// INT31 step AL: the per-run label (W600 (a)'s general text of W589 (b)).
fn input_alias_label(prog: &str, tag: &str, a: &(String, String, String, String)) -> String {
    format!(
        "label INPUT-ALIAS-TO-SOURCE-NODE ({} {}: the unbound input {} defaults to {} by the def text '( {} {} )'; the record shows no {} layer ({}); the native binds the {} node itself (the same planes, class, head and conversion); the run's recorded plan reads IN:{} at the {} use; ALS1, AUD3, W589, W600)",
        prog, tag, a.0, a.2, a.0, a.1, a.0, a.3, a.2, a.2, a.0
    )
}

#[cfg(test)]
mod int31_alias_tests {
    use super::*;
    fn pt() -> &'static str { effects_vecdir::path_str("record_extract/programs/programs.tsv") }
    fn geno(p: &str) -> fx_eval::build::Genotype {
        let c = Corpus::from_programs_tsv(&std::fs::read_to_string(pt()).unwrap());
        build_effect(&c, c.find_effect(p, None).unwrap()).unwrap()
    }
    fn reach(g: &fx_eval::build::Genotype) -> Vec<usize> {
        let mut seen = vec![false; g.nodes.len()];
        let mut st: Vec<usize> = g.dest.clone();
        while let Some(x) = st.pop() {
            if !seen[x] {
                seen[x] = true;
                st.extend(g.nodes[x].children.iter().copied());
            }
        }
        (0..g.nodes.len()).filter(|&k| seen[k]).collect()
    }
    fn s(v: &[&str]) -> Vec<String> {
        v.iter().map(|x| x.to_string()).collect()
    }
    // both claim forms refused INPUT-DEFAULT-ALIAS before step AL; the alias binds the target's node, and the aliased node is unreachable
    fn check(prog: &str, inp: &str, tgt: &str) {
        let mut g = geno(prog);
        let k = g.inputs.iter().position(|x| x.name == inp).unwrap();
        let t = g.inputs.iter().position(|x| x.name == tgt).unwrap();
        let ai = g.inputs[k].node;
        let ti = g.inputs[t].node;
        assert!(reach(&g).contains(&ai), "{} reads {} before the alias", prog, inp);
        let al = ae32_input_aliases(prog, "t", &g.inputs, 1 << t, &s(&[tgt]), &[]).unwrap();
        assert_eq!(al.len(), 1, "{}: one alias", prog);
        assert_eq!((al[0].0.as_str(), al[0].1.as_str(), al[0].2.as_str()), (inp, tgt, tgt));
        alias_input_nodes(&mut g, &al);
        let r = reach(&g);
        assert!(!r.contains(&ai) && r.contains(&ti), "{}: the {} use reads the {} node", prog, inp, tgt);
        let l = input_alias_label(prog, "t", &al[0]);
        assert!(l.contains(&format!("the def text '( {} {} )'", inp, tgt)) && l.contains(&format!("bit {} clear", k)));
    }
    #[test]
    fn al_distort_lens_to_source() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
        check("Distort_RGB", "Lens", "Source");
    }
    #[test]
    fn al_drop_shadow_matte_to_foreground() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
        check("Drop_Shadow", "Matte", "Foreground");
    }
    #[test]
    fn al_record_shows_layer_refuses() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
        let g = geno("Distort_RGB");
        let k = g.inputs.iter().position(|x| x.name == "Lens").unwrap();
        let e = ae32_input_aliases("Distort_RGB", "t", &g.inputs, 1 | (1 << k), &s(&["Source"]), &[]).unwrap_err();
        assert!(e.starts_with("INPUT-ALIAS-RECORD-MISMATCH (Distort_RGB t: the record shows a Lens layer bound"), "{}", e);
        let e = ae32_input_aliases("Distort_RGB", "t", &g.inputs, 1, &s(&["Source", "Lens"]), &[]).unwrap_err();
        assert!(e.starts_with("INPUT-ALIAS-RECORD-MISMATCH"), "{}", e);
    }
    #[test]
    fn al_unbound_target_still_refused() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
        let g = geno("Distort_RGB");
        let e = ae32_input_aliases("Distort_RGB", "t", &g.inputs, 0, &[], &[]).unwrap_err();
        assert!(e.starts_with("INPUT-DEFAULT-ALIAS: input Lens defaults to input Source"), "{}", e);
    }
}

/// W630 (b): the registered choice count N of the popup whose display name is DISPLAY in PROG's registration (record-backed, defined in effects_rtdata).
fn popup_choice_count(prog: &str, display: &str) -> Option<i32> {
    effects_rtdata::style_params_of(prog).find(|r| r.display_name == display).and_then(|r| r.popup_choices)
}

/// W630 (b): a host answer V (the one-based checkout integer) of a registered popup must lie in 1..N. A name or a popup without a registered
/// count is not judged here (CSET-NAME-NOT-ON-ROUTE keeps the unknown names).
fn popup_answer_check(prog: &str, display: &str, v: i32) -> Result<(), String> {
    if cfg!(feature = "popup_range_off_mut") {
        return Ok(());
    }
    let n = match popup_choice_count(prog, display) {
        Some(n) => n,
        None => return Ok(()),
    };
    let lo = if cfg!(feature = "popup_range_zero_based_mut") { 0 } else { 1 };
    let hi = if cfg!(feature = "popup_range_zero_based_mut") { n - 1 } else { n };
    let lo_bad = v < lo && !cfg!(feature = "popup_range_nolow_mut");
    let hi_bad = v > hi && !cfg!(feature = "popup_range_nohigh_mut");
    if lo_bad || hi_bad {
        return Err(format!("POPUP-ANSWER-OUT-OF-RANGE ({}: '{}' answer {} outside 1..{}; W630)", prog, display, v, n));
    }
    Ok(())
}

/// INT31 step L2: the registered default checkout integer of PROG's popup NAME (record name), with its display name, record and style.
fn registered_default(prog: &str, name: &str) -> Option<(i32, String, String, String)> {
    let r = effects_rtdata::style_param(prog, name)?;
    let v = r.default_checkout_integer?;
    Some((v, r.display_name.to_string(), r.record.to_string(), r.style.to_string()))
}

/// INT31 step L2: the checkout integer v of an owner opacity extra: the bound answer (host-answer form) or the registered default.
/// `text_default` is the layout text default, used only by the mutant l2_textconv_mut (the converted text default).
fn owner_opacity_v(prog: &str, name: &str, host: Option<i32>, text_default: f32) -> Result<(i32, Option<(String, String, String)>), String> {
    if let Some(a) = host {
        if !cfg!(feature = "l2_hostignored_mut") {
            return Ok((a, None));
        }
    }
    if cfg!(feature = "l2_textconv_mut") {
        return Ok((text_default as i32, None));
    }
    match registered_default(prog, name) {
        Some((v, disp, rec, style)) => Ok((v, Some((disp, rec, style)))),
        None => Err(format!("OPACITY-DEFAULT-UNRECORDED ({}: the host's registered default of '{}' is not in STYLE_PARAM_INDEX.tsv; W592)", prog, name)),
    }
}

/// INT31 step L2: OP's conversion f32(max(v - 1, 0)) of the one-based checkout integer.
fn owner_loop_value(v: i32) -> f32 {
    if cfg!(feature = "l2_nominus_mut") {
        return v as f32;
    }
    (v - 1).max(0) as f32
}

/// G2 (W638 (d) 4): one lane of a --set value as the record holds it. A decimal lane keeps the sign of zero ("-0" is 0x80000000); a lane
/// written as 0x followed by 1 to 8 hex digits is a RECORD's lane bits exactly (a parameter record's params_after_loop lane, any payload, NaNs
/// included); the hex form feeds a record's lane bits only and is not a host value. Anything else is no lane.
fn parse_lane(x: &str) -> Option<f32> {
    if let Some(h) = x.strip_prefix("0x").or_else(|| x.strip_prefix("0X")) {
        if h.is_empty() || h.len() > 8 || !h.bytes().all(|c| c.is_ascii_hexdigit()) {
            return None;
        }
        let bits = u32::from_str_radix(h, 16).ok()?;
        return Some(f32::from_bits(if cfg!(feature = "lane_bits_masked_mut") { bits & 0x7fff_ffff } else { bits }));
    }
    let v: f32 = x.parse().ok()?;
    if cfg!(feature = "lane_negzero_folded_mut") && v == 0.0 {
        return Some(0.0);
    }
    Some(v)
}

/// G2: the lanes of a --set value (comma separated); a malformed lane is dropped as before.
fn parse_set_lanes(l: &str) -> Vec<f32> {
    l.split(',').filter_map(parse_lane).collect()
}

/// The PARAM-BY-RECORD label of a parameter fed by record: the lane bits exactly as fed (text unchanged since the PARAM-UNITS step).
fn param_by_record_label(n: &str, v: &[f32]) -> String {
    format!("param {} record {:?} PARAM-BY-RECORD", n, v.iter().map(|x| format!("0x{:08x}", x.to_bits())).collect::<Vec<_>>())
}

/// Step L2b (W616 (c)): output_opacity. The checkout integer v is the bound answer (--cset 'Output Opacity=V', host answer form) or, on an
/// unbound row, the registered default; the loop value is f32(max(v - 1, 0)). A --set of output_opacity must equal it bit for bit
/// (OPACITY-SET-MISMATCH, as for opacity and input_opacity); an unbound row whose default is unrecorded refuses even when a --set is given.
/// Returns (loop value bits, v, registration; None for the bound answer).
fn output_opacity_checked(prog: &str, set: Option<f32>, host: Option<i32>, text_default: f32) -> Result<(u32, i32, Option<(String, String, String)>), String> {
    if cfg!(feature = "l2b_outset_taken_mut") {
        if let Some(sv) = set {
            return Ok((sv.to_bits(), 0, None));
        }
    }
    let (v, reg) = owner_opacity_v(prog, "output_opacity", host, text_default)?;
    let db = owner_loop_value(v).to_bits();
    if let Some(sv) = set {
        let differs = if cfg!(feature = "l2b_value_eq_mut") { sv != f32::from_bits(db) } else { sv.to_bits() != db };
        if differs {
            return Err(format!(
                "OPACITY-SET-MISMATCH: --set output_opacity={} differs from {} loop value {} ({}, W595, W616)",
                sv,
                if host.is_some() { "the host answer's" } else { "the registered default's" },
                f32::from_bits(db),
                if host.is_some() { "--cset Output Opacity, step L2b" } else { "STYLE_PARAM_INDEX.tsv" }
            ));
        }
    }
    Ok((db, v, reg))
}

/// Step L2b (W616 (c), W547): the parameter a --cset display name answers: the record name (opacity, input_opacity or output_opacity)
/// whose registered display name in STYLE_PARAM_INDEX.tsv equals the given name for this program; None for any other name.
fn cset_param(prog: &str, display: &str) -> Option<String> {
    for row in effects_rtdata::style_params_of(prog).filter(|r| r.display_name == display) {
        let r = row.record_name;
        if r == "opacity" || r == "input_opacity" || r == "output_opacity" {
            if cfg!(feature = "l2b_cset_input_as_opacity_mut") && r == "input_opacity" {
                return Some("opacity".to_string());
            }
            return Some(r.to_string());
        }
    }
    None
}

/// Step L2b: the loop value a --cset answer gives (OPC1's conversion for opacity and input_opacity, f32(max(v - 1, 0)) for output_opacity).
#[cfg(test)]
fn cset_loop_value(prog: &str, display: &str, v: i32) -> Option<(String, f32)> {
    let p = cset_param(prog, display)?;
    let x = if p == "output_opacity" { owner_loop_value(v) } else { fx_exec::opacity_conv::checkout_to_loop_opacity(0, v, None, None).ok()? };
    Some((p, x))
}

/// INT31 step L2: OWNER-OPACITY-DERIVED in W595 (d)'s text.
fn owner_opacity_label(prog: &str, tag: &str, name: &str, v: i32, reg: Option<&(String, String, String)>) -> String {
    let style = reg.map(|r| r.2.clone()).or_else(|| registered_default(prog, name).map(|r| r.3)).unwrap_or_else(|| prog.to_string());
    let src = match reg {
        None => format!("the bound answer v = {} (host-answer form)", v),
        Some((disp, rec, _)) => format!("the registered default v = {} of '{}' (STYLE_PARAM_INDEX.tsv, from {})", v, disp, rec),
    };
    format!(
        "label OWNER-OPACITY-DERIVED ({} {}: the owner arm's {} = f32(max(v - 1, 0)) for v the one-based popup checkout integer: {}; the claimed 3-choice opacity popups register v = 2 (1.0) and Time_Warp_RGB's Output Opacity v = 1 of 2 (0.0); --owner-opacity, when given, checked equal; OPC1, LYM3, W449, W592, W595)",
        style, tag, name, src
    )
}

#[cfg(test)]
mod int31_l2_tests {
    use super::*;
    fn loop_of(prog: &str, name: &str, host: Option<i32>, text: f32) -> f32 {
        owner_loop_value(owner_opacity_v(prog, name, host, text).unwrap().0)
    }
    // W595: Trails unbound 1.0; host answers v = 3 and v = 1 give 2.0 and 0.0; Time_Warp_RGB Output Opacity unbound 0.0. The layout text default (the
    // zero-based index of the default choice: 1 for opacity, 0 for output_opacity) is passed so that the converted-text mutant is exercised.
    #[test]
    fn l2_trails_unbound_is_registered_default() {
    if !effects_rtdata::have_tables() {
        return;
    }
        assert_eq!(registered_default("Trails_Autogen", "opacity").map(|x| x.0), Some(2));
        assert_eq!(loop_of("Trails_Autogen", "opacity", None, 1.0), 1.0);
    }
    #[test]
    fn l2_trails_host_answers() {
        assert_eq!(loop_of("Trails_Autogen", "opacity", Some(3), 1.0), 2.0);
        assert_eq!(loop_of("Trails_Autogen", "opacity", Some(1), 1.0), 0.0);
    }
    #[test]
    fn l2_time_warp_output_opacity_unbound() {
    if !effects_rtdata::have_tables() {
        return;
    }
        assert_eq!(registered_default("Time_Warp_RGB_Autogen", "output_opacity").map(|x| x.0), Some(1));
        assert_eq!(loop_of("Time_Warp_RGB_Autogen", "output_opacity", None, 0.0), 0.0);
    }
    // Step L2b (W616 (c)): Time_Warp_RGB_Autogen unbound, --set output_opacity=1.0 refuses, 0.0 passes; -0.0 refuses (bit compare);
    // no --set takes the registered default; an unrecorded default refuses even with a --set.
    #[test]
    fn l2b_output_opacity_set_one_refuses() {
    if !effects_rtdata::have_tables() {
        return;
    }
        let e = output_opacity_checked("Time_Warp_RGB_Autogen", Some(1.0), None, 0.0).unwrap_err();
        assert!(e.starts_with("OPACITY-SET-MISMATCH: --set output_opacity=1 differs from the registered default's loop value 0"), "{}", e);
    }
    #[test]
    fn l2b_output_opacity_set_zero_passes() {
    if !effects_rtdata::have_tables() {
        return;
    }
        let (bits, v, reg) = output_opacity_checked("Time_Warp_RGB_Autogen", Some(0.0), None, 0.0).unwrap();
        assert_eq!((bits, v), (0.0f32.to_bits(), 1));
        assert!(reg.is_some());
    }
    #[test]
    fn l2b_output_opacity_set_negative_zero_refuses() {
    if !effects_rtdata::have_tables() {
        return;
    }
        let e = output_opacity_checked("Time_Warp_RGB_Autogen", Some(-0.0), None, 0.0).unwrap_err();
        assert!(e.starts_with("OPACITY-SET-MISMATCH: --set output_opacity=-0"), "{}", e);
    }
    #[test]
    fn l2b_output_opacity_unset_is_registered_default() {
    if !effects_rtdata::have_tables() {
        return;
    }
        let (bits, v, _) = output_opacity_checked("Time_Warp_RGB_Autogen", None, None, 0.0).unwrap();
        assert_eq!((bits, v), (0.0f32.to_bits(), 1));
    }
    #[test]
    fn l2b_output_opacity_unrecorded_refuses_with_set() {
    if !effects_rtdata::have_tables() {
        return;
    }
        let e = output_opacity_checked("Not_A_Program", Some(0.0), None, 0.0).unwrap_err();
        assert!(e.starts_with("OPACITY-DEFAULT-UNRECORDED (Not_A_Program: the host's registered default of 'output_opacity'"), "{}", e);
    }
    // The bound row (TWC1's O2: Output Opacity=2) in the host answer form: v = 2 gives 1.0; --set 1.0 passes, --set 0.0 refuses.
    #[test]
    fn l2b_output_opacity_bound_answer() {
        let (bits, v, reg) = output_opacity_checked("Time_Warp_RGB_Autogen", Some(1.0), Some(2), 0.0).unwrap();
        assert_eq!((bits, v, reg.is_none()), (1.0f32.to_bits(), 2, true));
        let e = output_opacity_checked("Time_Warp_RGB_Autogen", Some(0.0), Some(2), 0.0).unwrap_err();
        assert!(e.starts_with("OPACITY-SET-MISMATCH: --set output_opacity=0 differs from the host answer's loop value 1 (--cset Output Opacity, step L2b"), "{}", e);
        assert_eq!(output_opacity_checked("Time_Warp_RGB_Autogen", None, Some(1), 0.0).unwrap().0, 0.0f32.to_bits());
    }
    // Step L2b (W547): display names resolve by the program's own table: Time_Warp_RGB's Input Opacity and Output Opacity, Trails' Opacity.
    #[test]
    fn l2b_cset_display_names() {
    if !effects_rtdata::have_tables() {
        return;
    }
        let tw = "Time_Warp_RGB_Autogen";
        assert_eq!(cset_loop_value(tw, "Input Opacity", 1), Some(("input_opacity".to_string(), 0.0)));
        assert_eq!(cset_loop_value(tw, "Input Opacity", 3), Some(("input_opacity".to_string(), 2.0)));
        assert_eq!(cset_loop_value(tw, "Output Opacity", 2), Some(("output_opacity".to_string(), 1.0)));
        assert_eq!(cset_loop_value("Trails_Autogen", "Opacity", 3), Some(("opacity".to_string(), 2.0)));
    }
    #[test]
    fn l2b_cset_unknown_names_refuse() {
    if !effects_rtdata::have_tables() {
        return;
    }
        assert_eq!(cset_param("Time_Warp_RGB_Autogen", "Opacity"), None);
        assert_eq!(cset_param("Time_Warp_RGB_Autogen", "Mocha Opacity"), None);
        assert_eq!(cset_param("Trails_Autogen", "Output Opacity"), None);
        assert_eq!(cset_param("Not_A_Program", "Opacity"), None);
    }
    #[test]
    fn l2_unrecorded_default_refuses() {
    if !effects_rtdata::have_tables() {
        return;
    }
        let e = owner_opacity_v("Not_A_Program", "opacity", None, 1.0).unwrap_err();
        assert!(e.starts_with("OPACITY-DEFAULT-UNRECORDED (Not_A_Program: the host's registered default of 'opacity'"), "{}", e);
    }
    #[test]
    fn l2_label_text() {
    if !effects_rtdata::have_tables() {
        return;
    }
        let r = registered_default("Trails_Autogen", "opacity").unwrap();
        let l = owner_opacity_label("Trails_Autogen", "t", "opacity", r.0, Some(&(r.1.clone(), r.2.clone(), r.3.clone())));
        assert!(l.starts_with("label OWNER-OPACITY-DERIVED (Trails t: the owner arm's opacity = f32(max(v - 1, 0)) for v the one-based popup checkout integer: the registered default v = 2 of 'Opacity' (STYLE_PARAM_INDEX.tsv, from CTX_Trails_Autogen_trAdv_64x32.json)"), "{}", l);
    }
}

#[cfg(test)]
mod g2_popup_tests {
    use super::*;
    const TW: &str = "Time_Warp_RGB_Autogen";
    // W630 (b): Trails' Opacity registers 3 choices, Time_Warp_RGB's Output Opacity 2.
    #[test]
    fn popup_counts_are_the_registered_ones() {
    if !effects_rtdata::have_tables() {
        return;
    }
        assert_eq!(popup_choice_count("Trails_Autogen", "Opacity"), Some(3));
        assert_eq!(popup_choice_count(TW, "Output Opacity"), Some(2));
        assert_eq!(popup_choice_count(TW, "Input Opacity"), Some(3));
        assert_eq!(popup_choice_count(TW, "Opacity"), None);
    }
    #[test]
    fn popup_answer_in_range_passes() {
    if !effects_rtdata::have_tables() {
        return;
    }
        for v in 1..=3 {
            assert!(popup_answer_check("Trails_Autogen", "Opacity", v).is_ok(), "{}", v);
        }
        assert!(popup_answer_check(TW, "Output Opacity", 1).is_ok());
        assert!(popup_answer_check(TW, "Output Opacity", 2).is_ok());
    }
    #[test]
    fn popup_answer_out_of_range_refuses() {
    if !effects_rtdata::have_tables() {
        return;
    }
        for v in [0, 4, -1, i32::MAX, i32::MIN] {
            let e = popup_answer_check("Trails_Autogen", "Opacity", v).unwrap_err();
            assert_eq!(e, format!("POPUP-ANSWER-OUT-OF-RANGE (Trails_Autogen: 'Opacity' answer {} outside 1..3; W630)", v));
        }
        let e = popup_answer_check(TW, "Output Opacity", 3).unwrap_err();
        assert_eq!(e, "POPUP-ANSWER-OUT-OF-RANGE (Time_Warp_RGB_Autogen: 'Output Opacity' answer 3 outside 1..2; W630)");
        assert!(popup_answer_check(TW, "Output Opacity", 0).is_err());
    }
    // W676 (a): every host type 7 row of the table carries a popup_choices count (59 of 59 today); a type 7 row without one fails.
    #[test]
    fn every_host_type_7_row_has_a_popup_count() {
    if !effects_rtdata::have_tables() {
        return;
    }
        let mut hdr: Vec<&str> = Vec::new();
        let (mut n7, mut with) = (0, 0);
        for l in effects_rtdata::text("data/STYLE_PARAM_INDEX.tsv").lines() {
            if l.starts_with('#') {
                continue;
            }
            let f: Vec<&str> = l.split('\t').collect();
            if f.first() == Some(&"program") {
                hdr = f;
                continue;
            }
            let col = |c: &str| hdr.iter().position(|h| *h == c).and_then(|k| f.get(k).copied());
            if col("host_type") == Some("7") {
                n7 += 1;
                if col("popup_choices").and_then(|x| x.parse::<i32>().ok()).map(|n| n >= 1).unwrap_or(false) {
                    with += 1;
                }
            }
        }
        assert_eq!((n7, with), (59, 59), "host type 7 rows with a popup_choices count");
    }
    #[test]
    fn popup_unknown_name_is_not_judged_here() {
    if !effects_rtdata::have_tables() {
        return;
    }
        assert!(popup_answer_check(TW, "Opacity", 99).is_ok());
    }
}

/// Step FH (W607 (c)): raw host frame planes (--frame-req) on the 'ae32' route refuse; host frames there come as words (--frame-words).
fn frame_content_check(prog: &str, ae32: bool, raw_frames: usize) -> Result<(), String> {
    if ae32 && raw_frames > 0 {
        return Err(format!("FRAME-CONTENT-UNCONVERTED ({}: a host frame was given as raw planes on the 'ae32' route; host frames are converted by the input's head like its Source frame; TWC1, W607)", prog));
    }
    Ok(())
}

#[cfg(test)]
mod int32_fh_tests {
    use super::*;
    #[test]
    fn fh_raw_frames_refused_on_ae32_only() {
        let e = frame_content_check("Time_Warp_RGB_Autogen", true, 1).unwrap_err();
        assert_eq!(e, "FRAME-CONTENT-UNCONVERTED (Time_Warp_RGB_Autogen: a host frame was given as raw planes on the 'ae32' route; host frames are converted by the input's head like its Source frame; TWC1, W607)");
        assert!(frame_content_check("Time_Warp_RGB_Autogen", false, 1).is_ok());
        assert!(frame_content_check("Time_Warp_RGB_Autogen", true, 0).is_ok());
    }
}

#[cfg(test)]
mod g2_lane_tests {
    use super::*;
    // W638 (d) 4: a parameter fed by record keeps the record's lane bits exactly.
    #[test]
    fn negative_zero_lane_is_fed_as_0x80000000() {
        assert_eq!(parse_lane("-0").unwrap().to_bits(), 0x8000_0000);
        assert_eq!(parse_lane("-0.0").unwrap().to_bits(), 0x8000_0000);
        assert_eq!(parse_lane("0").unwrap().to_bits(), 0);
        assert_eq!(parse_lane("0x80000000").unwrap().to_bits(), 0x8000_0000);
        assert_eq!(parse_lane("0x3f800000").unwrap().to_bits(), 0x3f80_0000);
        assert_eq!(parse_lane("0x7fc00001").unwrap().to_bits(), 0x7fc0_0001);
        assert_eq!(parse_lane("0xff800000").unwrap().to_bits(), 0xff80_0000);
        assert!(parse_lane("0x").is_none() && parse_lane("0x123456789").is_none() && parse_lane("0xzz").is_none() && parse_lane("x").is_none());
    }
    #[test]
    fn set_lane_list_keeps_each_lane_bit_pattern() {
        let v = parse_set_lanes("0,-0");
        assert_eq!(v.iter().map(|x| x.to_bits()).collect::<Vec<_>>(), vec![0, 0x8000_0000]);
        let w = parse_set_lanes("0x80000000,0x00000000,0.5");
        assert_eq!(w.iter().map(|x| x.to_bits()).collect::<Vec<_>>(), vec![0x8000_0000, 0, 0.5f32.to_bits()]);
    }
    #[test]
    fn param_by_record_label_shows_the_fed_bits() {
        let l = param_by_record_label("shift_mocha_mask", &parse_set_lanes("0,-0"));
        assert_eq!(l, "param shift_mocha_mask record [\"0x00000000\", \"0x80000000\"] PARAM-BY-RECORD");
    }
}

#[cfg(test)]
mod int32_bops_tests {
    use super::*;
    fn args(prog: &str, ans: &str) -> Vec<String> {
        ["fx_run", "programs.tsv", prog, "--size", "64x32", "--effect-answer", ans, "--effect-count", "2"].iter().map(|x| x.to_string()).collect()
    }
    #[test]
    fn bops_answer_2_runs_the_blur_only_def() {
        let (v, l) = style_select_args(&args("Blur_Motion_Curves_No_Transform_Autogen", "2")).unwrap();
        assert_eq!(v.len(), 5);
        assert_eq!(l.unwrap(), "label STYLE-PROGRAM (S_BlurMoCurves: Effect answer 2 of 2 gives style index 1 = Blur_Only, which runs the def Blur_Motion_Curves_No_Transform_Autogen; BOP1, W570; INT32)");
        // answer 1 selects Transform_and_Blur: the Blur_Only def does not run there
        assert!(style_select_args(&args("Blur_Motion_Curves_No_Transform_Autogen", "1")).unwrap_err().starts_with("STYLE-SELECT-MISMATCH"));
        // the Transform_and_Blur def at answer 1 keeps the ordinary label
        assert!(style_select_args(&args("Transform_and_Blur_Autogen", "1")).unwrap().1.unwrap().starts_with("label STYLE-SELECT (S_BlurMoCurves:"));
    }
    #[test]
    fn bops_answer_3_refused() {
        let e = style_select_args(&args("Blur_Motion_Curves_No_Transform_Autogen", "3")).unwrap_err();
        assert!(e.starts_with("STYLE-INDEX-OUT-OF-TABLE: S_BlurMoCurves Effect answer 3 gives index 2"), "{}", e);
        assert!(style_select_args(&args("Transform_and_Blur_Autogen", "3")).unwrap_err().starts_with("STYLE-INDEX-OUT-OF-TABLE"));
    }
}
