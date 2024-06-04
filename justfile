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

update:
    cd device && cargo update
    cd host && cargo update

outdated:
    cd device && cargo outdated -R
    cd host && cargo outdated -R

flash binary:
    cd device && cargo run --bin {{binary}}

attach binary:
    probe-rs attach --chip RP2040 device/target/thumbv6m-none-eabi/debug/{{binary}}

run binary:
    cd host && RUST_LOG=info cargo run --bin {{binary}}
