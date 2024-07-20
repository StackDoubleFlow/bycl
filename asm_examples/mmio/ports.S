.globl _cc_entry

_cc_entry:
  li x1, 0x80000000
  li x2, 0xAAAAAAAA
  li x3, 0xFEFEFEFE
  sw x2, 0(x1)
  sw x3, 4(x1)
  lw x1, 0(x1)
loop:
  j loop
  