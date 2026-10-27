//! Placeholders for everything the dump does not contain (imports, constants tables, strings, vtables).
//! Imports are trait methods named `stub_<hex rel>` (rel = target minus dump size); constants are
//! `UNKNOWN_<hex rel>`. Each one is recorded in MISSING_DATA.tsv.

pub trait Imports {}
