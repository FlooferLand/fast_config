#!/bin/bash

cargo clean

# Attempting to build the project
cargo build --features json
cargo build --features json5
cargo build --features toml
cargo build --features yaml
cargo build --features json5,toml,yaml

# Testing the project
cargo test --features json
cargo test --features json5
cargo test --features toml
cargo test --features yaml
cargo test --features json,json5,toml,yaml

# Enabling advanced (case) testing
export ADVANCED_TEST=true
cargo test --features json,json5,toml,yaml
