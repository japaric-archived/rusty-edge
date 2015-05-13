#!/bin/bash

# I run this in a Precise rootfs with the following command:
#
# $ systemd-nspawn su -c /rusty-edge/scripts/build-rust.sh rustbuild

set -x
set -e

: ${DROPBOX:=dropbox_uploader.sh}
: ${HOST:=x86_64-unknown-linux-gnu}
: ${SRC_DIR:=~/src}

# Update source to upstream
cd $SRC_DIR
git checkout unsized
git pull
git submodule update

# Get information about HEAD
HEAD_HASH=$(git rev-parse --short HEAD)
HEAD_DATE=$(TZ=UTC date -d @$(git show -s --format=%ct HEAD) +'%Y-%m-%d')
TARBALL=rust-$HEAD_DATE-$HEAD_HASH-$HOST

# build it
cd build
../configure \
  --enable-debug \
  --enable-optimize \
  --enable-ccache
make clean
make rustc-stage1 -j$(nproc)

# packgae
cd $HOST/stage1
tar czf ~/$TARBALL bin lib
cd ~
TARBALL_HASH=$(sha1sum $TARBALL | tr -s ' ' | cut -d ' ' -f 1)
mv $TARBALL $TARBALL-$TARBALL_HASH.tar.gz
TARBALL=$TARBALL-$TARBALL_HASH.tar.gz

# ship it
$DROPBOX -p upload $TARBALL .
rm $TARBALL
