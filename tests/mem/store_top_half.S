.globl _cc_entry

_cc_entry:
  li x2, 0xFFF1
  sh x2, 6(x0)
loop:
  j loop
