# Gate run by the land queue before merging.
test-gate:
    cargo fmt --check
    cargo clippy --all-targets -- -D warnings
    cargo test
