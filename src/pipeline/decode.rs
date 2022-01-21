use super::{instruction_fetch::InstructionFetch, PipelineStage, Stage};
use crate::register::RegFile;
use std::{cell::RefCell, sync::Arc};

#[derive(Debug, Clone)]
pub struct DecodedValues {
    pub instruction:    u32,
    pub opcode:         u32,
    pub rd:             u32,
    pub funct3:          u32,
    pub rs1:            u32,
    pub rs2:            u32,
    pub imm11_0:        u32,
    pub funct7:          u32,
    pub shamt:          u32,

    pub is_alu_operation: bool,
    pub is_store: bool,
    pub is_load: bool,
    pub imm32: i32,
}

impl DecodedValues {
    fn new() -> Self {
        Self {
            instruction:    0,
            opcode:         0,
            rd:             0,
            funct3:          0,
            rs1:            0,
            rs2:            0,
            imm11_0:        0,
            funct7:          0,
            shamt:          0,

            is_alu_operation: false,
            is_store:   false,
            is_load:    false,
            imm32: 0,
        }
    }
}

pub struct Decode<'a> {
    stage: Arc<RefCell<Stage>>,
    prev_stage: &'a InstructionFetch,

    reg_file: Arc<RegFile>,

    de_val: RefCell<DecodedValues>,
    de_val_ready: RefCell<DecodedValues>,
}

impl<'a> Decode<'a> {
    pub fn new(stage: Arc<RefCell<Stage>>,
        prev_stage: &'a InstructionFetch,
        reg_file: Arc<RegFile>) -> Self {
        Self {
            stage,
            prev_stage,

            reg_file,

            de_val: RefCell::new(DecodedValues::new()),
            de_val_ready: RefCell::new(DecodedValues::new()),
        }
    }

    pub fn get_decoded_values_out(&self) -> DecodedValues {
        self.de_val_ready.borrow().to_owned()
    }
}

impl<'a> PipelineStage for Decode<'a> {
    fn compute(&self) {
        if self.should_stall() { return; }

        let instruction = self.prev_stage.get_instruction_out().0;

        let mut val = self.de_val.borrow_mut();
        val.instruction = instruction;
        val.opcode      = instruction & 0x7f;
        val.rd          = (instruction >> 7) & 0x1f;
        val.funct3       = (instruction >> 12) & 0x7;
        val.imm11_0     = (instruction >> 20) & 0x7ff;
        val.funct7       = (instruction >> 25) & 0x7f;
        let rs1_addr = ((instruction >> 15) & 0x1f) as usize;
        let rs2_addr = ((instruction >> 20) & 0x1f) as usize;
        val.shamt       = rs2_addr as u32;

        val.rs1 = if rs1_addr == 0 { 0 } else { self.reg_file.borrow()[rs1_addr].0 };
        val.rs2 = if rs2_addr == 0 { 0 } else { self.reg_file.borrow()[rs2_addr].0 };

        val.is_alu_operation = val.opcode & 0b101_1111 == 0b001_0011;
        val.is_store         = val.opcode == 0b010_0011;
        val.is_load          = val.opcode == 0b000_0011;

        let s_imm = ((((instruction >> 25) & 0x7f) << 5) 
            | ((instruction >> 7) & 0x1f)) as i32;
        let i_imm = (val.imm11_0 << 21) as i32 >> 21;

        val.imm32 = if val.is_store {
            s_imm
        } else if val.is_alu_operation || val.is_load {
            i_imm
        } else {
            println!("Error: not implemented! opcode = {:#09b}, instruction = {:#010x}\n",
                &val.opcode, &val.instruction); // should add error handling logic
            0_i32
        };
    }

    fn latch_next(&self) {
        self.de_val_ready.replace(self.de_val.borrow().to_owned());
    }

    fn should_stall(&self) -> bool {
        !matches!(self.stage.borrow().to_owned(), Stage::DE)
    }
}
