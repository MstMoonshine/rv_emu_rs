use std::{
    fs::{self, File},
    io::Read,
};

use bus::RAM_START;
use register::{Register32, NUM_REGISTER};
use rv_system::RV32System;

mod bus;
mod pipeline;
mod register;
mod rv_system;

fn get_file_as_u32_vec(filename: &String) -> Vec<u32> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename)
        .expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
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
) -> ([Register32; NUM_REGISTER], Vec<u32>) {
    let rv32_sys = RV32System::new(rom_file);
    rv32_sys.run();

    (rv32_sys.get_reg(), rv32_sys.get_mem(0x210))
}

pub fn main() {
    let rom_file = get_file_as_u32_vec(&String::from(
        "test_payloads/build/test.bin",
    ));

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

    println!("{}", output);
}
