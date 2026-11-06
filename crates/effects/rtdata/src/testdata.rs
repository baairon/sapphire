//! Test-only data: the copies of the host tables that tests compare the definitions with and sweep.
//!
//! The copies are NOT part of the repository and no product path uses this module. They live in the folder `effects/rtdata` of the stored
//! test data (`$VEC_DIR`, else `crates/vectors`, the folder the other effect tests read), with the relative paths named in `ALL`. A test
//! that needs them calls `have_tables()` and returns when it prints `NOT RUN`, as the lift crates' vector tests do.
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

/// The name of the folder inside `$VEC_DIR` that holds the copies.
pub const SUBFOLDER: &str = "effects/rtdata";

/// The copies tests read (path relative to the folder).
pub const ALL: &[&str] = &[
    "data/STYLE_PARAM_INDEX.tsv",
    "data/TICKMAP_PARAM_RECORDS.tsv",
    "fx_eval/data/CLASS_FLAGS.tsv",
    "fx_eval/data/IMG_ONLY_ARGS.tsv",
    "fx_eval/data/INT_TYPED_ARGS.tsv",
    "fx_eval/data/NS_PARAMS.tsv",
    "fx_eval/data/PARAM_INPUTS.tsv",
    "fx_eval/data/PARAM_INPUT_RECORDS.tsv",
    "fx_eval/data/PARAM_RANGES.tsv",
    "fx_eval/data/PARAM_RANGES_REC.tsv",
    "fx_exec/data/COMBOS.tsv",
    "fx_exec/data/COMBOS_EXEC.tsv",
    "fx_exec/src/RECORDED_CELLS.tsv",
];

/// The folder of the copies: `$VEC_DIR/effects/rtdata`, else `crates/vectors/effects/rtdata`.
pub fn dir() -> PathBuf {
    let base = match std::env::var_os("VEC_DIR") {
        Some(v) => PathBuf::from(v),
        None => PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("..").join("vectors"),
    };
    let mut p = base;
    for part in SUBFOLDER.split('/') {
        p.push(part);
    }
    p
}

fn path(name: &str) -> PathBuf {
    let mut p = dir();
    for part in name.split('/') {
        p.push(part);
    }
    p
}

/// The message printed when a copy is absent.
pub fn missing_message(name: &str) -> String {
    format!("test data {} not found in {}: the copies of the host tables are test data under $VEC_DIR/{} (docs/effects.md)", name, dir().display(), SUBFOLDER)
}

fn load(name: &str) -> Vec<u8> {
    std::fs::read(path(name)).unwrap_or_else(|_| panic!("{}", missing_message(name)))
}

/// The bytes of a copy, read once.
pub fn bytes(name: &str) -> &'static [u8] {
    static CACHE: OnceLock<Mutex<HashMap<String, &'static [u8]>>> = OnceLock::new();
    let m = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    {
        let g = m.lock().unwrap();
        if let Some(v) = g.get(name) {
            return v;
        }
    }
    let b: &'static [u8] = Box::leak(load(name).into_boxed_slice());
    m.lock().unwrap().insert(name.to_string(), b);
    b
}

/// A copy as text, read once.
pub fn text(name: &str) -> &'static str {
    std::str::from_utf8(bytes(name)).unwrap_or_else(|_| panic!("test data {} is not UTF-8 text", name))
}

/// A copy that reads like a `&str` constant: `static X: Table = Table::new("fx_eval/data/X.tsv");` then `X.lines()` or `&*X`.
pub struct Table {
    name: &'static str,
    cell: OnceLock<&'static str>,
}

impl Table {
    pub const fn new(name: &'static str) -> Table {
        Table { name, cell: OnceLock::new() }
    }
}

impl std::ops::Deref for Table {
    type Target = str;
    fn deref(&self) -> &str {
        self.cell.get_or_init(|| text(self.name))
    }
}

/// `Ok` when every copy is present, else the message for the first one that is not.
pub fn require_all() -> Result<(), String> {
    for n in ALL {
        if !path(n).is_file() {
            return Err(missing_message(n));
        }
    }
    Ok(())
}

/// For tests: true when every copy is present; otherwise prints a `NOT RUN` line naming the test and returns false.
pub fn have_tables() -> bool {
    match require_all() {
        Ok(()) => true,
        Err(m) => {
            let t = std::thread::current();
            println!("NOT RUN {}: {}", t.name().unwrap_or("test"), m);
            false
        }
    }
}
