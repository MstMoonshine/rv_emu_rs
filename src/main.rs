use rv_system::RV32System;

mod bus;
mod pipeline;
mod register;
mod rv_system;

fn main() {
    let rv32_sys = RV32System::new(
        &[
            0x00100093, // addi x1, x0, 1
            0xfff00213, // addi x4, x0, -1
            0x00200113, // li x2, 2
            0x002081b3, // add x3, x1, x2
            0x800002b7, // li x5, 0x80000000
            0x0032a023, // sw x3, 0(x5)
        ]
    );

    rv32_sys.run();
    rv32_sys.reg_dump();
    rv32_sys.mem_dump(0x10);
}
