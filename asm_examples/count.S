
.globl _cc_entry

_cc_entry:
  nop
  li x1, 0
count:
  addi x1, x1, 1
  j count

  // If x1 gets set to 0xFE then branch stall is failing
  li x1, 0xFE
  li x1, 0xFE