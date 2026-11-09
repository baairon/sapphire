//! Node-level executor: runs `fx_eval::compile()` output in execution order. Temps are planar f32 images held
//! as values of the nodes (whole frame, one tile). In-place buffer reuse of the original plan (F13) does not change
//! pointwise or out-of-place results, so every run gets a fresh output.

use crate::kernels::{self, HOST_CLASSES};
use crate::selector::Tables;
use crate::kernels2::{geom_of, Out};
use crate::types::{xy_grid, Arg, ExecError, Geom, Image, KParams, Val};
use fx_eval::{compile, CompileOptions, CompiledProgram, ComboTable, Genotype, PlanNode};
use std::collections::BTreeMap;
use std::rc::Rc;

/// Explicit parameter bindings and host values for one render (nothing is read from a host).
#[derive(Clone, Debug)]
pub struct Binding {
    /// parameter name to lanes; absent = PROGRAM-DEFAULT (the program text's value)
    pub overrides: BTreeMap<String, Vec<f32>>,
    /// inputs bound to a constant instead of an image (the harness's unbound Mask = 1, NV3-A5)
    pub const_inputs: BTreeMap<String, Vec<f32>>,
    /// host getter lanes folded by the compiler (get_frame_number, get_curve_val); absent = zeros (NV3-A7)
    pub host: BTreeMap<String, Vec<f32>>,
    /// FRAME-INDEX-REQUIRED (set by fx_run on the 'ae32' route): get_frame_number must be given in `host`
    pub frame_index_required: bool,
    /// step v2: set by fx_run on the 'ae32' route only; copied into CompileOptions::v3_bound_count
    pub v3_bound_count: bool,
    pub input_elnum: BTreeMap<String, i32>,
    pub paramset: String,
    pub no_fold: bool,
    /// HOST-CURVES (CV1-A1): an unkeyed get_curve_val node folds to the lanes of its first argument (NV9-A1)
    pub host_curves_unkeyed: bool,
    /// keyed host curve answers (curve name, f32 bits of the requested time) -> lanes; Some = no default, an unanswered request is a compile error
    pub host_keyed: Option<BTreeMap<(String, u32), Vec<f32>>>,
    /// the claimed HOST-CURVE-VALUES input (fx_run --host-curve-values FILE)
    pub host_curve_values: Option<fx_eval::hostcurve::HostCurveValues>,
    /// HC-ZERO-LEGACY: the named state A zero fold; false = HC-NO-INTERFACE (a get_curve_val with neither interface is an error)
    pub hc_zero_legacy: bool,
    /// RANGES-OFF: skip the evaluator range clamp (also env NEG2_RANGES_OFF)
    pub ranges_off: bool,
    /// explicit bound-input mask of the compile (bit 0 = Source bound); the executor default is 1, `fx_run --bound-mask` sets it (A.2)
    pub bound_input_mask: u32,
    ///: the compiler's `__do_postdiv__` cell as a host input of the SMART context (1 in the recorded Blur_Color compiles,
    /// read from the plan text). None = the state A derivation (the program flag `dynamic`glue's compiler-reader measurement of instance).
    pub do_postdiv: Option<bool>,
    /// the compile context; None only in library unit tests. The CLIs require it (no default) and compile_for refuses an unsupported value.
    pub ctx: Option<fx_eval::CompileContext>,
    pub kp: KParams,
}

/// the binding of the auxiliary binaries (hsb_cmp, trails_cmp, hsb_e2e, eff_e2e). `--state-a-defaults` names the stored state A
/// semantics (Binding::defaults(): bound mask 1, no compile context, plus HC-ZERO-LEGACY) and is printed on stderr; otherwise `--bound-mask HEX` and the
/// four `--ctx-*` fields are required (no default), `--hc` selects the unkeyed host-curve interface, and a get_curve_val request without one is HC-NO-INTERFACE.
pub fn strict_binding(tool: &str, a: &[String]) -> Result<Binding, String> {
    let mut b = Binding::defaults();
    if a.iter().any(|x| x == "--state-a-defaults") {
        b.hc_zero_legacy = true;
        crate::fxrun::elog!("{}: STATE-A-DEFAULTS (bound mask 1, no compile context, HC-ZERO-LEGACY; not a counted run)", tool);
        return Ok(b);
    }
    let p = a.iter().position(|x| x == "--bound-mask").ok_or_else(|| format!("{}: --bound-mask HEX is required (or the named option --state-a-defaults)", tool))?;
    let v = a.get(p + 1).ok_or_else(|| format!("{}: --bound-mask needs a value", tool))?;
    b.bound_input_mask = u32::from_str_radix(v.trim_start_matches("0x"), 16).map_err(|_| format!("{}: --bound-mask must be hex", tool))?;
    let c = fx_eval::CompileContext::from_args(a)?;
    c.check_native()?;
    b.ctx = Some(c);
    b.host_curves_unkeyed = a.iter().any(|x| x == "--hc");
    Ok(b)
}

impl Binding {
    pub fn defaults() -> Binding {
        Binding {
            overrides: BTreeMap::new(),
            const_inputs: BTreeMap::new(),
            host: BTreeMap::new(),
            frame_index_required: false,
            v3_bound_count: false,
            input_elnum: BTreeMap::new(),
            paramset: String::new(),
            no_fold: false,
            host_curves_unkeyed: false,
            host_keyed: None,
            host_curve_values: None,
            hc_zero_legacy: false,
            ranges_off: false,
            bound_input_mask: 1,
            do_postdiv: None,
            ctx: None,
            kp: KParams::standin(),
        }
    }
    pub fn compile_options(&self) -> CompileOptions {
        CompileOptions {
            overrides: self.overrides.clone(),
            input_elnum: self.input_elnum.clone(),
            strict: false,
            paramset: self.paramset.clone(),
            no_fold: self.no_fold,
            const_inputs: self.const_inputs.clone(),
            host: self.host.clone(),
            minimal_context: false,
            bound_elnum: Default::default(),
            host_curves_unkeyed: self.host_curves_unkeyed,
            host_keyed: self.host_keyed.clone(),
            host_curve_values: self.host_curve_values.clone(),
            hc_zero_legacy: self.hc_zero_legacy,
            ranges_off: self.ranges_off,
            unbound_exception_off: false, //: the unbound-input exception stays on
            bound_input_mask: self.bound_input_mask, //: explicit; default 1 = Source bound; Binding field (A.2)
            mocha_data_present: false, // no host mocha data in the executor
            history: None, // COMPILE-HISTORY: the executor compiles a fresh instance (no earlier compile)
            presence_mask: None, // the executor binds real images by name; INPUT-PRESENCE-MASK wiring into the executor is NEXT
            mixed_mask: false,
            presence_derived: false,
            do_postdiv: self.do_postdiv,
            //: fx_run copies --ctx-word2 and --ctx-has-data into CompileOptions itself
            ctx_word2: self.ctx.as_ref().map(|c| c.stack_word2).unwrap_or(false),
            ctx_has_data: self.ctx.as_ref().map(|c| c.has_data).unwrap_or(0),
            v3_bound_count: self.v3_bound_count,
            xy_unshared: false,
            xy_place: false,
            xy_pres_keys: None,
        }
    }
}

