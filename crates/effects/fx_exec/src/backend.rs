//! The execution backend seam: where a plan node's kernel runs.
//!
//! `exec.rs` asks the installed [`Backend`] to run each kernel node. The CPU backend declines every node
//! ([`Fallback::CpuBackend`]) and the executor runs `kernels::run_any`, the reference code, unchanged: with no backend
//! installed, or with the CPU one, the executor behaves exactly as before the seam existed.
//!
//! The device backend ([`DeviceBackend`]) keeps images resident (`Val::Dev`) and runs a node on the device only when
//! all of these hold, else it declines and the executor runs that one node on the CPU with the same arguments:
//!
//! * the op is on the allowlist (`device_ops::ALLOWLIST`) and its kernel passed the self-check against the CPU kernel
//!   when the backend was built;
//! * the op's own `accepts` test holds for this combination tag, operand shapes and parameters;
//! * the device did not report an error and the memory budget held;
//! * the float output holds no NaN (checked on the device, so the output stays resident), because devices may write other
//!   NaN payloads than the CPU. The exception is an op on `device_ops::NAN_EXACT` whose self-check on this device reported
//!   equal NaN payloads and signs (`NanPayloads::Same`): its kernels write every NaN by explicit tests, so its NaN output is
//!   kept on the device. A NaN-exact op whose payloads differ on the device in use keeps the CPU rerun.
//!
//! A backend is installed per thread ([`install`]); the selection function for callers is
//! [`for_device`] (from `fx_device::select`) and `fx_device::select` itself.

use crate::device_ops::{self, DevArg, DeviceOp};
use crate::types::{Arg, DevVal, ExecError, Image, KParams, Val};
use fx_device::selfcheck::NanPayloads;
use fx_device::{Backend as DeviceHandle, DevError, DevImage, DeviceCtx};
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::rc::Rc;
use std::sync::Arc;

/// One kernel call as the executor sees it.
#[derive(Clone, Copy, Debug)]
pub struct NodeCall<'a> {
    pub idx: usize,
    pub class: &'a str,
    pub tag: i32,
    pub n: usize,
    pub dims: (usize, usize),
    pub kp: &'a KParams,
}

/// Why a backend did not run a node; the executor then runs it on the CPU.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Fallback {
    /// The CPU backend: not a fallback, the normal path.
    CpuBackend,
    /// The op has no device kernel (a recorded cpu_only op).
    NoKernel,
    /// The op has a kernel but is not on the allowlist, or its self-check failed.
    NotAllowlisted,
    /// The kernel does not cover this tag, operand shape or parameter.
    Unsupported(String),
    /// The device budget would be exceeded.
    Memory,
    /// The driver reported an error.
    Cl(String),
    /// A float output holds a NaN.
    NanOutput,
}

impl Fallback {
    /// The short key the report counts under.
    pub fn key(&self) -> &'static str {
        match self {
            Fallback::CpuBackend => "cpu",
            Fallback::NoKernel => "no_kernel",
            Fallback::NotAllowlisted => "not_allowlisted",
            Fallback::Unsupported(_) => "unsupported",
            Fallback::Memory => "memory",
            Fallback::Cl(_) => "opencl_error",
            Fallback::NanOutput => "nan_output",
        }
    }
}

/// Counts per class of what ran where.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Report {
    /// class to nodes run on the device
    pub device: BTreeMap<String, u64>,
    /// class to fallback key to nodes
    pub fallback: BTreeMap<String, BTreeMap<&'static str, u64>>,
    /// ops the self-check refused at build time, with the first mismatch
    pub refused: BTreeMap<String, String>,
}

impl Report {
    pub fn device_nodes(&self) -> u64 {
        self.device.values().sum()
    }
    pub fn fallback_nodes(&self) -> u64 {
        self.fallback.values().flat_map(|m| m.values()).sum()
    }
    /// One line: `device: copy 3, one_minus 1; fallback: warp_rel 2 (no_kernel 2)`.
    pub fn line(&self) -> String {
        let dev: Vec<String> = self.device.iter().map(|(c, n)| format!("{c} {n}")).collect();
        let fb: Vec<String> = self
            .fallback
            .iter()
            .map(|(c, m)| format!("{c} {} ({})", m.values().sum::<u64>(), m.iter().map(|(k, n)| format!("{k} {n}")).collect::<Vec<_>>().join(", ")))
            .collect();
        format!("device: {}; fallback: {}", if dev.is_empty() { "none".into() } else { dev.join(", ") }, if fb.is_empty() { "none".into() } else { fb.join(", ") })
    }
}

