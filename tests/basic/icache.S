.global _cc_entry

_cc_entry:
  li x1, 0
  .rept 254
  addi x1, x1, 1
  .endr
loop:
  j loop