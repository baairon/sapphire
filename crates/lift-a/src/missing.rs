//! Placeholders for everything that is not in the dump (imports, strings, constants, vtables).
//! The use sites are listed in the missing-data table of the lift analysis. Imports are trait methods named
//! `stub_<hex rel_to_end>`; constants are `UNKNOWN_<hex rel>` with a `todo` value.
