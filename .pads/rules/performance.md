# Solar Performance Rules

Performance work is valuable only after the relevant correctness surface
is measurable. A speed claim without a correctness gate is a research
note, not a compiler improvement.

## No-Claim Rules

- Parser performance claims require parser correctness gates:
  `cargo uitest`, `solc.syntax`, and `solc.yul`.
- Typechecker performance claims require a real `-Ztypeck` / TypeError
  corpus lane. `TESTER_MODE=solc-solidity` is parser-only.
- Codegen, optimizer, bytecode, or runtime performance claims require
  bytecode/runtime oracles for the selected subset.
- Warm or incremental claims require a declared cache, build context, or
  invalidation model. Re-running a cold compiler is not a warm-build
  benchmark.
- PGO, BOLT, allocator, and interner changes require profile evidence
  and corpus-specific before/after data.

## Required Report

Every performance PR must include:

- base SHA and head SHA,
- exact baseline and head commands,
- CPU model, OS, target triple, Rust toolchain, and Solar commit,
- solc version, EVM version, optimizer mode, and runs when relevant,
- corpus manifest and commit pin,
- primary metric and direction,
- median / p50, p95, p99 or max when latency is relevant,
- instruction counts or Callgrind data where available,
- peak RSS or allocation profile when memory is relevant,
- profile evidence naming the hot path changed,
- strongest passing correctness oracle,
- statement of what the benchmark does not prove.

## Bounded Experiment Order

1. Measurement substrate: phase timers, corpus manifests, stable command
   capture, peak RSS, and profile summaries.
2. Frontend fast paths: source-map laziness, lexer/parser fast paths,
   validation fusion, sema/typeck caches.
3. Lexer specialization / SIMD feasibility.
4. Interner, arena, and allocator matrix behind flags.
5. Parse/import graph cache prototype with stable source-unit IDs.
6. AnalysisHost-style LSP snapshot prototype, without protocol-heavy
   editor features.
7. PGO/BOLT reproduction on pinned corpora.
8. Full compiler benchmarking only after Standard JSON and
   bytecode/runtime oracles exist.

## Benchmark Commands

Current reproducible commands:

```bash
cargo bench -p solar-bench --bench criterion -- --quiet --format terse parser
uv --project benches/analyze run benches/analyze/main.py benches/README.md < benches/criterion.out
cargo bench -p solar-bench --bench iai
```

For Solar-vs-solc parser comparison:

```bash
export SOLC=/path/to/solc-0.8.31
cargo bench -p solar-bench --bench criterion -- --quiet --format terse parser
```

Full-compiler benchmarks against solc-bench, Sourcify extraction, or
Purplebench-style runtime workloads are future work until Solar has the
matching Standard JSON and runtime oracles.
