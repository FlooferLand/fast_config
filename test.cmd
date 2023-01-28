: # Universal command list to build and test the project locally
: # ---
: # "chmod +x test.cmd && ./test.cmd" to run it on Linux
: # ".\test.cmd" to run it on Windows

cargo clean

: # Attempting to build the project
cargo build --features json5
cargo build --features toml
cargo build --features yaml
cargo build --features json5,toml,yaml

: # Testing the project
cargo test --features json5
cargo test --features toml
cargo test --features yaml
cargo test --features json5,toml,yaml

: # Enabling advanced (case) testing
: $ADVANCED_TEST=true
:<<"::CMDLITERAL"
set "ADVANCED_TEST=true"
::CMDLITERAL

cargo test --features json5,toml,yaml
