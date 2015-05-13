#!/bin/bash

set -e
set -x

pushd ..
mkdir rust
cd rust
wget $URL
tar xf *.tar.gz
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$PWD/lib
export PATH=$PATH:$PWD/bin
popd
rustc -V

set +e
set +x
