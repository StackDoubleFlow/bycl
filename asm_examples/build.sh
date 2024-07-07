#!/bin/bash

BUILD_DIR=./build

mkdir -p $BUILD_DIR

clang --target=riscv32 -march=rv32i -Wl,-T../cc_fw/link.x -nostdlib $1 -o build/prog.elf
llvm-objcopy -O binary $BUILD_DIR/prog.elf $BUILD_DIR/prog.bin

byclc $BUILD_DIR/prog.bin -o $BUILD_DIR/prog.schem
