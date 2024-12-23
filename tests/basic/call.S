.globl _cc_entry

_cc_entry:
  li x2, 0
  la ra, fail
  call func // x1 (ra) should be 0x10
  addi x2, x2, 1 // The result should be 2 -> x2
  j loop
fail:
  li ra, 0xFE
loop:
  j loop

func:
  li x2, 1
  ret
