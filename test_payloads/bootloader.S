#define ROM_START 	0x40000000
#define RAM_START	0x80000000
#define MAGIC		0xcafe1234

.section .bootloader

	li x1, ROM_START
	li x2, RAM_START
	li x3, MAGIC
load:
	// load ROM content into RAM.
	// loop until met magic number
	lw x4, 0(x1)
	beq x3, x4, boot
	sw x4, 0(x2)
	addi x1, x1, 4
	addi x2, x2, 4
	j load

boot:
	auipc x3, 0x40000
	jalr x0, 8(x3)

	// setup stack
	li sp, 0x80001000

	// start
	jal main

	// stop the emulator
	.word	0