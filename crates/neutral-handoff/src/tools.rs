//! Finding the programs a run needs: ffmpeg, ffprobe, `vfx` and the stage commands.
//!
//! The lookup orders are those of `tools/apply.py`:
//!
//! | Program | Order |
//! |---|---|
//! | ffmpeg | `--ffmpeg-path`, `MEDIA_FFMPEG_PATH`, `FFMPEG`, `PATH`, then on Windows `%LOCALAPPDATA%\Microsoft\WinGet\Links`, `%ProgramFiles%\ffmpeg\bin`, `%ProgramFiles(x86)%\ffmpeg\bin` |
//! | ffprobe | the same with `--ffprobe-path`, `MEDIA_FFPROBE_PATH`, `FFPROBE` |
//! | `vfx` | `--vfx`, `VFX_PATH`, `$CARGO_TARGET_DIR/release`, the folder of the running executable, the twixtor checkout's `target/release` (`HANDOFF_TWIXTOR_DIR`, else a sibling of this repository found by its markers, never by its folder name), then `PATH` |
//!
//! A name given on purpose (a flag or a variable) that cannot be found is an error and never
//! falls through to the next place, so a typo cannot run another build.

use std::env;
use std::path::{Path, PathBuf};

use crate::error::{Error, Result};

/// How a lookup reads the environment; tests pass their own.
pub type EnvFn<'a> = &'a dyn Fn(&str) -> Option<String>;

/// The process environment.
pub fn process_env(name: &str) -> Option<String> {
    env::var_os(name).map(|v| v.to_string_lossy().into_owned())
}

fn is_file(path: &Path) -> bool {
    path.is_file()
}

/// `shutil.which`: a name with a folder part is checked where it is; a bare name is searched in
/// `path_var` (the current folder first on Windows), with the `PATHEXT` extensions on Windows.
pub fn which(cmd: &str, path_var: Option<&str>, pathext: Option<&str>) -> Option<PathBuf> {
    let windows = cfg!(windows);
    let exts: Vec<String> = if windows {
        pathext
            .filter(|p| !p.is_empty())
            .unwrap_or(".COM;.EXE;.BAT;.CMD;.VBS;.JS;.WS;.MSC")
            .split(';')
            .filter(|e| !e.is_empty())
            .map(|e| e.to_string())
            .collect()
    } else {
        Vec::new()
    };
    let names: Vec<String> = if windows {
        if exts.iter().any(|e| cmd.to_lowercase().ends_with(&e.to_lowercase())) {
            vec![cmd.to_string()]
        } else {
            let mut v: Vec<String> = exts.iter().map(|e| format!("{}{}", cmd, e)).collect();
            v.push(cmd.to_string());
            v
        }
    } else {
        vec![cmd.to_string()]
    };
    if cmd.contains('/') || (windows && cmd.contains('\\')) {
        for name in &names {
            let p = PathBuf::from(name);
            if is_file(&p) {
                return Some(p);
            }
        }
        return None;
    }
    let mut dirs: Vec<PathBuf> = Vec::new();
    if windows {
        dirs.push(PathBuf::from("."));
    }
    if let Some(path) = path_var {
        dirs.extend(env::split_paths(path).filter(|d| !d.as_os_str().is_empty()));
    }
    for dir in dirs {
        for name in &names {
            let candidate = dir.join(name);
            if is_file(&candidate) {
                return Some(candidate);
            }
        }
    }
    None
}

fn which_env(cmd: &str, env: EnvFn) -> Option<PathBuf> {
    which(cmd, env("PATH").as_deref(), env("PATHEXT").as_deref())
}

