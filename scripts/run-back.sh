#!/usr/bin/env bash

cargo-watch -x 'run -p backend' -w backend/src -w backend/Cargo.toml -w macros/src -w macros/Cargo.toml -w templates
