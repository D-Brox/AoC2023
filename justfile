#!/usr/bin/env just --justfile

default:
    @just --list

# Run checks
check: spellcheck clippy
    @cargo +nightly fmt --check
    @echo Checks were successful!

# Remove generated artifacts
clean:
    @cargo clean
    @echo Done!

# Build the project
build:
    @cargo build
    @echo Project successfully built!

run +ARGS="":
    @cargo run --release -- {{ARGS}}

# Lint the codebase
clippy +ARGS="":
    @cargo clippy --all-targets --all-features --workspace -- --deny warnings {{ARGS}}
    @echo Lint successful!

# Format the codebase
fmt +ARGS="":
    @cargo +nightly fmt --all -- {{ARGS}}
    @echo Codebase formatted successfully!

# Spellcheck the codebase
spellcheck +ARGS="--skip target*":
    @codespell --skip="./.git" --skip="./inputs" --builtin clear,rare,informal,code --ignore-words-list mut,crate {{ARGS}}
    @echo Spellings look good!
