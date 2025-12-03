#!/bin/bash
set -e

cargo fmt --all
cargo check --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features --fix --allow-dirty -- -D warnings
# cargo deny check all
# RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features
# cargo audit

echo "✅ All checks passed!"