/// ffmpeg or ffprobe. `explicit` is the flag, `env_names` the variables to try in order.
///
/// `Ok(None)` means nothing was found anywhere; `Err` means a name was given and does not exist.
pub fn resolve_media_tool(name: &str, explicit: Option<&str>, env_names: &[&str], env: EnvFn) -> Result<Option<PathBuf>> {
    let mut override_value = explicit.filter(|v| !v.is_empty()).map(|v| v.to_string());
    if override_value.is_none() {
        for var in env_names {
            if let Some(v) = env(var).filter(|v| !v.is_empty()) {
                override_value = Some(v);
                break;
            }
        }
    }
    if let Some(value) = override_value {
        return match which_env(&value, env) {
            Some(p) => Ok(Some(p)),
            None => Err(Error::tool_missing(format!("explicit {} executable not found: {}", name, value)).at(name)),
        };
    }
    if let Some(p) = which_env(name, env) {
        return Ok(Some(p));
    }
    let mut candidates: Vec<PathBuf> = Vec::new();
    if let Some(local) = env("LOCALAPPDATA") {
        candidates.push(Path::new(&local).join("Microsoft").join("WinGet").join("Links").join(format!("{}.exe", name)));
    }
    for var in ["ProgramFiles", "ProgramFiles(x86)"] {
        if let Some(root) = env(var) {
            candidates.push(Path::new(&root).join("ffmpeg").join("bin").join(format!("{}.exe", name)));
        }
    }
    Ok(candidates.into_iter().find(|c| is_file(c)))
}

/// The two programs every run needs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Media {
    /// The ffmpeg executable.
    pub ffmpeg: PathBuf,
    /// The ffprobe executable.
    pub ffprobe: PathBuf,
}

/// The text of the refusal when ffmpeg or ffprobe is missing (as `apply.py` words it).
pub const INSTALL_FFMPEG: &str = "this operation requires ffmpeg and ffprobe.\n  winget install Gyan.FFmpeg        (windows)\n  brew install ffmpeg               (macos)\n  sudo apt install ffmpeg           (debian/ubuntu)";

/// Find ffmpeg and ffprobe or say how to install them.
pub fn require_media(ffmpeg_flag: Option<&str>, ffprobe_flag: Option<&str>, env: EnvFn) -> Result<Media> {
    let ffmpeg = resolve_media_tool("ffmpeg", ffmpeg_flag, &["MEDIA_FFMPEG_PATH", "FFMPEG"], env)?;
    let ffprobe = resolve_media_tool("ffprobe", ffprobe_flag, &["MEDIA_FFPROBE_PATH", "FFPROBE"], env)?;
    match (ffmpeg, ffprobe) {
        (Some(ffmpeg), Some(ffprobe)) => Ok(Media { ffmpeg, ffprobe }),
        _ => Err(Error::tool_missing(INSTALL_FFMPEG).at("plan")),
    }
}

/// The file name of the effect renderer on this platform.
pub fn vfx_exe_name() -> &'static str {
    if cfg!(windows) {
        "vfx.exe"
    } else {
        "vfx"
    }
}

/// The markers that `contracts/tool.json` in `dir` lists for the tool `id`; empty when the file
/// is missing or names no such tool.
pub fn tool_markers(dir: &Path, id: &str) -> Vec<String> {
    let Ok(text) = std::fs::read_to_string(dir.join("contracts").join("tool.json")) else {
        return Vec::new();
    };
    let compact: String = text.chars().filter(|c| !c.is_whitespace()).collect();
    let Some(at) = compact.find(&format!("\"id\":\"{}\"", id)) else {
        return Vec::new();
    };
    let rest = &compact[at..];
    let key = "\"markers\":[";
    let Some(start) = rest.find(key) else {
        return Vec::new();
    };
    let list = &rest[start + key.len()..];
    let list = &list[..list.find(']').unwrap_or(0)];
    list.split(',').map(|m| m.trim_matches('"').to_string()).filter(|m| !m.is_empty()).collect()
}

/// Whether `dir` is a twixtor checkout, by its markers and never by its folder name: its
/// `contracts/tool.json` names the `twixtor` tool and every marker listed for it is there, or
/// `tools/apply.py` sits beside `contracts/`.
pub fn is_twixtor_checkout(dir: &Path) -> bool {
    if !dir.join("contracts").is_dir() {
        return false;
    }
    let markers = tool_markers(dir, "twixtor");
    dir.join("tools").join("apply.py").is_file() || (!markers.is_empty() && markers.iter().all(|m| dir.join(m).exists()))
}

