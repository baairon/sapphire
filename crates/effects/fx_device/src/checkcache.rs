//! The self-check result cache: which self-checks a device already passed, kept in the user's cache directory so a
//! process pays the self-check of a kernel group once per machine and driver instead of once per process.
//!
//! One file per device. The file name and the first lines carry the whole key: the platform, the device name, vendor
//! and version, the driver version, a format number, the FNV-1a digest of the full device program text, the build
//! options, the digest of the program binary the session loaded or built, and the build identity (the digest of the
//! running executable, which holds the CPU kernels, the device glue and the corpus the passes were measured against).
//! The rest of the file lists the tags (`base`, or an op class) that passed, one per line. A tag followed by ` nan_same` also
//! reported equal NaN payloads and signs to the CPU on every NaN output of its corpus; a tag without it counts as a pass whose
//! NaN payloads were not shown equal (an entry written before the field existed reads the same way). A changed kernel, a new driver, a
//! replaced program binary or a rebuilt executable changes the key, so the old file is unreadable and every check
//! runs again. When the executable or the loaded binary is unknown the cache is neither read nor written. Only passes are
//! recorded; a failed check is never cached. The file is written to a unique temporary name and renamed into place.
//!
//! `SAPPHIRE_DEVICE_CHECK_CACHE=0` turns the cache off (every check runs). `SAPPHIRE_CACHE_DIR` replaces the user
//! cache directory (the tests use it). A read or write error means the check runs; the cache never changes what a
//! kernel computes.

use crate::kernels::{self, BUILD_OPTIONS};
use crate::probe::DeviceInfo;
use crate::progcache::{fnv, fnv_update, FNV_BASIS};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::collections::HashMap;
use std::io::Read;
use std::sync::{Mutex, OnceLock};

const MAGIC: &str = "sapphire selfcheck 2";

fn enabled() -> bool {
    std::env::var_os("SAPPHIRE_DEVICE_CHECK_CACHE").is_none_or(|v| v != "0")
}

/// The user's cache directory for sapphire: `SAPPHIRE_CACHE_DIR`, else `XDG_CACHE_HOME` or `~/.cache` on Linux,
/// `~/Library/Caches` on macOS, `%LOCALAPPDATA%` on Windows, else the temporary directory; then `sapphire`.
pub fn folder() -> PathBuf {
    if let Some(d) = std::env::var_os("SAPPHIRE_CACHE_DIR") {
        return PathBuf::from(d);
    }
    let base = if cfg!(windows) {
        std::env::var_os("LOCALAPPDATA").map(PathBuf::from)
    } else if cfg!(target_os = "macos") {
        std::env::var_os("HOME").map(|h| PathBuf::from(h).join("Library").join("Caches"))
    } else {
        std::env::var_os("XDG_CACHE_HOME").map(PathBuf::from).filter(|p| p.is_absolute()).or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".cache")))
    };
    base.unwrap_or_else(std::env::temp_dir).join("sapphire")
}

/// The digest of the full device program text: it changes with any kernel source.
pub fn program_digest() -> u64 {
    static DIGEST: OnceLock<u64> = OnceLock::new();
    *DIGEST.get_or_init(|| fnv(kernels::device_program().as_bytes()))
}

/// The digest of the running executable's bytes, computed once per process on first use; `None` when it cannot be read.
pub fn build_identity() -> Option<u64> {
    static ID: OnceLock<Option<u64>> = OnceLock::new();
    *ID.get_or_init(|| {
        let mut f = std::fs::File::open(std::env::current_exe().ok()?).ok()?;
        let mut h = FNV_BASIS;
        let mut buf = vec![0u8; 1 << 20];
        loop {
            let n = f.read(&mut buf).ok()?;
            if n == 0 {
                return Some(h);
            }
            h = fnv_update(h, &buf[..n]);
        }
    })
}

/// What the key needs beyond the device: the executable digest and the digest of the loaded program binary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Ident {
    exe: u64,
    binary: u64,
}

fn device_id(info: &DeviceInfo) -> String {
    format!("{}\n{}\n{}\n{}\n{}", info.platform, info.name, info.vendor, info.version, info.driver)
}

fn binaries() -> &'static Mutex<HashMap<String, u64>> {
    static M: OnceLock<Mutex<HashMap<String, u64>>> = OnceLock::new();
    M.get_or_init(Default::default)
}

/// Records the digest of the device program binary a session loaded or built for this device. The self-check passes of
/// the device are valid only for that binary.
pub fn register_binary(info: &DeviceInfo, digest: u64) {
    binaries().lock().unwrap_or_else(|p| p.into_inner()).insert(device_id(info), digest);
}

fn ident(info: &DeviceInfo) -> Option<Ident> {
    let binary = *binaries().lock().unwrap_or_else(|p| p.into_inner()).get(&device_id(info))?;
    Some(Ident { exe: build_identity()?, binary })
}

