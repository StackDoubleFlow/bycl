#!/usr/bin/env bash

BUILD_DIR=./build
TOOLCHAIN=../toolchain/install/bin
CC=$TOOLCHAIN/riscv32-unknown-elf-gcc
OBJDUMP=$TOOLCHAIN/riscv32-unknown-elf-objdump
OBJCOPY=$TOOLCHAIN/riscv32-unknown-elf-objcopy
BYCLC=../schem_gen/target/debug/byclc

mkdir -p $BUILD_DIR

$CC -Wl,-T../cc_fw/link.x -nostdlib $1 -o build/prog.elf

# For debugging
$OBJDUMP -d build/prog.elf --disassembler-color=terminal --visualize-jumps=color

$OBJCOPY -O binary $BUILD_DIR/prog.elf $BUILD_DIR/prog.bin

$BYCLC $BUILD_DIR/prog.bin -o $BUILD_DIR/prog.schem
