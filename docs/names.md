# Names

Every function, class, stub, effect and parameter name comes from the name bank. Names live in tables and documents; the code keeps its `f_XXXXXXXX` symbols. A name is the bank's assignment for its offset.

## The name bank

| File | Rows | Key columns |
|---|---|---|
| `docs/data/names.tsv` | <!--f:names_total-->2,702<!--/f--> functions | `symbol` (the id), `offset`, `exact_name` (exact C++ name), `rust_label`, `kind`, `origin`, `mangled`, `source` (the crate file of the port) |
| `docs/data/stub_names.tsv` | <!--f:stubs_total-->458<!--/f--> import stubs | `stub_rel_off`, `exact_target`, `library_or_origin`, `mangled` |
| `docs/data/classes_names.tsv` | <!--f:classes_total-->16<!--/f--> local classes | `vtable_offset`, `class_name`, `base_class`, `mangled_vtable` |
| `docs/data/effect_names.tsv` | <!--f:effects_total-->272<!--/f--> effect names | `name`, `file_offset`, `virtual_address`, `section` |
| `docs/data/param_candidates.tsv` | <!--f:params_total-->1,040<!--/f--> parameter candidates | `candidate`, `file_offset`, `virtual_address`, `section` |

All <!--f:fn_total-->2,702<!--/f--> function ids of the bank agree with `docs/data/FUNCTION_MAP.tsv`.

## Origin

<!--gen:origins-->
| origin | method | std | function | destructor | constructor | initialiser | total |
|---|---|---|---|---|---|---|---|
| bundled_library | 1,376 | 0 | 369 | 139 | 99 | 0 | 1,983 |
| std | 0 | 514 | 0 | 0 | 0 | 0 | 514 |
| undetermined | 7 | 0 | 109 | 8 | 5 | 7 | 136 |
| component | 49 | 0 | 0 | 12 | 8 | 0 | 69 |
<!--/gen:origins-->

Per the bank, <!--f:origin_bundled-->1,983<!--/f--> of the <!--f:fn_total-->2,702<!--/f--> functions are bundled library code, <!--f:origin_std-->514<!--/f--> std, <!--f:origin_undetermined-->136<!--/f--> undetermined and <!--f:origin_component-->69<!--/f--> component. The largest class, `CNTV2Card`, has <!--f:cntv2card_fns-->906<!--/f--> functions.

<!--gen:name_kinds-->
| kind | functions |
|---|---|
| method | 1,432 |
| std | 514 |
| function | 478 |
| destructor | 159 |
| constructor | 112 |
| initialiser | 7 |
<!--/gen:name_kinds-->

## Stubs

The <!--f:stubs_total-->458<!--/f--> import stubs are named by their target and where the bank places it:

<!--gen:stub_origins-->
| library or origin | stubs |
|---|---|
| `local weak definition` | 299 |
| `/usr/lib/libc++.1.dylib` | 79 |
| `/usr/lib/libSystem.B.dylib` | 61 |
| `/System/Library/Frameworks/IOKit.framework/Versions/A/IOKit` | 12 |
| `/System/Library/Frameworks/CoreFoundation.framework/Versions/A/CoreFoundation` | 7 |
<!--/gen:stub_origins-->

## Effects and parameters

The bank holds the <!--f:effects_total-->272<!--/f--> effect names and the <!--f:params_total-->1,040<!--/f--> parameter candidates as strings with their file offsets, for example `S_Aurora`, `S_AutoPaint` and `S_BandPass`. No effect is mapped to functions, classes, parameters or behaviour, and no candidate is bound to an effect or function; a candidate is not a confirmed parameter.

| List | Page |
|---|---|
| effect names, verbatim | [generated/effects.md](generated/effects.md) |
| parameter candidates, verbatim | [generated/parameters.md](generated/parameters.md) |

## Name check

Each bank name was compared with its function's code: argument registers against the signature, getter and setter partners, vtable stores of constructors and destructors, callee names, size and shape. Verdicts are in `docs/data/namecheck_classes.tsv` and `docs/data/namecheck_rest.tsv`, stub verdicts in `docs/data/namecheck_stubs.tsv`; they are inputs to the confirmation levels, not conclusions.