/// Explicit input planes, by input name (Source, Front, Back, Mask, xy, ...), all `width * height` floats per channel.
#[derive(Clone, Debug)]
pub struct Inputs {
    pub width: usize,
    pub height: usize,
    pub images: BTreeMap<String, Image>,
}

/// HOST-FRAMES: other frames supplied by the caller (get_frame by frame offset, get_prev_dest).
#[derive(Clone, Debug, Default)]
pub struct HostFrames {
    pub prev_dest: Option<Image>,
    pub frames: BTreeMap<i32, Image>,
    /// NV5-FRAME-ROUND-OLD: true selects round(t); false (default) is the measured frame rule floor(max(0, t)) of `frame_key`.
    pub frame_round_old: bool,
    /// Some on the verified render path; get_frame then asks the request the plugin makes (twframe::route_request) and
    /// takes the frame keyed by (frame, stored bits) from `by_request`; None keeps the state A rule above (frame_key) and `frames`.
    pub tw_route: Option<crate::twframe::TwRoute>,
    pub by_request: BTreeMap<(i32, u32), Image>,
}

#[derive(Clone, Debug)]
pub struct RunRec {
    pub node: usize,
    pub class: String,
    pub operands: Vec<i32>,
    pub tag: i32,
    pub out_channels: usize,
}

#[derive(Clone, Debug)]
pub struct ExecOutput {
    pub dest: Image,
    /// the image save_prev_dest stored (to hand to the next frame as `HostFrames::prev_dest`)
    pub saved_prev_dest: Option<Image>,
    pub trace: Vec<RunRec>,
    pub diagnostics: Vec<String>,
}

pub fn combo_table() -> ComboTable {
    ComboTable::from_defs()
}

/// the per-writer xy list of one run: (base phase in bytes mod 32, reused from render 1) per writer in execution order, and the render (1 or 2).
pub static XY_LIST: std::sync::Mutex<Option<(Vec<(i32, bool)>, u32)>> = std::sync::Mutex::new(None);
/// the writers of the placement model as the executor uses them: (consumer (gid, slot) list, base, head mode, preserve)
/// per perspective-class node gid, true when its destination is a row buffer in the placement model's plan
pub static PERSP_ROWBUF: std::sync::Mutex<std::collections::BTreeMap<usize, bool>> = std::sync::Mutex::new(std::collections::BTreeMap::new());
pub static XY_WRITERS: std::sync::Mutex<Vec<(Vec<(usize, usize)>, i32, crate::types::XyHeadMode, bool)>> = std::sync::Mutex::new(Vec::new());

/// The head form of one default-xy writer from its placement in the plan: a writer allocated outside every exec block is a plane whose
/// rows take their own address phase (PerRow); a writer allocated in the line list of the block that writes it is one row buffer (OneHead).
/// The single decision of the form, used on the --xy-base-list route and on the --xy-base route alike.
pub fn head_form(place: fx_eval::xyplace::Place) -> crate::types::XyHeadMode {
    match place {
        fx_eval::xyplace::Place::Plane => crate::types::XyHeadMode::PerRow,
        fx_eval::xyplace::Place::RowBuffer => crate::types::XyHeadMode::OneHead,
    }
}

/// Map the run's xy bases onto the writers of the placement model (compile_placed). With --xy-base-list the entries are the bases of the writers in
/// execution order; with --xy-base B and no list the run is the list route at render 1 with B for every writer (a plan with no writer is left
/// unchanged). Head form from the placement (head_form); the perspective out2 and perspective_alpha out1 destinations take their form from the same
/// plan. A preserve writer runs in render 1 only: its render 2 entry must be the render 1 base, marked reused. Each writer keeps its own consumer
/// list and its own base.
/// The observe-only notes a compile keeps for the census labels of a run (if_neq_const decisions, eq_const scan counts, non-constant argument
/// nodes, v3_to_v4_if4 folds) describe the compile of the plan the run executes. The compiles of the placement model on the base route leave them
/// empty again, so the labels of a base route run read as before.
fn drop_placement_notes() {
    let _ = fx_eval::compile::ifneq_trace_take();
    let _ = fx_eval::compile::eq_const_scan_take();
    let _ = fx_eval::compile::ifneq_arg1_nonconst_take();
    let _ = fx_eval::fold::v3_take();
}

fn clear_xy_writers() {
    XY_WRITERS.lock().unwrap().clear();
    PERSP_ROWBUF.lock().unwrap().clear();
}

fn xy_writers_for(program: &Genotype, binding: &Binding) -> Result<(), ExecError> {
    let explicit = XY_LIST.lock().unwrap().clone();
    let (list, render, uniform) = match explicit {
        Some((l, r)) => (l, r, false),
        None if binding.kp.xy_base >= 0 => (Vec::new(), 1u32, true),
        None => {
            clear_xy_writers();
            return Ok(());
        }
    };
    let (_, ws) = fx_eval::xyplace::compile_placed(program, &combo_table(), &binding.compile_options()).map_err(|e| ExecError::BadPlan(format!("compile_placed: {}", e)))?;
    if uniform && ws.is_empty() {
        drop_placement_notes();
        clear_xy_writers();
        return Ok(());
    }
    let list = if uniform { vec![(binding.kp.xy_base, false); ws.len()] } else { list };
    if ws.len() != list.len() {
        return Err(ExecError::BadPlan(format!("XY-BASE-LIST-MISMATCH (the placement model has {} default-xy writers, --xy-base-list gives {})", ws.len(), list.len())));
    }
    let mut out = Vec::new();
    for (w, &(base, reused)) in ws.iter().zip(list.iter()) {
        if render == 2 && w.preserve && !reused {
            return Err(ExecError::BadPlan(format!("XY-BASE-PRESERVE-REUSE-REQUIRED (writer ref{} is inside a preserve group: render 2 reuses its render 1 image, so its entry is the render 1 base marked r1)", w.node)));
        }
        let mode = head_form(w.place);
        out.push((w.consumers.clone(), base, mode, w.preserve));
    }
    *XY_WRITERS.lock().unwrap() = out;
    // the perspective out2 / perspective_alpha out1 destinations take their row phase form from the same plan
    let pp: BTreeMap<usize, bool> = fx_eval::xyplace::persp_places(program, &combo_table(), &binding.compile_options()).map_err(|e| ExecError::BadPlan(format!("persp_places: {}", e)))?.into_iter().collect();
    *PERSP_ROWBUF.lock().unwrap() = pp;
    if uniform {
        drop_placement_notes();
    }
    Ok(())
}

