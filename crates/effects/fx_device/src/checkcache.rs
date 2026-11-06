//! The self-check result cache: which self-checks a device already passed, kept in the user's cache directory so a
//! process pays the self-check of a kernel group once per machine and driver instead of once per process.
//!
//! One file per device. The file name and the first lines carry the whole key: the platform, the device name, vendor
//! and version, the driver version, a format number and the FNV-1a digest of the full device program text. The
//! rest of the file lists the tags (`base`, or an op class) that passed. A changed kernel changes the digest, a new
//! driver changes the key, so either one makes the old file unreadable and every check runs again. Only passes are
//! recorded; a failed check is never cached. The file is written to a unique temporary name and renamed into place.
//!
//! `SAPPHIRE_DEVICE_CHECK_CACHE=0` turns the cache off (every check runs). `SAPPHIRE_CACHE_DIR` replaces the user
//! cache directory (the tests use it). A read or write error means the check runs; the cache never changes what a
//! kernel computes.

use crate::kernels;
use crate::probe::DeviceInfo;
use crate::progcache::fnv;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::OnceLock;

const MAGIC: &str = "sapphire selfcheck 1";

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

fn key_text(info: &DeviceInfo) -> String {
    format!("{MAGIC}\n{}\n{}\n{}\n{}\n{}\nprogram {:016x}", info.platform, info.name, info.vendor, info.version, info.driver, program_digest())
}

fn path(info: &DeviceInfo) -> PathBuf {
    folder().join(format!("selfcheck-{:016x}.txt", fnv(key_text(info).as_bytes())))
}

fn read_tags(info: &DeviceInfo) -> Vec<String> {
    let Ok(text) = std::fs::read_to_string(path(info)) else { return Vec::new() };
    let key = key_text(info);
    match text.strip_prefix(&key).and_then(|r| r.strip_prefix('\n')) {
        Some(rest) => rest.lines().filter(|l| !l.is_empty()).map(str::to_string).collect(),
        None => Vec::new(),
    }
}

/// True when `tag` passed on this device, driver and program text in an earlier run.
pub fn passed(info: &DeviceInfo, tag: &str) -> bool {
    enabled() && read_tags(info).iter().any(|t| t == tag)
}

/// Records that `tag` passed (merged with what the file already holds; written atomically). Errors are ignored.
pub fn record_pass(info: &DeviceInfo, tag: &str) {
    if !enabled() {
        return;
    }
    let mut tags = read_tags(info);
    if tags.iter().any(|t| t == tag) {
        return;
    }
    tags.push(tag.to_string());
    tags.sort();
    let mut body = key_text(info);
    body.push('\n');
    for t in &tags {
        body.push_str(t);
        body.push('\n');
    }
    let final_path = path(info);
    static SEQ: AtomicU64 = AtomicU64::new(0);
    let tmp = final_path.with_extension(format!("tmp{}.{}", std::process::id(), SEQ.fetch_add(1, Ordering::Relaxed)));
    if std::fs::create_dir_all(folder()).is_err() {
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

    fn info(driver: &str) -> DeviceInfo {
        DeviceInfo { index: 0, platform: "P".into(), platform_version: "1".into(), name: "D".into(), board: String::new(), vendor: "V".into(), version: "OpenCL 3.0".into(), driver: driver.into(), c_version: String::new(), profile: "FULL_PROFILE".into(), device_type: 4, little_endian: true, available: true, compiler_available: true, global_memory: 0, max_allocation: 0, ..Default::default() }
    }

    #[test]
    fn a_pass_is_remembered_per_driver_and_tag() {
        let dir = std::env::temp_dir().join(format!("sapphire-checkcache-test-{}", std::process::id()));
        std::env::set_var("SAPPHIRE_CACHE_DIR", &dir);
        let a = info("1.0");
        assert!(!passed(&a, "base"));
        record_pass(&a, "base");
        record_pass(&a, "copy");
        assert!(passed(&a, "base") && passed(&a, "copy") && !passed(&a, "one_minus"));
        assert!(!passed(&info("2.0"), "base"), "a new driver version invalidates the entry");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
