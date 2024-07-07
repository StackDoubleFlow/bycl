
.globl _cc_entry

_cc_entry:
  li x1, 0xF0
  li x2, 0xF1
  li x3, 0xF2
  li x4, 0xF3
  sb x4, 0(x0)
  sb x3, 1(x0)
  sb x2, 2(x0)
  sb x1, 3(x0)

  li x1, 0xFFF0
  li x2, 0xFFF1
  sh x1, 4(x0)
  sh x2, 6(x0)

  li x1, 0xF0F1F2F3
  sw x1, 8(x0)
loop:
  j loop