pub fn compile_for(program: &Genotype, binding: &Binding) -> Result<CompiledProgram, ExecError> {
    xy_writers_for(program, binding)?;
    //: with the context carried into CompileOptions, word 2 = 1 is planned by the preserve-group model
    // (fx_eval::preserve); the other native refusals of check_native stay, and the motion_blur and downsample programs are refused by name.
    let w2 = binding.ctx.as_ref().map(|c| c.stack_word2).unwrap_or(false);
    if let Some(c) = &binding.ctx {
        if w2 { c.check_native_preserve() } else { c.check_native() }.map_err(ExecError::Shape)?;
    }
    //: on the verified render path F is a required explicit input; a program that reads get_frame_number
    // with no host value refuses instead of folding zeros. State A callers (frame_index_required false) keep NV3-A7's zeros.
    if binding.frame_index_required && !binding.host.contains_key("get_frame_number") && program.nodes.iter().any(|g| g.name == "get_frame_number") {
        return Err(ExecError::BadPlan("compile: FRAME-INDEX-REQUIRED (ctx: the program reads get_frame_number and the run gives no F)".into()));
    }
    // the frame rate is a per-run host value on the verified render path; the folded default of the minimal context is not used there
    if binding.frame_index_required && !binding.host.contains_key("get_frames_per_second") && program.nodes.iter().any(|g| g.name == "get_frames_per_second") {
        return Err(ExecError::BadPlan("compile: FPS-UNGIVEN (the program reads get_frames_per_second and the run gives no rate; pass --host-value get_frames_per_second=RATE)".into()));
    }
    let cp = compile(program, &combo_table(), &binding.compile_options()).map_err(|e| ExecError::BadPlan(format!("compile: {}", e)))?;
    if w2 {
        if let Some(e) = fx_eval::preserve::model_refusal(&cp) {
            return Err(ExecError::Shape(e));
        }
    }
    Ok(cp)
}

/// The entry point: compile with the binding, then run on the explicit inputs.
pub fn execute(program: &Genotype, binding: &Binding, inputs: &Inputs, host: &HostFrames) -> Result<ExecOutput, ExecError> {
    let cp = compile_for(program, binding)?;
    execute_compiled(&cp, binding, inputs, host)
}

/// Classes of the run list (not folded) that this executor cannot run: the unsupported list of one plan.
pub fn unsupported_classes(cp: &CompiledProgram) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    for nd in &cp.nodes {
        if (nd.kind == "func" || nd.kind == "auto") && !nd.b160 && !nd.folded && !kernels::is_supported(&nd.prim) {
            let c = kernels::normalize_class(&nd.prim).to_string();
            if !v.contains(&c) {
                v.push(c);
            }
        }
        if nd.kind == "special" && nd.name != "xy" && nd.name != "Output" && !v.contains(&nd.name) {
            v.push(format!("special:{}", nd.name));
        }
    }
    v
}

/// "R-PARITY (constant operand binding: the binder calls set_arg_const at
///  (virtual), twins : lane j = src[min(j, count - 1)] for j = 0..3, integer-typed lanes converted by
/// x86 cvttss2si (NaN or out of range gives 0x80000000), image-only arguments refused; 1,624 of 1,624 constructed cases equal in the emulator
/// at counts 1 to 4, float and integer-typed, special values; 4 of 4 mutants killed; comp_premult reads raw lanes;
/// counts 0 and above 4 refused by name; premise CONST-ROUTE-)". Execution time only: the plan dump keeps fold.rs lanes.
/// The lanes stay f32 here; an integer-typed consumer converts at its own site with the x86 model (fx_common::x86), never a saturating cast.
/// Features cst1_raw_cp (no fill) and fill0_mut (lanes n..3 = 0) are the gate's controls, test builds only.
fn const_val(nd: &PlanNode) -> Result<Val, ExecError> {
    // : "CONST-COUNT-NEGATIVE-OFF-CLAIM (plan constants with count -1 in LightWrap, Matte_Ops, Matte_Ops_Comp and
    // Original: the recorded binding kept (n = 1, raw lanes, no fill) so the survey stays comparable; not R-PARITY; outside the 13 claimed styles;
    // refused by name on the verified render path)". The verified render path ('ae32', fx_run) refuses any count below 1 before execution.
    if nd.elnum < 0 {
        return Ok(Val::Const { n: 1, lanes: nd.lanes });
    }
    if nd.elnum == 0 || nd.elnum > 4 {
        return Err(ExecError::Shape(format!("CONST-COUNT-OFF-CONTRACT: constant node {} has count {} (set_arg_const is claimed at counts 1 to 4)", nd.idx, nd.elnum)));
    }
    let n = nd.elnum as usize;
    let mut lanes = nd.lanes;
    if !cfg!(feature = "cst1_raw_cp") {
        for j in n..4 {
            lanes[j] = if cfg!(feature = "fill0_mut") { 0u32 } else { lanes[n - 1] }; // lanes hold f32 bits; 0 is +0.0
        }
    }
    Ok(Val::Const { n, lanes })
}

fn broadcast(a: &Arg, ch: usize, n: usize) -> Result<Image, ExecError> {
    match a {
        Arg::Img(i) => {
            if i.channels() != ch {
                return Err(ExecError::Shape(format!("host value has {} channels, {} needed", i.channels(), ch)));
            }
            Ok((*i).clone())
        }
        Arg::Const { lanes, .. } => Ok(Image::from_const(lanes, ch, n)),
        Arg::Mip(_) => Err(ExecError::Shape("a ripmap object cannot be broadcast as a host value".into())),
    }
}

