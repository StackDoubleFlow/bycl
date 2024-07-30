.globl _cc_entry

_cc_entry:
    li x1, 0xF0F1F2F3
    sw x1, 0(x0)

    lw x1, 0(x0)
    lh x2, 0(x0)
    lh x3, 2(x0)
    lb x4, 0(x0)
    lb x5, 1(x0)
    lb x6, 2(x0)
    lb x7, 3(x0)

end:
  j end
