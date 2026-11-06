//! Device sessions and resident images: upload, download, the memory budget and the NaN scan.
// Provenance: adapted from twixtor plugins/retime-engine/src/device/image.rs and selfcheck.rs (Session) (8087d06).
//!
//! A [`DevImage`] holds the planes of an executor image (planar f32, `n` samples per plane) as one buffer
//! per plane. Uploading and downloading copy the words unchanged, so a round trip returns the same bits, NaN
//! payloads and negative zeros included. Buffers count against the device budget (three quarters of its global
//! memory, [`crate::probe::memory_budget`]); an allocation that would pass it is [`DevError::Memory`] and the
//! executor runs that node on the CPU.

use crate::cl::{self, Access, Arg, Buffer, ClError, Context, Kernel, Program, Queue, Range};
use crate::kernels::{self, BUILD_OPTIONS};
use crate::probe::{self, DeviceInfo};
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Why a device step did not produce a result. The executor reruns the node on the CPU for every variant.
#[derive(Debug)]
pub enum DevError {
    /// The driver or a kernel build reported an error.
    Cl(ClError),
    /// The device budget would be exceeded.
    Memory { need: u64, used: u64, budget: u64 },
    /// A float output holds a NaN; the device never supplies such an output.
    NanOutput,
    /// The step cannot run on the device for this input (an empty image, a setting the kernel does not cover).
    Unsupported(String),
}

impl From<ClError> for DevError {
    fn from(e: ClError) -> DevError {
        DevError::Cl(e)
    }
}

impl fmt::Display for DevError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DevError::Cl(e) => write!(f, "device error: {e}"),
            DevError::Memory { need, used, budget } => write!(f, "device memory: {need} bytes needed, {used} used of {budget}"),
            DevError::NanOutput => write!(f, "a float output holds a NaN"),
            DevError::Unsupported(why) => write!(f, "not supported on the device: {why}"),
        }
    }
}

impl std::error::Error for DevError {}

/// Bytes placed on the device and the limit.
#[derive(Debug)]
pub struct MemTrack {
    used: AtomicU64,
    budget: u64,
}

impl MemTrack {
    fn reserve(&self, bytes: u64) -> Result<(), DevError> {
        let mut cur = self.used.load(Ordering::SeqCst);
        loop {
            let next = cur.saturating_add(bytes);
            if next > self.budget {
                return Err(DevError::Memory { need: bytes, used: cur, budget: self.budget });
            }
            match self.used.compare_exchange(cur, next, Ordering::SeqCst, Ordering::SeqCst) {
                Ok(_) => return Ok(()),
                Err(seen) => cur = seen,
            }
        }
    }

    fn release(&self, bytes: u64) {
        self.used.fetch_sub(bytes, Ordering::SeqCst);
    }

    pub fn used(&self) -> u64 {
        self.used.load(Ordering::SeqCst)
    }

    pub fn budget(&self) -> u64 {
        self.budget
    }
}

/// A context, an in-order queue and the one built program on a device.
pub struct Session {
    pub context: Context,
    pub queue: Queue,
    pub program: Program,
    pub info: DeviceInfo,
    pub mem: Arc<MemTrack>,
}

impl fmt::Debug for Session {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Session({})", self.info.display_name())
    }
}

impl Session {
    /// Builds the device program ([`kernels::device_program`]) with [`BUILD_OPTIONS`] through the program cache.
    pub fn open(device: &cl::Device, info: &DeviceInfo) -> Result<Session, ClError> {
        Session::open_source(device, info, &kernels::device_program(), BUILD_OPTIONS)
    }

    /// Builds `source` with `options` (the tests that a relaxing build is caught pass their own).
    pub fn open_source(device: &cl::Device, info: &DeviceInfo, source: &str, options: &str) -> Result<Session, ClError> {
        let _create = cl::serialize_creation();
        let context = Context::new(device)?;
        let queue = Queue::new(&context)?;
        let program = crate::progcache::build(&context, device, source, options)?;
        // The self-check passes are valid only for the binary this session runs: register its digest for the cache key.
        if source == kernels::device_program() && options == BUILD_OPTIONS {
            if let Ok(binary) = program.binary() {
                crate::checkcache::register_binary(info, crate::progcache::fnv(&binary));
            }
        }
        let mem = Arc::new(MemTrack { used: AtomicU64::new(0), budget: probe::memory_budget(info) });
        Ok(Session { context, queue, program, info: info.clone(), mem })
    }

