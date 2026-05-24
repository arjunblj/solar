#!/usr/bin/env bash
# .pads/setup.sh - bootstrap the Solar workspace inside a fresh sandbox.
#
# Idempotent. Designed to survive running inside a pre-warmed E2B template
# that already installs rust 1.95.0 + nightly, cargo-nextest, typos, forge,
# solc 0.8.31, and runs `cargo fetch`. When the template has set CARGO_HOME /
# CARGO_TARGET_DIR and warmed those caches, leave them alone so we don't
# defeat the pre-warm.

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
    echo "  --check  report tool versions without installing anything"
    exit 0
    ;;
  *)
    echo "usage: bash .pads/setup.sh [--check]" >&2
    exit 2
    ;;
esac

print_tool_versions() {
  echo "[pads/setup] tool versions"
  rustc --version || true
  cargo --version || true
  cargo +nightly --version 2>/dev/null || echo "[pads/setup] nightly toolchain unavailable"
  cargo nextest --version || true
  typos --version || true
  forge --version || true
  if [[ -n "${SOLC:-}" ]]; then
    "$SOLC" --version || true
  else
    echo "[pads/setup] SOLC is unset"
  fi
  python3 --version || true
  echo "[pads/setup] CARGO_HOME=${CARGO_HOME:-<default>}"
  echo "[pads/setup] CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-<default>}"
}

if [[ -z "${SOLC:-}" ]] && command -v solc >/dev/null 2>&1; then
  export SOLC="$(command -v solc)"
fi

export PATH="${CARGO_HOME:+$CARGO_HOME/bin:}$HOME/.cargo/bin:$HOME/.foundry/bin:$HOME/.solc-select/bin:$PATH"

if [[ "$check_only" == "1" ]]; then
  print_tool_versions
  exit 0
fi

# Solidity test corpus required for tester runs.
git submodule update --init --checkout testdata/solidity

# rustfmt.toml uses unstable features so cargo fmt MUST run under nightly.
# Install nightly + 1.95.0 if missing. These are no-ops in a pre-warmed
# E2B template.
rustup toolchain install 1.95.0 --profile minimal --component clippy --component rustfmt 2>/dev/null || true
rustup toolchain install nightly --profile minimal --component clippy --component rustfmt 2>/dev/null || true
rustup default 1.95.0 2>/dev/null || true
cargo --version >/dev/null
cargo +nightly fmt --version >/dev/null

# Cargo subcommands the harness/CI rely on.
if ! command -v cargo-nextest >/dev/null 2>&1; then
  cargo install --locked cargo-nextest
fi
if ! command -v typos >/dev/null 2>&1; then
  cargo install --locked typos-cli
fi

# Optional advisory tools: opt in via PADS_INSTALL_ADVISORY_TOOLS=1.
if [[ "${PADS_INSTALL_ADVISORY_TOOLS:-0}" == "1" ]]; then
  cargo install --locked cargo-hack || true
  cargo install --locked cargo-deny || true
  cargo install --locked cargo-codspeed || true
  cargo install --locked cargo-docs-rs || true
fi

# Foundry / forge for differential / replay lanes.
if ! command -v forge >/dev/null 2>&1; then
  curl -L https://foundry.paradigm.xyz | bash || true
  "$HOME/.foundry/bin/foundryup" || true
fi

# solc 0.8.31 via solc-select.
if ! command -v solc-select >/dev/null 2>&1; then
  python3 -m pip install --user solc-select || true
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
    echo "[pads/setup] warning: solc 0.8.31 not found; differential oracles unavailable" >&2
  fi
fi

# Warm cargo cache (no-op when template already cached).
cargo fetch --locked

print_tool_versions