/// a program with a blur_res popup at Half or Quarter runs its body as the original `exec-downsampled` command does (module `downsampled`):
/// the input planes are halved L times, the body runs on a context of ceil(W/f) x ceil(H/f) pixels whose geometry floats (left, right, assumption_h)
/// are divided by f and whose frame width (blur render scale) is the small width, the root planes are doubled L times. Everything else
/// (convert-from-source before, convert-to-dest after) stays at full resolution in the callers. `kp.ds_off` / env DS1_OFF=1 = pre-DS1.
pub fn execute_compiled(cp: &CompiledProgram, binding: &Binding, inputs: &Inputs, host: &HostFrames) -> Result<ExecOutput, ExecError> {
    crate::backend::ensure_env_backend();
    if crate::backend::required_device_missing() {
        return Err(ExecError::Unsupported { class: "device".into(), reason: "DEVICE-REQUIRED-UNAVAILABLE: SAPPHIRE_REQUIRE_DEVICE=1 and no device was installed".into() });
    }
    let f = fx_eval::cpuplan::ds_factor(cp);
    let r = if f > 1 && !binding.kp.ds_off { execute_downsampled(cp, binding, inputs, host, f) } else { execute_plain(cp, binding, inputs, host) };
    if r.is_ok() {
        crate::backend::note_report();
    }
    r
}

fn scaled_geom(g: &Geom, f: u32) -> Geom {
    let ff = f as f32;
    Geom { left: g.left / ff, right: g.right / ff, aspect: g.aspect, assumption_e: g.assumption_e, assumption_h: g.assumption_h / ff, roi_x: g.roi_x, roi_y: g.roi_y }
}

fn execute_downsampled(cp: &CompiledProgram, binding: &Binding, inputs: &Inputs, host: &HostFrames, f: u32) -> Result<ExecOutput, ExecError> {
    let lv = crate::downsampled::levels_of_factor(f);
    let (w, h) = (inputs.width, inputs.height);
    let (w2, h2) = crate::downsampled::small_dims(w, h, lv);
    // the small plane base mod 32 is a HOST value (MEASURED 0 at 64x32 and 50x7, 16 at 63x33 and 49x9); DS2_PHASE overrides the KParams value for the comparison tools
    let phase = std::env::var("DS2_PHASE").ok().and_then(|v| v.parse::<u32>().ok()).unwrap_or(binding.kp.ds_dst_phase);
    let mut images: BTreeMap<String, Image> = BTreeMap::new();
    for (name, im) in inputs.images.iter() {
        if name == "xy" {
            continue; // the body generates its own xy variable at the small size (`variable 2` runs inside the command)
        }
        if im.n != w * h {
            return Err(ExecError::Shape(format!("input {} has {} samples per plane, frame is {}", name, im.n, w * h)));
        }
        let mut planes = Vec::with_capacity(im.planes.len());
        for p in im.planes.iter() {
            planes.push(crate::downsampled::downsample_plane(p, w, h, lv, phase).0);
        }
        images.insert(name.clone(), Image { n: w2 * h2, planes });
    }
    let mut b2 = binding.clone();
    let g = scaled_geom(&geom_of(&binding.kp, w, h), f);
    b2.kp.geom = Some(g);
    // the body writer (the last writer of the placement model) sets the head of the small-size xy rows; without a writer list the KParams head stands
    let mut kp_body = binding.kp;
    if let Some(wb) = XY_WRITERS.lock().unwrap().last() {
        kp_body.xy_head = ((32 - wb.1 as usize % 32) % 32) as i32 / 4 % 8;
    }
    let xy = ds_xy(&kp_body, w, h, w2, h2, f);
    images.insert("xy".to_string(), xy);
    let hoist = fx_eval::cpuplan::ds_hoist(cp);
    let mut pre: BTreeMap<usize, Val> = BTreeMap::new();
    if !hoist.is_empty() {
        // (FITTED): the hoisted nodes run at FULL size on the full size xy variable; the ones read by the body are downsampled like an input
        let only: std::collections::BTreeSet<usize> = hoist.iter().copied().collect();
        let (_o, vals) = eval_graph(cp, binding, inputs, host, Some(&only), &BTreeMap::new())?;
        for (p, nd) in cp.nodes.iter().enumerate() {
            let read_by_body = cp.nodes.iter().any(|m| (m.kind == "func" || m.kind == "auto") && !m.b160 && !m.folded && !only.contains(&m.idx) && m.args.contains(&nd.idx));
            if only.contains(&nd.idx) && read_by_body {
                if let Some(Val::Img(im)) = vals[p].as_ref() {
                    let mut planes = Vec::with_capacity(im.planes.len());
                    for pl in im.planes.iter() {
                        planes.push(crate::downsampled::downsample_plane(pl, w, h, lv, phase).0);
                    }
                    pre.insert(nd.idx, Val::Img(Rc::new(Image { n: w2 * h2, planes })));
                }
            }
        }
    }
    let small = Inputs { width: w2, height: h2, images };
    // the compiler's tail (the nodes the CPU-driver plan places after the command) leaves the body: the body runs at the small size up to its root,
    // the root is upsampled, then the tail runs at full size on the upsampled root (the plan's order; the small-plane alpha clamp of the tail node runs here)
    let (tail, body_root) = fx_eval::cpuplan::ds_tail_split(cp);
    let skip: std::collections::BTreeSet<usize> = tail.iter().copied().collect();
    let mut out = eval_graph_r(cp, &b2, &small, host, None, &pre, &skip, body_root)?.0;
    let mut planes = Vec::with_capacity(out.dest.planes.len());
    for p in out.dest.planes.iter() {
        planes.push(crate::downsampled::upsample_plane(p, w, h, lv));
    }
    out.dest = Image { n: w * h, planes };
    if let Some(br) = body_root {
        let mut pre_t: BTreeMap<usize, Val> = BTreeMap::new();
        pre_t.insert(br, Val::Img(Rc::new(out.dest.clone())));
        let (tout, _) = eval_graph_r(cp, binding, inputs, host, Some(&skip), &pre_t, &std::collections::BTreeSet::new(), Some(cp.root))?;
        out.dest = tout.dest;
        out.trace.extend(tout.trace);
        out.diagnostics.extend(tout.diagnostics);
    }
    out.diagnostics.push(format!("exec-downsampled factor {} ({} levels), body at {}x{}", f, lv, w2, h2));
    Ok(out)
}

/// the alias flag of the sharpen class from the plan the executor runs: every sharpen run-primitive writes its first source ref, or none does (mixed is an error).
fn sharpen_alias_of(cp: &CompiledProgram) -> Result<bool, ExecError> {
    let plan = fx_eval::cpuplan::plan_text_mode(cp, fx_eval::cpuplan::PlanMode::CpuDriver);
    let mut al: Vec<bool> = Vec::new();
    for l in plan.lines().filter(|l| l.starts_with("CMD") && l.contains("run-primitive")) {
        let t: Vec<&str> = l.split_whitespace().collect();
        if let Some(pos) = t.iter().position(|x| *x == "sharpen") {
            if pos >= 1 && pos + 1 < t.len() {
                al.push(t[pos - 1] == t[pos + 1]);
            }
        }
    }
    if al.iter().any(|x| *x != al[0]) {
        return Err(ExecError::Shape(format!("sharpen: mixed alias flags in the plan {:?}", al)));
    }
    Ok(al.first().copied().unwrap_or(false))
}

