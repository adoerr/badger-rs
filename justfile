clean:
    cd device && cargo clean
    cd host && cargo clean

build:
    cd device && cargo build
    cd host && cargo build

sort:
    cd device && cargo sort -w -g
    cd host && cargo sort -w -g

check:
    cd device && \
    cargo +nightly fmt --all && \
    cargo +nightly clippy --workspace --all-features -- -D warnings

    cd host && \
    cargo +nightly fmt --all && \
    cargo +nightly clippy --workspace --all-features -- -D warnings
