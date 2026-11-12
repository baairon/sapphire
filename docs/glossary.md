# Glossary

The technical terms of the function record, with the emulator that judges both parts and the effects' parity record, one entry each. Pages link a term here on its first use.

## Dump

One raw x86-64 code section of a separate code image of a build of the plugin: per the class names of the [name bank](#name-bank), video output code for AJA and Blackmagic hardware (the section itself holds no names). It is the code of the [function record](#function-record), which the Rust ports reproduce. No byte of it is in this repository ([lift crates: the input](lifting.md#the-input)).

## Function record

The `lift-*` crates with the tables and evidence behind them: for each function of the [dump](#dump), its exact name, its call edges, a Rust [port](#port) and the evidence that the port matches the original. It is a reverse-engineering reference of that code; the effects use none of it ([how the two parts relate](../README.md#effects-and-stages)).

## Port

The Rust function that reproduces one function of the dump, under the id `f_XXXXXXXX` (the 8 hex digits of its offset in the dump).

## Missing reference

Data, an import or a constant that the code uses and that is not in the dump ([missing data](missing-data.md)).

## Placeholder

The stand-in a port uses for a [missing reference](#missing-reference): an `Env` access by its `rel_to_end` (`data_va`, `stub_va`).

## Stub

An import stub: a short entry outside the dump through which the code calls a library function. Each stub is identified by its offset and named by the [name bank](#name-bank) (`docs/data/stub_names.tsv`). In the name confirmation levels, `stub` is also the level of a function with a constant return or an empty body ([names: confirmation levels](names.md#confirmation-levels)).

## Name bank

The tables under `docs/data/` that name every function, stub, local class, effect and parameter candidate by offset (`names.tsv`, `stub_names.tsv`, `classes_names.tsv`, `effect_names.tsv`, `param_candidates.tsv`). The bank is the source of truth for names; code symbols stay `f_XXXXXXXX` ([names](names.md)).

## Emulator

The program that runs the original x86-64 code in place of a processor and judges every port and every rebuilt effect against it. The function record's oracle and world generator run its functions in Unicorn ([harness](harness.md), [reproduce: what needs the archive](reproduce.md#what-needs-the-archive)). The effects' kernel and whole-render comparisons also run the original in Unicorn, with an engine for the original's AVX, AVX2 and FMA (VEX) instructions that follows Intel's specification (SDM), not silicon ([effects: limits](effects.md#limits-common-to-the-effects)).

## Parity record

The record behind the <!--f:effects_rebuilt-->19<!--/f--> rebuilt effects, separate from the [function record](#function-record): the verified comparisons of each effect with the original plugin running in the [emulator](#emulator), and the recorded renders, the outputs of the final binary kept as the expected result of each case ([effects: what rebuilt means](effects.md#what-rebuilt-means)). It is kept outside the repository; `docs/data/effects_status.tsv` holds its counts.

## Oracle vector

One recorded case of a self-contained function of the dump, run in the [emulator](#emulator): its arguments, its returns and its final buffers ([harness: oracle](harness.md#oracle)).

## World

A seeded model of everything the dump lacks, in which a function of the dump and its port both run on the same inputs ([harness: world](harness.md#world)).

## Compared world

A world whose case status is `ok`: the original function ran to completion. Only compared worlds are compared with the port and counted in any figure.

## Skipped world

A world whose case status is not `ok`, because the original function faulted or timed out. It is not compared.

## Branch outcome

One direction, taken or not taken, of one conditional branch. The [strictest](#strictest) standard needs every branch outcome of a function seen in a compared world or explained under the [harness model](#harness-model).

## Harness model

The fixed assumptions under which a port is compared with its original: seeded [worlds](#world), the [callee model](#callee-model), the zeroed stack, the [landing pad](#landing-pad) category and the two [harness rules](#harness-rules) HR-P2 and HR-FP ([harness](harness.md), [assumptions](assumptions.md)).

## Callee model

How the harness answers a call it does not execute: the callee writes no memory, clobbers the caller-saved registers and returns seeded words in `rax`, `rdx` and `xmm0` ([assumptions: callee model](assumptions.md#callee-model)).

## Landing pad

A block reached only through unwind tables. It cannot run in the harness, because no callee ever throws, so its outcomes are removed from the unseen outcomes before an audit verdict ([assumptions: landing pads](assumptions.md#landing-pads)).

## Harness rules

The two rules HR-P2 and HR-FP, part of the harness model since 2026-11-05 ([harness: harness rules](harness.md#harness-rules-hr-p2-and-hr-fp)).

## HR-P2

Frame disjointness: a buffer argument never lies in the function's own stack frame. Worlds that violate it are outside the harness model.

## HR-FP

Fault parity: a branch outcome that only faulting executions can take counts as explained, given a proof that every such execution faults and a measurement that the port faults the same way.

## Precondition label

A stated condition on a row's inputs that is not part of the harness model, recorded in the `labels` column of `docs/data/audit_close.tsv`. A row whose result rests on one has the evidence class `assumed`. The only label still in use is A1, the valid sort range of `f_0001b438` ([assumptions: valid sort range](assumptions.md#valid-sort-range)).

## Tier

The kind of comparison a function received. Every function has exactly one tier ([parity: tiers](parity.md#tiers)).

## P1

The tier of a self-contained function whose port equals the original bit for bit on its [oracle vectors](#oracle-vector).

## P2v

The tier of a function that is not self-contained, compared with its original in [worlds](#world). Its status is `done` when it passed the world check and `partial` when it did not.

## P2

The tier of a port with the exact control flow and arithmetic whose missing references are placeholders. It is not compared, so it is not parity. No function has it.

## P2-std

The tier of a recognised libc++ container, string or smart pointer instantiation mapped to the Rust std type. No function has it.

## P3

The tier of a function that needs data that cannot be guessed. No function has it.

## Strict

Strict parity: the P1 rows, plus the P2v rows with status `done`.

## Strictest

Strict parity with every [branch outcome](#branch-outcome) seen in a compared world or explained under the harness model alone, with no [precondition label](#precondition-label). The standard was revised on 2026-11-05: the harness model gained the rules HR-P2 and HR-FP, and the evidence class `audit3` counts toward it. Under the earlier definition `audit3` does not count ([parity: standards](parity.md#standards)).

## Evidence class

The `evidence` column of `docs/data/FUNCTION_MAP.tsv`: what a row's parity result rests on. The ids are `p1`, `second`, `ledger`, `audit2`, `audit3`, `assumed`, `anyworld` and `-` ([evidence: evidence classes](evidence.md#evidence-classes)).

## Unproven

A function that never matched the original. It counts in no parity figure. None remains.
