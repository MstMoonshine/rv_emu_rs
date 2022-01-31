.section .bootloader
	// setup stack
	li	sp, 0x80000160
	jal main

	// stop the emulator
	.word	0