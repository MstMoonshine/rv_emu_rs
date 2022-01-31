OUTPUT_ARCH( "riscv" )

SECTIONS
{
	. = 0x40000000;

	.text :
	{
		*(.bootloader)
		*(.text)
	}

	. = ALIGN(0x1000);
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

	.bss : {
		*(.bss)
		*(.bss.*)
		*(.sbss*)
	}

}