fn execute_plain(cp: &CompiledProgram, binding: &Binding, inputs: &Inputs, host: &HostFrames) -> Result<ExecOutput, ExecError> {
    // the values of the nodes are not read after the root: a device backend keeps its intermediates resident and only the root comes back
    Ok(eval_graph_core(cp, binding, inputs, host, None, &BTreeMap::new(), &std::collections::BTreeSet::new(), None)?.0)
}

/// The node loop. `only` = evaluate just these computed nodes (inputs, constants and special variables always); `pre` = values supplied for nodes
/// (the hoisted nodes computed at full size and downsampled). With `only` the root is not read (the returned ExecOutput is empty).
fn eval_graph(cp: &CompiledProgram, binding: &Binding, inputs: &Inputs, host: &HostFrames, only: Option<&std::collections::BTreeSet<usize>>, pre: &BTreeMap<usize, Val>) -> Result<(ExecOutput, Vec<Option<Val>>), ExecError> {
    eval_graph_r(cp, binding, inputs, host, only, pre, &std::collections::BTreeSet::new(), None)
}

/// `skip` = computed nodes that are not evaluated (the tail of the exec-downsampled command, which runs after the upsample); `root_sel` = the node read as
/// the result instead of the program root, also when `only` is given (the body root at the small size, the program root for the tail at full size).
fn eval_graph_r(cp: &CompiledProgram, binding: &Binding, inputs: &Inputs, host: &HostFrames, only: Option<&std::collections::BTreeSet<usize>>, pre: &BTreeMap<usize, Val>, skip: &std::collections::BTreeSet<usize>, root_sel: Option<usize>) -> Result<(ExecOutput, Vec<Option<Val>>), ExecError> {
    let (out, mut vals) = eval_graph_core(cp, binding, inputs, host, only, pre, skip, root_sel)?;
    // callers that read the node values get host images (a device backend's resident values come back here)
    for v in vals.iter_mut() {
        if let Some(Val::Dev(d)) = v {
            let im = d.materialize()?.clone();
            *v = Some(Val::Img(Rc::new(im)));
        } else if let Some(Val::DevMip(d)) = v {
            let m = d.materialize()?.clone();
            *v = Some(Val::Mip(Rc::new(m)));
        }
    }
    Ok((out, vals))
}

