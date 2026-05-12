# AGENTS.md

Guidance for AI coding agents working in this repository.

## Project Overview

Solar is a blazingly fast, modular Solidity compiler written in Rust, aiming to be a modern alternative to solc.

## Bootstrap

Before any non-trivial local or sandboxed work, initialize the repo and toolchain explicitly:

```bash
bash .pads/setup.sh
```

Sandboxed agents should run `bash .pads/setup.sh` when present; it performs the same
bootstrap idempotently and keeps `PADS.md` as the source of truth.
The setup script is safe both in container sandboxes and local checkouts; when `/workspace`
is absent it uses a temporary cache root unless `CARGO_HOME` or `CARGO_TARGET_DIR` is set.
Nested Solidity submodules are intentionally not part of the default bootstrap;
initialize them only when a selected corpus or upstream build actually requires them.
Docs/perf helper tools such as `cargo-docs-rs`, `cargo-hack`, and `cargo-codspeed`
are advisory unless the task explicitly targets those lanes; install them with
`PADS_INSTALL_ADVISORY_TOOLS=1 bash .pads/setup.sh`.

For true differential or perf work against solc, provide a pinned local compiler:

```bash
export SOLC=/path/to/solc-0.8.31
```

## Commands

```bash
cargo build                                      # Build
cargo nextest run --workspace                    # Run tests (faster than cargo test)
cargo uitest                                     # Run UI tests
cargo uibless                                    # Update UI test expectations
cargo +nightly fmt --all                         # Format (CI uses nightly)
cargo clippy --workspace --all-targets           # Lint
cargo run -- file.sol                            # Run compiler
cargo run -- -Zhelp                              # Unstable flags help
```

The `testdata/solidity` submodule is required for corpus-backed test modes. `TESTER_MODE=solc-solidity` and `TESTER_MODE=solc-yul` are parser/Yul corpus oracles, but they are not a substitute for running against a real `solc` binary when claiming differential parity. `TESTER_MODE=solc-solidity` does not prove TypeError or typechecker parity until a distinct `-Ztypeck`/TypeError lane exists.

## Architecture

- **solar-parse**: Lexer and parser
- **solar-ast**: AST definitions and visitors
- **solar-sema**: Semantic analysis (symbol resolution, type checking)
- **solar-interface**: Diagnostics and source management
- **solar-cli**: Command-line interface

Pipeline: Lexing → Parsing → Semantic Analysis → (IR → Codegen, planned)

### Visitor Pattern

Use `type BreakValue = Never` if visitor never breaks. Override `visit_*` methods and always call `walk_*` to continue traversal:

```rust
fn visit_expr(&mut self, expr: &'ast Expr) -> ControlFlow<Self::BreakValue> {
    // Your logic here
    walk_expr(self, expr)  // Always use walk_* for child traversal
}
```

## Testing

- **Unit tests**: In source files
- **UI tests**: In `tests/ui/`, verify compiler output
- Auxiliary files go in `auxiliary/` subdirectory

### UI Test Annotations

```solidity
//@compile-flags: --emit=abi
contract Test {
    uint x; //~ ERROR: message here
    //~^ NOTE: note about previous line
}
```

Annotations: `//~ ERROR:`, `//~ WARN:`, `//~ NOTE:`, `//~ HELP:`
Use `^` or `v` to point to lines above/below.

## Diagnostics Style

Error messages should follow these conventions:

- **No full stops**: Error messages should not end with periods
- **Use backticks for code**: Use `` `identifier` `` instead of `"identifier"` for code references
- **Main message is concise**: Keep the primary error message short and direct
- **Use subdiagnostics**: Add context via `note`, `help`, and `span_note`:
  - `note`: Additional context about why the error occurred
  - `help`: Actionable suggestion for how to fix the error
  - `span_note`: Point to related code locations (e.g., "overridden function is here")

Example:
```rust
self.dcx()
    .err("cannot override non-virtual function")
    .code(error_code!(4334))
    .span(base.span)
    .span_note(overriding.span, "overriding function is here")
    .help("add `virtual` to the base function to allow overriding")
    .emit();
```

## Notes

- **Symbol comparisons**: Use `sym::name` or `kw::Keyword` instead of `.as_str()` for performance. Add new symbols to `crates/macros/src/symbols.rs`.
- **Arena allocation**: AST nodes use arenas for performance.
- **Benchmarks**: See @benches/README.md to benchmark when working on performance-critical code.
