.globl _cc_entry

_cc_entry:
  li x2, 0xFFF0
  sh x2, 4(x0)
loop:
  j loop