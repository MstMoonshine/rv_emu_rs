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

// li	x1, 1
// li	x2, 2
// add	x3, x1, x2

// li	x4, 0x80000000
// sw	x3, 0(x4)

// li	x5, 0xdeadbeef
// lw	x6, 0(x4)
// add	x7, x5, x6

// sw	x7, 4(x4)
// sh	x7, 8(x4)
// sb	x7, 12(x4)

    let file = &[
        0x00100093,
        0x00200113,
        0x002081b3,
        0x80000237,
        0x00322023,
        0xdeadc2b7,
        0xeef28293,
        0x00022303,
        0x006283b3,
        0x00722223,
        0x00721423,
        0x00720623,
    ];

    let rv32_sys = RV32System::new(file);

    rv32_sys.run();
    rv32_sys.reg_dump();
    rv32_sys.mem_dump(0x10);
}
