
#!/usr/bin/env bash

BUILD_DIR=./build

mkdir -p $BUILD_DIR

../toolchain/install/bin/riscv32-unknown-elf-gcc -Wl,-T../cc_fw/link.x -nostartfiles -D__BYCL__ -Os -flto c_bootstrap.S $1 -o build/prog.elf

# For debugging
llvm-objdump -d build/prog.elf

llvm-objcopy -O binary $BUILD_DIR/prog.elf $BUILD_DIR/prog.bin

../schem_gen/target/debug/byclc $BUILD_DIR/prog.bin -o $BUILD_DIR/prog.schem
