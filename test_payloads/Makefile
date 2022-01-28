target_dir := build
src := $(wildcard *.c)
lds := $(wildcard *.lds)
asm := $(wildcard *.S)
targets := $(src:.c=.bin)

CC := riscv64-unknown-elf-gcc
FLAGS := -O0 -T$(lds) -nostdlib -march=rv32i -mabi=ilp32 

OBJCOPY := riscv64-unknown-elf-objcopy
OBJDUMP := riscv64-unknown-elf-objdump

OBJDUMP_FLAGS = -Mnumeric,no-aliases

.PRECIOUS: $(target_dir)/%.elf

all: dir $(addprefix $(target_dir)/, $(targets))

dir:
	mkdir -p $(target_dir)

$(target_dir)/%.elf: %.c
	$(CC) $< $(asm) -o $@ $(FLAGS)

$(target_dir)/%.bin: $(target_dir)/%.elf
	$(OBJDUMP) $(OBJDUMP_FLAGS) -d $< > $<.objdump
	$(OBJCOPY) -O binary $< $@

clean:
	-@rm -rf $(target_dir)