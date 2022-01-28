use super::{
    memory_access::MemoryAccessWidth, PipelineStage, Stage,
};
use crate::bus::Bus;
use std::{cell::RefCell, sync::Arc};

#[derive(Debug, Clone, Copy)]
pub struct PCUpdateInfo {
    pub should_update: bool,
    pub pc_new: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct InstructionFetchValues {
    pub pc: u32,
    pub pc_plus_four: u32,
    pub instruction: u32,
}

impl InstructionFetchValues {
    pub fn new(entry_point: u32) -> Self {
        Self {
            pc: entry_point,
            pc_plus_four: entry_point + 4,
            instruction: 0_u32,
        }
    }
}

pub struct InstructionFetch {
    stage: Arc<RefCell<Stage>>,

    bus: Arc<Bus>,

    if_val: RefCell<InstructionFetchValues>,
    if_val_ready: RefCell<InstructionFetchValues>,
}

impl InstructionFetch {
    pub fn new(
        stage: Arc<RefCell<Stage>>,
        bus: Arc<Bus>,
    ) -> Self {
        Self {
            stage,

            bus: bus.clone(),

            if_val: RefCell::new(
                InstructionFetchValues::new(
                    bus.memory_layout.rom_start as u32,
                ),
            ),
            if_val_ready: RefCell::new(
                InstructionFetchValues::new(
                    bus.memory_layout.rom_start as u32,
                ),
            ),
        }
    }
}

impl PipelineStage<PCUpdateInfo, InstructionFetchValues>
    for InstructionFetch
{
    fn compute(&self, values: PCUpdateInfo) {
        if self.should_stall() {
            return;
        }

        let mut val = self.if_val.borrow_mut();

        let addr = val.pc as usize;
        val.instruction = self
            .bus
            .read(addr, MemoryAccessWidth::Word)
            .expect("Instruction Fetch Error");

        val.pc = addr as u32 + 4;
    }

    fn should_stall(&self) -> bool {
        !matches!(self.stage.borrow().to_owned(), Stage::DE)
    }

    fn latch_next(&self) {
        self.if_val_ready
            .replace(self.if_val.borrow().to_owned());
    }

    fn get_values_out(&self) -> InstructionFetchValues {
        self.if_val_ready.borrow().to_owned()
    }
}
