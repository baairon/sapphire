//! Run-time data tables of the effect crates.
//!
//! Some tables the effects need (lookup tables, parameter ranges, class and combination tables) were taken from the original
//! plugin and are not part of this repository's main files. They are read at run time from a data folder:
//! `$EFFECTS_DATA_DIR` when set, else `crates/effects/runtime_data` (the folder of the optional data bundle). Each table has the
//! path it had in the effect tree (for example `fx_eval/data/CLASS_FLAGS.tsv`). When a table is absent the first use stops with a
//! message that names the table and the folder; `require_all` lets a command line tool check them all up front, and tests call
//! `have_tables` and print a `NOT RUN` line when the folder is absent, as the lift crates' vector tests do.
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

/// Every table the effect crates read at run time (path relative to the data folder).
pub const ALL: &[&str] = &[
    "data/STYLE_PARAM_INDEX.tsv",
    "data/TICKMAP_PARAM_RECORDS.tsv",
    "ext/wave_s4/rebuild_sincos/port/data/dtab_file.hex",
    "ext/wave_s4/rebuild_sincos/port/data/stab_file.hex",
    "fx_eval/data/CLASS_FLAGS.tsv",
    "fx_eval/data/IMG_ONLY_ARGS.tsv",
    "fx_eval/data/INT_TYPED_ARGS.tsv",
    "fx_eval/data/NS_PARAMS.tsv",
    "fx_eval/data/PARAM_INPUTS.tsv",
    "fx_eval/data/PARAM_INPUT_RECORDS.tsv",
    "fx_eval/data/PARAM_RANGES.tsv",
    "fx_eval/data/PARAM_RANGES_REC.tsv",
    "fx_eval/data/svml_cos_tab.bin",
    "fx_eval/data/svml_sin_tab.bin",
    "fx_exec/data/COMBOS.tsv",
    "fx_exec/data/COMBOS_EXEC.tsv",
    "fx_exec/src/RECORDED_CELLS.tsv",
];

/// The data folder: `$EFFECTS_DATA_DIR`, else `crates/effects/runtime_data`.
pub fn dir() -> PathBuf {
    match std::env::var_os("EFFECTS_DATA_DIR") {
        Some(v) => PathBuf::from(v),
        None => PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("runtime_data"),
    }
}

fn path(name: &str) -> PathBuf {
    let mut p = dir();
    for part in name.split('/') {
        p.push(part);
    }
    p
}

/// The message printed when a table is absent.
pub fn missing_message(name: &str) -> String {
    format!(
        "effects runtime data table {} not found in {}: the tables taken from the original plugin are an optional bundle (docs/effects.md); set EFFECTS_DATA_DIR to the folder that holds them",
        name,
        dir().display()
    )
}

fn load(name: &str) -> Vec<u8> {
    std::fs::read(path(name)).unwrap_or_else(|_| panic!("{}", missing_message(name)))
}

/// The bytes of a table, read once.
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

/// A table as text, read once.
pub fn text(name: &str) -> &'static str {
    let b = bytes(name);
    std::str::from_utf8(b).unwrap_or_else(|_| panic!("effects runtime data table {} is not UTF-8 text", name))
}

/// A table that reads like a `&str` constant: `static X: Table = Table::new("fx_eval/data/X.tsv");` then `X.lines()` or `&*X`.
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

/// `Ok` when every table is present, else the message for the first one that is not.
pub fn require_all() -> Result<(), String> {
    for n in ALL {
        if !path(n).is_file() {
            return Err(missing_message(n));
        }
    }
    Ok(())
}

/// For tests: true when every table is present; otherwise prints a `NOT RUN` line naming the test and returns false.
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