| Verdict | Functions |
|---|---|
| checked | <!--f:nc_checked-->2,700<!--/f--> |
| align | <!--f:nc_align-->2,188<!--/f--> (<!--f:nc_align_high-->664<!--/f--> high, <!--f:nc_align_med-->947<!--/f--> medium, <!--f:nc_align_low-->577<!--/f--> low confidence) |
| weak (no contradiction, no specific support) | <!--f:nc_weak-->511<!--/f--> |
| mismatch | <!--f:nc_mismatch-->1<!--/f--> (`f_0006590c`, an 8 byte placeholder body) |

| Test of the mapping as a whole | Result |
|---|---|
| Argument counts against the bank signatures | fit <!--f:arity_fit_pct-->97.2<!--/f-->% of <!--f:arity_fit_base-->2,353<!--/f--> lifted functions, against about <!--f:arity_shift_pct-->68<!--/f-->% with every name shifted by one function: no systematic offset shift |
| Class hierarchies | all <!--f:class_hier_match-->16<!--/f--> match the structure inferred from the code; all <!--f:ctor_dtor_own_vtable-->54<!--/f--> constructors and destructors that reference a local vtable reference their own class |
| Stubs | <!--f:stub_align-->183<!--/f--> align, <!--f:stub_weak-->275<!--/f--> weak, <!--f:stub_mismatch-->0<!--/f--> mismatch |

## Confirmation levels

Each function has one level in `docs/data/name_confirmation.tsv` (columns `symbol`, `name`, `level`, `evidence_ids`, `evidence`):

<!--gen:confirmation-->
| confirmation | functions | meaning |
|---|---|---|
| confirmed | 1,085 | at least two independent positive pieces of evidence |
| consistent | 875 | one positive piece of evidence, nothing against |
| twin | 432 | flagged by the name check as interchangeable with a differently named function (`twin_interchangeable`); the code cannot decide |
| stub | 68 | constant return or empty body; the code cannot decide |
| weak | 241 | no positive evidence from the code |
| mismatch | 1 | evidence against the bank name |
<!--/gen:confirmation-->

A twin is a function the name check flagged as interchangeable with a differently named function (`twin_interchangeable` in `docs/data/namecheck_rest.tsv`, <!--f:twin_fns-->432<!--/f--> functions); <!--f:const_result_fns-->10<!--/f--> functions carry the `CONSTANT_RESULT` flag. `twin` and `stub` cannot be decided from the code and are never relabelled as confirmed.

| Evidence id | Positive evidence |
|---|---|
| `arity` | the argument registers read equal the signature, two or more (`this` counts) |
| `vtable` | a constructor or destructor stores its own class vtable |
| `base` | a constructor or destructor calls the matching base class constructor or destructor |
| `delete` | a destructor calls its own destructor, then the operator delete stub |
| `regs` | a getter whose body reads a register, or a setter whose body writes one |
| `pair` | a Get or Set name that shares its register with the partner of the opposite verb |
| `fields` | a `this` offset above 0x8 shared with another function of the same class |
| `calls` | a caller or callee of the same class, or one sharing a rare type or identifier with the name |
| `stubs` | an import stub target sharing a rare identifier with the name |
| `consts` | constants shared with a named partner: index tables, sibling registers, string or device id constants |
| `shape` | the body shape fits the name: constructor, destructor, getter, setter, predicate, string builder or static initialiser |
| `deep` | a deep read found the name aligned, at high or medium confidence |
| `know` | knowledge of a library the bank names, not evidence from the repository; counts toward `consistent` only, and <!--f:conf_consistent_know_only-->69<!--/f--> `consistent` rows rest on it alone |
| `-` | none |

Evidence against: an argument register count unlike the signature, a kind that contradicts the body, a severity of 2 or more in `docs/data/namecheck_checks.tsv`, or a mismatch verdict.

## Rules

| Rule | Detail |
|---|---|
| The bank is the source of truth | confirmation work never changes a name; a name that looks wrong is reported with evidence, not renamed |
| Symbols stay | code symbols remain `f_XXXXXXXX`, the stable ids of tests and vectors; names are never applied as code renames |
| Bank name first | a page or tool shows the bank name first with the id beside it |
| Effects and parameters | an effect is named only by its exact bank string in code font (`S_Aurora`), never shortened, described, invented or placed beside a function, class or parameter; a parameter candidate is quoted verbatim |
| Origin | bundled library and std functions are described by their bank origin, never as component code |

## Looking up a name

`python -I tools/fn_info.py ID|NAME|OFFSET [--json]` prints a function's bank name, origin, kind, class, tier, name check and confirmation level. The per-function reference pages, one per offset range, are listed in [generated/index.md](generated/index.md).
