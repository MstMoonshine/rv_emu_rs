pub mod instruction_fetch;
pub mod decode;
pub mod execute;
pub mod memory_access;
pub mod write_back;

#[derive(Debug, Clone, Copy)]
pub enum Stage {
    IF,
    DE,
    EXE,
    MEM,
    WB,
}

pub trait PipelineStage {
    fn compute(&self);
    fn latch_next(&self);
    fn should_stall(&self) -> bool;
}

// #[cfg(test)]
// #[test]
// fn test() {
//     use std::{cell::RefCell, sync::Arc};
//     use crate::{
//         bus::Bus,
//         pipeline::{decode::Decode, execute::Execute, instruction_fetch::InstructionFetch, memory_access::MemoryAccess, write_back::WriteBack}, register::{Register32, NUM_REGISTER},
//     };

//     fn show_de(stage_de: &Decode) {
//         println!("Decoded values: {:?}", stage_de.get_decoded_values_out());
//     }

//     fn show_exe(stage_exe: &Execute) {
//         println!("Execution Values: {:?}", stage_exe.get_execution_values_out());
//     }

//     let bus = Arc::new(Bus::new(&[
//         0x00100093, // addi x1, x0, 1
//         0xfff00213, // addi x4, x0, -1
//         0x00200113, // li x2, 2
//         0x002081b3, // add x3, x1, x2
//         0x0032a023, // sw x3, 0(x5)
//     ]));
//     let reg_file = Arc::new(RefCell::new(
//         [Register32(0); NUM_REGISTER]
//     ));

//     // for testing
//     // reg_file.borrow_mut()[3] = Register32(0xdead_beef);
//     reg_file.borrow_mut()[5] = Register32(0x8000_0000);

//     let stage = Arc::new(RefCell::new(Stage::IF));

//     let stage_if = InstructionFetch::new(stage.clone(), bus.clone());
//     let stage_de = Decode::new(stage.clone(), &stage_if, reg_file.clone());
//     let stage_exe = Execute::new(stage.clone(), &stage_de);
//     let stage_mem = MemoryAccess::new(stage.clone(), &stage_exe, bus.clone());
//     let stage_wb = WriteBack::new(stage.clone(), &stage_mem, reg_file.clone());

//     for _ in 0..((bus.memory_layout.rom_size / 4 + 1) * 5) {
//         stage_if.compute();
//         stage_de.compute();
//         stage_exe.compute();
//         stage_mem.compute();
//         stage_wb.compute();

//         // show_de(&stage_de);
//         // show_exe(&stage_exe);

//         stage_if.latch_next();
//         stage_de.latch_next();
//         stage_exe.latch_next();
//         stage_mem.latch_next();
//         stage_wb.latch_next();

//         let current_stage = stage.borrow().to_owned();
//         let next_stage = match current_stage {
//             Stage::IF => Stage::DE,
//             Stage::DE => Stage::EXE,
//             Stage::EXE => Stage::MEM,
//             Stage::MEM => Stage::WB,
//             Stage::WB => Stage::IF,
//         };

//         stage.replace(next_stage);
//     }

//     let addr = 0x8000_0000 as usize;

//     println!("{:#010x}: {:#010x}", 
//         addr,
//         bus.read(addr, memory_access::MemoryAccessWidth::Word)
//         .expect("error")
//     );
//     println!("{:?}", reg_file);
// }
