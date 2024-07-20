ENTRY(_cc_entry)
SECTIONS
{
  .text : {
    *(.text._cc_entry)
    *(.text.*)
  }
  . = 0x400;
  .data : { *(.data*) }
  .rodata : { *(.rodata*) }
  .bss : { *(.bss*) }
  /DISCARD/ : { *(.comment) *(.gnu*) *(.note*) *(.eh_frame*) *(.interp) }
}
