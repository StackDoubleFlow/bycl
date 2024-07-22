.globl _cc_entry

_cc_entry:
  nop
  j write_o
  sw x0, 0(x0)
  sw x0, 4(x0)

.org 0x94
write_o:
  addi x3, x3, 13

loop:
  j loop