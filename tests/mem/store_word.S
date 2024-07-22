.globl _cc_entry

_cc_entry:
  li x1, 0xF0F1F2F3
  sw x1, 8(x0)
loop:
  j loop