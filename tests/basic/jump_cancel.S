// This program tests a variety of jump edge cases that needs to be handled.
// If an issue is detected, x1 != 1

.globl _cc_entry

_cc_entry:
  li x1, 0

  // Test basic cancellation of next two instructions
  j a
  addi x1, x1, 1
  addi x1, x1, 1
a:

  // Test cancellation of jumps
  j b
  jal x2, fail // use x2 as return address
  jal x2, fail
b:

  // Test cancellation of load rerun
  addi x2, x1, 1
  sw x2, 0(x0)
  j c
  nop
  lw x1, 0(x0)
c:

  // Test cancel forward
  j d
  addi x1, x1, 1
  addi x1, x1, 1
d:
  mv x1, x1

 // The result should be 1
  addi x1, x1, 1

loop:
  j loop

fail:
  addi x1, x1, 1
  jr x2 // x2 has return address
