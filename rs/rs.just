@_default:
    just --list rs
    echo '{{BOLD + YELLOW}}Run with {{NORMAL}}`rs `{{BOLD + YELLOW}} prefix, e.g.'
    echo '    {{GREEN}}$ just rs run'

alias b := build
alias c := check
alias r := run
alias t := test
alias l := lint
alias fmt := format

[group("init"), doc("Install dev-tools (requires the Rust toolchain)")]
init:
    cargo install cargo-nextest
    cargo install typos-cli


# Install dev-tools including tools used sparingly (requires the Rust toolchain)
[group("init")]
init-extra: init
    cargo install cargo-limit
    cargo install cargo-shear
    cargo install cargo-hack
    cargo install cargo-msrv

# Quickly check if code compiles
check *args="--workspace":
    cargo check {{args}}

build *args="--workspace":
    cargo build {{args}}

run *args:
    cargo run {{args}}

test *args="--workspace":
    cargo test {{args}}

[group("lint")]
lint *args="--workspace":
    cargo clippy {{args}}

[group("lint")]
format *args:
    cargo fmt {{args}}

# Check for typos
[group("lint")]
typos:
    # Should remove '..' when it's run from the project root
    typos ..

ci $RUSTFLAGS="--deny warnings":
    just rs format --check
    just rs lint
    just rs test
    just rs typos

test-run: \
    (run "--quiet -- version update") \
    (run "--quiet -- version get")


# NOTE: requires cargo-msrv (see more: https://gribnau.dev/cargo-msrv/)
[doc("Find the minimum supported Rust version")]
find-msrv:
    cargo msrv find

check-msrv:
    cargo hack check --rust-version --workspace --all-targets --ignore-private