pub trait Backend {
    fn name(&self) -> String;
    /// Runs the node and returns its value, or says why not. `args` are the operand values in plan order.
    fn try_run(&mut self, call: &NodeCall, args: &[Val]) -> Result<Val, Fallback>;
    /// Brings a device value to the host so the CPU can read it.
    fn materialize(&mut self, v: &Val) -> Result<(), ExecError> {
        match v {
            Val::Dev(d) => {
                d.materialize()?;
            }
            Val::DevMip(d) => {
                d.materialize()?;
            }
            _ => {}
        }
        Ok(())
    }
    fn report(&self) -> Report {
        Report::default()
    }
    /// Called when a plan evaluation starts: drops per plan caches (uploaded copies of host images).
    fn begin(&mut self) {}
}

/// The reference backend: declines every node, so the executor's own kernel call runs.
pub struct CpuBackend;

impl Backend for CpuBackend {
    fn name(&self) -> String {
        "cpu".into()
    }
    fn try_run(&mut self, _call: &NodeCall, _args: &[Val]) -> Result<Val, Fallback> {
        Err(Fallback::CpuBackend)
    }
}

/// The device backend over one opened device.
pub struct DeviceBackend {
    ctx: Arc<DeviceCtx>,
    /// classes admitted: allowlisted and self-checked
    admitted: BTreeMap<&'static str, &'static DeviceOp>,
    /// admitted classes whose NaN outputs stay on the device: on `NAN_EXACT` and self-checked with equal NaN payloads
    nan_exact: BTreeSet<&'static str>,
    /// uploads memoised by the host image's address (the Rc is kept so the address stays valid)
    uploads: HashMap<*const Image, (Rc<Image>, Rc<DevVal>)>,
    report: Report,
}

impl DeviceBackend {
    /// Every allowlisted op, each admitted only if its kernel passes the self-check against the CPU kernel.
    pub fn new(ctx: Arc<DeviceCtx>) -> DeviceBackend {
        DeviceBackend::with_allowlist(ctx, device_ops::ALLOWLIST)
    }

    /// Only the named classes (a subset of the table), each self-checked. A name with no kernel is ignored.
    /// An op on `NAN_EXACT` keeps its NaN outputs on the device only when its self-check (run now, or recorded in the self-check
    /// cache with the NaN payload field) reported equal NaN payloads.
    pub fn with_allowlist(ctx: Arc<DeviceCtx>, allow: &[&str]) -> DeviceBackend {
        DeviceBackend::admit(ctx, allow, true)
    }

    /// The admission of [`with_allowlist`](Self::with_allowlist); `use_cache` false runs every self-check and records nothing
    /// (a session built from another program text than the one the cache key names).
    fn admit(ctx: Arc<DeviceCtx>, allow: &[&str], use_cache: bool) -> DeviceBackend {
        let mut admitted = BTreeMap::new();
        let mut nan_exact = BTreeSet::new();
        let mut report = Report::default();
        let allow: BTreeSet<&str> = allow.iter().copied().collect();
        for op in device_ops::DEVICE_OPS {
            if !allow.contains(op.class) {
                continue;
            }
            let cached = if use_cache { fx_device::checkcache::passed_nan(&ctx.info, op.class) } else { None };
            let nan_same = match cached {
                Some(same) => same,
                None => match device_ops::selfcheck(&ctx.session, op) {
                    Ok(t) => {
                        let same = t.nan_payloads == NanPayloads::Same;
                        if use_cache {
                            fx_device::checkcache::record_result(&ctx.info, op.class, same);
                        }
                        same
                    }
                    Err(m) => {
                        report.refused.insert(op.class.to_string(), m);
                        continue;
                    }
                },
            };
            admitted.insert(op.class, op);
            if device_ops::keeps_nan_outputs(op.class, nan_same) {
                nan_exact.insert(op.class);
            }
        }
        DeviceBackend { ctx, admitted, nan_exact, uploads: HashMap::new(), report }
    }

    pub fn admitted(&self) -> Vec<&'static str> {
        self.admitted.keys().copied().collect()
    }

    /// The admitted ops whose NaN outputs stay on the device (on `NAN_EXACT`, self-checked with equal NaN payloads).
    pub fn nan_exact(&self) -> Vec<&'static str> {
        self.nan_exact.iter().copied().collect()
    }

    fn resident(&mut self, v: &Val) -> Result<Rc<DevVal>, Fallback> {
        match v {
            Val::Dev(d) => Ok(d.clone()),
            Val::Img(im) => {
                let key = Rc::as_ptr(im);
                if let Some((_, d)) = self.uploads.get(&key) {
                    return Ok(d.clone());
                }
                let planes: Vec<&[f32]> = im.planes.iter().map(|p| p.as_slice()).collect();
                let dev = DevImage::upload(&self.ctx.session, &planes).map_err(map_err)?;
                let d = Rc::new(DevVal::new(dev));
                self.uploads.insert(key, (im.clone(), d.clone()));
                Ok(d)
            }
            _ => Err(Fallback::Unsupported("operand is not an image".into())),
        }
    }

    fn count(&mut self, class: &str, which: Result<(), Fallback>) {
        match which {
            Ok(()) => *self.report.device.entry(class.to_string()).or_default() += 1,
            Err(f) => *self.report.fallback.entry(class.to_string()).or_default().entry(f.key()).or_default() += 1,
        }
    }
}

