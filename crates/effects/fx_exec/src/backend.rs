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
//! * the float output holds no NaN (checked on the device, so the output stays resident). A NaN is never taken from the
//!   device because devices write other NaN payloads than the CPU.
//!
//! A backend is installed per thread ([`install`]); the selection function for callers is
//! [`for_device`] (from `fx_device::select`) and `fx_device::select` itself.

use crate::device_ops::{self, DevArg, DeviceOp};
use crate::types::{Arg, DevVal, ExecError, Image, KParams, Val};
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
    pub fn with_allowlist(ctx: Arc<DeviceCtx>, allow: &[&str]) -> DeviceBackend {
        let mut admitted = BTreeMap::new();
        let mut report = Report::default();
        let allow: BTreeSet<&str> = allow.iter().copied().collect();
        for op in device_ops::DEVICE_OPS {
            if !allow.contains(op.class) {
                continue;
            }
            if fx_device::checkcache::passed(&ctx.info, op.class) {
                admitted.insert(op.class, op);
                continue;
            }
            match device_ops::selfcheck(&ctx.session, op) {
                Ok(_) => {
                    fx_device::checkcache::record_pass(&ctx.info, op.class);
                    admitted.insert(op.class, op);
                }
                Err(m) => {
                    report.refused.insert(op.class.to_string(), m);
                }
            }
        }
        DeviceBackend { ctx, admitted, uploads: HashMap::new(), report }
    }

    pub fn admitted(&self) -> Vec<&'static str> {
        self.admitted.keys().copied().collect()
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
        if out.has_nan().map_err(map_err)? {
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
