#!/bin/sh
# À lancer avant chaque commit / merge : zéro warning exigé.
set -e
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
