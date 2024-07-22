
.globl _cc_entry

_cc_entry:
    li x1, 0xF0F1F2F3
    sw x1, 0(x0)

    lw x1, 0(x0)  // x1: 0xF0F1F2F3
    lhu x2, 0(x0) // x2: 0x0000F2F3
    lhu x3, 2(x0) // x3: 0x0000F0F1
    lbu x4, 0(x0) // x4: 0x000000F3
    lbu x5, 1(x0) // x5: 0x000000F2
    lbu x6, 2(x0) // x6: 0x000000F1
    lbu x7, 3(x0) // x7: 0x000000F0

end:
  j end