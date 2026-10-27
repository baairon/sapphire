//! Placeholders for everything that is not in the dump (imports, strings, constants, vtables).
//! See findings/port-a/MISSING_DATA.tsv for the use sites. Imports are trait methods named
//! `stub_<hex rel_to_end>`; constants are `UNKNOWN_<hex rel>` with a `todo` value.
