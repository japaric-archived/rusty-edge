#!/bin/bash

set -e
set -x

for example in */; do
    if [ "$example" == "scripts/"  ]; then
        continue
    fi

    pushd $example
    rustc demo.rs
    ./demo
    popd
done
