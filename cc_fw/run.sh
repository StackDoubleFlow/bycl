#!/bin/bash

ARCH=riscv32i-unknown-none-elf
OUT_DIR=`realpath ./target/$ARCH/release`

cargo build --release
riscv64-linux-gnu-objcopy -O binary $OUT_DIR/cc_fw $OUT_DIR/cc_fw.bin

( cd ../sim && ./run_model.sh models/default $OUT_DIR/cc_fw.bin )
