use super::{
    instruction_fetch::InstructionFetchValues, PipelineStage,
    Stage,
};
use crate::register::RegFile;
use std::{cell::RefCell, sync::Arc};

#[derive(Debug, Clone, Copy)]
pub struct DecodedValues {
    pub instruction: u32,
    pub opcode: u32,
    pub rd: u32,
    pub funct3: u32,
    pub rs1: u32,
    pub rs2: u32,
    pub imm11_0: u32,
    pub funct7: u32,
    pub shamt: u32,

    pub pc: u32,
    pub pc_plus_four: u32,

    pub is_alu_operation: bool,
    pub is_store: bool,
    pub is_load: bool,
    pub is_lui: bool,
    pub is_jal: bool,
    pub is_jalr: bool,
    pub is_branch: bool,

    pub imm32: i32,
}

impl DecodedValues {
    fn new() -> Self {
        Self {
            instruction: 0,
            opcode: 0,
            rd: 0,
            funct3: 0,
            rs1: 0,
            rs2: 0,
            imm11_0: 0,
            funct7: 0,
            shamt: 0,

            pc: 0,
            pc_plus_four: 0,

            is_alu_operation: false,
            is_store: false,
            is_load: false,
            is_lui: false,
            is_jal: false,
            is_jalr: false,
            is_branch: false,

            imm32: 0,
        }
    }
}

pub struct Decode {
    stage: Arc<RefCell<Stage>>,

    reg_file: Arc<RegFile>,

    de_val: RefCell<DecodedValues>,
    de_val_ready: RefCell<DecodedValues>,
}

impl Decode {
    pub fn new(
        stage: Arc<RefCell<Stage>>,
        reg_file: Arc<RegFile>,
    ) -> Self {
        Self {
            stage,

            reg_file,

            de_val: RefCell::new(DecodedValues::new()),
            de_val_ready: RefCell::new(DecodedValues::new()),
        }
    }
}

impl PipelineStage<InstructionFetchValues, DecodedValues>
    for Decode
{
    fn compute(&self, values: InstructionFetchValues) {
        if self.should_stall() {
            return;
        }

        let if_val = values;
        let instruction = if_val.instruction;

        let mut val = self.de_val.borrow_mut();
        val.instruction = instruction;
        val.opcode = instruction & 0x7f;
        val.rd = (instruction >> 7) & 0x1f;
        val.funct3 = (instruction >> 12) & 0x7;
        val.imm11_0 = (instruction >> 20) & 0xfff;
        val.funct7 = (instruction >> 25) & 0x7f;
        let rs1_addr = ((instruction >> 15) & 0x1f) as usize;
        let rs2_addr = ((instruction >> 20) & 0x1f) as usize;
        val.shamt = rs2_addr as u32;

        val.pc = if_val.pc;
        val.pc_plus_four = if_val.pc_plus_four;

        val.rs1 = if rs1_addr == 0 {
            0
        } else {
            self.reg_file.borrow()[rs1_addr].0
        };
        val.rs2 = if rs2_addr == 0 {
            0
        } else {
            self.reg_file.borrow()[rs2_addr].0
        };

        val.is_alu_operation =
            val.opcode & 0b101_1111 == 0b001_0011;
        val.is_store = val.opcode == 0b010_0011;
        val.is_lui = val.opcode == 0b011_0111;
        val.is_load = val.opcode == 0b000_0011;
        val.is_jal = val.opcode == 0b110_1111;
        val.is_jalr = val.opcode == 0b110_0111;
        val.is_branch = val.opcode == 0b110_0011;

        let u_imm = (instruction >> 12 << 12) as i32;
        let s_imm = (((((instruction >> 25) & 0x7f) << 5)
            | ((instruction >> 7) & 0x1f))
            << 20) as i32
            >> 20;
        let i_imm = (val.imm11_0 << 20) as i32 >> 20; // signed extended

        let j_imm_20 = (instruction & (0x1 << 31)) >> 31;
        let j_imm_10_1 = (instruction & (0x3ff << 21)) >> 21;
        let j_imm_11 = (instruction & (0x1 << 20)) >> 20;
        let j_imm_19_12 = (instruction & (0xff << 12)) >> 12;
        let j_imm = (((j_imm_20 << 20)
            | (j_imm_19_12 << 12)
            | (j_imm_11 << 11)
            | (j_imm_10_1 << 1))
            << 11) as i32
            >> 11;

        let b_imm_12 = (instruction & (0x1 << 31)) >> 31;
        let b_imm_10_5 = (instruction & (0x3f << 25)) >> 25;
        let b_imm_4_1 = (instruction & (0xf << 8)) >> 8;
        let b_imm_11 = (instruction & (0x1 << 7)) >> 7;
        let b_imm = (((b_imm_12 << 12)
            | (b_imm_11 << 11)
            | (b_imm_10_5 << 5)
            | (b_imm_4_1 << 1))
            << 19) as i32
            >> 19;

        val.imm32 = if val.is_store {
            s_imm
        } else if val.is_lui {
            u_imm
        } else if val.is_alu_operation
            || val.is_load
            || val.is_jalr
        {
            i_imm
        } else if val.is_jal {
            j_imm
        } else if val.is_branch {
            b_imm
        } else {
            if val.instruction != 0 {
                println!(
                    "Error: not implemented! opcode = {:#09b}, \
                    instruction = {:#010x}\n",
                    &val.opcode, &val.instruction
                ); // should add error handling logic
            }
            0_i32
        };
    }

    fn should_stall(&self) -> bool {
        !matches!(self.stage.borrow().to_owned(), Stage::DE)
    }

    fn get_values_out(&self) -> DecodedValues {
        self.de_val_ready.borrow().to_owned()
    }

    fn latch_next(&self) {
        self.de_val_ready
            .replace(self.de_val.borrow().to_owned());
    }
}