/// The twixtor checkouts to look in: `HANDOFF_TWIXTOR_DIR` when it is set, else the siblings of
/// this repository that pass [`is_twixtor_checkout`], in path order.
pub fn twixtor_checkouts(env: EnvFn) -> Vec<PathBuf> {
    if let Some(dir) = env("HANDOFF_TWIXTOR_DIR").filter(|v| !v.is_empty()) {
        return vec![PathBuf::from(dir)];
    }
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..");
    let own = repo.canonicalize().ok();
    let mut found: Vec<PathBuf> = std::fs::read_dir(repo.join(".."))
        .map(|rd| rd.filter_map(|e| e.ok().map(|e| e.path())).filter(|p| p.is_dir() && p.canonicalize().ok() != own && is_twixtor_checkout(p)).collect())
        .unwrap_or_default();
    found.sort();
    found
}

/// Find `vfx`: `--vfx`, `VFX_PATH`, the build folders, then `PATH`.
pub fn find_vfx(explicit: Option<&str>, env: EnvFn, exe_dir: Option<&Path>, extra_dirs: &[PathBuf]) -> Result<PathBuf> {
    let absolute = |p: &Path| std::path::absolute(p).unwrap_or_else(|_| p.to_path_buf());
    if let Some(value) = explicit.filter(|v| !v.is_empty()) {
        let p = Path::new(value);
        if !p.is_file() {
            return Err(Error::tool_missing(format!("vfx not found at {}", value)).at("vfx"));
        }
        return Ok(absolute(p));
    }
    if let Some(value) = env("VFX_PATH").filter(|v| !v.is_empty()) {
        let p = Path::new(&value);
        if !p.is_file() {
            return Err(Error::tool_missing(format!("VFX_PATH points at a missing file: {}", value)).at("vfx"));
        }
        return Ok(absolute(p));
    }
    let mut dirs: Vec<PathBuf> = Vec::new();
    if let Some(target) = env("CARGO_TARGET_DIR").filter(|v| !v.is_empty()) {
        dirs.push(Path::new(&target).join("release"));
    }
    if let Some(dir) = exe_dir {
        dirs.push(dir.to_path_buf());
    }
    dirs.extend(extra_dirs.iter().cloned());
    for dir in dirs {
        let candidate = dir.join(vfx_exe_name());
        if is_file(&candidate) {
            return Ok(absolute(&candidate));
        }
    }
    if let Some(found) = which_env("vfx", env) {
        return Ok(found);
    }
    Err(Error::tool_missing(
        "vfx not found. build it with `cargo build --release -p fx-cli` in the twixtor repository, or pass --vfx PATH / set VFX_PATH",
    )
    .at("vfx"))
}

/// `find_vfx` with the process environment and the default places.
pub fn find_vfx_default(explicit: Option<&str>) -> Result<PathBuf> {
    let exe_dir = env::current_exe().ok().and_then(|p| p.parent().map(|d| d.to_path_buf()));
    // a path given on purpose decides alone, so the sibling folders are listed only without one
    let given = explicit.is_some_and(|v| !v.is_empty()) || process_env("VFX_PATH").is_some_and(|v| !v.is_empty());
    let checkouts = if given { Vec::new() } else { twixtor_checkouts(&process_env) };
    let releases: Vec<PathBuf> = checkouts.iter().map(|d| d.join("target").join("release")).collect();
    find_vfx(explicit, &process_env, exe_dir.as_deref(), &releases)
}

