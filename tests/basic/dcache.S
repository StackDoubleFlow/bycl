
.global _cc_entry

#define DCACHE_BASE 0x400
#define DCACHE_SIZE 0x400

_cc_entry:
    li x1, 0
    li x2, DCACHE_BASE
    li x3, 0xAAAAAAAA
    li x4, DCACHE_BASE + DCACHE_SIZE

store_loop:
    sw x3, 0(x2)
    sw x3, 4(x2)
    addi x2, x2, 8
    ble x2, x4, store_loop

    li x2, DCACHE_BASE
load_loop:
    lw x5, 0(x2)
    lw x6, 0(x2)
    bne x5, x3, fail
    bne x6, x3, fail
    addi x2, x2, 8
    ble x2, x4, load_loop

    li x1, 1
end:
    j end

fail:
    li x1, 0xFEFE
    j end



