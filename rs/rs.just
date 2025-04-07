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

test-run: \
    (run "--quiet -- version update") \
    (run "--quiet -- version get")