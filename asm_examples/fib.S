.globl _cc_entry

#define N 32

_cc_entry:
  li a0, N
  li a1, 0
  li a2, 1
loop:
  add a3, a2, a1
  sw a3, 0(zero)
  addi a0, a0, -1
  mv a2, a1
  mv a1, a3
  bnez a0, loop
end:
  j end