fn eval_graph_core(cp: &CompiledProgram, binding: &Binding, inputs: &Inputs, host: &HostFrames, only: Option<&std::collections::BTreeSet<usize>>, pre: &BTreeMap<usize, Val>, skip: &std::collections::BTreeSet<usize>, root_sel: Option<usize>) -> Result<(ExecOutput, Vec<Option<Val>>), ExecError> {
    crate::backend::with_backend(|b| b.begin());
    let n = inputs.width.checked_mul(inputs.height).ok_or(ExecError::Shape("frame size overflow".into()))?;
    let tables = Tables::embedded();
    let mut pos_of: BTreeMap<usize, usize> = BTreeMap::new();
    for (p, nd) in cp.nodes.iter().enumerate() {
        pos_of.insert(nd.idx, p);
    }
    let mut vals: Vec<Option<Val>> = vec![None; cp.nodes.len()];
    let mut trace = Vec::new();
    let mut diagnostics = Vec::new();
    let mut saved: Option<Image> = None;
    // one Val per default-xy writer of the placement model (empty unless --xy-base-list is given; the caller's xy image wins)
    let xyw = XY_WRITERS.lock().unwrap().clone();
    let xyv: Vec<Val> = if inputs.images.contains_key("xy") {
        Vec::new()
    } else {
        xyw.iter()
            .map(|(_, base, mode, _)| {
                let mut kp = binding.kp;
                kp.xy_base = *base;
                kp.xy_head_mode = *mode;
                Val::Img(Rc::new(crate::kernels2::default_xy(&kp, inputs.width, inputs.height)))
            })
            .collect()
    };
    for (p, nd) in cp.nodes.iter().enumerate() {
        if let Some(v) = pre.get(&nd.idx) {
            vals[p] = Some(v.clone());
            continue;
        }
        if let Some(o) = only {
            if (nd.kind == "func" || nd.kind == "auto") && !nd.b160 && !nd.folded && !o.contains(&nd.idx) {
                continue;
            }
        }
        if skip.contains(&nd.idx) {
            continue;
        }
        let v: Val = match nd.kind {
            "input" => {
                let im = inputs.images.get(&nd.name).ok_or_else(|| ExecError::MissingInput(nd.name.clone()))?;
                if im.n != n {
                    return Err(ExecError::Shape(format!("input {} has {} samples per plane, frame is {}", nd.name, im.n, n)));
                }
                if nd.elnum > 0 && im.channels() != nd.elnum as usize {
                    return Err(ExecError::Shape(format!("input {} has {} channels, plan says {}", nd.name, im.channels(), nd.elnum)));
                }
                Val::Img(Rc::new(im.clone()))
            }
            "const" => const_val(nd)?,
            "func" | "auto" if nd.b160 || nd.folded => const_val(nd)?,
            "func" | "auto" => {
                let mut argvals: Vec<Val> = Vec::with_capacity(nd.args.len());
                for (ak, a) in nd.args.iter().enumerate() {
                    let q = *pos_of.get(a).ok_or_else(|| ExecError::BadPlan(format!("node {} argument {} not in plan", nd.idx, a)))?;
                    if q >= p {
                        return Err(ExecError::BadPlan(format!("node {} uses node {} before it is computed", nd.idx, a)));
                    }
                    let v = vals[q].as_ref().ok_or_else(|| ExecError::BadPlan(format!("node {} argument {} has no value", nd.idx, a)))?;
                    let an = &cp.nodes[q];
                    if !xyv.is_empty() && an.kind == "special" && an.name == "xy" {
                        match xyw.iter().position(|w| w.0.contains(&(nd.gid, ak))) {
                            Some(wi) => {
                                argvals.push(xyv[wi].clone());
                                continue;
                            }
                            None => return Err(ExecError::BadPlan(format!("XY-WRITER-UNMAPPED (node {} gid {} slot {} reads xy and no writer of the placement model lists it)", nd.idx, nd.gid, ak))),
                        }
                    }
                    argvals.push(v.clone());
                }
                let c = kernels::normalize_class(&nd.prim).to_string();
                if !kernels::is_supported(&c) {
                    return Err(ExecError::Unsupported { class: c, reason: "no kernel wrapper in fx_exec (builtin not wired or not rebuilt)".into() });
                }
                let operands: Vec<i32> = argvals.iter().map(|a| a.count()).collect();
                let (outval, tag): (Val, i32) = if HOST_CLASSES.contains(&c.as_str()) {
                    let args = crate::backend::host_args(&argvals)?;
                    let (i, t) = run_host(&c, nd.elnum, &args, n, host, &mut saved)?;
                    (Val::Img(Rc::new(i)), t)
                } else {
                    let tag = tables.select_counts(&c, &operands);
                    if tag < 0 {
                        return Err(ExecError::NoCombo { class: c, operands });
                    }
                    // HOST-COUNT = W x H of this evaluation, explicit; alias from the executed plan (CPU driver text)
                    let mut kpe = binding.kp;
                    if kpe.sharpen_count.is_none() {
                        kpe.sharpen_count = Some(n);
                    }
                    if c == "sharpen" {
                        kpe.sharpen_alias = sharpen_alias_of(cp)?;
                    }
                    if let Some(&rb) = PERSP_ROWBUF.lock().unwrap().get(&nd.gid) {
                        if c == "perspective" {
                            kpe.persp_rowbuf = rb;
                        } else if c == "perspective_alpha" {
                            kpe.palpha_rowbuf = rb;
                        }
                    }
                    // the backend seam: a device backend may run the node and keep the result resident; any decline runs this one node on the CPU
                    let call = crate::backend::NodeCall { idx: nd.idx, class: &c, tag, n, dims: (inputs.width, inputs.height), kp: &kpe };
                    let v = match crate::backend::with_backend(|b| b.try_run(&call, &argvals)) {
                        Some(Ok(v)) => v,
                        _ => {
                            let args = crate::backend::host_args(&argvals)?;
                            match kernels::run_any(&c, tag, &args, n, (inputs.width, inputs.height), &kpe)? {
                                Out::Img(i) => Val::Img(Rc::new(i)),
                                Out::Mip(m) => Val::Mip(Rc::new(m)),
                            }
                        }
                    };
                    (v, tag)
                };
                // a make_mipmap node whose consumers all read it linewise has the plain image format: its value is the base image
                let outval = match outval {
                    Val::Mip(m) if c == "make_mipmap" && nd.idx != cp.root && fx_eval::cpuplan::node_format(cp, nd.idx) == 2 => Val::Img(Rc::new(m.base.clone())),
                    Val::DevMip(d) if c == "make_mipmap" && nd.idx != cp.root && fx_eval::cpuplan::node_format(cp, nd.idx) == 2 => Val::Img(Rc::new(d.materialize()?.base.clone())),
                    other => other,
                };
                let out_channels = match &outval {
                    Val::Img(i) => i.channels(),
                    Val::Mip(m) => m.channels,
                    Val::Dev(d) => d.channels(),
                    Val::DevMip(d) => d.channels,
                    Val::Const { n, .. } => *n,
                };
                if nd.elnum > 0 && out_channels != nd.elnum as usize {
                    diagnostics.push(format!("node {} {}: output has {} channels, the compiler's elnum is {}", nd.idx, c, out_channels, nd.elnum));
                }
                trace.push(RunRec { node: nd.idx, class: c, operands, tag, out_channels });
                outval
            }
            "special" if nd.name == "xy" => {
                // NV9-A3: the xy variable (2 channel coordinate plane): the caller's `xy` image, else the grid of the host geometry
                let im = match inputs.images.get("xy") {
                    Some(i) => i.clone(),
                    None => crate::kernels2::default_xy(&binding.kp, inputs.width, inputs.height),
                };
                if im.n != n || im.channels() != 2 {
                    return Err(ExecError::Shape(format!("xy has {} channels x {} samples, needed 2 x {}", im.channels(), im.n, n)));
                }
                Val::Img(Rc::new(im))
            }
            "special" if nd.name == "Output" => {
                // the destination type donor: a constant operand of -999 in each of the destination's channels (the plan text prints
                // the same lanes); classes read only its lane count
                Val::Const { n: nd.elnum.clamp(1, 4) as usize, lanes: [(-999.0f32).to_bits(); 4] }
            }
            other => return Err(ExecError::Unsupported { class: format!("special:{}", nd.name), reason: format!("plan node kind {}", other) }),
        };
        vals[p] = Some(v);
    }
    if only.is_some() && root_sel.is_none() {
        return Ok((ExecOutput { dest: Image { n: 0, planes: vec![] }, saved_prev_dest: saved, trace, diagnostics }, vals));
    }
    let rp = *pos_of.get(&root_sel.unwrap_or(cp.root)).ok_or_else(|| ExecError::BadPlan("root not in plan".into()))?;
    let root = vals[rp].as_ref().ok_or_else(|| ExecError::BadPlan("root has no value".into()))?;
    let dest = match root {
        Val::Img(i) => (**i).clone(),
        Val::Const { lanes, n: ln } => Image::from_const(lanes, (*ln).max(1), n),
        Val::Mip(_) | Val::DevMip(_) => return Err(ExecError::Shape("the destination is a ripmap object".into())),
        Val::Dev(d) => d.materialize()?.clone(),
    };
    Ok((ExecOutput { dest, saved_prev_dest: saved, trace, diagnostics }, vals))
}

/// Frame rule (MEASURED on the original): absolute frame floor(max(0, current + t)), current frame 0; NaN and negative read frame 0.
pub fn frame_key(t: f32) -> i32 {
    // / M1: : vxorps xmm1; vmaxss xmm1,xmm1,t (= if 0 > t {0} else {t}: NaN stays NaN); vroundss mode 1 (floor); vcvtss2si (NaN or out of range gives 0x80000000)
    let c = if 0.0 > t { 0.0 } else { t };
    let f = c.floor();
    if f.is_nan() || f >= 2147483648.0 || f < -2147483648.0 {
        i32::MIN
    } else {
        f as i32
    }
}

