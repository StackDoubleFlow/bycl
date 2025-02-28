.globl _cc_entry

_cc_entry:
  li x1, 0

  // check beq/bne
  li a1, 31
  li a2, 5000
  beq a1, a2, fail
  bne a1, a2, a
  j fail
a:
  li a1, 5000
  bne a1, a2, fail

  // check blt
  li a1, 10
  li a2, 5
  blt a1, a2, fail // 10 < 5
  li a1, -1
  blt a1, a2, b // -1 < 5
  j fail
b:
  li a2, -5
  blt a1, a2, fail // -1 < -5

  // check bltu
  li a1, 10
  li a2, 5
  bltu a1, a2, fail // 10 < 5
  li a1, 0
  li a2, 0xFFFFFFFF
  bltu a1, a2, c // 0 < 0xFFFFFFFF
  j fail
c:
  bltu a2, a1, fail // 0xFFFFFFFF < 0

  li x1, 1

loop:
  j loop

fail:
  li x1, 0xFEFE
  j loop
