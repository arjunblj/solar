# PR Shape - Winning Templates

Concrete templates and examples for the four most common PR archetypes on this fork. Every template encodes the rules in `pr-rubric.md`; copy these as starting points.

## Archetype: `sema-bugfix` (single typeck/sema invariant fix)

Title: `fix(sema): <one-sentence invariant>`

Example: `fix(sema): require named arguments for overloaded function call`

```markdown
## Summary

`crates/sema/src/typeck/checker.rs` now rejects calls to overloaded functions that omit a named-argument disambiguator when the positional resolution is ambiguous, matching solc 0.8.31's behavior under `-Ztypeck`.

## Behavior diff vs solc 0.8.31

| Input | solar (before) | solar (after) | solc 0.8.31 |
| --- | --- | --- | --- |
| `f({a:1});` over `f(uint), f(int)` | accepted (resolved to `f(uint)`) | error: ambiguous overload | error: ambiguous overload |
| `f(1);` (single overload) | accepted | accepted | accepted |
| `f({a:1});` (single overload) | accepted | accepted | accepted |

## Failing oracle (before)

```
$ cargo nextest run -p solar-sema -- ui::typeck::overloads::ambiguous_named
... 1 failed: ambiguous_named did not produce expected error
```

## Passing oracle (after)

```
$ cargo nextest run -p solar-sema -- ui::typeck::overloads::ambiguous_named
... 1 passed
```

## Fixtures

- `tests/ui/typeck/overloads/ambiguous_named.sol` (new)
- `tests/ui/typeck/overloads/ambiguous_named.stderr` (new)

Reference: solc `libsolidity/analysis/TypeChecker.cpp::checkExpressionContext`, `testdata/solidity/test/libsolidity/syntaxTests/functionCalls/named_args/`.

## Scope-out

Does not touch positional disambiguation logic. Follow-up: extend the same check to overloaded library functions resolved via `using`.

## Oracle tier

T3 (cargo uitest) + T2 (cargo nextest p solar-sema). Does not affect codegen or runtime.
```

## Archetype: `sema-feature` (multi-file new feature within a campaign)

Title: `feat(sema): <one-sentence capability>`

Example: `feat(sema): add view/pure state-mutability inference skeleton`

```markdown
## Summary

Introduces `crates/sema/src/typeck/view_pure.rs` with a HIR walker that computes `EffectiveMutability` for every function. This is the A1 slice of the view-pure-checker campaign; subsequent PRs add diagnostics, suggestions, and override propagation.

## Behavior diff vs solc 0.8.31

This PR is the inference skeleton; no diagnostics are emitted yet. Behavior on existing UI tests is unchanged. Once A4 (asm opcode mutability) lands, the diff becomes meaningful.

## Failing oracle (before)

`crates/sema/src/typeck/view_pure.rs` does not exist; `cargo build -p solar-sema --features view-pure-skeleton` fails.

## Passing oracle (after)

```
$ cargo build -p solar-sema
$ cargo nextest run -p solar-sema -- typeck::view_pure
... 4 passed
```

## Fixtures

- `tests/ui/typeck/view_pure/skeleton_pure.sol` (new) - exercises the inference engine on a pure function.
- `tests/ui/typeck/view_pure/skeleton_view.sol` (new)
- `tests/ui/typeck/view_pure/skeleton_nonpayable.sol` (new)
- `tests/ui/typeck/view_pure/skeleton_payable.sol` (new)

Reference: solc `libsolidity/analysis/ViewPureChecker.{h,cpp}`, `testdata/solidity/test/libsolidity/syntaxTests/viewPureChecker/`.

## Scope-out

- Does not propagate mutability across modifier composition (slice A6).
- Does not emit diagnostics or suggestions (slices A7-A9).
- Does not touch override stricter rule (slice A8).

## Oracle tier

T2 (cargo nextest) + T3 (cargo uitest). Does not change emitted ABI or runtime behavior.
```

## Archetype: `codegen-replay` (one slice from feat/codegen-mir replayed onto fork main)

Title: `feat(codegen): <one-sentence slice description>`

Example: `feat(codegen): add MIR text format with parser, printer, and roundtrip tests`

