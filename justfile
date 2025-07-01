set shell := ["bash", "-euc"]

check:
    cargo fmt --check --all
    cargo clippy --all-targets -- -Dwarnings

test:
    cargo test