fn key_text_for(info: &DeviceInfo, id: Ident) -> String {
    format!(
        "{MAGIC}\n{}\nprogram {:016x}\noptions {BUILD_OPTIONS}\nbinary {:016x}\nbuild {:016x}",
        device_id(info),
        program_digest(),
        id.binary,
        id.exe
    )
}

fn path_for(info: &DeviceInfo, id: Ident) -> PathBuf {
    folder().join(format!("selfcheck-{:016x}.txt", fnv(key_text_for(info, id).as_bytes())))
}

fn read_tags_for(info: &DeviceInfo, id: Ident) -> Vec<String> {
    let Ok(text) = std::fs::read_to_string(path_for(info, id)) else { return Vec::new() };
    let key = key_text_for(info, id);
    match text.strip_prefix(&key).and_then(|r| r.strip_prefix('\n')) {
        Some(rest) => rest.lines().filter(|l| !l.is_empty()).map(str::to_string).collect(),
        None => Vec::new(),
    }
}

/// The field written after a tag whose self-check showed equal NaN payloads.
const NAN_SAME: &str = "nan_same";

/// A cache line split into its tag and whether it carries the NaN payload field.
fn parse_line(line: &str) -> (&str, bool) {
    match line.split_once(' ') {
        Some((tag, rest)) => (tag, rest.split(' ').any(|f| f == NAN_SAME)),
        None => (line, false),
    }
}

/// True when `tag` passed on this device, driver and program text in an earlier run.
pub fn passed(info: &DeviceInfo, tag: &str) -> bool {
    if !enabled() {
        return false;
    }
    ident(info).is_some_and(|id| passed_for(info, id, tag))
}

fn passed_for(info: &DeviceInfo, id: Ident, tag: &str) -> bool {
    nan_same_for(info, id, tag).is_some()
}

/// `Some(nan_same)` when `tag` passed on this device in an earlier run: `nan_same` is true only when that pass recorded equal
/// NaN payloads (an entry without the field gives false). `None` when no pass is recorded.
pub fn passed_nan(info: &DeviceInfo, tag: &str) -> Option<bool> {
    if !enabled() {
        return None;
    }
    ident(info).and_then(|id| nan_same_for(info, id, tag))
}

fn nan_same_for(info: &DeviceInfo, id: Ident, tag: &str) -> Option<bool> {
    read_tags_for(info, id).iter().map(|l| parse_line(l)).find(|(t, _)| *t == tag).map(|(_, same)| same)
}

/// Records that `tag` passed (merged with what the file already holds; written atomically). Errors are ignored.
pub fn record_pass(info: &DeviceInfo, tag: &str) {
    record_result(info, tag, false);
}

/// Records that `tag` passed and whether its NaN payloads were equal to the CPU's. Errors are ignored.
pub fn record_result(info: &DeviceInfo, tag: &str, nan_same: bool) {
    if !enabled() {
        return;
    }
    if let Some(id) = ident(info) {
        record_result_for(info, id, tag, nan_same);
    }
}

#[cfg(test)]
fn record_pass_for(info: &DeviceInfo, id: Ident, tag: &str) {
    record_result_for(info, id, tag, false);
}

