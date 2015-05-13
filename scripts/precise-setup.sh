#!/bin/bash

# sets up a Ubuntu Precise rootfs for rust builds (part I)

#
# $ systemd-nspawn su -c /rusty-edge/scripts/precise-setup.sh rustbuild
#

set -e
set -x

: ${SRC_DIR:=~/src}

## setup dropbox_uploader.sh
dropbox_uploader.sh

## fetch rust source
git clone --recursive https://github.com/japaric/rust $SRC_DIR
cd $SRC_DIR
mkdir build
cd build

# sanity check
../configure --enable-ccache --enable-debug --enable-optimize
