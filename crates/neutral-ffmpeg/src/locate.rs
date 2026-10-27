//! Finding the ffmpeg and ffprobe executables.
//!
//! Resolution order for each tool:
//! 1. the explicit path set on [`Binaries`],
//! 2. the `FFMPEG` / `FFPROBE` environment variable,
//! 3. a search of `PATH` (with `.exe` appended on Windows).
//!
//! A higher priority source that is set but unusable is an error, not a reason
//! to fall through: a typo in a configured path should not silently run a
//! different ffmpeg.

use std::env;
use std::ffi::OsStr;
use std::path::{Component, Path, PathBuf};

use crate::error::{Error, Result};

/// Where to find the two executables. Unset fields use the environment and `PATH`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Binaries {
    /// Explicit path (or bare program name) of `ffmpeg`.
    pub ffmpeg: Option<PathBuf>,
    /// Explicit path (or bare program name) of `ffprobe`.
    pub ffprobe: Option<PathBuf>,
}

impl Binaries {
    /// Both tools resolved from the environment and `PATH`.
    pub fn new() -> Binaries {
        Binaries::default()
    }

    /// Use this `ffmpeg`.
    pub fn with_ffmpeg(mut self, path: impl Into<PathBuf>) -> Binaries {
        self.ffmpeg = Some(path.into());
        self
    }

    /// Use this `ffprobe`.
    pub fn with_ffprobe(mut self, path: impl Into<PathBuf>) -> Binaries {
        self.ffprobe = Some(path.into());
        self
    }

    /// Resolve `ffmpeg` now.
    pub fn ffmpeg_path(&self) -> Result<PathBuf> {
        resolve_tool(
            "ffmpeg",
            "FFMPEG",
            self.ffmpeg.as_deref(),
            env::var_os("FFMPEG").as_deref(),
            env::var_os("PATH").as_deref(),
        )
    }

    /// Resolve `ffprobe` now.
    pub fn ffprobe_path(&self) -> Result<PathBuf> {
        resolve_tool(
            "ffprobe",
            "FFPROBE",
            self.ffprobe.as_deref(),
            env::var_os("FFPROBE").as_deref(),
            env::var_os("PATH").as_deref(),
        )
    }
}

/// A single path component with no directory part, like `ffmpeg`.
fn is_bare_name(p: &Path) -> bool {
    let mut it = p.components();
    matches!((it.next(), it.next()), (Some(Component::Normal(_)), None))
}

fn file_candidates(dir: &Path, name: &OsStr) -> Vec<PathBuf> {
    let mut v = vec![dir.join(name)];
    if cfg!(windows) && Path::new(name).extension().is_none() {
        let mut with_ext = name.to_os_string();
        with_ext.push(".exe");
        v.push(dir.join(with_ext));
    }
    v
}

fn search_path(name: &OsStr, path_var: Option<&OsStr>) -> Option<PathBuf> {
    let path_var = path_var?;
    for dir in env::split_paths(path_var) {
        if dir.as_os_str().is_empty() {
            continue;
        }
        for candidate in file_candidates(&dir, name) {
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }
    None
}

/// Turn a configured value (explicit option or environment variable) into a file.
fn from_setting(setting: &Path, path_var: Option<&OsStr>) -> std::result::Result<PathBuf, String> {
    if is_bare_name(setting) {
        if let Some(found) = search_path(setting.as_os_str(), path_var) {
            return Ok(found);
        }
    }
    if setting.is_file() {
        return Ok(setting.to_path_buf());
    }
    if cfg!(windows) && setting.extension().is_none() {
        let with_ext = setting.with_extension("exe");
        if with_ext.is_file() {
            return Ok(with_ext);
        }
    }
    if is_bare_name(setting) {
        Err(format!("'{}' is not on PATH", setting.display()))
    } else {
        Err(format!("'{}' is not a file", setting.display()))
    }
}

/// Resolution logic with the environment passed in, so it can be tested without
/// touching the process environment.
pub fn resolve_tool(
    tool: &'static str,
    env_name: &'static str,
    explicit: Option<&Path>,
    env_value: Option<&OsStr>,
    path_var: Option<&OsStr>,
) -> Result<PathBuf> {
    let mut tried = Vec::new();
    if let Some(setting) = explicit {
        return from_setting(setting, path_var).map_err(|why| {
            tried.push(format!("explicit option: {}", why));
            Error::ToolNotFound { tool, tried }
        });
    }
    tried.push("explicit option: not set".to_string());
    if let Some(value) = env_value.filter(|v| !v.is_empty()) {
        return from_setting(Path::new(value), path_var).map_err(|why| {
            tried.push(format!("{} environment variable: {}", env_name, why));
            Error::ToolNotFound { tool, tried }
        });
    }
    tried.push(format!("{} environment variable: not set", env_name));
    if let Some(found) = search_path(OsStr::new(tool), path_var) {
        return Ok(found);
    }
    tried.push(match path_var {
        Some(_) => "PATH: no executable found".to_string(),
        None => "PATH: not set".to_string(),
    });
    Err(Error::ToolNotFound { tool, tried })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nothing_found_lists_every_place_tried() {
        let err = resolve_tool("ffmpeg", "FFMPEG", None, None, Some(OsStr::new(""))).unwrap_err();
        let shown = err.to_string();
        assert!(shown.contains("explicit option"));
        assert!(shown.contains("FFMPEG environment variable"));
        assert!(shown.contains("PATH"));
        let err = resolve_tool("ffmpeg", "FFMPEG", None, None, None).unwrap_err();
        assert!(err.to_string().contains("PATH: not set"));
    }

    #[test]
    fn bare_names_are_recognised() {
        assert!(is_bare_name(Path::new("ffmpeg")));
        assert!(is_bare_name(Path::new("ffmpeg.exe")));
        assert!(!is_bare_name(Path::new("bin/ffmpeg")));
        assert!(!is_bare_name(Path::new("./ffmpeg")));
        assert!(!is_bare_name(Path::new("")));
    }
}
