.globl _cc_entry

_cc_entry:
  li x2, 0x55
  li x3, 0xAA
  li x4, 0b100

  mv x1, x2
  andi x5, x4, 1
  beqz x5, end
  mv x1, x3

end:
  j end