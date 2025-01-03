#!/usr/bin/env bash

BUILD_DIR=./build
TOOLCHAIN=../toolchain/install/bin
CC=$TOOLCHAIN/riscv32-unknown-elf-gcc
OBJDUMP=$TOOLCHAIN/riscv32-unknown-elf-objdump
OBJCOPY=$TOOLCHAIN/riscv32-unknown-elf-objcopy
BYCLC=../schem_gen/target/debug/byclc

mkdir -p $BUILD_DIR

$CC -Wl,-T../cc_fw/link.x -nostartfiles -D__BYCL__ -Os -flto c_bootstrap.S $1 -g -o build/prog.elf

# For debugging
if [[ -z $NO_DEBUG ]]; then
    $OBJDUMP -d build/prog.elf --disassembler-color=terminal --visualize-jumps=color
fi

$OBJCOPY -O binary $BUILD_DIR/prog.elf $BUILD_DIR/prog.bin

$BYCLC $BUILD_DIR/prog.bin -o $BUILD_DIR/prog.schem

if [[ $ORE_ADMIN ]]; then
    sftp crapo <<EOF
cd /home/mcadmin/actual_schematics/$ORE_ADMIN
put $BUILD_DIR/prog.schem
exit
EOF
fi
