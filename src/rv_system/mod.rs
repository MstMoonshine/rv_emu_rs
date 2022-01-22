use std::{sync::Arc, cell::RefCell};

use crate::{bus::Bus, register::{RegFile, Register32, NUM_REGISTER}, pipeline::{instruction_fetch::InstructionFetch, execute::Execute, memory_access::MemoryAccess, decode::Decode, write_back::WriteBack, Stage, PipelineStage}};

pub struct RV32System {
	stage: 		Arc<RefCell<Stage>>,

	bus: 		Arc<Bus>,
	reg_file: 	Arc<RegFile>,

	stage_if: 	Arc<InstructionFetch>,
	stage_de:	Arc<Decode>,
	stage_exe:	Arc<Execute>,
	stage_mem:	Arc<MemoryAccess>,
	stage_wb:	Arc<WriteBack>,
}

impl RV32System {
	pub fn new(rom_file: &[u32]) -> Self {
		let stage = Arc::new(RefCell::new(Stage::IF));
		let bus = Arc::new(Bus::new(rom_file));
		let reg_file = Arc::new(RefCell::new(
			[Register32(0); NUM_REGISTER]
		));

		let stage_if = Arc::new(
			InstructionFetch::new(stage.clone(), bus.clone())
		);
		let stage_de = Arc::new(
			Decode::new(stage.clone(), stage_if.clone(), reg_file.clone())
		);
		let stage_exe = Arc::new(
			Execute::new(stage.clone(), stage_de.clone())
		);
		let stage_mem = Arc::new(
			MemoryAccess::new(stage.clone(), stage_exe.clone(), bus.clone())
		);
		let stage_wb = Arc::new(
			WriteBack::new(stage.clone(), stage_mem.clone(), reg_file.clone())
		);

		Self {
			stage,
			bus,
			reg_file,

			stage_if,
			stage_de,
			stage_exe,
			stage_mem,
			stage_wb,
		}
	}

	pub fn run(&self) {
		for _ in 0..((self.bus.memory_layout.rom_size / 4 + 1) * 5) {
			self.compute();
			self.latch_next();

			let current_stage = self.stage.borrow().to_owned();
			let next_stage = match current_stage {
				Stage::IF => Stage::DE,
				Stage::DE => Stage::EXE,
				Stage::EXE => Stage::MEM,
				Stage::MEM => Stage::WB,
				Stage::WB => Stage::IF,
			};
			self.stage.replace(next_stage);
		}
	}

	pub fn reg_dump(&self) {
		println!("Register Dump");
		for (i, reg) in self.reg_file.borrow().into_iter().enumerate() {
			println!("x{}:\t{:#010x}", i, reg.0);
		}
	}

	pub fn mem_dump(&self, size: usize) {
		println!("Memory Dump");
		self.bus.mem_dump(size);
	}

	pub fn get_mem(&self, size: usize) -> Vec<u32> {
        self.bus.get_mem(size)
    }

	pub fn get_reg(&self) -> [Register32; NUM_REGISTER] {
		self.reg_file.borrow().to_owned()
	}

	fn compute(&self) {
		self.stage_if.compute();
		self.stage_de.compute();
		self.stage_exe.compute();
		self.stage_mem.compute();
		self.stage_wb.compute();
	}
	
	fn latch_next(&self) {
		self.stage_if.latch_next();
		self.stage_de.latch_next();
		self.stage_exe.latch_next();
		self.stage_mem.latch_next();
		self.stage_wb.latch_next();
	}
}