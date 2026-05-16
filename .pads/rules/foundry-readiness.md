# Solar Foundry Readiness Rules

Foundry support is a process and artifact contract, not just successful
direct file compilation. Treat this file as the readiness checklist for
Standard JSON, artifact, source-map, cache, and future backend work.

## Foundry Process Contract

Before claiming Foundry compatibility, name the exact surface:

- `--version` output and compiler identity,
- `--standard-json` stdin/stdout behavior,
- process exit behavior and JSON error reporting,
- base path, include path, allow paths, and remappings,
- `settings.outputSelection` subset behavior,
- unsupported input and output field diagnostics.

`solar $(forge re) src/Contract.sol` proves only direct frontend
ingestion. It does not prove Foundry build-info replay or artifact
compatibility.

## Required Artifact Pillars

Golden Standard JSON fixtures should cover:

- `abi`,
- `evm.bytecode`,
- `evm.deployedBytecode`,
- `evm.methodIdentifiers`,
- `metadata`,
- `storageLayout` and `transientStorageLayout`,
- source maps,
- `linkReferences`,
- `immutableReferences`,
- generated sources,
- `gasEstimates`,
- `userdoc` and `devdoc`.

Fields Solar cannot produce yet should be explicitly unsupported with a
stable diagnostic or ledger entry. Do not approximate source maps,
metadata, link references, immutable references, storage layout, or
bytecode object shape.

## Artifact Consumers

Foundry scripts, `vm.getCode`, `vm.getDeployedCode`, debugger/backtraces,
verification workflows, Chisel, console support, and multi-profile builds
consume compiler artifacts. Tests should preserve:

- artifact naming and `sourceName` / contract nesting,
- creation and runtime bytecode separation,
- constructor argument expectations,
- library linking and link placeholders,
- metadata hash settings,
- version-qualified artifact names,
- profile-specific artifact names,
- source-map `s:l:f:j:m` semantics including modifier depth.

## Backend Reference

Treat upstream `feat/codegen-mir` and PR #693 as reference material only.
The safe backend path is:

1. MIR foundation.
2. HIR-to-MIR lowering coverage map.
3. Liveness.
4. Phi elimination.
5. Basic stack scheduling for <=16 visible stack values.
6. Full stack scheduling and spilling.
7. Assembler label/jump resolution.
8. Bytecode/runtime equivalence.
9. Optimizations.

The equivalence harness from #704 / #760 is a gate before Foundry
codegen replacement. PR #760's known-red mismatches make it useful
reporting infrastructure, not correctness evidence.

## Safe Autonomous Work

- Write the Foundry solc-surface conformance spec.
- Add Standard JSON input/output golden fixtures for supported frontend
  fields.
- Capture `forge config --json`, `forge remappings`, and build-info from
  a pinned tiny fixture.
- Add artifact-consumer fixture design for `vm.getCode`,
  `vm.getDeployedCode`, scripts, source maps, libraries, immutables, and
  metadata.
- Build unsupported-field ledgers without claiming replay support.

## Stop Conditions

Stop and escalate or produce a blocker when a task:

- tries to ship `forge build --use solar` replacement before T7/T8 gates,
- blind-merges #693 or `feat/codegen-mir`,
- claims optimizer-run semantics without solc comparison,
- omits `evmVersion` for opcode or runtime behavior,
- treats successful parser/ABI output as bytecode/runtime readiness.
