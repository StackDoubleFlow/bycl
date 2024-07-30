#!/usr/bin/env bash

INSTALL_DIR=$(realpath "$(dirname "$0")/install")

cd riscv-gnu-toolchain
echo $INSTALL_DIR
./configure --prefix=$INSTALL_DIR --with-arch=rv32i --with-abi=ilp32 --enable-multilib
make -j$(nproc)
