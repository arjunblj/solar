#!/usr/bin/env bash
set -euo pipefail

check_only=0
case "${1:-}" in
  "")
    ;;
  --check)
    check_only=1
    ;;
  -h|--help)
    echo "usage: bash .pads/setup.sh [--check]"
    echo "  --check  report readiness/tool versions without installing anything"
    exit 0
    ;;
  *)
    echo "usage: bash .pads/setup.sh [--check]" >&2
    exit 2
    ;;
esac

repo_root="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
if [[ -z "${PADS_CACHE_ROOT:-}" ]]; then
  if [[ -d /workspace && -w /workspace ]]; then
    PADS_CACHE_ROOT=/workspace
  else
    PADS_CACHE_ROOT="${TMPDIR:-/tmp}/pads-solar"
  fi
fi

export CARGO_HOME="${CARGO_HOME:-$PADS_CACHE_ROOT/.cargo-home}"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$PADS_CACHE_ROOT/.cargo-target}"
export PATH="$CARGO_HOME/bin:$HOME/.cargo/bin:$HOME/.foundry/bin:$HOME/.solc-select/bin:$PATH"

print_tool_versions() {
  echo "[pads/setup] tool versions"
  rustc --version || true
  cargo --version || true
  cargo nextest --version || true
  typos --version || true
  forge --version || true
  if [[ -n "${SOLC:-}" ]]; then
    "$SOLC" --version || true
  else
    echo "[pads/setup] SOLC is unset"
  fi
  jq --version || true
  uv --version || true
  node --version || true
  npm --version || true
  pnpm --version || true
  anvil --version || true
  if command -v cargo-codspeed >/dev/null 2>&1; then
    cargo codspeed --version || true
  else
    echo "[pads/setup] cargo-codspeed unavailable"
  fi
  echo "[pads/setup] SOLC=${SOLC:-}"
}

if [[ -z "${SOLC:-}" ]] && command -v solc >/dev/null 2>&1; then
  export SOLC="$(command -v solc)"
fi

if [[ "$check_only" == "1" ]]; then
  print_tool_versions
  exit 0
fi

mkdir -p "$CARGO_HOME" "$CARGO_TARGET_DIR"

git submodule update --init --checkout testdata/solidity

# Solar's rustfmt.toml uses unstable rustfmt features (imports_granularity,
# wrap_comments, format_macro_matchers, …) so `cargo fmt` MUST run under
# nightly. Install nightly rustfmt + clippy together and verify both work
# before declaring setup successful — silent fallback to stable rustfmt
# produces 100+ "unstable features are only available in nightly channel"
# warnings and exits 1, breaking every fmt verifier in the harness.
rustup toolchain install 1.88.0 --profile minimal --component clippy --component rustfmt
rustup toolchain install nightly --profile minimal --component clippy --component rustfmt
cargo +1.88.0 fmt --version >/dev/null
cargo +nightly fmt --version >/dev/null

if ! command -v cargo-nextest >/dev/null 2>&1; then
  cargo install --locked cargo-nextest
fi
if ! command -v typos >/dev/null 2>&1; then
  cargo install --locked typos-cli
fi

# Advisory tools: useful for docs/perf/feature-matrix work, but not required
# for a basic kickoff. Opt in when the run needs docs/perf/feature-matrix work;
# missing advisory tools are reported by the version summary below.
if [[ "${PADS_INSTALL_ADVISORY_TOOLS:-0}" == "1" ]]; then
  cargo install --locked cargo-docs-rs || true
  cargo install --locked cargo-hack || true
  cargo install cargo-codspeed || true
fi

if ! command -v forge >/dev/null 2>&1; then
  curl -L https://foundry.paradigm.xyz | bash
  "$HOME/.foundry/bin/foundryup"
fi

if ! command -v solc-select >/dev/null 2>&1; then
  python3 -m pip install --user solc-select
fi
if command -v solc-select >/dev/null 2>&1; then
  solc-select install 0.8.31 || true
  solc-select use 0.8.31 || true
fi
if [[ -z "${SOLC:-}" ]]; then
  if command -v solc >/dev/null 2>&1; then
    export SOLC="$(command -v solc)"
  else
    export SOLC=""
    echo "[pads/setup] warning: solc 0.8.31 not found; solc differential oracles are unavailable until SOLC is set" >&2
  fi
fi

python3 -m pip install --user -r scripts/pads/requirements.txt
python3 scripts/pads/spec-sync.py
python3 scripts/pads/tier0-guard.py
python3 scripts/pads/validate.py

cargo fetch --locked

print_tool_versions
