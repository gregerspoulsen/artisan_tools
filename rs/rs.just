@_default:
    just --list rs
    echo '{{BOLD + YELLOW}}Run with {{NORMAL}}`rs `{{BOLD + YELLOW}} prefix, e.g.'
    echo '    {{GREEN}}$ just rs run'

export RUSTFLAGS := "--deny warnings"

alias b := build
alias c := check
alias r := run
alias t := test
alias l := lint
alias fmt := format

# Quickly check if code compiles
check *args:
    cargo check {{args}}

build *args:
    cargo build {{args}}

run *args:
    cargo run {{args}}

test *args:
    cargo test {{args}}

lint *args:
    cargo clippy {{args}}

format *args:
    cargo fmt {{args}}

ci:
    just rs format --check
    just rs lint
    just rs test

test-run: \
    (run "--quiet -- version update") \
    (run "--quiet -- version get")


# NOTE: requires cargo-msrv (see more: https://gribnau.dev/cargo-msrv/)
[doc("Find the minimum supported Rust version")]
find-msrv:
    cargo msrv find

check-msrv:
    cargo hack check --rust-version --workspace --all-targets --ignore-private