/// HOST-FRAMES classes. get_frame: the first constant argument is the frame offset (NV5-A6). get_prev_dest: the host
/// previous destination, else its argument (INFERRED: the fallback value). save_prev_dest: identity that records.
pub fn run_host(class: &str, elnum: i32, args: &[Arg], n: usize, host: &HostFrames, saved: &mut Option<Image>) -> Result<(Image, i32), ExecError> {
    let ch = if elnum > 0 { elnum as usize } else { 4 };
    match class {
        "get_frame" => {
            let off = args
                .iter()
                .find_map(|a| if let Arg::Const { lanes, .. } = a { Some(f32::from_bits(lanes[0])) } else { None })
                .ok_or_else(|| ExecError::BadPlan("get_frame without a constant frame offset".into()))?;
            if let Some(q) = &host.tw_route {
                let r = crate::twframe::route_request(q, off).map_err(ExecError::Shape)?;
                // REQUEST-LIST: one line per request, compared with the record's host_log arguments
                crate::fxrun::elog!("HOST-REQUEST\tframe={}\tstored={:08x}\targ={}", r.frame, r.stored.to_bits(), r.arg);
                let im = host.by_request.get(&(r.frame, r.stored.to_bits())).ok_or_else(|| {
                    ExecError::MissingHostFrame(format!("get_frame request frame {} stored {} (bits {:08x}) argument {}", r.frame, r.stored, r.stored.to_bits(), r.arg))
                })?;
                if im.n != n {
                    return Err(ExecError::Shape(format!("host frame {},{} has {} samples per plane, frame is {}", r.frame, r.stored, im.n, n)));
                }
                return Ok((im.clone(), im.channels() as i32));
            }
            let key = if host.frame_round_old { off.round() as i32 } else { frame_key(off) };
            if std::env::var("TW2_LOGFRAMES").is_ok() { crate::fxrun::elog!("NATIVE_FETCH {}", key); }
            let im = host.frames.get(&key).ok_or_else(|| ExecError::MissingHostFrame(format!("get_frame offset {}", key)))?;
            if im.n != n {
                return Err(ExecError::Shape(format!("host frame {} has {} samples per plane, frame is {}", key, im.n, n)));
            }
            Ok((im.clone(), im.channels() as i32))
        }
        "get_prev_dest" => {
            let tag = args.first().map(|a| a.count()).unwrap_or(ch as i32);
            let tag = if tag == 1 { 3 } else { tag };
            match &host.prev_dest {
                Some(im) => {
                    if im.channels() as i32 != tag || im.n != n {
                        return Err(ExecError::Shape(format!("prev_dest is {} channels x {}, needed {} x {}", im.channels(), im.n, tag, n)));
                    }
                    Ok((im.clone(), tag))
                }
                None => {
                    let a = args.first().ok_or_else(|| ExecError::MissingHostFrame("prev_dest".into()))?;
                    Ok((broadcast(a, tag as usize, n)?, tag))
                }
            }
        }
        _ => {
            let a = args.first().ok_or_else(|| ExecError::BadPlan("save_prev_dest without an argument".into()))?;
            let im = broadcast(a, if let Arg::Img(i) = a { i.channels() } else { ch }, n)?;
            *saved = Some(im.clone());
            let c = im.channels() as i32;
            Ok((im, c))
        }
    }
}

/// the whole effect on HOST MEMORY. `hc.src[0]` is the host source (format 16, B,G,R,A f32, cell 1): it goes through the native
/// convert-from-source (per row) and becomes the input named `source_name`; the root image goes through the native convert-to-dest
/// with `hc.mode`. Other inputs (masks, xy) are passed as planes in `other`. Returns the plane-level output and the host destination.
pub struct HostExecOutput {
    pub exec: ExecOutput,
    /// host destination memory (height * width * 4 floats, B,G,R,A); equals `dest_prefill` when the writer does not write (mode 0)
    pub host_dest: Vec<f32>,
    pub wrote: bool,
    /// the planes the native convert-from-source produced (premultiplied, non-finite samples removed)
    pub source_planes: Image,
}

pub fn execute_host(
    program: &Genotype,
    binding: &Binding,
    hc: &crate::hostconv::HostConvert,
    source_name: &str,
    other: &BTreeMap<String, Image>,
    width: usize,
    height: usize,
    host: &HostFrames,
) -> Result<HostExecOutput, ExecError> {
    hc.check_claim()?;
    if hc.ae32.is_some() {
        // the 'ae32' route never falls back to the format-16 source; its whole-effect entry is `execute_ae32`
        return Err(ExecError::Shape("the 'ae32' route runs through execute_ae32 (u32 host words in and out), never the format-16 host source".into()));
    }
    let src0 = hc.src.first().ok_or_else(|| ExecError::MissingInput("host source 0".into()))?;
    let planes = crate::hostconv::host_to_image(src0, width, height)?;
    let mut images = other.clone();
    images.insert(source_name.to_string(), planes.clone());
    let inputs = Inputs { width, height, images };
    let exec = execute(program, binding, &inputs, host)?;
    // (ported by, merge): a 3 plane destination (Time_Warp_RGB): turns mode 1 into mode 3 for an odd plane count; the 3 channel writer arm
    // (modes 0, 2, 3) never reads the fourth plane, so a zero plane stands in for it
    let (dest_in, mode_in) = crate::hostconv::dest_for_host(&exec.dest, hc.mode)?;
    let (host_dest, wrote) = crate::hostconv::image_to_host(&dest_in, width, height, mode_in, &hc.dest_prefill)?;
    Ok(HostExecOutput { exec, host_dest, wrote, source_planes: planes })
}

/// the whole effect on the 'ae32' route (format 14 in, code 14 out). `source_names[i]` is the plan input that 'ae32' source i
/// becomes (convert-from-source i through the recorded or derived reader head, `hostconv14::ae32_to_image`); `other` holds the remaining
/// inputs as planes; the root image goes through `hostconv14::ae32_dest` with the route's destination record (DEST-RECORD-REQUIRED).
pub struct Ae32ExecOutput {
    pub exec: ExecOutput,
    /// the host destination words, A,R,G,B u32 per pixel (4 * width * height)
    pub dest_words: Vec<u32>,
    /// the planes each source reader produced, by source index
    pub source_planes: Vec<Image>,
}

