OUTPUT_ARCH( "riscv" )

SECTIONS
{
	. = 0x80000000;

	.text :
	{
		*(.bootloader)
		*(.text)
	}

	. = ALIGN(0x100);
	.rodata :
	{
		*(.rdata)
		*(.rodata)
	}

	.data :
	{
		*(.data)
		*(.data.*)
	}

	.bss :
	{
		*(.bss)
		*(.bss.*)
		*(.sbss*)
	}
}