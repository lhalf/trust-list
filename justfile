set shell := ["bash", "-euc"]

build:
    cargo build --locked

check:
    cargo fmt --check --all
    cargo clippy --all-targets -- -Dwarnings

test:
    cargo test