fn record_result_for(info: &DeviceInfo, id: Ident, tag: &str, nan_same: bool) {
    let mut tags = read_tags_for(info, id);
    let line = if nan_same { format!("{tag} {NAN_SAME}") } else { tag.to_string() };
    if tags.iter().any(|t| *t == line) {
        return;
    }
    tags.retain(|t| parse_line(t).0 != tag);
    tags.push(line);
    tags.sort();
    let mut body = key_text_for(info, id);
    body.push('\n');
    for t in &tags {
        body.push_str(t);
        body.push('\n');
    }
    let final_path = path_for(info, id);
    static SEQ: AtomicU64 = AtomicU64::new(0);
    let tmp = final_path.with_extension(format!("tmp{}.{}", std::process::id(), SEQ.fetch_add(1, Ordering::Relaxed)));
    if crate::progcache::create_private_dir(&folder()).is_err() {
        return;
    }
    if std::fs::write(&tmp, body).is_ok() && std::fs::rename(&tmp, &final_path).is_ok() {
        return;
    }
    let _ = std::fs::remove_file(&tmp);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, MutexGuard};

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    /// Points `SAPPHIRE_CACHE_DIR` at a private directory for one test and holds a process-wide lock, so tests that
    /// set the variable never overlap. The variable is restored and the directory removed on drop.
    struct CacheDir {
        dir: PathBuf,
        prev: Option<std::ffi::OsString>,
        _lock: MutexGuard<'static, ()>,
    }

    impl CacheDir {
        fn new(name: &str) -> Self {
            let lock = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
            let dir = std::env::temp_dir().join(format!("sapphire-checkcache-{}-{}", name, std::process::id()));
            let prev = std::env::var_os("SAPPHIRE_CACHE_DIR");
            let _ = std::fs::remove_dir_all(&dir);
            std::env::set_var("SAPPHIRE_CACHE_DIR", &dir);
            CacheDir { dir, prev, _lock: lock }
        }
    }

    impl Drop for CacheDir {
        fn drop(&mut self) {
            match &self.prev {
                Some(v) => std::env::set_var("SAPPHIRE_CACHE_DIR", v),
                None => std::env::remove_var("SAPPHIRE_CACHE_DIR"),
            }
            let _ = std::fs::remove_dir_all(&self.dir);
        }
    }

    fn info(driver: &str) -> DeviceInfo {
        DeviceInfo { index: 0, platform: "P".into(), platform_version: "1".into(), name: "D".into(), board: String::new(), vendor: "V".into(), version: "OpenCL 3.0".into(), driver: driver.into(), c_version: String::new(), profile: "FULL_PROFILE".into(), device_type: 4, little_endian: true, available: true, compiler_available: true, global_memory: 0, max_allocation: 0, ..Default::default() }
    }

    fn id(exe: u64, binary: u64) -> Ident {
        Ident { exe, binary }
    }

    #[test]
    fn a_pass_is_remembered_per_driver_and_tag() {
        let cache = CacheDir::new("pass");
        let dir = cache.dir.clone();
        let a = info("1.0");
        let i = id(1, 2);
        assert!(!passed_for(&a, i, "base"));
        record_pass_for(&a, i, "base");
        record_pass_for(&a, i, "copy");
        assert!(passed_for(&a, i, "base") && passed_for(&a, i, "copy") && !passed_for(&a, i, "one_minus"));
        assert!(!passed_for(&info("2.0"), i, "base"), "a new driver version invalidates the entry");
    }

    #[test]
    fn the_nan_payload_field_is_recorded_and_an_entry_without_it_is_not_nan_exact() {
        let cache = CacheDir::new("nan");
        let dir = cache.dir.clone();
        let a = info("3.0");
        let i = id(5, 6);
        assert_eq!(nan_same_for(&a, i, "+"), None);
        record_pass_for(&a, i, "+");
        assert_eq!(nan_same_for(&a, i, "+"), Some(false), "an entry without the field is a pass that is not NaN exact");
        record_result_for(&a, i, "+", true);
        record_result_for(&a, i, "copy", false);
        assert_eq!(nan_same_for(&a, i, "+"), Some(true));
        assert_eq!(nan_same_for(&a, i, "copy"), Some(false));
        assert!(passed_for(&a, i, "+") && passed_for(&a, i, "copy"));
        record_result_for(&a, i, "+", false);
        assert_eq!(nan_same_for(&a, i, "+"), Some(false), "a later result replaces the line");
        assert_eq!(read_tags_for(&a, i).iter().filter(|l| parse_line(l).0 == "+").count(), 1);
        // a file written in the earlier form (tags only) reads as passes without the field
        let mut body = key_text_for(&a, i);
        body.push_str("\nbase\nmax\n");
        std::fs::write(path_for(&a, i), body).unwrap();
        assert_eq!(nan_same_for(&a, i, "max"), Some(false));
        assert!(passed_for(&a, i, "base"));
        let _ = dir;
    }

    #[test]
    fn two_build_identities_or_binaries_give_different_keys() {
        let a = info("1.0");
        let k = key_text_for(&a, id(1, 2));
        assert_ne!(k, key_text_for(&a, id(3, 2)), "another executable");
        assert_ne!(k, key_text_for(&a, id(1, 4)), "another program binary");
        assert_ne!(path_for(&a, id(1, 2)), path_for(&a, id(3, 2)));
        assert!(k.contains(BUILD_OPTIONS), "the build options are part of the key");
    }

    #[test]
    fn an_entry_written_under_one_identity_is_not_read_under_another() {
        let cache = CacheDir::new("ident");
        let dir = cache.dir.clone();
        let a = info("9.0");
        record_pass_for(&a, id(10, 20), "base");
        assert!(passed_for(&a, id(10, 20), "base"));
        assert!(!passed_for(&a, id(11, 20), "base"), "a rebuilt executable");
        assert!(!passed_for(&a, id(10, 21), "base"), "a replaced program binary");
        // even a file copied onto the other identity's name is refused: the key text inside differs
        std::fs::copy(path_for(&a, id(10, 20)), path_for(&a, id(11, 20))).unwrap();
        assert!(!passed_for(&a, id(11, 20), "base"));
    }

    #[test]
    fn the_cache_is_unused_for_a_device_without_a_registered_binary() {
        let cache = CacheDir::new("unreg");
        let dir = cache.dir.clone();
        let a = info("unregistered");
        record_pass(&a, "base");
        assert!(!passed(&a, "base"));
        assert!(!dir.exists(), "nothing is written");
    }
}
