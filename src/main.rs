use rv_system::RV32System;

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

fn main() {
    let file = &[
        0x00100093, // li	x1, 1
        0x00200113, // li	x2, 2
        0x002081b3, // add	x3, x1, x2
        0x80000237, // li	x4, 0x80000000
        0x00322023, // sw	x3, 0(x4)
        0xdeadc2b7, // li	x5, 0xdeadbeef
        0xeef28293, // (cont.)
        0x00022303, // lw	x6, 0(x4)
        0x006283b3, // add	x7, x5, x6
        0x00722223, // sw	x7, 4(x4)
        0x00721423, // sh	x7, 8(x4)
        0x00720623, // sb	x7, 12(x4)
    ];

    let rv32_sys = RV32System::new(file);

    rv32_sys.run();
    rv32_sys.reg_dump();
    rv32_sys.mem_dump(0x10);
}
