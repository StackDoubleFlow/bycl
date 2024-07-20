#!/usr/bin/env bash

set -e

PATH="$PATH:$(dirname "$0")/../toolchain/install/bin"

cargo build --release

ARCH=riscv32i-unknown-none-elf
OUT_DIR=`realpath ./target/$ARCH/release`

riscv32-unknown-elf-objcopy -O binary $OUT_DIR/cc_fw $OUT_DIR/cc_fw.bin

( cd ../sim && ./run_model.sh models/default $OUT_DIR/cc_fw.bin )
