# Getting started

Install Rust, clone the repository, then run `cargo run -p nexus-cli -- demo`. Explore the available capabilities with `cargo run -p nexus-cli -- robot inspect nxr-1` and execute an explicit skill with `cargo run -p nexus-cli -- task run --skill walk_to --target table`.

All Phase I core actions run in the local simulator. Add `--no-ai` to make the no-model mode explicit; the rule planner is already deterministic.
