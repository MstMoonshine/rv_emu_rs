use std::{cell::RefCell, sync::Arc};

use crate::register::RegFile;

use super::{
    memory_access::MemoryAccessValues, PipelineStage, Stage,
};

#[derive(Debug, Clone, Copy)]
pub struct WriteBackValues {
    _dummy: u8,
}

pub struct WriteBack {
    stage: Arc<RefCell<Stage>>,

    reg_file: Arc<RegFile>,
}

impl WriteBack {
    pub fn new(
        stage: Arc<RefCell<Stage>>,
        reg_file: Arc<RegFile>,
    ) -> Self {
        Self { stage, reg_file }
    }
}

impl PipelineStage<MemoryAccessValues, WriteBackValues>
    for WriteBack
{
    fn compute(&self, values: MemoryAccessValues) {
        if self.should_stall() {
            return;
        }

        let mem_val = values;

        let write_back_value = mem_val.write_back_value;
        let rd = mem_val.rd;
        let is_alu_operation = mem_val.is_alu_operation;
        let is_load = mem_val.is_load;
        let is_lui = mem_val.is_lui;
        let is_jal = mem_val.is_jal;
        let is_jalr = mem_val.is_jalr;
        let is_auipc = mem_val.is_auipc;

        let should_write_back = is_alu_operation
            | is_load
            | is_lui
            | is_jal
            | is_jalr
            | is_auipc;

        if should_write_back {
            if rd != 0 {
                self.reg_file.borrow_mut()[rd as usize].0 =
                    write_back_value;
            }
        }
    }

    fn should_stall(&self) -> bool {
        !matches!(self.stage.borrow().to_owned(), Stage::WB)
    }

    fn latch_next(&self) {}

    fn get_values_out(&self) -> WriteBackValues {
        WriteBackValues { _dummy: 0 }
    }
}
