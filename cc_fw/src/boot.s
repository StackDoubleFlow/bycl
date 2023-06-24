.globl _cc_entry

.section ".text.cc_entry"

_cc_entry:
    li sp, 0x500
    j cc_fw_entry
