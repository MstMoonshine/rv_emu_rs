use std::{cell::RefCell, sync::Arc};

use crate::{
    bus::Bus,
    pipeline::{
        decode::Decode, execute::Execute,
        instruction_fetch::InstructionFetch,
        memory_access::MemoryAccess, write_back::WriteBack,
        PipelineStage, Stage,
    },
    register::{RegFile, Register32, NUM_REGISTER},
};

pub struct RV32System {
    stage: Arc<RefCell<Stage>>,

    bus: Arc<Bus>,
    reg_file: Arc<RegFile>,

    stage_if: InstructionFetch,
    stage_de: Decode,
    stage_exe: Execute,
    stage_mem: MemoryAccess,
    stage_wb: WriteBack,
}

impl RV32System {
    pub fn new(rom_file: &[u32]) -> Self {
        let stage = Arc::new(RefCell::new(Stage::IF));
        let bus = Arc::new(Bus::new(rom_file));
        let reg_file = Arc::new(RefCell::new(
            [Register32(0); NUM_REGISTER],
        ));

        let stage_if =
            InstructionFetch::new(stage.clone(), bus.clone());
        let stage_de =
            Decode::new(stage.clone(), reg_file.clone());
        let stage_exe = Execute::new(stage.clone());
        let stage_mem =
            MemoryAccess::new(stage.clone(), bus.clone());
        let stage_wb =
            WriteBack::new(stage.clone(), reg_file.clone());

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
        for _ in
            0..((self.bus.memory_layout.rom_size / 4 + 1) * 5)
        {
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
        for (i, reg) in
            self.reg_file.borrow().into_iter().enumerate()
        {
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
        let pc_update_info = self.stage_exe.get_pc_update_info();

        self.stage_if.compute(pc_update_info);
        self.stage_de.compute(self.stage_if.get_values_out());
        self.stage_exe.compute(self.stage_de.get_values_out());
        self.stage_mem.compute(self.stage_exe.get_values_out());
        self.stage_wb.compute(self.stage_mem.get_values_out());
    }

    fn latch_next(&self) {
        self.stage_if.latch_next();
        self.stage_de.latch_next();
        self.stage_exe.latch_next();
        self.stage_mem.latch_next();
        self.stage_wb.latch_next();
    }
}