fn map_err(e: DevError) -> Fallback {
    match e {
        DevError::Memory { .. } => Fallback::Memory,
        DevError::NanOutput => Fallback::NanOutput,
        DevError::Unsupported(s) => Fallback::Unsupported(s),
        DevError::Cl(c) => Fallback::Cl(c.to_string()),
    }
}

impl Backend for DeviceBackend {
    fn name(&self) -> String {
        format!("device {}", self.ctx.info.display_name())
    }

    fn try_run(&mut self, call: &NodeCall, args: &[Val]) -> Result<Val, Fallback> {
        let r = self.run_inner(call, args);
        self.count(call.class, r.as_ref().map(|_| ()).map_err(|e| e.clone()));
        r
    }

    fn report(&self) -> Report {
        self.report.clone()
    }

    fn begin(&mut self) {
        self.uploads.clear();
    }
}

impl DeviceBackend {
    fn run_inner(&mut self, call: &NodeCall, args: &[Val]) -> Result<Val, Fallback> {
        let Some(op) = self.admitted.get(call.class).copied() else {
            return Err(if device_ops::DEVICE_OPS.iter().any(|o| o.class == call.class) { Fallback::NotAllowlisted } else { Fallback::NoKernel });
        };
        let shapes: Vec<device_ops::ArgShape> = args
            .iter()
            .map(|a| match a {
                Val::Img(i) => device_ops::ArgShape::Img { channels: i.channels() },
                Val::Dev(d) => device_ops::ArgShape::Img { channels: d.channels() },
                Val::Const { n, .. } => device_ops::ArgShape::Const { n: *n },
                Val::Mip(_) | Val::DevMip(_) => device_ops::ArgShape::Mip,
            })
            .collect();
        (op.accepts)(call, &shapes).map_err(|why| Fallback::Unsupported(why.to_string()))?;
        let mut held: Vec<Option<Rc<DevVal>>> = Vec::with_capacity(args.len());
        for a in args {
            held.push(match a {
                Val::Img(_) | Val::Dev(_) => Some(self.resident(a)?),
                _ => None,
            });
        }
        let dargs: Vec<DevArg> = args
            .iter()
            .zip(&held)
            .map(|(a, h)| match (a, h) {
                (Val::Const { n, lanes }, _) => DevArg::Const { n: *n, lanes: *lanes },
                (Val::Mip(m), _) => DevArg::Mip(m.as_ref()),
                (Val::DevMip(m), _) => DevArg::DevMip(m.as_ref()),
                (_, Some(d)) => DevArg::Img(&d.dev),
                _ => unreachable!("an operand that is neither an image nor a constant was refused by accepts"),
            })
            .collect();
        let out = (op.run)(&self.ctx.session, call, &dargs).map_err(map_err)?;
        if !self.nan_exact.contains(call.class) && out.has_nan().map_err(map_err)? {
            return Err(Fallback::NanOutput);
        }
        if call.class == "make_mipmap" {
            // the pyramid stays resident as a ripmap object for the device samplers; a CPU consumer brings it to the host
            let rip = crate::device_ops_ripmap::wrap_ripmap(call, &dargs, out).map_err(map_err)?;
            return Ok(Val::DevMip(Rc::new(rip)));
        }
        Ok(Val::Dev(Rc::new(DevVal::new(out))))
    }
}

/// A boxed backend for a selected handle: the device backend for `Backend::Device`, else the CPU backend.
pub fn for_device(handle: &DeviceHandle) -> Box<dyn Backend> {
    match handle {
        DeviceHandle::Device(ctx) => Box::new(DeviceBackend::new(ctx.clone())),
        _ => Box::new(CpuBackend),
    }
}

/// `fx_device::select(kind)` and its backend in one call: the entry point for `--device`. The handle is returned too so the caller
/// can print `describe()` for a refusal. The notices this selection owes (see [`queue_notices`]) are printed at the start of the next
/// execution, on the thread that renders: selection can run before the caller's log is installed and while the caller holds the
/// standard stream locks.
pub fn select(kind: fx_device::DeviceKind) -> (Box<dyn Backend>, DeviceHandle) {
    let h = fx_device::select(kind);
    queue_notices(kind, &h);
    (for_device(&h), h)
}

