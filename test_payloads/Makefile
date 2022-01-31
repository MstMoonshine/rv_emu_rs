target_dir := build
src := $(wildcard src/*.c)
lds := $(wildcard *.lds)
asm := $(wildcard *.S)
targets := $(notdir $(src:.c=.bin))

GNU_TOOL := 

RISCV64 := riscv64-unknown-elf
RISCV32 := riscv32-unknown-elf

ifneq (, $(shell which $(RISCV32)-gcc))
	GNU_TOOL := $(RISCV32)
else ifneq (, $(shell which $(RISCV64)-gcc))
	GNU_TOOL := $(RISCV64)
endif

CC := $(RISCV64)-gcc
OBJCOPY := $(RISCV64)-objcopy
OBJDUMP := $(RISCV64)-objdump

ifneq (, $(GNU_TOOL))
	CC := $(GNU_TOOL)-gcc
	OBJCOPY := $(GNU_TOOL)-objcopy
	OBJDUMP := $(GNU_TOOL)-objdump
endif

FLAGS := -O0 -T$(lds) -nostdlib -march=rv32i -mabi=ilp32 
OBJDUMP_FLAGS = -Mnumeric,no-aliases

.PRECIOUS: $(target_dir)/%.elf

all: dir $(addprefix $(target_dir)/, $(targets))

dir:
	mkdir -p $(target_dir)

$(target_dir)/%.elf: src/%.c $(asm) $(lds)
	$(CC) $< $(asm) -o $@ $(FLAGS)

$(target_dir)/%.bin: $(target_dir)/%.elf
	$(OBJDUMP) $(OBJDUMP_FLAGS) -d $< > $<.objdump
	$(OBJCOPY) -O binary $< $@.tmp
	{ cat $@.tmp; printf '\x34\x12\xfe\xca'; } > $@
	rm $@.tmp

clean:
	-@rm -rf $(target_dir)