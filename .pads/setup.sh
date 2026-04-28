#!/usr/bin/env bash
set -euo pipefail

export CARGO_HOME="${CARGO_HOME:-/workspace/.cargo-home}"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/workspace/.cargo-target}"
export PATH="$CARGO_HOME/bin:$HOME/.cargo/bin:$HOME/.foundry/bin:$HOME/.solc-select/bin:$PATH"

git submodule update --init --checkout testdata/solidity

rustup toolchain install 1.88.0 nightly --profile minimal
rustup component add clippy rustfmt --toolchain 1.88.0
rustup component add clippy rustfmt --toolchain nightly

cargo install --locked cargo-nextest typos-cli cargo-docs-rs || true
cargo install cargo-hack cargo-codspeed || true

python3 -m pip install --user -r scripts/pads/requirements.txt
python3 scripts/pads/spec-sync.py
python3 scripts/pads/tier0-guard.py

cargo fetch --locked
cargo nextest --version
