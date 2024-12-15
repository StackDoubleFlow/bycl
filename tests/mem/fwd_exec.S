
.globl _cc_entry

_cc_entry:
    li x1, 0
    li x2, 0xF0F1F2F2
    sw x2, 0(x0)
    lw x1, 0(x0)
    addi x1, x1, 1

end:
  j end
