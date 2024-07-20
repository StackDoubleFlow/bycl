#!/usr/bin/env bash

cd riscv-gnu-toolchain
./configure --prefix=$(dirname "$0")/install --with-arch=rv32i --with-abi=ilp32 --enable-multilib
make -j$(nproc)
