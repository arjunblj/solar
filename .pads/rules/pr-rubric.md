# PR Rubric

This rubric judges whether a PR is mergeable into `arjunblj/solar:main`. The harness's reviewer pass uses it; human operators reviewing the autonomous output use it; workers should write their PR bodies against it.

## Scope envelope

| Class       | Lines (additions + deletions) | Files | Crates |
| ----------- | ----------------------------- | ----- | ------ |
| `micro_fix` | <=80                          | 1-3   | 1      |
| `normal_pr` | 80-300                        | 3-8   | 1-2    |
| `large_pr`  | 200-1000                      | 6-20  | 1-3    |
| `milestone_pr` | 800-2500                  | 15-40 | 2-4    |

Above 2500 lines or 4 crates: split into multiple PRs unless a single semantic invariant genuinely requires the larger surface (e.g., one `feat/codegen-mir` slice). Even then, document why no smaller slice is reviewable.

## Required body sections

Every PR body must contain, in this order:

1. **Summary** — one or two sentences naming the invariant the PR proves and the crate(s) it touches.
2. **Behavior diff vs solc 0.8.31** — a fenced table showing 3-5 inputs and the before/after Solar output alongside the solc reference output. Skip only if the lane is explicitly infrastructure-only.
3. **Failing oracle (before)** — the exact shell command that fails before this PR is applied, plus the error excerpt.
4. **Passing oracle (after)** — the same command, succeeding, plus the success excerpt.
5. **Named corpus / fixtures** — every fixture file added or changed, listed by full path. If solc syntaxTests directories were used as semantic reference, name them.
6. **Scope-out** — what is intentionally NOT done in this PR but should follow up. Write the next slice's title.
7. **Strongest passing oracle tier** (T0-T10 from PADS.md `## Correctness Oracle Ladder`).

For codegen-mir-split PRs add: **Upstream reference** — the commit / branch on `paradigmxyz/solar:feat/codegen-mir` whose work this slice re-authors, plus what was deliberately omitted.

For performance PRs add: **Baseline + head commands**, **environment** (CPU / rustc / target triple), **before/after numbers with noise policy**, **profile evidence**, **correctness oracle result for the optimized stage**.

## Banned filler words

Reject PR bodies that lean on filler verbs and adjectives. Common offenders:

- comprehensive
- robust
- enhance
- leverage
- intricate
- nuanced
- holistic
- thoughtful
- intuitive
- streamline
- seamlessly

Replace with concrete behavior. "Comprehensive type checking" -> "checks 14 of the 16 cases under `syntaxTests/typeck/operators/` that solc rejects."

## Hard rejection classes

Auto-reject PRs that:

- Touch only `tests/`, `tools/tester/`, `scripts/`, `docs/`, `.pads/`, or `.github/` while the assigned lane is a substantive `crates/*` lane (the production-code coercion). Infrastructure-only PRs are acceptable shipping units only when the lane is explicitly `archetypes: [oracle-port-from-solc]` or similar test-infra archetype.
- Edit or rebless UI snapshots (`*.stderr`, `*.stdout`, `*.snap`) without a paired source change.
- Grow skip / xfail lists without an issue link, reason, owner track, and revisit condition.
- Bundle multiple invariants into one PR. Split into per-invariant PRs.
- Claim solc compatibility without naming solc version, EVM version, optimizer mode, corpus, and oracle tier.
- Claim performance gains without paired correctness oracle + same-environment baseline + before/after numbers + profile evidence.
- Touch `testdata/solidity/**`, `.github/workflows/**`, `rust-toolchain.toml`, `deny.toml`, `clippy.toml`, `rustfmt.toml` without explicit operator approval.

## Affirmative patterns (what merges fast)

- A clear single-invariant title following Conventional Commits.
- Body opens with the invariant in plain English, names the file(s), and quotes the failing oracle.
- Diff shape is "production change in `crates/sema/...` + paired `tests/ui/...` fixture pair", or "new module under `crates/<name>/src/...` for a brand-new feature with an isolated test file."
- The behavioral diff vs solc table is honest about edge cases the PR deliberately defers.
- The PR body explicitly cites the `solc` reference (file path + line range or `syntaxTests/` directory).
- The PR's CI is green: `cargo build`, `cargo nextest`, `cargo +nightly fmt --check`, `cargo clippy -- -D warnings`, `typos`.

## Process / hygiene

- Conventional Commits in the title (`feat(sema):`, `fix(typeck):`, `feat(codegen):`, `perf(parser):`, `refactor(interface):`, `test(typeck):`, `chore(deps):`).
- Descriptive title (no `WIP`, no `tmp`, no `please review`).
- One topic per PR. If the PR is fixing N issues, that's N PRs.
- Linked issue or upstream reference in the body where applicable. For typeck slices, link `paradigmxyz/solar#615`. For codegen slices, link `paradigmxyz/solar#693` and the specific source file/section being re-authored.
- Mark the PR ready for review only when CI is green AND the worker has run the lane's oracle commands locally and pasted the output in the body.
