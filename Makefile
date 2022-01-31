DOCKER = docker
ifeq (Linux, $(shell uname))
	DOCKER := sudo docker
endif
DOCKERFILE_DIR = ./docker
PAYLOADS_DIR = test_payloads
DOCKER_COMMANDS = make -C /$(PAYLOADS_DIR)
PAYLOADS_ABS = $(shell pwd)/$(PAYLOADS_DIR)
PAYLOADS_SRC = $(wildcard $(PAYLOADS_DIR)/src/*.c)
PAYLOADS_DEP = $(PAYLOADS_DIR)/link.lds $(PAYLOADS_DIR)/bootloader.S
PAYLOADS_SRC_FILE = $(notdir $(PAYLOADS_SRC))
TARGETS_DIR = $(PAYLOADS_DIR)/build
TARGETS = $(TARGETS_DIR)/$(PAYLOADS_SRC_FILE:.c=.bin)

PAYLOAD ?= $(TARGETS_DIR)/quicksort.bin

RISCV64 := riscv64-unknown-elf
RISCV32 := riscv32-unknown-elf
ifneq (, $(shell which $(RISCV32)-gcc))
	GNU_TOOL := $(RISCV32)
else ifneq (, $(shell which $(RISCV64)-gcc))
	GNU_TOOL := $(RISCV64)
endif

all: $(TARGETS) wasm

$(TARGETS): $(PAYLOADS_SRC) $(PAYLOADS_DEP)
ifneq (, $(GNU_TOOL))
	make -C $(PAYLOADS_DIR)
else
	$(DOCKER) buildx build --platform linux/amd64 -t riscv-toolchain $(DOCKERFILE_DIR)
	$(DOCKER) run --rm -v $(PAYLOADS_ABS):/$(PAYLOADS_DIR) --name riscv-dev riscv-toolchain $(DOCKER_COMMANDS)
	$(DOCKER) rmi riscv-toolchain
endif

wasm:
	wasm-pack build --target web

run: $(TARGETS)
	cargo run $(PAYLOAD)

clean:
	make clean -C $(PAYLOADS_DIR)
	# cargo clean