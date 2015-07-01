#!/bin/bash

# sets up a Ubuntu Precise rootfs for rust builds (part I)

#
# run this script in freshly debootstrapped Precise rootfs
#
# $ debootstrap precise /chroot/precise/rust http://archive.ubuntu.com/ubuntu
# $ cd /chroot/precise/rust
# $ systemd-nspawn /rusty-edge/scripts/precise-setup-root.sh
#

set -e
set -x

## install g++
apt-get update -qq
apt-get install -qq --force-yes build-essential
apt-get install -qq --force-yes python-software-properties
add-apt-repository -y ppa:ubuntu-toolchain-r/test
apt-get update -qq
apt-get install -qq --force-yes gcc-4.8 g++-4.8
update-alternatives --install /usr/bin/gcc gcc /usr/bin/gcc-4.8 50 --slave /usr/bin/g++ g++ /usr/bin/g++-4.8

## install dropbox_uploader.sh
apt-get install -qq curl git
cd ~
git clone https://github.com/andreafabrizi/Dropbox-Uploader
cd /usr/bin
cp /root/Dropbox-Uploader/dropbox_uploader.sh .

## install rust build dependencies
apt-get install -qq ccache file python

## add rustbuild user
useradd -m rustbuild
