# Missing data

The data, imports and constants the ported code reads that are not in the dump (missing references), how they are addressed, and how the harness stands in for them.

## Count

<!--f:missing_refs-->19,563<!--/f--> missing references, by kind:

<!--gen:missing-->
| kind | references | distinct items | functions |
|---|---|---|---|
| import | 9470 | 294 | 459 |
| const | 6009 | 5445 | 127 |
| other | 4036 | 2412 | 291 |
| vtable | 48 | 27 | 31 |
<!--/gen:missing-->

`docs/data/MISSING_DATA.md` lists the most referenced items of each kind.

## Coordinates

A missing reference is identified by `rel_to_end`: its address minus the address of the first byte after the dump. With the dump mapped at 0x10000000 and `CODE_N` its size, `rel_to_end = ADDR - 0x10000000 - CODE_N`. The oracle, the index, the crates and the vector files all use it.

| Context | Form |
|---|---|
| Oracle fault | `fault READ_UNMAPPED ... 0xADDR`, ADDR at or past 0x10000000 plus the dump size |
| Port | `env.data_va(REL)` for data, `env.stub_va(REL)` for import stubs ([harness: the Env trait](harness.md#the-env-trait)) |
| Placeholder | the port's stand-in for a missing reference is that `Env` access; no port uses the names `stub_<hex rel>` and `UNKNOWN_<hex rel>` that each crate's `missing` module declares |
| Lifted runtime | `CODE_N` in each `x86rt.rs` |

A `rel_to_end` below minus `CODE_N` points below the dump's first byte, into the header zone, which is not in the dump either.

## Kinds

| Kind | What it is | How the harness stands in |
|---|---|---|
| import | a call through an import stub (allocation, strings, streams, exceptions, system library functions); stub names in `docs/data/stub_names.tsv` | a trapped call with a seeded result ([assumptions: callee model](assumptions.md#callee-model)) |
| const | a constant read from a data section (floating point constants, masks, tables) | a seeded word, the same for the function and its port |
| other | strings and other data references | a seeded word or pointer |
| vtable | a virtual table address | an address in the world; calls through it are trapped |

With the binary's data sections, each missing reference would take its real value through the same `Env` access, with no change to the ports.

## Strings in registration code

Registration code is the large functions the bank names `RegisterExpert::RegisterExpert()` and `RegisterExpert::Setup...` (such as `RegisterExpert::SetupAudioRegs()`), the heaviest callers of the import stub `0x7b1` (`basic_string::__init(char const*, unsigned long)` in the bank). They build many `std::string` values through it, each with an immediate length. The characters are in a data section that is not in the dump: the ports reproduce the calls, lengths and data addresses, which the call trace compares, but not the characters. `crates/lift-4/src/models.rs` models the stub as `string_ctor`.
