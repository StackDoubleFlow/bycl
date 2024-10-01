.globl _cc_entry

_cc_entry:
  li x1, 0b1000
  srli x2, x1, 2
  nop // Check to see that it's able to fwd from mem
  mv x1, x2
loop:
  j loop
