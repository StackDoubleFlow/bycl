ENTRY(_cc_entry)
SECTIONS
{
  . = 0;
  .text.cc_entry : { *(.text.cc_entry) }
  .text : { *(.text) }
  .data : { *(.data) }
  .rodata : { *(.rodata) }
  .bss : { *(.bss) }
}
