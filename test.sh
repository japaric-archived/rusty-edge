#!/bin/bash

set -e
set -x

for example in unsized_types/*/; do
    pushd $example
    cargo build
    valgrind target/debug/$(basename $example)
    cargo build --release
    valgrind target/release/$(basename $example)
    popd
done
