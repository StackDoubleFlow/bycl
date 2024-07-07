.globl _cc_entry

#define N 16

_cc_entry:
  li a0, N
  li a1, 0
  li a2, 1
loop:
  add x1, a2, a1

  // store result
  // sw a3, 0(zero)
  // mv x1, a3

  addi a0, a0, -1
  mv a2, a1
  mv a1, x1
  bnez a0, loop
end:
  j end