/// Check that the program of a stage command exists before anything runs. Returns its path.
pub fn require_program(program: &str, env: EnvFn, stage: &str) -> Result<PathBuf> {
    match which_env(program, env) {
        Some(p) => Ok(p),
        None => Err(Error::tool_missing(format!("{}: the program `{}` was not found", stage, program)).at(stage)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn sandbox(name: &str) -> PathBuf {
        let dir = env::temp_dir().join(format!("sapphire-tools-{}-{}", name, std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn touch(dir: &Path, name: &str) -> PathBuf {
        let p = dir.join(name);
        fs::write(&p, b"x").unwrap();
        p
    }

    fn exe(name: &str) -> String {
        if cfg!(windows) {
            format!("{}.exe", name)
        } else {
            name.to_string()
        }
    }

    #[test]
    fn which_finds_a_bare_name_on_the_path() {
        let dir = sandbox("which");
        let p = touch(&dir, &exe("sometool"));
        let found = which("sometool", Some(&dir.to_string_lossy()), Some(".EXE")).expect("found");
        assert!(found.to_string_lossy().to_lowercase().ends_with(&exe("sometool").to_lowercase()), "{:?}", found);
        assert!(found.is_file());
        assert!(which("nosuchtool", Some(&dir.to_string_lossy()), Some(".EXE")).is_none());
        let direct = which(&p.to_string_lossy(), None, None).expect("a path with a folder is checked where it is");
        assert!(direct.is_file());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn explicit_flag_beats_variables_and_never_falls_through() {
        let dir = sandbox("explicit");
        let good = touch(&dir, &exe("mine"));
        let other = touch(&dir, &exe("other"));
        let env_map = |name: &str| match name {
            "MEDIA_FFMPEG_PATH" => Some(other.to_string_lossy().to_string()),
            "PATH" => Some(dir.to_string_lossy().to_string()),
            _ => None,
        };
        let found = resolve_media_tool("ffmpeg", Some(&good.to_string_lossy()), &["MEDIA_FFMPEG_PATH", "FFMPEG"], &env_map)
            .unwrap()
            .unwrap();
        assert_eq!(found, good);
        let from_env = resolve_media_tool("ffmpeg", None, &["MEDIA_FFMPEG_PATH", "FFMPEG"], &env_map).unwrap().unwrap();
        assert_eq!(from_env, other);
        let err = resolve_media_tool("ffmpeg", Some("definitely-not-here"), &["FFMPEG"], &env_map).unwrap_err();
        assert_eq!(err.code, crate::error::Code::ToolMissing);
        assert_eq!(err.message, "explicit ffmpeg executable not found: definitely-not-here");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn nothing_found_is_none_then_the_install_message() {
        let empty = sandbox("empty");
        let env_map = |name: &str| if name == "PATH" { Some(empty.to_string_lossy().to_string()) } else { None };
        assert_eq!(resolve_media_tool("ffmpeg", None, &["FFMPEG"], &env_map).unwrap(), None);
        let err = require_media(None, None, &env_map).unwrap_err();
        assert!(err.message.starts_with("this operation requires ffmpeg and ffprobe."));
        assert_eq!(err.exit_code(), 6);
        let _ = fs::remove_dir_all(&empty);
    }

    #[test]
    fn winget_links_are_a_last_resort() {
        let local = sandbox("winget");
        let links = local.join("Microsoft").join("WinGet").join("Links");
        fs::create_dir_all(&links).unwrap();
        let p = touch(&links, "ffprobe.exe");
        let env_map = |name: &str| match name {
            "LOCALAPPDATA" => Some(local.to_string_lossy().to_string()),
            "PATH" => Some(String::new()),
            _ => None,
        };
        assert_eq!(resolve_media_tool("ffprobe", None, &["FFPROBE"], &env_map).unwrap(), Some(p));
        let _ = fs::remove_dir_all(&local);
    }

    #[test]
    fn twixtor_is_found_by_markers_not_by_folder_name() {
        let dir = sandbox("markers");
        let by_apply = dir.join("one");
        fs::create_dir_all(by_apply.join("contracts")).unwrap();
        fs::create_dir_all(by_apply.join("tools")).unwrap();
        fs::write(by_apply.join("tools").join("apply.py"), "").unwrap();
        assert!(is_twixtor_checkout(&by_apply));
        let tool_json = r#"{"tools": [{"id": "sapphire", "markers": ["crates/neutral-handoff/Cargo.toml"]}, {"id": "twixtor", "markers": ["crates/fx-cli/Cargo.toml"]}]}"#;
        let by_json = dir.join("two");
        fs::create_dir_all(by_json.join("contracts")).unwrap();
        fs::write(by_json.join("contracts").join("tool.json"), tool_json).unwrap();
        assert_eq!(tool_markers(&by_json, "twixtor"), ["crates/fx-cli/Cargo.toml"]);
        assert!(!is_twixtor_checkout(&by_json), "a tool.json alone, as in every repository that vendors contracts/");
        fs::create_dir_all(by_json.join("crates").join("fx-cli")).unwrap();
        fs::write(by_json.join("crates").join("fx-cli").join("Cargo.toml"), "").unwrap();
        assert!(is_twixtor_checkout(&by_json));
        let named = dir.join("twixtor");
        fs::create_dir_all(named.join("target").join("release")).unwrap();
        assert!(!is_twixtor_checkout(&named), "the folder name counts for nothing");
        let pinned = |name: &str| (name == "HANDOFF_TWIXTOR_DIR").then(|| named.to_string_lossy().to_string());
        assert_eq!(twixtor_checkouts(&pinned), [named.clone()]);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn vfx_lookup_order() {
        let dir = sandbox("vfx");
        let a = dir.join("a");
        let b = dir.join("b");
        let c = dir.join("c");
        for d in [&a, &b, &c] {
            fs::create_dir_all(d).unwrap();
        }
        let in_a = touch(&a, vfx_exe_name());
        let in_b = touch(&b, vfx_exe_name());
        let flagged = touch(&c, "custom-vfx");
        let none = |_: &str| None;
        let abs = |p: &Path| std::path::absolute(p).unwrap();
        let by_flag = find_vfx(Some(&flagged.to_string_lossy()), &none, Some(&a), &[b.clone()]).unwrap();
        assert_eq!(by_flag, abs(&flagged));
        let by_env = |name: &str| if name == "VFX_PATH" { Some(flagged.to_string_lossy().to_string()) } else { None };
        assert_eq!(find_vfx(None, &by_env, Some(&a), &[]).unwrap(), abs(&flagged));
        assert_eq!(find_vfx(None, &none, Some(&a), &[b.clone()]).unwrap(), abs(&in_a), "the running executable's folder first");
        assert_eq!(find_vfx(None, &none, Some(&c), &[b.clone()]).unwrap(), abs(&in_b), "then the sibling checkout");
        let target = |name: &str| if name == "CARGO_TARGET_DIR" { Some(dir.to_string_lossy().to_string()) } else { None };
        fs::create_dir_all(dir.join("release")).unwrap();
        let in_target = touch(&dir.join("release"), vfx_exe_name());
        assert_eq!(find_vfx(None, &target, Some(&a), &[]).unwrap(), abs(&in_target), "the target folder beats the executable's");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn vfx_errors_name_the_place_that_failed() {
        let none = |_: &str| None;
        let err = find_vfx(Some("no/such/vfx"), &none, None, &[]).unwrap_err();
        assert_eq!(err.message, "vfx not found at no/such/vfx");
        let env_map = |name: &str| if name == "VFX_PATH" { Some("gone.exe".to_string()) } else { None };
        let err = find_vfx(None, &env_map, None, &[]).unwrap_err();
        assert_eq!(err.message, "VFX_PATH points at a missing file: gone.exe");
        let nothing = |name: &str| if name == "PATH" { Some(String::new()) } else { None };
        let err = find_vfx(None, &nothing, None, &[]).unwrap_err();
        assert!(err.message.starts_with("vfx not found. build it with"));
        assert_eq!(err.code, crate::error::Code::ToolMissing);
    }

    #[test]
    fn a_missing_stage_program_is_tool_missing() {
        let nothing = |name: &str| if name == "PATH" { Some(String::new()) } else { None };
        let err = require_program("no-such-stage-program", &nothing, "stage2").unwrap_err();
        assert_eq!(err.code, crate::error::Code::ToolMissing);
        assert_eq!(err.stage.as_deref(), Some("stage2"));
        assert_eq!(err.message, "stage2: the program `no-such-stage-program` was not found");
    }
}
