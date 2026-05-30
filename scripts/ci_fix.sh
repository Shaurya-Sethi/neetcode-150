#!/usr/bin/env bash
set -euo pipefail

cargo fmt --all
cargo clippy --fix --allow-dirty --workspace --all-targets -- -D warnings
cargo test --workspace
