//! Device selection: the request ([`DeviceKind`]), the choice among the probed devices and the [`Backend`] handle
//! the executor is given.
// Provenance: adapted from twixtor crates/fx-core/src/device.rs (DeviceRequest) and plugins/retime-engine/src/device/exec/open.rs (8087d06).
//!
//! `select` never fails: a request that cannot be served gives [`Backend::Refused`] with the reasons, which behaves
//! as the CPU. The pixels never depend on the request.

use crate::image::Session;
use crate::probe::{self, DeviceInfo};
use crate::selfcheck::{self, Tally};
use std::fmt;
use std::sync::Arc;

/// Which device a render runs on. The text form is `cpu`, `gpu`, `gpu=N` or `auto`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum DeviceKind {
    /// The CPU, the reference path. The default.
    #[default]
    Cpu,
    /// The first usable device (a GPU, else any usable device such as a CPU runtime).
    Gpu,
    /// Device number `N` of the probe order.
    GpuIndex(u32),
    /// The first usable GPU-type device, else the CPU.
    Auto,
}

impl DeviceKind {
    /// Reads `cpu`, `gpu`, `gpu=N` (decimal digits only) or `auto`; blanks around are ignored.
    pub fn parse(text: &str) -> Option<DeviceKind> {
        match text.trim() {
            "cpu" => Some(DeviceKind::Cpu),
            "gpu" => Some(DeviceKind::Gpu),
            "auto" => Some(DeviceKind::Auto),
            other => {
                let n = other.strip_prefix("gpu=")?;
                if n.is_empty() || !n.bytes().all(|b| b.is_ascii_digit()) {
                    return None;
                }
                n.parse().ok().map(DeviceKind::GpuIndex)
            }
        }
    }
}

impl fmt::Display for DeviceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceKind::Cpu => write!(f, "cpu"),
            DeviceKind::Gpu => write!(f, "gpu"),
            DeviceKind::GpuIndex(n) => write!(f, "gpu={n}"),
            DeviceKind::Auto => write!(f, "auto"),
        }
    }
}

/// An opened device that passed the probe and the base self-check.
#[derive(Debug)]
pub struct DeviceCtx {
    pub session: Arc<Session>,
    pub info: DeviceInfo,
    pub base: Tally,
    /// The base self-check was not run in this process: an earlier run passed it on this device, driver and program text
    /// ([`crate::checkcache`]); `base` is then empty.
    pub base_cached: bool,
}

/// What `select` hands the executor.
#[derive(Clone, Debug)]
pub enum Backend {
    /// No device: every node runs on the CPU.
    Cpu,
    /// A device ready for the per-op admission in the executor.
    Device(Arc<DeviceCtx>),
    /// A device was requested and refused; the text says why. Behaves as the CPU.
    Refused { kind: DeviceKind, reasons: Vec<String> },
}

impl Backend {
    pub fn is_device(&self) -> bool {
        matches!(self, Backend::Device(_))
    }

    pub fn device(&self) -> Option<&Arc<DeviceCtx>> {
        match self {
            Backend::Device(d) => Some(d),
            _ => None,
        }
    }

    /// One line for a note on stderr.
    pub fn describe(&self) -> String {
        match self {
            Backend::Cpu => "device: cpu".into(),
            Backend::Device(d) => format!("device: {} ({}, driver {}), base self-check {}", d.info.display_name(), d.info.platform, d.info.driver, if d.base_cached { "passed earlier on this driver (cached)".to_string() } else { d.base.to_string() }),
            Backend::Refused { kind, reasons } => format!("device: --device {kind} refused, running on the CPU: {}", reasons.join("; ")),
        }
    }
}

/// Every device of the system runtime with the reasons it is refused (empty when usable). `Err` text when no runtime loads.
pub fn list() -> Result<Vec<(DeviceInfo, Vec<String>)>, String> {
    let p = probe::probe().map_err(|e| e.to_string())?;
    Ok(p.devices.into_iter().map(|d| {
        let r = probe::requirements(&d);
        (d, r)
    }).collect())
}

