# Solar Upstream Map

Use upstream Solar as reference evidence, not as code to merge blindly.
Refresh links and branch tips before dispatching work that depends on
them.

## Intent

- Solar targets Solidity `0.8.*` compatibility and is not production
  complete.
- Upstream has no public plan to diverge from Solidity semantics.
- `solc 0.8.31` is the pinned typechecker reference in upstream issue
  #615.
- Public API stability is not promised for Rust library consumers; semver
  tracks binaries, and library users are told to pin exact versions.
- Foundry integration is product-critical, but public evidence points
  first to parser/AST/context/LSP/tooling and Standard JSON surfaces, not
  a merged full codegen replacement.

## Watchlists

### Frontend and Typechecker

- #615: typechecker parity tracker; remains the high-confidence policy
  source for `-Ztypeck` and `solc 0.8.31` evidence.
- #663 / #737: TypeError corpus exposure; useful source material, but do
  not treat parser-only corpus exits as typechecker passes.
- #617, #640, #643, #657, #661, #717: implicit conversions,
  byte/string/address conversions, variable declarations, and call
  typechecking history.
- #649 and related stacked PRs: integer-literal constant folding and
  literal type preservation.

### Parser, Yul, and NatSpec

- #743: parser precedence edge.
- #754 / #475 / #508: frontend parsing/analysis performance and lexer
  fast paths.
- #415 / #652: Yul and inline-assembly lowering boundary; reference-only
  until the HIR path is merged.
- #567: NatSpec lowering; artifact compatibility work should not become
  a doc-generator product.

### Codegen and Runtime

- #687: MIR/codegen dependency graph and bytecode-equivalence roadmap.
- #693 / `feat/codegen-mir`: broad draft codegen branch; extract only
  with attribution and proof boundary.
- #694-#704: liveness, phi elimination, stack scheduling, spilling,
  assembler, optimizer, and runtime-equivalence issue series.
- #760: runtime-equivalence CI shape that is known red on runtime
  mismatches; useful as reporting infrastructure, not correctness proof.

### Editor Surface

- #394: LSP roadmap.
- #401: draft basic LSP; not a stable foundation for autonomous editor
  features.
- #417 / #418 / #419 / #420 / #421 / #416: lifecycle, symbol tables,
  completion, go-to-definition, inlay hints, and flychecks.
- Foundry #11619 / #11448: downstream Solar LSP integration; wait for
  upstream Solar release and closed lifecycle blockers.

## Reference-Only Branch Names

Do not create autonomous fork branches that collide with visible
upstream branch names. Use fork-local prefixes such as
`pads/typeck/...`, `pads/parser/...`, `pads/stdjson/...`,
`pads/foundry/...`, and `pads/codegen-ref/...`.

Known upstream names to avoid include:

- `feat/codegen-mir`
- `dani/gungraun`
- `onbjerg/lsp-scaffolding`
- `georgios/payable-functions`
- `georgios/codegen-external-calls-events-storage`
- `feat/typeck-view-pure`
- `feat/typeck-implicit-conversions`
- `feat/hir-pretty-printer`
- `docs/solc-divergence`
- `feat/typeck-enable-tests`
- `feat/typeck-static-analyzer`

## Unfreeze Criteria For Editor Work

LSP, formatter, standalone editor extensions, rename/refactor, and
independent linter work stay reference-only until:

- upstream #401 is merged and released,
- #417 is closed,
- #418 symbol-table work has landed,
- incremental document-sync ICEs are resolved,
- frontend/typechecker diagnostics are trustworthy enough for editor
  consumption,
- a worker can name the exact editor-facing invariant and oracle.
