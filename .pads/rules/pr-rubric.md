# Solar PR Rubric

A useful autonomous PR is a reviewable compiler invariant with honest
proof. It is not a theme, a phase heading, or a formatter-only patch.

## Required in the PR body

- **Scope.** What invariant changed and why now.
- **Source refs.** Issue, fixture ID, corpus family, upstream PR /
  commit, or measurement artifact. For upstream-mined work: exact
  source branch / commit / PR, omitted commits, local conflicts.
- **Files touched and why each owner file is relevant.**
- **Verification.** Exact commands, exit codes, and proof boundary
  (which tier and what it does not prove). For corpus work: before /
  after counts. For solc parity: solc version (default pin `0.8.31`),
  EVM version, optimizer mode, compared field. For performance:
  baseline commit, head commit, environment, benchmark command,
  before / after numbers, profile evidence, correctness oracle for
  the optimized stage.
- **Risks.** Unsupported outputs, skipped / deferred checks, inherited
  failures, remaining gaps.
- **Follow-up.** The next measured gap unlocked by this PR.

## Reject or revise when

- The task asks for typeck / corpus movement but no failing fixture
  IDs or `-Ztypeck` lane evidence is named.
- A semantic task passes only `cargo +nightly fmt --all --check`.
- A Standard JSON task claims artifact parity before the Solar
  Standard JSON front door exists on fork main.
- A typeck task uses `TESTER_MODE=solc-solidity` as proof. That mode
  is parser-only (`--stop-after=parsing`) and ignores TypeError
  exits.
- A bytecode / runtime / performance task lacks the required proof
  tier (T7 / T8 for codegen claims; correctness oracle + baseline +
  profile for perf claims).
- The diff edits generated artifacts, sandbox output, corpus caches,
  or protected paths (`testdata/solidity/**`, `.github/workflows/**`,
  `deny.toml`, `clippy.toml`, `rustfmt.toml`, `rust-toolchain.toml`).
- The diff blesses UI snapshots (`.stderr` / `.stdout` / `.snap`)
  without a paired source change and a reviewed semantic before /
  after.
- The patch only adds scaffolding and does not produce a measurement,
  verifier, fixture, or behavior change.

### Foundry-readiness rejection criteria

In addition to the rules above, reject Foundry-adjacent PRs when:

- The PR claims Foundry support without naming which pillar it
  touches: process contract, JSON I/O, source maps, or cache
  identity. See `.pads/rules/foundry-readiness.md`.
- The PR generates source maps, metadata, library link references,
  immutable references, or storage layout "approximately" rather
  than matching solc's shape or labeling the field unsupported.
- The PR claims `forge build --use solar` style codegen replacement
  before #704 runtime-equivalence evidence is green.
- The PR counts `solar $(forge re) src/Contract.sol` as Foundry
  support; that proves only direct frontend ingestion.
- The PR implements `--optimize-runs` semantics from scratch instead
  of mirroring solc behavior or failing unsupported.
- The PR adds hardfork-gated opcode support without an `evmVersion`
  gate and a pinned-solc differential test.

### Editor-surface deferral

LSP, formatter, doc generator, rename / refactor, and editor-extension
work is **reference-only** until the unfreeze criteria in
`.pads/rules/upstream-map.md` are met (upstream #401 merged and
released, #417 closed, #418 landed, incremental document-sync ICEs
resolved, frontend compatibility oracles gateable). Reject autonomous
PRs that implement editor-surface features before unfreeze.

### Upstream-mining rejection criteria

Reject upstream-mined PRs when:

- They blind-merge `feat/codegen-mir` or PR #693.
- They omit the source branch / commit list, omitted changes, and
  the local proof boundary.
- They claim correctness based on draft-branch CI that is known red
  (e.g. PR #760 reports expected runtime mismatches).
- They re-introduce work that is already on fork main (PR #761 is
  already merged at 0573b99 / 8d26b64; do not regenerate it).

## Acceptable blocked handoff

A blocker is useful when it names:

- exact command or file inspected;
- why the task cannot proceed safely;
- missing fixture / tool / proof dependency;
- next concrete measurement or implementation step;
- the track and proof tier the next step targets.

## Conventional commits

Solar's recent history uses Conventional Commit titles
(`feat(sema): ...`, `fix(parser): ...`, `docs(pads): ...`). Follow the
same pattern unless local fork history clearly says otherwise. The
title should describe what behavior shipped, not which phase or
oracle was touched.
