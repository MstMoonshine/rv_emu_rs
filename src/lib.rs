extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

extern crate web_sys;

use crate::register::{Register32, NUM_REGISTER};
use bus::RAM_START;
use rv_system::RV32System;

mod bus;
mod pipeline;
mod register;
mod rv_system;

fn get_rom_file(rom_file: &[u8]) -> Vec<u32> {
    rom_file
        .chunks(4)
        .into_iter()
        .map(|chunk| {
            u32::from_le_bytes(
                chunk.try_into().expect("cast error"),
            )
        })
        .collect()
}

fn run(
    rom_file: &[u32],
    mem_dump_size: usize
) -> ([Register32; NUM_REGISTER], Vec<u32>) {
    let rv32_sys = RV32System::new(rom_file);
    rv32_sys.run();

    (rv32_sys.get_reg(), rv32_sys.get_mem(mem_dump_size / 4))
}

fn get_output(
    reg: &[Register32; NUM_REGISTER],
    mem: &Vec<u32>,
) -> String {
    let reg_out = reg
        .into_iter()
        .zip(0..reg.len())
        .map(|(reg, i)| {
            let mut out = format!("x{}: {:#010x}\t", i, reg.0);
            if (i + 1) % 4 == 0 {
                out = out + "\n";
            }
            out
        })
        .collect::<String>();

    let mem_out = mem
        .into_iter()
        .zip(0..mem.len())
        .map(|(val, i)| {
            let mut out = format!("{:#010x} ", val);
            if i % 4 == 0 {
                out = String::from(format!(
                    "\n{:#010x}: ",
                    RAM_START + i * 4
                )) + &out;
            }
            out
        })
        .collect::<String>();

    let output = String::from("Register Dump:\n\n")
        + &reg_out
        + &"\nMemory Dump:\n"
        + &mem_out;

    output
}

#[wasm_bindgen]
pub fn emulate(rom_file: &[u8], mem_dump_size: usize) -> String {
    let rom = get_rom_file(rom_file);

    let (reg, mem) = run(&rom, mem_dump_size);

    let output = get_output(&reg, &mem);

    output
}
