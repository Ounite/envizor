OUTPUT_FORMAT(binary)
ENTRY(main)
SECTIONS
{
    . = 0x7c00;
    .text : { *(.text.main) *(.text) }
    .rodata : { *(.rodata) }
    .data : { *(.data) }
    .bss : { *(.bss) }
    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
        *(.rel.eh_frame)
        *(.note*)
    }
}