pub fn execute_ae32(
    cp: &CompiledProgram,
    binding: &Binding,
    route: &crate::hostconv14::Ae32Route,
    source_names: &[String],
    other: &BTreeMap<String, Image>,
    width: usize,
    height: usize,
    host: &HostFrames,
) -> Result<Ae32ExecOutput, ExecError> {
    if source_names.len() != route.sources.len() {
        return Err(ExecError::Shape(format!("'ae32': {} source names for {} sources", source_names.len(), route.sources.len())));
    }
    let mut images = other.clone();
    let mut source_planes = Vec::new();
    for (nm, s) in source_names.iter().zip(route.sources.iter()) {
        // a class-3 input (input node of elnum 3) whose record head is the variant-B 4-plane head gets its first three lanes
        let im = if binding.input_elnum.get(nm) == Some(&3) && s.planes == 4 && s.reader_head == crate::hostconv14::VB4_HEAD {
            crate::hostconv14::class3_vb4_lanes(s, width, height)?
        } else {
            crate::hostconv14::ae32_to_image(s, width, height)?
        };
        if images.insert(nm.clone(), im.clone()).is_some() {
            return Err(ExecError::Shape(format!("'ae32': input {} given both as a host source and as planes", nm)));
        }
        source_planes.push(im);
    }
    let inputs = Inputs { width, height, images };
    //: the alpha a copy-alpha postdivide reads is the 1-plane source conversion of the same host words (
    // record: channel 0 of the host data, the pixel alpha). A source whose 1-plane conversion fails gives no plane (the arm then refuses by name).
    let alphas: Vec<Vec<f32>> = route
        .sources
        .iter()
        .map(|s| {
            let one = crate::hostconv14::Ae32Source { words: s.words.clone(), reader_head: crate::hostconv14::R1_HEAD, planes: 1 };
            crate::hostconv14::ae32_to_image(&one, width, height).ok().and_then(|im| im.planes.into_iter().next()).unwrap_or_default()
        })
        .collect();
    crate::kernels2::set_copy_alpha_planes(Some(alphas));
    let res = execute_compiled(cp, binding, &inputs, host);
    crate::kernels2::set_copy_alpha_planes(None);
    let exec = res?;
    let dest_words = crate::hostconv14::ae32_dest(route, &exec.dest, width, height)?;
    Ok(Ae32ExecOutput { exec, dest_words, source_planes })
}

/// (outcome A): the Source-independent constant route. The plan has no input node (srcindep::route_check);
/// the host Source world is given and its planes are not read; the constant lanes go through the destination convert as on the ordinary route.
/// Mutant (build-time env SIP1_MUT=1): render 0 instead of the constant.
pub fn execute_ae32_const(
    cp: &CompiledProgram,
    binding: &Binding,
    route: &crate::hostconv14::Ae32Route,
    other: &BTreeMap<String, Image>,
    width: usize,
    height: usize,
    host: &HostFrames,
) -> Result<Ae32ExecOutput, ExecError> {
    let inputs = Inputs { width, height, images: other.clone() };
    let mut exec = execute_compiled(cp, binding, &inputs, host)?;
    if option_env!("SIP1_MUT") == Some("1") {
        for p in exec.dest.planes.iter_mut() {
            for v in p.iter_mut() {
                *v = 0.0;
            }
        }
    }
    let dest_words = crate::hostconv14::ae32_dest(route, &exec.dest, width, height)?;
    Ok(Ae32ExecOutput { exec, dest_words, source_planes: Vec::new() })
}

/// the `variable 2` xy plane inside the exec-downsampled body, from the variable record the original builds (no longer fitted).
/// STATIC: the record is filled by (AVX2 body) from the call record geometry floats rec,
/// which the exec-downsampled execute has divided by f (MEASURED: -0.25, 31.75, -8.25, 23.75 at Half 64x32):
///   xstep = 2 / (assumption_g - assumption_f) (vdivss : 2.0 from)
///   x0 = fma(0 - assumption_f, xstep, -1) (vfmsub213ss; the region origin 0 comes from the output)
///   ystep = 2 / (geom33_a - assumption_h) (vdivss)
///   y0 = -(fma(assumption_e, ystep, 1)) - assumption_h*ystep, one rounding (vfmadd213ss, vfnmsub213ss)
/// and the `variable` kernel (AVX2 body) writes x_k = x0 + k*xstep (accumulating, `x_ramp`) and y_row = fma(-row, ystep, -y0)
/// (vfnmsub213ss). MEASURED: the record fields (y0, x0, xstep, ystep) at the kernel entry are
/// -0.484375, -0.984375, 0.0625, 0.0625 at Half 64x32 (orig_nodes/N_*B01*.json, `var`). `geom33_a` = t + u with the same t, u as assumption_h = t - u
/// (, Geom::dump); a caller-supplied geometry has no geom33_a here and keeps ystep = xstep. DS2_XY=old restores the fitted form
/// (x_k = s + k*(f*d) with d = 2/W of the full frame) for the differential test.
fn ds_xy(kp: &KParams, w: usize, h: usize, w2: usize, h2: usize, f: u32) -> Image {
    let g = geom_of(kp, w, h);
    let ff = f as f32;
    let (xs, x0, ys, y0);
    if std::env::var("DS2_XY").map(|v| v == "old").unwrap_or(false) {
        let wf = w as f32;
        let d = 2.0f32 / wf;
        let dd = d * ff;
        xs = dd;
        x0 = -1.0f32 + 1.0f32 / wf;
        ys = dd;
        y0 = (g.assumption_h / ff).mul_add(dd, 1.0);
        return ds_xy_build(kp, w2, h2, xs, x0, ys, -y0);
    }
    let (assumption_f, assumption_g, assumption_h) = (g.left / ff, g.right / ff, g.assumption_h / ff);
    xs = 2.0f32 / (assumption_g - assumption_f);
    x0 = (0.0f32 - assumption_f).mul_add(xs, -1.0);
    if kp.geom.is_none() && kp.geom_mode == crate::types::GeomMode::Dump {
        let a = kp.host_aspect;
        let t = 0.5f32.mul_add(h as f32, -0.5);
        let u = (g.right - g.left) / (a + a);
        let geom33_a = (t + u) / ff;
        ys = 2.0f32 / (geom33_a - assumption_h);
    } else {
        ys = xs;
    }
    let one = g.assumption_e.mul_add(ys, 1.0);
    y0 = (-(assumption_h)).mul_add(ys, -one);
    ds_xy_build(kp, w2, h2, xs, x0, ys, y0)
}

fn ds_xy_build(kp: &KParams, w2: usize, h2: usize, xs: f32, x0: f32, ys: f32, y0rec: f32) -> Image {
    let row = crate::types::x_ramp(w2, kp.xy_head.max(0) as usize, x0, xs);
    let mut px = Vec::with_capacity(w2 * h2);
    let mut py = Vec::with_capacity(w2 * h2);
    for y in 0..h2 {
        px.extend_from_slice(&row);
        let yv = (-(y as f32)).mul_add(ys, -y0rec);
        py.extend(std::iter::repeat(yv).take(w2));
    }
    Image { n: w2 * h2, planes: vec![px, py] }
}
