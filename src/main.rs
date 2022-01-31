use std::{
    env,
    fs::{self, File},
    io::Read,
    process::exit,
};

use bus::RAM_START;
use register::{Register32, NUM_REGISTER};
use rv_system::RV32System;

mod bus;
mod pipeline;
mod register;
mod rv_system;

fn get_file_as_u32_vec(filename: &String) -> Vec<u32> {
    let mut f = File::open(&filename).expect("File not found");
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
                    RAM_START + i * 16
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

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("\nUsage: {} [filename]\n", args[0]);
        exit(0);
    }

    let file_path = &args[1];
    let rom_file = get_file_as_u32_vec(file_path);

    println!("File len: {}", rom_file.len());
    println!("{} instructions\n", rom_file.len() / 4);

    let (reg, mem) = run(&rom_file);

    let output = get_output(&reg, &mem);

    println!("{}", output);
}
