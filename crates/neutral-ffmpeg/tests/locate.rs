//! Resolution order of ffmpeg and ffprobe, using real files under CARGO_TARGET_TMPDIR.

mod common;

use std::env;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::{Path, PathBuf};

use neutral_ffmpeg::locate::resolve_tool;
use neutral_ffmpeg::Error;

fn exe(name: &str) -> String {
    if cfg!(windows) {
        format!("{}.exe", name)
    } else {
        name.to_string()
    }
}

fn touch(root: &Path, rel: &str) -> PathBuf {
    let p = root.join(rel);
    fs::create_dir_all(p.parent().unwrap()).unwrap();
    fs::write(&p, b"stub").unwrap();
    p
}

fn path_of(dirs: &[&Path]) -> OsString {
    env::join_paths(dirs.iter().copied()).unwrap()
}

#[test]
fn explicit_option_wins_over_environment_and_path() {
    let root = common::scratch("locate_explicit");
    let a = touch(&root, &format!("a/{}", exe("ffmpeg")));
    let b = touch(&root, &format!("b/{}", exe("ffmpeg")));
    let c = touch(&root, &format!("c/{}", exe("ffmpeg")));
    let path = path_of(&[c.parent().unwrap()]);
    let got = resolve_tool("ffmpeg", "FFMPEG", Some(&a), Some(b.as_os_str()), Some(&path)).unwrap();
    assert_eq!(got, a);
}

#[test]
fn environment_wins_over_path() {
    let root = common::scratch("locate_env");
    let b = touch(&root, &format!("b/{}", exe("ffmpeg")));
    let c = touch(&root, &format!("c/{}", exe("ffmpeg")));
    let path = path_of(&[c.parent().unwrap()]);
    let got = resolve_tool("ffmpeg", "FFMPEG", None, Some(b.as_os_str()), Some(&path)).unwrap();
    assert_eq!(got, b);
}

#[test]
fn path_is_searched_in_order_and_skips_missing_directories() {
    let root = common::scratch("locate_path");
    let first = touch(&root, &format!("one/{}", exe("ffprobe")));
    let second = touch(&root, &format!("two/{}", exe("ffprobe")));
    let path = path_of(&[first.parent().unwrap(), second.parent().unwrap()]);
    assert_eq!(resolve_tool("ffprobe", "FFPROBE", None, None, Some(&path)).unwrap(), first);
    let path = path_of(&[Path::new("definitely-missing-dir"), second.parent().unwrap()]);
    assert_eq!(resolve_tool("ffprobe", "FFPROBE", None, None, Some(&path)).unwrap(), second);
}

#[test]
fn bare_names_are_looked_up_on_path() {
    let root = common::scratch("locate_bare");
    let tool = touch(&root, &format!("bin/{}", exe("my-ffmpeg")));
    let path = path_of(&[tool.parent().unwrap()]);
    let got = resolve_tool("ffmpeg", "FFMPEG", Some(Path::new("my-ffmpeg")), None, Some(&path)).unwrap();
    assert_eq!(got, tool);
    let got = resolve_tool("ffmpeg", "FFMPEG", None, Some(OsStr::new("my-ffmpeg")), Some(&path)).unwrap();
    assert_eq!(got, tool);
}

#[test]
fn bad_explicit_option_does_not_fall_through() {
    let root = common::scratch("locate_bad_explicit");
    let good = touch(&root, &format!("g/{}", exe("ffmpeg")));
    let path = path_of(&[good.parent().unwrap()]);
    let err = resolve_tool("ffmpeg", "FFMPEG", Some(Path::new("no/such/ffmpeg")), Some(good.as_os_str()), Some(&path))
        .unwrap_err();
    match err {
        Error::ToolNotFound { tool, tried } => {
            assert_eq!(tool, "ffmpeg");
            assert!(tried.iter().any(|l| l.contains("explicit option") && l.contains("not a file")), "{:?}", tried);
        }
        other => panic!("{:?}", other),
    }
}

#[test]
fn bad_environment_value_does_not_fall_through() {
    let root = common::scratch("locate_bad_env");
    let good = touch(&root, &format!("g/{}", exe("ffprobe")));
    let path = path_of(&[good.parent().unwrap()]);
    let err = resolve_tool("ffprobe", "FFPROBE", None, Some(OsStr::new("no/such/ffprobe")), Some(&path)).unwrap_err();
    match err {
        Error::ToolNotFound { tried, .. } => {
            assert!(tried.iter().any(|l| l.contains("FFPROBE environment variable")), "{:?}", tried);
        }
        other => panic!("{:?}", other),
    }
}

#[test]
fn empty_environment_value_counts_as_unset() {
    let root = common::scratch("locate_empty_env");
    let good = touch(&root, &format!("g/{}", exe("ffmpeg")));
    let path = path_of(&[good.parent().unwrap()]);
    let got = resolve_tool("ffmpeg", "FFMPEG", None, Some(OsStr::new("")), Some(&path)).unwrap();
    assert_eq!(got, good);
}

#[test]
fn directories_are_not_accepted() {
    let root = common::scratch("locate_dir");
    fs::create_dir_all(root.join("ffmpeg")).unwrap();
    let path = path_of(&[root.as_path()]);
    assert!(resolve_tool("ffmpeg", "FFMPEG", None, None, Some(&path)).is_err());
}

#[test]
fn live_resolution_finds_a_runnable_ffmpeg_when_present() {
    let Some(bins) = common::require("live_resolution_finds_a_runnable_ffmpeg_when_present") else {
        return;
    };
    let run = common::ffmpeg(&bins, &["-version"]);
    assert!(run.ok, "ffmpeg -version failed: {}", run.stderr);
    assert!(String::from_utf8_lossy(&run.stdout).starts_with("ffmpeg version"));
}
