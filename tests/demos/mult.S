
.globl _cc_entry

_cc_entry:
  li x2, 3 // A
  li x3, 2 // B

  li x1, 0 // Result

loop:
  // If we have 1 on B's LSB, we add A to result
  andi x4, x3, 1
  beqz x4, even
  add x1, x1, x2

even:
  srli x3, x3, 1 // B >> 1
  slli x2, x2, 1 // A << 1

  // Once B is 0, we are done
  bnez x3, loop

end:
  j end