    pub fn kernel(&self, name: &str) -> Result<Kernel, ClError> {
        Kernel::new(&self.program, name)
    }

    /// Uploads `data` read only (not counted against the budget: used by the self-check).
    pub fn input<T: cl::Pod>(&self, data: &[T]) -> Result<Buffer<T>, ClError> {
        Buffer::from_slice(&self.context, data, Access::ReadOnly)
    }

    pub fn output<T: cl::Pod>(&self, len: usize) -> Result<Buffer<T>, ClError> {
        Buffer::new(&self.context, len, Access::WriteOnly)
    }

    /// Runs a kernel over `n` items.
    pub fn launch(&self, name: &str, n: usize, args: &[Arg<'_>]) -> Result<(), ClError> {
        let mut k = self.kernel(name)?;
        self.queue.run(&mut k, Range::D1(n), args)
    }
}

/// A planar f32 image on the device.
pub struct DevImage {
    session: Arc<Session>,
    pub n: usize,
    planes: Vec<Buffer<f32>>,
    bytes: u64,
}

impl fmt::Debug for DevImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DevImage({} planes x {})", self.planes.len(), self.n)
    }
}

impl Drop for DevImage {
    fn drop(&mut self) {
        self.session.mem.release(self.bytes);
    }
}

impl DevImage {
    fn account(session: &Arc<Session>, channels: usize, n: usize) -> Result<u64, DevError> {
        if n == 0 || channels == 0 {
            return Err(DevError::Unsupported("an image without samples".into()));
        }
        let bytes = (channels as u64).saturating_mul(n as u64).saturating_mul(4);
        session.mem.reserve(bytes)?;
        Ok(bytes)
    }

    /// Copies host planes to the device.
    pub fn upload(session: &Arc<Session>, planes: &[&[f32]]) -> Result<DevImage, DevError> {
        let n = planes.first().map(|p| p.len()).unwrap_or(0);
        if planes.iter().any(|p| p.len() != n) {
            return Err(DevError::Unsupported("planes of different lengths".into()));
        }
        let bytes = DevImage::account(session, planes.len(), n)?;
        let mut bufs = Vec::with_capacity(planes.len());
        for p in planes {
            match Buffer::from_slice(&session.context, p, Access::ReadWrite) {
                Ok(b) => bufs.push(b),
                Err(e) => {
                    session.mem.release(bytes);
                    return Err(e.into());
                }
            }
        }
        Ok(DevImage { session: session.clone(), n, planes: bufs, bytes })
    }

    /// An image of `channels` planes of `n` samples whose contents the caller's kernels write.
    pub fn alloc(session: &Arc<Session>, channels: usize, n: usize) -> Result<DevImage, DevError> {
        let bytes = DevImage::account(session, channels, n)?;
        let mut bufs = Vec::with_capacity(channels);
        for _ in 0..channels {
            match Buffer::new(&session.context, n, Access::ReadWrite) {
                Ok(b) => bufs.push(b),
                Err(e) => {
                    session.mem.release(bytes);
                    return Err(e.into());
                }
            }
        }
        Ok(DevImage { session: session.clone(), n, planes: bufs, bytes })
    }

    pub fn channels(&self) -> usize {
        self.planes.len()
    }

    pub fn plane(&self, c: usize) -> &Buffer<f32> {
        &self.planes[c]
    }

    pub fn session(&self) -> &Arc<Session> {
        &self.session
    }

    /// Copies the planes back; NaNs and payloads are returned as they are.
    pub fn download(&self) -> Result<Vec<Vec<f32>>, DevError> {
        let mut out = Vec::with_capacity(self.planes.len());
        for b in &self.planes {
            out.push(self.session.queue.read(b)?);
        }
        Ok(out)
    }

    /// True when any plane holds a NaN, found by a kernel so the image stays resident (one word is read back).
    pub fn has_nan(&self) -> Result<bool, DevError> {
        let flag: Buffer<u32> = Buffer::from_slice(&self.session.context, &[0u32], Access::ReadWrite)?;
        for b in &self.planes {
            self.session.launch("any_nan_f32", self.n, &[b.arg(), flag.arg(), Arg::U32(self.n as u32)])?;
        }
        Ok(self.session.queue.read(&flag)?[0] != 0)
    }
}