/// Opens device `index` of the probe order, checks the requirements and the base self-check.
pub fn open(index: usize) -> Result<DeviceCtx, Vec<String>> {
    let (device, info) = probe::open(index).map_err(|e| vec![e.to_string()])?;
    let reasons = probe::requirements(&info);
    if !reasons.is_empty() {
        return Err(reasons);
    }
    let session = Session::open(&device, &info).map_err(|e| vec![format!("program build: {e}")])?;
    let session = Arc::new(session);
    if crate::checkcache::passed(&info, "base") {
        return Ok(DeviceCtx { session, info, base: selfcheck::Tally::empty(), base_cached: true });
    }
    match selfcheck::check_base(&session) {
        Ok(base) => {
            crate::checkcache::record_pass(&info, "base");
            Ok(DeviceCtx { session, info, base, base_cached: false })
        }
        Err(r) => Err(vec![format!("base self-check: {r}")]),
    }
}

/// The device `kind` asks for. `Cpu` is the default and never touches OpenCL.
pub fn select(kind: DeviceKind) -> Backend {
    if kind == DeviceKind::Cpu {
        return Backend::Cpu;
    }
    let refuse = |reasons: Vec<String>| Backend::Refused { kind, reasons };
    let probed = match probe::probe() {
        Ok(p) => p,
        Err(_) if kind == DeviceKind::Auto => return Backend::Cpu,
        Err(e) => return refuse(vec![e.to_string()]),
    };
    let pick = match kind {
        DeviceKind::GpuIndex(n) => {
            if (n as usize) < probed.devices.len() {
                Some(n as usize)
            } else {
                return refuse(vec![format!("no OpenCL device {n} ({} listed)", probed.devices.len())]);
            }
        }
        DeviceKind::Gpu => probe::preferred(&probed.devices),
        DeviceKind::Auto => {
            let gpus: Vec<DeviceInfo> = probed.devices.iter().filter(|d| d.type_name() == "gpu").cloned().collect();
            probe::preferred(&gpus).map(|i| gpus[i].index)
        }
        DeviceKind::Cpu => None,
    };
    let Some(index) = pick else {
        if kind == DeviceKind::Auto {
            return Backend::Cpu;
        }
        let mut reasons: Vec<String> = probed.devices.iter().map(|d| format!("device {} {}: {}", d.index, d.display_name(), probe::requirements(d).join(", "))).collect();
        if reasons.is_empty() {
            reasons.push("no OpenCL platform with a device".into());
        }
        return refuse(reasons);
    };
    match open(index) {
        Ok(ctx) => Backend::Device(Arc::new(ctx)),
        Err(reasons) if kind == DeviceKind::Auto => {
            let _ = reasons;
            Backend::Cpu
        }
        Err(reasons) => refuse(reasons),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_four_forms() {
        assert_eq!(DeviceKind::parse("cpu"), Some(DeviceKind::Cpu));
        assert_eq!(DeviceKind::parse("gpu"), Some(DeviceKind::Gpu));
        assert_eq!(DeviceKind::parse("auto"), Some(DeviceKind::Auto));
        assert_eq!(DeviceKind::parse(" gpu=12 "), Some(DeviceKind::GpuIndex(12)));
        for bad in ["", "CPU", "gpu=", "gpu=-1", "gpu=+1", "gpu=x", "gpu=4294967296", "gpu 1", "opencl", "cpu=0"] {
            assert_eq!(DeviceKind::parse(bad), None, "`{bad}`");
        }
        for k in [DeviceKind::Cpu, DeviceKind::Gpu, DeviceKind::GpuIndex(3), DeviceKind::Auto] {
            assert_eq!(DeviceKind::parse(&k.to_string()), Some(k));
        }
    }

    #[test]
    fn cpu_never_opens_a_runtime() {
        assert!(matches!(select(DeviceKind::Cpu), Backend::Cpu));
        assert_eq!(DeviceKind::default(), DeviceKind::Cpu);
    }
}