thread_local! {
    static PENDING: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
    static REQUIRED_MISSING: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

fn env_is_one(name: &str) -> bool {
    std::env::var(name).as_deref() == Ok("1")
}

/// Queues the `device: ` lines for a selection:
///
/// * an explicit `gpu` or `gpu=N` that ends on the CPU says why and that the render is on the CPU;
/// * `auto` says the same only when `SAPPHIRE_DEVICE_REPORT=1`, which also names the device that was opened;
/// * with `SAPPHIRE_REQUIRE_DEVICE=1`, any request other than `cpu` that installs no device refuses every execution on the thread (`DEVICE-REQUIRED-UNAVAILABLE`, status 3 through `sapphire stage apply`).
fn queue_notices(kind: fx_device::DeviceKind, h: &DeviceHandle) {
    if kind == fx_device::DeviceKind::Cpu {
        return;
    }
    if h.is_device() {
        if env_is_one("SAPPHIRE_DEVICE_REPORT") {
            PENDING.with(|p| p.borrow_mut().push(h.describe()));
        }
        return;
    }
    let why = match h {
        DeviceHandle::Refused { reasons, .. } => reasons.join("; "),
        _ => "no usable OpenCL device".to_string(),
    };
    if env_is_one("SAPPHIRE_REQUIRE_DEVICE") {
        REQUIRED_MISSING.with(|c| c.set(true));
        PENDING.with(|p| p.borrow_mut().push(format!("device: DEVICE-REQUIRED-UNAVAILABLE: --device {kind}: {why}")));
        return;
    }
    if kind != fx_device::DeviceKind::Auto || env_is_one("SAPPHIRE_DEVICE_REPORT") {
        PENDING.with(|p| p.borrow_mut().push(format!("device: {why}; rendering on the CPU")));
    }
}

/// Prints the queued notices through the executor's log, the same path as every other line of a run. A missing required device is not
/// fatal here: [`required_device_missing`] tells the executor to refuse the run, which then ends with the refusal status and the
/// label on the log.
fn flush_notices() {
    let lines: Vec<String> = PENDING.with(|p| std::mem::take(&mut *p.borrow_mut()));
    for l in &lines {
        crate::fxrun::elog!("{}", l);
    }
}

/// True once a selection on this thread found `SAPPHIRE_REQUIRE_DEVICE=1` and no device: every execution on the thread is refused with
/// `DEVICE-REQUIRED-UNAVAILABLE`.
pub fn required_device_missing() -> bool {
    REQUIRED_MISSING.with(|c| c.get())
}

thread_local! {
    static CURRENT: RefCell<Option<Box<dyn Backend>>> = const { RefCell::new(None) };
}

/// Restores the previous backend of the thread when dropped; `report` takes the installed backend's counts first.
pub struct Installed {
    prev: Option<Box<dyn Backend>>,
}

impl Drop for Installed {
    fn drop(&mut self) {
        CURRENT.with(|c| *c.borrow_mut() = self.prev.take());
    }
}

/// Installs `b` as this thread's backend until the guard drops.
pub fn install(b: Box<dyn Backend>) -> Installed {
    let prev = CURRENT.with(|c| c.borrow_mut().replace(b));
    Installed { prev }
}

/// The report of the installed backend (empty when none is installed).
pub fn current_report() -> Report {
    CURRENT.with(|c| c.borrow().as_ref().map(|b| b.report()).unwrap_or_default())
}

thread_local! {
    static ENV_CHECKED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// The environment hook: when no backend is installed on this thread and `SAPPHIRE_DEVICE` names a device (`gpu`, `gpu=N` or
/// `auto`), selects it once and installs it for the thread. Unset or `cpu` changes nothing. An explicit `install` always wins.
/// `SAPPHIRE_DEVICE_REPORT=1` prints the placement counts after each top level execution ([`note_report`]).
pub fn ensure_env_backend() {
    flush_notices();
    if ENV_CHECKED.with(|c| c.replace(true)) {
        return;
    }
    if CURRENT.with(|c| c.borrow().is_some()) {
        return;
    }
    let Some(kind) = std::env::var("SAPPHIRE_DEVICE").ok().and_then(|v| fx_device::DeviceKind::parse(&v)) else { return };
    if kind == fx_device::DeviceKind::Cpu {
        return;
    }
    let (b, _handle) = select(kind);
    flush_notices();
    CURRENT.with(|c| *c.borrow_mut() = Some(b));
}

/// Prints the placement counts to stderr when `SAPPHIRE_DEVICE_REPORT` is set and a backend is installed.
pub fn note_report() {
    if !env_is_one("SAPPHIRE_DEVICE_REPORT") {
        return;
    }
    if let Some(r) = CURRENT.with(|c| c.borrow().as_ref().map(|b| (b.name(), b.report()))) {
        if r.0 != "cpu" {
            crate::fxrun::elog!("{}", r.1.line());
        }
    }
}

/// Runs `f` on the installed backend; `None` when no backend is installed (the plain CPU path).
pub fn with_backend<R>(f: impl FnOnce(&mut dyn Backend) -> R) -> Option<R> {
    CURRENT.with(|c| c.borrow_mut().as_mut().map(|b| f(b.as_mut())))
}

/// Host arguments for a CPU run of a node whose operands may be device images: materializes them first.
pub fn host_args<'a>(vals: &'a [Val]) -> Result<Vec<Arg<'a>>, ExecError> {
    for v in vals {
        match v {
            Val::Dev(d) => {
                d.materialize()?;
            }
            Val::DevMip(d) => {
                d.materialize()?;
            }
            _ => {}
        }
    }
    Ok(vals.iter().map(|v| v.as_arg()).collect())
}

