#!/bin/bash
set -e

echo "Running CI pipeline..."

cargo build --locked --workspace
cargo nextest run
cargo clippy -- -D warnings
cargo fmt --check

echo "CI pipeline completed successfully."
