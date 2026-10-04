# Gate run by the land queue before merging.
test-gate:
    cargo fmt --check
    cargo clippy --all-targets -- -D warnings
    cargo test

# Regenerate the README sample session, hero screenshot and README block.
readme:
    scripts/readme-samples.sh