#[cfg(test)]
mod notice_tests {
    use super::*;
    use fx_device::DeviceKind;

    fn refused(kind: DeviceKind) -> DeviceHandle {
        DeviceHandle::Refused { kind, reasons: vec!["no OpenCL platform with a device".into()] }
    }

    fn logged(kind: DeviceKind, h: &DeviceHandle) -> String {
        let ((), io) = crate::fxrun::with_stage_io(crate::fxrun::StageIo::default(), || {
            queue_notices(kind, h);
            flush_notices();
        });
        io.log
    }

    #[test]
    fn an_explicit_request_that_ends_on_the_cpu_says_so_and_auto_is_silent() {
        if env_is_one("SAPPHIRE_DEVICE_REPORT") || env_is_one("SAPPHIRE_REQUIRE_DEVICE") {
            return;
        }
        for kind in [DeviceKind::Gpu, DeviceKind::GpuIndex(1)] {
            let text = logged(kind, &refused(kind));
            assert_eq!(text, "device: no OpenCL platform with a device; rendering on the CPU\n", "{kind}");
        }
        assert_eq!(logged(DeviceKind::Auto, &DeviceHandle::Cpu), "");
        assert_eq!(logged(DeviceKind::Cpu, &DeviceHandle::Cpu), "");
    }
}

/// NaN-exact ops on a real device (prints NOT RUN without one): every op on `device_ops::NAN_EXACT` is byte exact on its corpus
/// and its NaN cases, every op on `NAN_EXACT_WRITTEN` reports equal NaN payloads (an op on `NAN_EXACT_COMPILED` prints its result),
/// a NaN output of such an op stays on the device with the CPU's bits, and an op whose self-check finds other NaN payloads (a
/// kernel with another NaN rule) still reruns a NaN output on the CPU.
#[cfg(test)]
mod device_nan_exact_tests {
    use super::*;
    use crate::device_ops::{NAN_EXACT_COMPILED, NAN_EXACT_WRITTEN, NAN_TWIN_CAP};
    use crate::types::Image;
    use fx_device::selfcheck::Tally;
    use fx_device::{Backend as Handle, DeviceKind, Session};

    fn device() -> Option<Arc<DeviceCtx>> {
        match fx_device::select(DeviceKind::Gpu) {
            Handle::Device(d) => Some(d),
            other => {
                not_run(&format!("no usable OpenCL device ({})", other.describe()));
                None
            }
        }
    }

    /// A test that needs a device and found none: it prints NOT RUN, or fails when `SAPPHIRE_REQUIRE_DEVICE=1`.
    fn not_run(why: &str) {
        if std::env::var("SAPPHIRE_REQUIRE_DEVICE").as_deref() == Ok("1") {
            panic!("device required but unavailable: {why}");
        }
        println!("NOT RUN: {why}");
    }

