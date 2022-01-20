pub mod decode;
pub mod execute;
pub mod instruction_fetch;

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

#[cfg(test)]
#[test]
fn test() {
    use std::{cell::RefCell, sync::Arc};

    use crate::{
        bus::Bus,
        pipeline::{decode::Decode, execute::Execute, instruction_fetch::InstructionFetch},
    };

    fn show_de(stage_de: &Decode) {
        println!("Decoded values: {:?}", stage_de.get_decoded_values_out());
    }

    fn show_exe(stage_exe: &Execute) {
        println!("ALU Result: {:?}", stage_exe.get_alu_result_out());
    }

    let bus = Arc::new(Bus::new(&[0x00100093, 0x00200113, 0x002081b3]));
    let stage = Arc::new(RefCell::new(Stage::IF));

    let stage_if = InstructionFetch::new(bus.clone(), stage.clone());
    let stage_de = Decode::new(stage.clone(), &stage_if);
    let stage_exe = Execute::new(stage.clone(), &stage_de);

    for _ in 0..15 {
        stage_if.compute();
        stage_de.compute();
        stage_exe.compute();

        show_de(&stage_de);
        show_exe(&stage_exe);

        stage_if.latch_next();
        stage_de.latch_next();
        stage_exe.latch_next();

        let current_stage = stage.borrow().to_owned();
        let next_stage = match current_stage {
            Stage::IF => Stage::DE,
            Stage::DE => Stage::EXE,
            Stage::EXE => Stage::IF,
            _ => Stage::IF,
        };

        stage.replace(next_stage);
    }
}
