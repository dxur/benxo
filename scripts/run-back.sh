#!/usr/bin/env bash
set -e

REQUIRED_RUST_VERSION="1.88.0"

if ! rustc --version | grep -q "$REQUIRED_RUST_VERSION"; then
    echo "Installing Rust $REQUIRED_RUST_VERSION..."
    rustup install "$REQUIRED_RUST_VERSION"
    rustup default "$REQUIRED_RUST_VERSION"
fi

cd backend
cargo watch -x 'run'
