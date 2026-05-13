# Solar PR Rubric

A useful autonomous PR is a reviewable compiler invariant with honest proof. It is not a theme, a phase heading, or a formatter-only patch.

## Required in the PR body

- Scope: what invariant changed and why now.
- Source refs: issue, fixture ID, corpus family, upstream PR/commit, or measurement artifact.
- Files touched and why each owner file is relevant.
- Verification: exact commands, exit codes, and proof boundary.
- Risks: unsupported outputs, skipped/deferred checks, inherited failures, or remaining gaps.
- Follow-up: the next measured gap unlocked by this PR.

## Reject or revise when

- The task asks for typeck/corpus movement but no failing fixture IDs are named.
- A semantic task only passes `cargo +nightly fmt --all --check`.
- A Standard JSON task claims artifact parity before the front door exists.
- A bytecode/runtime/performance task lacks the required proof tier.
- The diff edits generated artifacts, sandbox output, corpus caches, or protected paths.
- The patch only adds scaffolding and does not produce a measurement, verifier, fixture, or behavior change.

## Acceptable blocked handoff

A blocker is useful when it names:

- exact command or file inspected,
- why the task cannot proceed safely,
- missing fixture/tool/proof dependency,
- next concrete measurement or implementation step.
