//! Where the tests of the effect crates find the stored data of the original (recorded plans, emulator rows, fixtures and the program corpus).
//!
//! The data is not part of the repository. It lives in `VEC_DIR/effects/`, with the same relative paths the effect tree used
//! (`fixtures/...`, `fx_eval/data/orig/...`, `record_extract/programs/programs.tsv`, `wave_s9/...`). `VEC_DIR` defaults to
//! `crates/vectors`, the folder the lift crates read their vectors from. A test whose data is absent prints a `NOT RUN` line
//! and returns, as the conformance suite does for an absent peer.
use std::path::{Path, PathBuf};

/// The stored-data folder: `$VEC_DIR`, else `crates/vectors` of the workspace.
pub fn vec_dir() -> PathBuf {
    match std::env::var_os("VEC_DIR") {
        Some(v) => PathBuf::from(v),
        None => Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..").join("vectors"),
    }
}

/// The folder of the effect data: `VEC_DIR/effects`.
pub fn root() -> PathBuf {
    vec_dir().join("effects")
}

/// `VEC_DIR/effects/<rel>` as a path.
pub fn path(rel: &str) -> PathBuf {
    let mut p = root();
    for part in rel.split('/').filter(|s| !s.is_empty()) {
        p.push(part);
    }
    p
}

fn leak(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

/// `VEC_DIR/effects/<rel>` as a string that lives for the whole test run (a drop-in for a path literal).
pub fn path_str(rel: &str) -> &'static str {
    leak(path(rel).to_string_lossy().into_owned())
}

/// Like `path_str` with a trailing separator, for code that appends a file name to the string.
pub fn dir_str(rel: &str) -> &'static str {
    let mut s = path(rel).to_string_lossy().into_owned();
    s.push('/');
    leak(s)
}

/// The programs.tsv corpus the tests build effects from.
pub fn programs_tsv() -> &'static str {
    path_str("record_extract/programs/programs.tsv")
}

/// True when `VEC_DIR/effects/<rel>` exists. Otherwise prints a `NOT RUN` line that names the test and the missing path.
pub fn have(rel: &str) -> bool {
    let p = path(rel);
    if p.exists() {
        return true;
    }
    let t = std::thread::current();
    println!("NOT RUN {}: needs the stored effect data {} (set VEC_DIR; see docs/effects.md)", t.name().unwrap_or("test"), p.display());
    false
}

/// True when every `VEC_DIR/effects/<rel>` exists. Otherwise prints one `NOT RUN` line for the first missing path.
pub fn have_all(rels: &[&str]) -> bool {
    rels.iter().all(|r| have(r))
}

/// A scratch folder for the files tests write (never the source tree): `<temp>/sapphire_effects_out`.
pub fn out_dir() -> PathBuf {
    let d = std::env::temp_dir().join("sapphire_effects_out");
    let _ = std::fs::create_dir_all(&d);
    d
}
