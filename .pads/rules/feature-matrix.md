# Solar Feature Matrix Rules

This file turns the language-edge research into dispatch rules. It is
not a full compatibility table; it names the surfaces where parser
awareness must not be mistaken for semantic, artifact, or runtime
support.

## Default Classification

- **Parser-aware** means Solar can lex or parse the construct.
- **Frontend-measured** means Solar has fixtures and pinned `solc`
  diagnostics or Standard JSON frontend comparison.
- **Artifact-measured** means ABI, NatSpec, metadata, storage layout,
  source maps, or method identifiers are compared through Standard JSON.
- **Runtime-measured** means bytecode executes against pinned solc output
  in the same EVM version and optimizer mode.

Any claim must name which classification it reaches.

## Feature Coverage Snapshot

Counts are feature-scoped tester totals when a committed runner exposes
them. Use `unavailable` when the repository has no deterministic
feature-scoped counter yet; do not infer support from a parser keyword or
from unrelated whole-corpus totals.

Feature | Corpus count | Pass | Fail | Xfail | Unsupported | Current classification | Count evidence
--- | --- | --- | --- | --- | --- | --- | ---
standard-json | unavailable | unavailable | unavailable | unavailable | unavailable | needs verification | No committed feature-scoped Standard JSON corpus counter in `tools/tester`.
transient | unavailable | unavailable | unavailable | unavailable | unavailable | parser-aware only until measured | No committed feature-scoped transient-storage tester counter.
UDVT | unavailable | unavailable | unavailable | unavailable | unavailable | parser-aware only until measured | No committed feature-scoped user-defined value type tester counter.
errors | unavailable | unavailable | unavailable | unavailable | unavailable | parser-aware only until measured | No committed feature-scoped custom-error tester counter.
inheritance | unavailable | unavailable | unavailable | unavailable | unavailable | parser-aware only until measured | No committed feature-scoped inheritance tester counter.
asm | unavailable | unavailable | unavailable | unavailable | unavailable | parser-aware only until measured | No committed feature-scoped inline-assembly tester counter.
libraries | unavailable | unavailable | unavailable | unavailable | unavailable | parser-aware only until measured | No committed feature-scoped library tester counter.
immutables | unavailable | unavailable | unavailable | unavailable | unavailable | parser-aware only until measured | No committed feature-scoped immutable-variable tester counter.

## Safer Frontend Starters

These are acceptable autonomous tasks when they add focused fixtures and
use pinned `solc` evidence:

- fixed / ufixed diagnostics,
- file-level constants and free functions,
- `using ... global` and user-defined operator diagnostics,
- selector-overload and ABI-visible signature diagnostics,
- receive/fallback syntax and payability diagnostics,
- NatSpec parse/lowering cases for `userdoc` / `devdoc`,
- custom storage `layout at` parser and frontend diagnostics,
- import/remapping/base-path/include-path source identity fixtures.

## High-Risk Language Families

Require human review and stronger oracles before semantic or codegen
claims:

- ABI encoder/decoder behavior, packed encoding, selector collisions,
  and custom errors.
- Storage layout, transient storage, UDVT storage, custom base slots,
  mapping/array clearing, packed inheritance slots.
- Yul, inline assembly, memory-safe annotations, object-mode Yul, and
  `verbatim_`.
- Constructor argument encoding, immutables, library linking, metadata,
  CREATE2 address-sensitive bytecode.
- Receive/fallback runtime dispatch, `try/catch`, low-level calls,
  revert data, panic codes, and forged/bubbled custom-error data.
- Inheritance linearization, `virtual`/`override`, `super`, state
  variable overrides, modifier ordering, base constructor order.
- Hardfork-gated opcodes and semantics: PUSH0, TLOAD/TSTORE, MCOPY,
  BLOBHASH, BLOBBASEFEE, SELFDESTRUCT after EIP-6780, Prague/Osaka/CLZ.

## Hardfork Rules

- Any opcode or runtime-behavior PR must name `evmVersion`.
- Symbol presence in Solar is not support.
- Opcode availability must be differential-tested against pinned solc
  for the same EVM version.
- Runtime claims require T8 evidence; artifact-only or parser-only
  evidence must be labeled as such.

## Import and Source Identity

For import/remapping changes, fixtures should include relative,
transitive, remapped, base-path, include-path, circular, and blocked-path
cases. Treat `--allow-paths` as incomplete until the TODO in the current
CLI/config path is gone and covered by fixtures.

## No-Claim Defaults

- Do not infer unsupported status from missing public evidence. Mark
  unknown surfaces as `needs verification`.
- Do not infer correctness from parser support.
- Do not require verbatim solc diagnostic prose unless the issue or
  fixture explicitly targets an error code, severity, span, or message.
- Prefer semantic parity plus Solar diagnostic style over blind
  message-copying.
