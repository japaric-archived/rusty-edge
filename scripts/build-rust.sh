#!/bin/bash

# I run this in a Precise rootfs with the following command:
#
# $ systemd-nspawn su -c /rusty-edge/scripts/build-rust.sh rustbuild

set -x
set -e

: ${DROPBOX:=dropbox_uploader.sh}
: ${HOST:=x86_64-unknown-linux-gnu}
: ${SRC_DIR:=~/src}
: ${DIST_DIR:=~/dist}

# Update source to upstream
cd $SRC_DIR
git reset --hard HEAD
git fetch
git checkout origin/edge
git submodule update

# Get information about HEAD
HEAD_HASH=$(git rev-parse --short HEAD)
HEAD_DATE=$(TZ=UTC date +'%Y-%m-%d')
TARBALL=rust-$HEAD_DATE-$HEAD_HASH-$HOST

# build it
cd build
../configure \
  --enable-ccache \
  --prefix=/
make clean
make -j$(nproc)

# package
rm -rf $DIST_DIR/*
DESTDIR=$DIST_DIR make install -j$(nproc)
cd $DIST_DIR
tar czf ~/$TARBALL bin lib
cd ~
TARBALL_HASH=$(sha1sum $TARBALL | tr -s ' ' | cut -d ' ' -f 1)
mv $TARBALL $TARBALL-$TARBALL_HASH.tar.gz
TARBALL=$TARBALL-$TARBALL_HASH.tar.gz

# ship it
$DROPBOX -p upload $TARBALL .
rm $TARBALL

# clean up
rm -rf $DIST_DIR/*
