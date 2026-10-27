//! Output and scratch paths, named the way `tools/apply.py` names them.
//!
//! - [`default_output`]: `<input stem>_<effect>.mp4` (`.mkv` with `--lossless`), with `_1`, `_2`,
//!   ... added while the name is taken.
//! - [`partial_path`]: `<output stem>.partial-<pid>.<ext>`, the file an encode writes before the
//!   output is verified and moved into place.
//! - [`temp_dir`] and [`make_scratch`]: the folder for the layer clips, the stage logs and the
//!   spool file of `vfx`, an `apply_XXXXXXXX` folder in the system temp folder.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::pytext::splitext;

/// `path` with `_1`, `_2`, ... before the extension while it exists.
pub fn unique_path(path: &str) -> String {
    if !Path::new(path).exists() {
        return path.to_string();
    }
    let (base, ext) = splitext(path);
    let mut i = 1;
    while Path::new(&format!("{}_{}{}", base, i, ext)).exists() {
        i += 1;
    }
    format!("{}_{}{}", base, i, ext)
}

/// The output path when `-o` is not given.
pub fn default_output(src: &str, operation: &str, ext: &str) -> String {
    let (base, _) = splitext(src);
    unique_path(&format!("{}_{}.{}", base, operation, ext.trim_start_matches('.')))
}

/// The in-progress name of an output: the extension stays last because ffmpeg picks the container from it.
pub fn partial_path(destination: &str) -> String {
    let (root, ext) = splitext(destination);
    format!("{}.partial-{}{}", root, std::process::id(), ext)
}

/// True when both paths name the same existing file (compared after resolving them).
pub fn same_file(a: &str, b: &str) -> bool {
    match (fs::canonicalize(a), fs::canonicalize(b)) {
        (Ok(x), Ok(y)) => {
            if cfg!(windows) {
                x.to_string_lossy().to_lowercase() == y.to_string_lossy().to_lowercase()
            } else {
                x == y
            }
        }
        _ => false,
    }
}

/// `os.path.lexists`: the path exists, a dangling link included.
pub fn lexists(path: &str) -> bool {
    fs::symlink_metadata(path).is_ok()
}

/// Remove a file, ignoring every error.
pub fn discard(path: impl AsRef<Path>) {
    let _ = fs::remove_file(path);
}

/// The system temp folder, chosen the way Python's `tempfile.gettempdir` does: `TMPDIR`, `TEMP`,
/// `TMP`, then the platform default. A variable that names a missing folder is skipped.
pub fn temp_dir(env: &dyn Fn(&str) -> Option<String>) -> PathBuf {
    for var in ["TMPDIR", "TEMP", "TMP"] {
        if let Some(value) = env(var).filter(|v| !v.is_empty()) {
            let dir = std::path::absolute(&value).unwrap_or_else(|_| PathBuf::from(&value));
            if dir.is_dir() {
                return dir;
            }
        }
    }
    std::env::temp_dir()
}

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn random_name() -> String {
    const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789_";
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos() as u64).unwrap_or(0);
    let mut x = nanos ^ ((std::process::id() as u64) << 32) ^ COUNTER.fetch_add(1, Ordering::Relaxed).wrapping_mul(0x9E37_79B9_7F4A_7C15);
    let mut name = String::from("apply_");
    for _ in 0..8 {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        name.push(CHARS[(x % CHARS.len() as u64) as usize] as char);
    }
    name
}

/// Create a new, empty `apply_XXXXXXXX` folder in `parent`.
pub fn make_scratch(parent: &Path) -> io::Result<PathBuf> {
    for _ in 0..100 {
        let dir = parent.join(random_name());
        match fs::create_dir(&dir) {
            Ok(()) => return Ok(dir),
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(e),
        }
    }
    Err(io::Error::new(io::ErrorKind::AlreadyExists, "no unused scratch folder name"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sandbox(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("sapphire-unit-{}-{}", name, std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn unique_path_counts_up() {
        let dir = sandbox("unique");
        let base = dir.join("clip_retime.mp4");
        let base_s = base.to_string_lossy().to_string();
        assert_eq!(unique_path(&base_s), base_s);
        fs::write(&base, b"x").unwrap();
        let second = unique_path(&base_s);
        assert!(second.ends_with("clip_retime_1.mp4"), "{}", second);
        fs::write(&second, b"x").unwrap();
        assert!(unique_path(&base_s).ends_with("clip_retime_2.mp4"));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn default_output_names() {
        let dir = sandbox("default");
        let src = dir.join("a.b.mov").to_string_lossy().to_string();
        let out = default_output(&src, "retime", "mp4");
        assert!(out.ends_with("a.b_retime.mp4"), "{}", out);
        let lossless = default_output(&src, "retime", ".mkv");
        assert!(lossless.ends_with("a.b_retime.mkv"), "{}", lossless);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn partial_keeps_the_extension_last() {
        let p = partial_path("out/clip.mp4");
        assert_eq!(p, format!("out/clip.partial-{}.mp4", std::process::id()));
        let none = partial_path("out/clip");
        assert_eq!(none, format!("out/clip.partial-{}", std::process::id()));
    }

    #[test]
    fn same_file_sees_through_relative_spellings() {
        let dir = sandbox("same");
        let f = dir.join("f.bin");
        fs::write(&f, b"x").unwrap();
        let a = f.to_string_lossy().to_string();
        let b = dir.join(".").join("f.bin").to_string_lossy().to_string();
        assert!(same_file(&a, &b));
        let other = dir.join("g.bin");
        fs::write(&other, b"x").unwrap();
        assert!(!same_file(&a, &other.to_string_lossy()));
        assert!(!same_file(&a, &dir.join("missing").to_string_lossy()));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn temp_dir_prefers_the_variables_in_python_order() {
        let dir = sandbox("temp");
        let a = dir.join("a");
        let b = dir.join("b");
        fs::create_dir_all(&a).unwrap();
        fs::create_dir_all(&b).unwrap();
        let (sa, sb) = (a.to_string_lossy().to_string(), b.to_string_lossy().to_string());
        let env = |name: &str| match name {
            "TMPDIR" => Some(dir.join("missing").to_string_lossy().to_string()),
            "TEMP" => Some(sa.clone()),
            "TMP" => Some(sb.clone()),
            _ => None,
        };
        assert_eq!(temp_dir(&env), a);
        let none = |_: &str| None;
        assert_eq!(temp_dir(&none), std::env::temp_dir());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn scratch_folders_are_new_and_named_like_apply_py() {
        let dir = sandbox("scratch");
        let one = make_scratch(&dir).unwrap();
        let two = make_scratch(&dir).unwrap();
        assert_ne!(one, two);
        let name = one.file_name().unwrap().to_string_lossy().to_string();
        assert!(name.starts_with("apply_") && name.len() == "apply_".len() + 8, "{}", name);
        assert!(one.is_dir());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn lexists_and_discard() {
        let dir = sandbox("lexists");
        let f = dir.join("f");
        assert!(!lexists(&f.to_string_lossy()));
        fs::write(&f, b"x").unwrap();
        assert!(lexists(&f.to_string_lossy()));
        discard(&f);
        assert!(!f.exists());
        discard(&f);
        let _ = fs::remove_dir_all(&dir);
    }
}
