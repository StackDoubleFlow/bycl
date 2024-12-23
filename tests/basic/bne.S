.globl _cc_entry

_cc_entry:
  li x1, 0
  li a5, 31
  li x6, 32
  nop
  nop
  addi a5, a5, 1
  bne a5, a6, loop
  li x1, 1
loop:
  j loop
