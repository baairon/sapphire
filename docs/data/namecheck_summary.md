# Name check summary (2026-10-26)

Source tables: namecheck_classes.tsv, namecheck_rest.tsv, namecheck_stubs.tsv, namecheck_fingerprints.tsv, namecheck_checks.tsv.

- 2,700 of 2,702 functions have a verdict: 2,188 align (664 high, 947 medium, 577 low confidence), 511 weak, 1 mismatch (`f_0006590c`, an 8 byte placeholder body).
- Argument counts fit the signature for 97.2% of 2,353 lifted functions (about 67 to 68% if shifted by one function): no systematic shift.
- All 16 class hierarchies match the structure inferred from the code; 54 of 54 constructors and destructors that reference a local vtable reference their own class.
- Stubs: 458 checked, 183 align, 275 weak, 0 mismatch; the earlier role hypotheses were confirmed 116, refined 23, refuted 18 (157 had one).
- 432 functions sit in byte identical twin sets with different names; 10 constant return functions carry no semantic evidence.
- Most checks are script level: about 200 deep reads and 79 sampled.
