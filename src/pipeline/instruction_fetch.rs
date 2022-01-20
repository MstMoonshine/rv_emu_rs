use std::{sync::Arc, cell::RefCell, ops::Deref};

use crate::{bus::Bus, register::Register32};

use super::{PipelineStage, State};

pub struct InstructionFetch {
	bus: Arc<Bus>,
	state: Arc<State>,

	pc: RefCell<Register32>,
	pc_ready: RefCell<Register32>,

	instruction: RefCell<Register32>,
	instruction_ready: RefCell<Register32>,	
}

impl InstructionFetch {
	pub fn new(bus: Arc<Bus>, state: Arc<State>) -> Self {
		Self {
			bus: bus.clone(),
			state: state.clone(),

			pc: RefCell::new(Register32(bus.memory_layout.rom_start as u32)),		
			pc_ready: RefCell::new(Register32(bus.memory_layout.rom_start as u32)),

			instruction: RefCell::new(Register32(0)),
			instruction_ready: RefCell::new(Register32(0)),
		}
	}

	fn get_instruction_out(&self) -> Register32 {
		*self.instruction_ready.borrow()
	}
}

impl PipelineStage for InstructionFetch {
    fn compute(&self) {
		if !self.should_stall() {
			let addr = self.pc.borrow().0 as usize;
			let ins = self.bus.read(addr)
				.expect("Instruction Fetch Error");
			self.instruction.replace(Register32(ins));

			self.pc.replace(Register32(addr as u32 + 4));
		}
    }

    fn latch_next(&self) {
		self.instruction_ready.replace(*self.instruction.borrow());
		self.pc_ready.replace(*self.pc.borrow());
    }

    fn should_stall(&self) -> bool {
		!matches!(self.state.deref(), State::IF)
    }
}

#[cfg(test)]
#[test]
fn test() {
	fn show_if(stage_if: &InstructionFetch) {
		println!("pc: {:#010x}", stage_if.pc.borrow().0);
		println!("pc_ready: {:#010x}", stage_if.pc_ready.borrow().0);

		println!("instruction: {:#010x}", stage_if.instruction.borrow().0);
		println!("instruction_ready: {:#010x}", stage_if.instruction_ready.borrow().0);
	}

	let bus = Bus::new(&[0x1122_3344_u32, 0xdead_beef, 1, 2, 3, 4, 5, 0xaabbccdd, 0x44332211]);
	let state = State::IF;

	let stage_if = InstructionFetch::new(Arc::new(bus), Arc::new(state));
	show_if(&stage_if);

	for i in 0..5 {
		println!("------- {} -------", i);

		stage_if.compute();

		show_if(&stage_if);

		stage_if.latch_next();
	}
}