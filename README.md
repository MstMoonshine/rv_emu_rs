# 32-bit RISC-V Emulator in Rust

This is a RISC-V emulator supoorting RV32I instruction set written in Rust, inspired by [Francis Stokes's RISC-V Emulator in Typescript](https://github.com/LowLevelJavaScript/RISC-V-Emulator). (He made a fascinating series of [videos](https://www.youtube.com/watch?v=ER7h4ZTe19A&list=PLP29wDx6QmW4sXTvFYgbHrLygqH8_oNEH) on this topic!) You can either run this repository as a stand-alone binary or a WASM package, which can be embeded to a website. [Try it out here!](https://mstmoonshine.github.io/p/rv32_emu/)

---

## Usage

Make sure you have rust developing environment installed. RISC-V GNU Toolchain is not necessary. If the toolchain is not found in your PATH, a docker image containing the toolchain will be pulled automatically.

### Stand alone binary

The emulator supports running customized C program. To run your own C program, perform the following steps:
1. Put the C file in `test_payloads/src` directory
2. Run `make` in the top directory
3. Run `cargo run test_payloads/build/[filename].bin`

Note:
- The only output method is using register/memory dump. Currently no privileged level is supported so no syscall can be handled, which means functions like `printf` are not allowed.
- `.bin` suffix is necessary, which will be automatically generated along with the elf file.

### WASM Package

The emulator can also be compiled into a WASM lib. Run `make` in the top directory and you will get a `pkg` directory containing the library. To use the library inside JS/TS, copy the `pkg` directory to your project and import it as the following:

```javascript
import init, { emulate } from "./pkg/rv_emu_rs.js";

init()
    .then(() => {
        // your codes here...

        ...
        // romArray is a u8 array containing binary codes
        // memSize is the length of memory dump
        emulate(romArray, memSize);
        ...
    });


```