    fn op(class: &str) -> &'static device_ops::DeviceOp {
        device_ops::DEVICE_OPS.iter().find(|o| o.class == class).expect("a NaN-exact op has a kernel")
    }

    /// Self-checks one NaN-exact op with its NaN cases and prints the result. A refusal (a bit difference outside NaN words) is an
    /// error for every op; the admission rule is checked on the measured payloads.
    fn nan_exact_selfcheck(ctx: &DeviceCtx, class: &str) -> Result<Tally, String> {
        let o = op(class);
        let corpus = (o.corpus)();
        let twins = crate::device_ops_pointwise::nan_twins(&corpus, NAN_TWIN_CAP);
        let nan_in = twins.iter().flat_map(|c| c.args.iter()).flat_map(|i| i.planes.iter()).flatten().filter(|v| v.is_nan()).count();
        let t = device_ops::selfcheck(&ctx.session, o).map_err(|m| format!("{class}: refused: {m}"))?;
        println!("NANEXACT {class:18} cases {} + {} NaN cases, NaN input words {nan_in}: {t}", corpus.len(), twins.len());
        let same = t.nan_payloads == NanPayloads::Same;
        assert_eq!(device_ops::keeps_nan_outputs(class, same), same, "{class}: the admission must follow the self-check");
        Ok(t)
    }

    /// Ops whose NaN pick is written in the CPU source: byte equality and equal NaN payloads on every device build.
    #[test]
    fn written_nan_exact_ops_report_equal_nan_payloads_on_the_device() {
        let Some(ctx) = device() else { return };
        let mut bad = Vec::new();
        for class in NAN_EXACT_WRITTEN {
            match nan_exact_selfcheck(&ctx, class) {
                Ok(t) if t.nan_payloads != NanPayloads::Same => bad.push(format!("{class}: {t}")),
                Ok(_) => {}
                Err(m) => bad.push(m),
            }
        }
        assert!(bad.is_empty(), "{bad:#?}");
    }

    /// Ops whose NaN pick is the compiled CPU instruction's: byte equality is asserted; the NaN payload result is information
    /// only (another compiler or host may pick otherwise, and then the op keeps the CPU rerun of NaN outputs).
    #[test]
    fn compiled_nan_exact_ops_are_byte_exact_and_print_their_nan_payloads() {
        let Some(ctx) = device() else { return };
        let mut bad = Vec::new();
        for class in NAN_EXACT_COMPILED {
            match nan_exact_selfcheck(&ctx, class) {
                Ok(t) => println!("COMPILED-PICK {class:12} nan payloads {}", if t.nan_payloads == NanPayloads::Same { "same" } else { "differ (NaN outputs rerun on the CPU)" }),
                Err(m) => bad.push(m),
            }
        }
        assert!(bad.is_empty(), "{bad:#?}");
    }

    fn nan_image() -> Image {
        // a NaN pair (0x7fc00001 + 0xffc0beef), a signalling NaN with a finite value, inf - inf and plain values
        let a: Vec<f32> = [0x7fc0_0001u32, 0x7f80_0001, 0x7f80_0000, 0x3f80_0000, 0x0000_0000].iter().map(|b| f32::from_bits(*b)).collect();
        let b: Vec<f32> = [0xffc0_beefu32, 0x4000_0000, 0x7f80_0000, 0x4000_0000, 0x8000_0000].iter().map(|b| f32::from_bits(*b)).collect();
        Image { n: 5, planes: vec![a, b] }
    }

    /// Runs `-` (plane 0 minus plane 1 of the image, as two one channel operands) through the backend.
    fn run_sub(b: &mut DeviceBackend) -> (Result<Val, Fallback>, Vec<f32>) {
        let im = nan_image();
        let a = Rc::new(Image { n: 5, planes: vec![im.planes[0].clone()] });
        let c = Rc::new(Image { n: 5, planes: vec![im.planes[1].clone()] });
        let kp = KParams::standin();
        let call = NodeCall { idx: 0, class: "-", tag: 1, n: 5, dims: (5, 1), kp: &kp };
        let want = match crate::kernels::run_any("-", 1, &[Arg::Img(&a), Arg::Img(&c)], 5, (5, 1), &kp).unwrap() {
            crate::kernels2::Out::Img(i) => i.planes[0].clone(),
            _ => unreachable!(),
        };
        (b.try_run(&call, &[Val::Img(a), Val::Img(c)]), want)
    }

    #[test]
    fn a_nan_output_of_a_nan_exact_op_stays_on_the_device_with_the_cpu_bits() {
        let Some(ctx) = device() else { return };
        let mut b = DeviceBackend::with_allowlist(ctx, &["-"]);
        assert_eq!(b.admitted(), vec!["-"]);
        assert_eq!(b.nan_exact(), vec!["-"], "the self-check of `-` must report equal NaN payloads");
        let (got, want) = run_sub(&mut b);
        assert!(want.iter().any(|v| v.is_nan()), "the case must produce NaN on the CPU");
        let Ok(Val::Dev(d)) = got else { panic!("expected a device result, got {got:?}") };
        d.materialize().unwrap();
        let host = Val::Dev(d);
        let words: Vec<u32> = host.as_arg().planes()[0].iter().map(|v| v.to_bits()).collect();
        assert_eq!(words, want.iter().map(|v| v.to_bits()).collect::<Vec<_>>());
        assert_eq!(b.report().fallback.get("-"), None);
    }

    /// The NaN pick helper with the operand order swapped: finite results are unchanged, NaN pairs give the second operand.
    fn swapped_pick_program() -> String {
        let src = fx_device::kernels::device_program();
        let from = "    if (isnan(x)) {\n        return as_float(as_uint(x) | 0x00400000u);\n    }\n    if (isnan(y)) {\n        return as_float(as_uint(y) | 0x00400000u);\n    }";
        let to = "    if (isnan(y)) {\n        return as_float(as_uint(y) | 0x00400000u);\n    }\n    if (isnan(x)) {\n        return as_float(as_uint(x) | 0x00400000u);\n    }";
        assert_eq!(src.matches(from).count(), 1, "the pick helper text");
        src.replacen(from, to, 1)
    }

    fn mutant_ctx(ctx: &DeviceCtx, source: &str) -> Arc<DeviceCtx> {
        let (dev, info) = fx_device::probe::open(ctx.info.index).unwrap();
        let s = Session::open_source(&dev, &info, source, fx_device::kernels::BUILD_OPTIONS).unwrap();
        Arc::new(DeviceCtx { session: Arc::new(s), info, base: Tally::empty(), base_cached: false })
    }

    #[test]
    fn a_nan_exact_op_whose_payloads_differ_still_reruns_on_the_cpu() {
        let Some(ctx) = device() else { return };
        let m = mutant_ctx(&ctx, &swapped_pick_program());
        let t = device_ops::selfcheck(&m.session, op("-")).expect("finite results are unchanged, so the op is still admitted");
        assert!(matches!(t.nan_payloads, NanPayloads::Differ(_)), "the swapped pick must show in the NaN payloads: {t}");
        // the mutant program is not the one the cache key names, so the admission runs without the cache
        let mut b = DeviceBackend::admit(m, &["-"], false);
        assert_eq!(b.admitted(), vec!["-"]);
        assert!(b.nan_exact().is_empty(), "an op with other NaN payloads is not NaN exact on this device");
        let (got, _) = run_sub(&mut b);
        assert_eq!(got.err(), Some(Fallback::NanOutput));
        assert_eq!(b.report().fallback.get("-").and_then(|m| m.get("nan_output")), Some(&1));
    }

    /// Single edits of the NaN-exact OpenCL source, each with the ops whose self-check must refuse it (a mismatch, or other NaN
    /// payloads, which keeps the op off the NaN-exact set).
    const MUTANTS: &[(&str, &str, &str, &[&str])] = &[
        ("pick2 operand order", "    if (isnan(x)) {\n        return as_float(as_uint(x) | 0x00400000u);\n    }\n    if (isnan(y)) {\n        return as_float(as_uint(y) | 0x00400000u);\n    }", "    if (isnan(y)) {\n        return as_float(as_uint(y) | 0x00400000u);\n    }\n    if (isnan(x)) {\n        return as_float(as_uint(x) | 0x00400000u);\n    }", &["+", "-", "*", "/", "diff"]),
        ("pick2 quiet bit constant", "float nanx_pick2(float x, float y) {\n    if (isnan(x)) {\n        return as_float(as_uint(x) | 0x00400000u);", "float nanx_pick2(float x, float y) {\n    if (isnan(x)) {\n        return as_float(as_uint(x) | 0x00200000u);", &["+", "-", "*", "/", "diff", "postdivide"]),
        ("pick2 default NaN sign", "        return as_float(as_uint(y) | 0x00400000u);\n    }\n    return as_float(0xffc00000u);", "        return as_float(as_uint(y) | 0x00400000u);\n    }\n    return as_float(0x7fc00000u);", &["+", "-", "*", "/"]),
        ("pick3 operand order", "    if (isnan(a)) {\n        return as_float(as_uint(a) | 0x00400000u);\n    }\n    if (isnan(b)) {", "    if (isnan(b)) {\n        return as_float(as_uint(b) | 0x00400000u);\n    }\n    if (isnan(a)) {", &["screen_super", "dissolve"]),
        ("sub pick operand swap", "    float r = x - y;\n    return isnan(r) ? nanx_pick2(x, y) : r;", "    float r = x - y;\n    return isnan(r) ? nanx_pick2(y, x) : r;", &["-", "diff", "dissolve"]),
        ("dissolve difference operand swap", "nanx_fma(t, nanx_sub(b, a), a)", "nanx_fma(t, nanx_sub(a, b), a)", &["dissolve"]),
        ("fma pick order", "    return isnan(r) ? nanx_pick3(b, a, c) : r;", "    return isnan(r) ? nanx_pick3(a, b, c) : r;", &["screen_super", "dissolve"]),
        ("fma replaced by an unfused multiply and add", "    float r = fma(a, b, c);\n    return isnan(r)", "    float r = a * b + c;\n    return isnan(r)", &["screen_super", "dissolve"]),
        ("add with a constant: constant first", "            r = pointwise_bin_f(op, pb[i], as_float(ca));", "            r = pointwise_bin_f(op, as_float(ca), pb[i]);", &["+", "*"]),
        ("postdivide compare direction", "    bool lt_eps = 0x1p-13f > a;", "    bool lt_eps = 0x1p-13f >= a;", &["postdivide"]),
        ("postdivide reciprocal select arm", "            r = nanx_mul(x, inv);", "            r = nanx_div(x, a);", &["postdivide"]),
        ("postdivide copy arm quiets a signalling NaN", "    } else {\n        r = x;\n    }\n    out[i] = r;", "    } else {\n        r = as_float(as_uint(x) | 0x00400000u);\n    }\n    out[i] = r;", &["postdivide"]),
        ("diff sign mask constant", "    return as_float(as_uint(nanx_sub(x, y)) & 0x7fffffffu);", "    return as_float(as_uint(nanx_sub(x, y)) & 0xffffffffu);", &["diff"]),
        ("screen extension subtrahend", "        float y = nanx_sub(nanx_add(one, x), x);", "        float y = nanx_sub(nanx_add(one, x), one);", &["screen_super"]),
        ("ripmap sum pair pick operand", "        return a | 0x00400000u;\n    }\n    if (na) {", "        return b | 0x00400000u;\n    }\n    if (na) {", &["make_mipmap"]),
        ("ripmap sum signalling select arm", "        if (sa && !sb) {\n            return b;", "        if (sa && !sb) {\n            return a | 0x00400000u;", &["make_mipmap"]),
        ("ripmap sum signalling compare direction", "    int sa = (a & 0x00400000u) == 0u;", "    int sa = (a & 0x00400000u) != 0u;", &["make_mipmap"]),
        ("ripmap sum default NaN sign", "    if (isnan(s)) {\n        return 0xffc00000u;", "    if (isnan(s)) {\n        return 0x7fc00000u;", &["make_mipmap"]),
        ("ripmap NaN test mantissa mask", "    return (v & 0x7f800000u) == 0x7f800000u && (v & 0x007fffffu) != 0u;", "    return (v & 0x7f800000u) == 0x7f800000u && (v & 0x003fffffu) != 0u;", &["make_mipmap"]),
        ("ripmap horizontal sum order", "(1u + x) * (uint)c + ch] = mm_avg(b, a);", "(1u + x) * (uint)c + ch] = mm_avg(a, b);", &["make_mipmap"]),
        ("ripmap vertical sum order", "    rip[doff + (1u + y) * stride + k] = mm_avg(b, a);", "    rip[doff + (1u + y) * stride + k] = mm_avg(a, b);", &["make_mipmap"]),
        ("ripmap average constant", "    return as_uint(0x1p-1f * as_float(s));", "    return as_uint(0x1p-2f * as_float(s));", &["make_mipmap"]),
    ];

    /// The mutation check of the NaN-exact kernels: every mutant is its own program build and must be refused by the self-check of
    /// at least one of its ops (run directly, without the self-check cache). Run with `-- --ignored` (each build takes a while on PoCL).
    #[test]
    #[ignore]
    fn nan_exact_source_mutants_are_refused() {
        let Some(ctx) = device() else { return };
        let base = fx_device::kernels::device_program();
        let mut prev = fx_device::progcache::fnv(base.as_bytes());
        let mut survivors = Vec::new();
        for (name, from, to, ops) in MUTANTS {
            assert_eq!(base.matches(from).count(), 1, "mutant `{name}`: the edited text must occur once");
            let src = base.replacen(from, to, 1);
            let digest = fx_device::progcache::fnv(src.as_bytes());
            assert!(digest != fx_device::progcache::fnv(base.as_bytes()) && digest != prev, "mutant `{name}` equals the base or the previous mutant");
            prev = digest;
            let m = mutant_ctx(&ctx, &src);
            let mut verdicts = Vec::new();
            let mut killed = false;
            for class in *ops {
                match device_ops::selfcheck(&m.session, op(class)) {
                    Err(e) => {
                        killed = true;
                        verdicts.push(format!("{class}: mismatch ({})", e.chars().take(160).collect::<String>()));
                    }
                    Ok(t) if t.nan_payloads != NanPayloads::Same => {
                        killed = true;
                        verdicts.push(format!("{class}: NaN payloads differ"));
                    }
                    Ok(_) => verdicts.push(format!("{class}: equal")),
                }
            }
            println!("MUTANT {name} program {digest:016x}: {} | {}", if killed { "KILLED" } else { "SURVIVED" }, verdicts.join("; "));
            if !killed {
                survivors.push(*name);
            }
        }
        assert!(survivors.is_empty(), "surviving mutants: {survivors:?}");
    }
}
