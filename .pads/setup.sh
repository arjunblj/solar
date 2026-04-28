#!/usr/bin/env bash
set -euo pipefail

export CARGO_HOME="${CARGO_HOME:-/workspace/.cargo-home}"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/workspace/.cargo-target}"
export PATH="$CARGO_HOME/bin:$HOME/.cargo/bin:$HOME/.foundry/bin:$HOME/.solc-select/bin:$PATH"
export SOLC="${SOLC:-$HOME/.local/bin/solc}"

git submodule update --init --checkout testdata/solidity

rustup toolchain install 1.88.0 nightly --profile minimal
rustup component add clippy rustfmt --toolchain 1.88.0
rustup component add clippy rustfmt --toolchain nightly

cargo install --locked cargo-nextest typos-cli cargo-docs-rs || true
cargo install cargo-hack cargo-codspeed || true

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

python3 -m pip install --user -r scripts/pads/requirements.txt
python3 scripts/pads/spec-sync.py
python3 scripts/pads/tier0-guard.py

cargo fetch --locked
cargo nextest --version
forge --version
solc --version
