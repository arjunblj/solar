# Task Rules
- Task type: implementation
- Worker type: engineer
- Model profile: normal-build
- Context mode: focused
- Task branch: task/9b7a2dbe012f
## Command Policy
- Do not pipe command output through `tail`, `head`, `sed`, `grep`, or similar truncators; command output is captured and truncated automatically.
- Before the first edit, do not run workspace-wide cargo gates such as `cargo build --workspace`, `cargo check --workspace`, `cargo test --workspace`, `cargo clippy --workspace`, or `cargo nextest run --workspace`.
- Use the narrowest configured verification command, a package-scoped command (`-p` / `--package`), or submit_blocked with exact missing evidence.
## Verification Ladder
- No explicit verification commands configured.