```markdown
## Summary

Stands up `crates/codegen/` and adds the MIR text format (parser + printer) with a roundtrip property test. This is the first slice of the codegen-mir-split campaign; downstream slices add the pass manager, lowering, and EVM emit. Compiler integration stays gated until the full critical path is in place.

## Behavior diff vs solc 0.8.31

Not applicable (infrastructure-only). Codegen integration is gated; this slice does not change `cargo run -- file.sol` output.

## Failing oracle (before)

`cargo build -p solar-codegen` fails (crate does not exist).

## Passing oracle (after)

```
$ cargo build -p solar-codegen
$ cargo nextest run -p solar-codegen -- mir::text::roundtrip
... 12 passed
```

## Fixtures

- `crates/codegen/Cargo.toml` (new)
- `crates/codegen/src/lib.rs` (new)
- `crates/codegen/src/mir/mod.rs` (new)
- `crates/codegen/src/mir/text.rs` (new)
- `crates/codegen/tests/text_roundtrip.rs` (new) - 12 sample MIR programs round-trip cleanly.

## Upstream reference

Re-authors `paradigmxyz/solar:feat/codegen-mir` commit `<sha>` (file `crates/codegen/src/mir/text.rs`). Deliberately omitted: pass manager wiring (slice 2), HIR-to-MIR lowering (slices 3-5), EVM emit (slices 8-9). Re-authoring is preferred over wholesale cherry-pick because the upstream branch carries 196k LOC and conflicts heavily with `main`.

## Scope-out

- No `solar-codegen` integration into the compiler driver yet (gated behind `--features codegen` until the bytecode-equivalence harness validates output).
- No optimizer passes yet (slices 11+).

## Oracle tier

T2 (crate-scoped nextest). Bytecode-diff and runtime-diff oracles activate when slices 8-9 land.
```

## Archetype: `cli-feature` (Standard JSON / CLI front-door slice)

Title: `feat(cli): <one-sentence capability>`

Example: `feat(cli): emit Standard JSON contracts[].evm.bytecode object stub`

```markdown
## Summary

`solar --standard-json` now produces an `evm.bytecode` object on output, populated with `linkReferences`, `immutableReferences`, and an empty `object` placeholder. This is the G7 slice of the standard-json campaign; G8 wires actual bytecode hex once codegen-mir-split delivers the emit module.

## Behavior diff vs solc 0.8.31

| Input | solar (before) | solar (after) | solc 0.8.31 |
| --- | --- | --- | --- |
| Standard JSON of `Counter.sol` with `outputSelection: { "*": { "*": ["evm.bytecode"] } }` | no `evm` field on output | `contracts.Counter.evm.bytecode = { object: "", linkReferences: {}, immutableReferences: {} }` | `contracts.Counter.evm.bytecode = { object: "<hex>", ... }` |

The `object` field will be populated once codegen-mir-split lands. Until then, downstream consumers see the structured envelope and can detect "Solar codegen unavailable" via the empty hex string.

## Failing oracle (before)

```
$ cargo nextest run -p solar-compiler -- ui::cli::standard_json::evm_bytecode_envelope
... 1 failed: expected `contracts.Counter.evm.bytecode` field, found undefined
```

## Passing oracle (after)

```
$ cargo nextest run -p solar-compiler -- ui::cli::standard_json::evm_bytecode_envelope
... 1 passed
```

## Fixtures

- `tests/ui/cli/standard_json/evm_bytecode_envelope/input.json` (new)
- `tests/ui/cli/standard_json/evm_bytecode_envelope/expected_output.json` (new) - solc 0.8.31's response with `object` blanked.

## Scope-out

- Empty bytecode `object` is the placeholder; G8 fills it from solar-codegen.
- No `evm.deployedBytecode` yet; slice G9.

## Oracle tier

T5 (Standard JSON frontend parity on the envelope shape). Bytecode parity (T7) waits on codegen-mir-split.
```

## How to use these templates

Pick the archetype matching your lane's `archetypes:` field in `PADS.md`. Copy the structure verbatim. Replace the example content with your invariant. Keep all six required body sections; do not skip "Scope-out" or "Oracle tier."

The reviewer auto-checks for these sections; missing sections trigger `revise` not `approve`.
