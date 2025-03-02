.global _cc_entry

_cc_entry:
  li x1, 0
  li a0, 23
  li a1, 1

loop:
  sll a2, a1, a0
  or x1, x1, a2
  addi a0, a0, -1
  bnez a1, loop

end:
  j end