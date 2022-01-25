use std::{cell::RefCell, sync::Arc};

use crate::register::RegFile;

use super::{Stage, memory_access::MemoryAccess, PipelineStage};

pub struct WriteBack {
	stage: Arc<RefCell<Stage>>,
    prev_stage: Arc<MemoryAccess>,

	reg_file: Arc<RegFile>,
}

impl WriteBack {
	pub fn new(stage: Arc<RefCell<Stage>>,
		prev_stage: Arc<MemoryAccess>,
		reg_file: Arc<RegFile>) -> Self {
		Self {
			stage,
			prev_stage,

			reg_file,
		}
	}
}

impl PipelineStage for WriteBack {
    fn compute(&self) {
		if self.should_stall() { return; }

		let mem_val = self.prev_stage.get_memory_access_values_out();

		let write_back_value = mem_val.write_back_value;
		let rd = mem_val.rd;
		let is_alu_operation = mem_val.is_alu_operation;
		let is_load = mem_val.is_load;
		let is_load_i = mem_val.is_load_i;

		if is_alu_operation || is_load || is_load_i {
			self.reg_file.borrow_mut()[rd as usize].0 = write_back_value;
		}

    }

    fn latch_next(&self) { }

    fn should_stall(&self) -> bool {
        !matches!(self.stage.borrow().to_owned(), Stage::WB)
    }
}