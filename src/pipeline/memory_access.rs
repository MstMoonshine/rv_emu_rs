use std::{cell::RefCell, sync::Arc};

use num_enum::TryFromPrimitive;

use crate::bus::{Bus};

use super::{Stage, execute::Execute, PipelineStage};

#[derive(Debug, Clone, Copy)]
pub struct MemoryAccessValues {
	pub	rd:		u32,
	pub	rs1: 	u32,
	pub	rs2: 	u32,
	pub	funct3:	u32,

	pub	is_alu_operation: 	bool,
	pub	is_store: 			bool,
	pub	is_load: 			bool,
	pub	is_lui: 			bool,
	
	pub	imm32: i32,
	pub	write_back_value: u32,
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
			is_load:			false,
			is_lui:			false,

			imm32: 0_i32,
			write_back_value: 0,
		}
	}
}

#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(u32)]
pub enum MemoryAccessWidth {
	Byte		= 0b00,
	HalfWord	= 0b01,
	Word		= 0b10,
}

pub struct MemoryAccess {
	stage: Arc<RefCell<Stage>>,
	prev_stage: Arc<Execute>,

	bus: Arc<Bus>,

	mem_val: RefCell<MemoryAccessValues>,
	mem_val_ready: RefCell<MemoryAccessValues>,
}

impl MemoryAccess {
	pub fn new(stage: Arc<RefCell<Stage>>,
		prev_stage: Arc<Execute>,
		bus: Arc<Bus>) -> Self {
		Self {
			stage,
			prev_stage,

			bus,

			mem_val: RefCell::new(MemoryAccessValues::new()),
			mem_val_ready: RefCell::new(MemoryAccessValues::new()),
		}
	}

	pub fn get_memory_access_values_out(&self) -> MemoryAccessValues {
		self.mem_val_ready.borrow().to_owned()
	}
}

impl PipelineStage for MemoryAccess {
    fn compute(&self) {
		if self.should_stall() { return; }
		
		let exe_val = self.prev_stage.get_execution_values_out();
		let mut mem_val = self.mem_val.borrow_mut();

		mem_val.rd = exe_val.rd;
		mem_val.rs1 = exe_val.rs1;
		mem_val.rs2 = exe_val.rs2;
		mem_val.funct3 = exe_val.funct3;
		mem_val.is_alu_operation = exe_val.is_alu_operation;
		mem_val.is_store = exe_val.is_store;
		mem_val.is_load = exe_val.is_load;
		mem_val.is_lui = exe_val.is_lui;
		mem_val.imm32 = exe_val.imm32;
		mem_val.write_back_value = exe_val.alu_result;

		// this line should be done in the ALU
		let addr = (mem_val.rs1 as i32 + mem_val.imm32) as u32 as usize;

		if mem_val.is_store {
			let width = MemoryAccessWidth::try_from(mem_val.funct3 & 0b11)
			.expect("Invalid store width");
			self.bus.write(addr, mem_val.rs2, width)
			.expect("Memory store error");
		} else if mem_val.is_load || mem_val.is_lui {
			let signed_extend = mem_val.funct3 & 0b100 == 0;
			let width = MemoryAccessWidth::try_from(mem_val.funct3 & 0b11)
			.expect("Invalid load width");
			let val = self.bus.read(addr, width)
			.expect("Memory load error");

			mem_val.write_back_value = if mem_val.is_lui {
				mem_val.imm32 as u32
			} else if signed_extend {
				match width {
					MemoryAccessWidth::Byte => {
						val as i8 as i32 as u32
					},
					MemoryAccessWidth::HalfWord => {
						val as i16 as i32 as u32
					},
					_ => val
				}
			} else {
				val
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