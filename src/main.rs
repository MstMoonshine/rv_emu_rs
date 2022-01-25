use register::{Register32, NUM_REGISTER};
use rv_system::RV32System;
use bus::RAM_START;

mod bus;
mod pipeline;
mod register;
mod rv_system;


// 0x00100093, // addi x1, x0, 1
// 0xfff00213, // addi x4, x0, -1
// 0x00200113, // li x2, 2
// 0x002081b3, // add x3, x1, x2
// 0x800002b7, // li x5, 0x80000000 (lui	t0,0x80000)
// 0x0032a023, // sw x3, 0(x5)

fn run(rom_file: &str) -> ([Register32; NUM_REGISTER], Vec<u32>) {
    let instructions: Vec<u32> = rom_file.split_whitespace()
    .map(|ins| u32::from_str_radix(ins, 16).unwrap())
    .collect();
    
    let file = &instructions[..];

    let rv32_sys = RV32System::new(file);
    rv32_sys.run();

    (rv32_sys.get_reg(), rv32_sys.get_mem(0x110))
}

pub fn main() {

    let rom_file = "
    3e800093
    7d008113
    c1810193
    83018213
    3e820293
    ";

    let (reg, mem) = run(rom_file);

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
        output = output + &format!("{:#010x}: {:#010x} {:#010x} {:#010x} {:#010x}\n",
            RAM_START + &i * 16,
            val.0,
            val.1,
            val.2,
            val.3,
        )
    }

    println!("{}", output);
}