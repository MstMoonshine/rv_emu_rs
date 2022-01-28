extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
// macro_rules! log {
//     ( $( $t:tt )* ) => {
//         web_sys::console::log_1(&format!( $( $t )* ).into());
//     }
// }

// -----

use crate::register::{Register32, NUM_REGISTER};
use bus::RAM_START;
use rv_system::RV32System;

mod bus;
mod pipeline;
mod register;
mod rv_system;

// // test file
// 0x00100093, // li	x1, 1
// 0x00200113, // li	x2, 2
// 0x002081b3, // add	x3, x1, x2
// 0x80000237, // li	x4, 0x80000000
// 0x00322023, // sw	x3, 0(x4)
// 0xdeadc2b7, // li	x5, 0xdeadbeef
// 0xeef28293, // (cont.)
// 0x00022303, // lw	x6, 0(x4)
// 0x006283b3, // add	x7, x5, x6
// 0x00722223, // sw	x7, 4(x4)
// 0x00721423, // sh	x7, 8(x4)
// 0x00720623, // sb	x7, 12(x4)

fn run(
    rom_file: &str,
) -> ([Register32; NUM_REGISTER], Vec<u32>) {
    let instructions: Vec<u32> = rom_file
        .split_whitespace()
        .map(|ins| u32::from_str_radix(ins, 16).unwrap())
        .collect();

    let file = &instructions[..];

    let rv32_sys = RV32System::new(file);
    rv32_sys.run();

    (rv32_sys.get_reg(), rv32_sys.get_mem(0x110))
}

#[wasm_bindgen]
pub fn emulate(rom_file: String) -> String {
    let (reg, mem) = run(&rom_file);

    let mut output = String::new();

    output = output + &format!("Register dump:\n");
    for i in 0..31 {
        let reg_x = reg[i].0;
        output = output + &format!("x{}: {:#010x}\n", i, reg_x);
    }

    output = output + &format!("-----\n");
    output = output + &format!("Memory dump:\n");
    for i in 0..mem.len() / 4 {
        let val = (
            mem[i * 4],
            mem[i * 4 + 1],
            mem[i * 4 + 2],
            mem[i * 4 + 3],
        );
        output = output
            + &format!(
                "{:#010x}: {:#010x} {:#010x} {:#010x} {:#010x}\n",
                RAM_START + &i * 16,
                val.0,
                val.1,
                val.2,
                val.3,
            )
    }

    output
}
