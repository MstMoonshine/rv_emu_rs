use std::{cell::RefCell, sync::Arc};

use num_enum::TryFromPrimitive;

use crate::bus::{Bus};

use super::{Stage, execute::Execute, PipelineStage};

#[derive(Debug, Clone, Copy)]
pub struct MemoryAccessValues {
	rd:		u32,
	rs1: 	u32,
	rs2: 	u32,
	funct3:	u32,

	is_alu_operation: 	bool,
	is_store: 			bool,

	imm32: 		i32,
	alu_result: u32,
}

impl MemoryAccessValues {
	pub fn new() -> Self {
		Self {
			rd: 	0,
			rs1:    0,
			rs2:    0,
			funct3: 0,

			is_alu_operation:	false,
			is_store:			false,

			imm32: 			0_i32,
			alu_result: 	0,
		}
	}
}

#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(u32)]
pub enum MemoryAccessWidth {
	Byte		= 0b000,
	HalfWord	= 0b001,
	Word		= 0b010,
}

pub struct MemoryAccess<'a> {
	stage: Arc<RefCell<Stage>>,
	prev_stage: &'a Execute<'a>,

	bus: Arc<Bus>,

	mem_val: RefCell<MemoryAccessValues>,
	mem_val_ready: RefCell<MemoryAccessValues>,
}

impl<'a> MemoryAccess<'a> {
	pub fn new(stage: Arc<RefCell<Stage>>,
		prev_stage: &'a Execute,
		bus: Arc<Bus>) -> Self {
		Self {
			stage: stage.clone(),
			prev_stage,

			bus: bus.clone(),

			mem_val: RefCell::new(MemoryAccessValues::new()),
			mem_val_ready: RefCell::new(MemoryAccessValues::new()),
		}
	}
}

impl<'a> PipelineStage for MemoryAccess<'a> {
    fn compute(&self) {
		if self.should_stall() { return; }
		
		let exe_val = self.prev_stage.get_execution_values_out();
		let mut mem_val = self.mem_val.borrow_mut();

		mem_val.rd = exe_val.rd;
		mem_val.rs1 = exe_val.rs1;
		mem_val.rs2 = exe_val.rs2;
		mem_val.is_alu_operation = exe_val.is_alu_operation;
		mem_val.is_store = exe_val.is_store;
		mem_val.imm32 = exe_val.imm32;
		mem_val.alu_result = exe_val.alu_result;

		if mem_val.is_store {
			let addr = (mem_val.rs1 as i32 + mem_val.imm32) as usize;

			match MemoryAccessWidth::try_from(mem_val.funct3) {
				Ok(MemoryAccessWidth::Byte) => {
					// access bus here
					// self.bus.write(addr, mem_val.rs2, MemoryAccessWidth::Byte);
				},
				_ => {

				},
			};

		}
    }

    fn latch_next(&self) {
		self.mem_val_ready.replace(self.mem_val.borrow().to_owned());
    }

    fn should_stall(&self) -> bool {
        !matches!(self.stage.borrow().to_owned(), Stage::MEM)
    }
}