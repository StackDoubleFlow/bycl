.globl _cc_entry

_cc_entry:
  li x2, 0x80000000
  li x3, 0

  // H
  li x1, 0b11111
  sw x1, 0(x2)
  sw x3, 4(x2)
  addi x3, x3, 1

  li x1, 0b00100
  sw x1, 0(x2)
  sw x3, 4(x2)
  addi x3, x3, 1

  li x1, 0b11111
  sw x1, 0(x2)
  sw x3, 4(x2)
  addi x3, x3, 2

  // E
  li x1, 0b11111
  sw x1, 0(x2)
  sw x3, 4(x2)
  addi x3, x3, 1

  li x1, 0b10101
  sw x1, 0(x2)
  sw x3, 4(x2)
  addi x3, x3, 2

  // L
  li x1, 0b11111
  sw x1, 0(x2)
  sw x3, 4(x2)
  addi x3, x3, 1

  li x1, 0b00001
  sw x1, 0(x2)
  sw x3, 4(x2)
  addi x3, x3, 2

  // L
  li x1, 0b11111
  sw x1, 0(x2)
  sw x3, 4(x2)
  addi x3, x3, 1

  li x1, 0b00001
  sw x1, 0(x2)
  sw x3, 4(x2)
  addi x3, x3, 2

  // O
  li x1, 0b11111
  sw x1, 0(x2)
  sw x3, 4(x2)
  addi x3, x3, 1

  li x1, 0b10001
  sw x1, 0(x2)
  sw x3, 4(x2)
  addi x3, x3, 1

  li x1, 0b11111
  sw x1, 0(x2)
  sw x3, 4(x2)
  addi x3, x3, 1
loop:
  j loop
  