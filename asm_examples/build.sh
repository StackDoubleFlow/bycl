#!/usr/bin/env bash

BUILD_DIR=./build

mkdir -p $BUILD_DIR

clang --target=riscv32 -march=rv32i -Wl,-T../cc_fw/link.x -nostdlib -fuse-ld=lld $1 -o build/prog.elf

# For debugging
llvm-objdump -d build/prog.elf

llvm-objcopy -O binary $BUILD_DIR/prog.elf $BUILD_DIR/prog.bin

../schem_gen/target/debug/byclc $BUILD_DIR/prog.bin -o $BUILD_DIR/prog.schem
