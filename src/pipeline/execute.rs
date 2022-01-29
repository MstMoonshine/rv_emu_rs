use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::{cell::RefCell, sync::Arc};

use super::instruction_fetch::PCUpdateInfo;
use super::{decode::DecodedValues, PipelineStage, Stage};

#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(u32)]
enum ALUOperation {
    ADD = 0b000, // also SUB
    SLL = 0b001,
    SLT = 0b010,
    SLTU = 0b011,
    XOR = 0b100,
    SRL = 0b101, // also SRA
    OR = 0b110,
    AND = 0b111,
}

#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(u32)]
enum BranchType {
    BEQ = 0b000,
    BNE = 0b001,
    BLT = 0b100,
    BGE = 0b101,
    BLTU = 0b110,
    BGEU = 0b111,
}

#[derive(Debug, Clone, Copy)]
pub struct ExecutionValues {
    pub rd: u32,
    pub funct3: u32,
    pub rs1: u32,
    pub rs2: u32,
    pub imm11_0: u32,
    pub shamt: u32,
    pub imm32: i32,

    pub pc: u32,
    pub pc_plus_four: u32,

    pub alu_result: u32,
    pub is_alu_operation: bool,
    pub is_store: bool,
    pub is_load: bool,
    pub is_lui: bool,
    pub is_jal: bool,
    pub is_jalr: bool,
    pub is_branch: bool,
}

impl ExecutionValues {
    pub fn new() -> Self {
        Self {
            rd: 0,
            funct3: 0,
            rs1: 0,
            rs2: 0,
            imm11_0: 0,
            shamt: 0,
            imm32: 0,

            pc: 0,
            pc_plus_four: 0,

            alu_result: 0,
            is_alu_operation: false,
            is_store: false,
            is_load: false,
            is_lui: false,
            is_jal: false,
            is_jalr: false,
            is_branch: false,
        }
    }
}

pub struct Execute {
    stage: Arc<RefCell<Stage>>,

    pc_update_info: RefCell<PCUpdateInfo>,

    exe_val: RefCell<ExecutionValues>,
    exe_val_ready: RefCell<ExecutionValues>,
}

impl Execute {
    pub fn new(stage: Arc<RefCell<Stage>>) -> Self {
        Self {
            stage,

            pc_update_info: RefCell::new(PCUpdateInfo {
                should_update: false,
                pc_new: 0,
            }),

            exe_val: RefCell::new(ExecutionValues::new()),
            exe_val_ready: RefCell::new(ExecutionValues::new()),
        }
    }

    pub fn get_pc_update_info(&self) -> PCUpdateInfo {
        self.pc_update_info.borrow().to_owned()
    }
}

impl PipelineStage<DecodedValues, ExecutionValues> for Execute {
    fn compute(&self, values: DecodedValues) {
        if self.should_stall() {
            return;
        }

        let de_val = values;
        let mut exe_val = self.exe_val.borrow_mut();
        let mut pc_update_info =
            self.pc_update_info.borrow_mut();

        exe_val.rd = de_val.rd;
        exe_val.funct3 = de_val.funct3;
        exe_val.rs1 = de_val.rs1;
        exe_val.rs2 = de_val.rs2;
        exe_val.imm11_0 = de_val.imm11_0;
        exe_val.shamt = de_val.shamt;
        exe_val.imm32 = de_val.imm32;
        exe_val.is_alu_operation = de_val.is_alu_operation;
        exe_val.is_store = de_val.is_store;
        exe_val.is_load = de_val.is_load;
        exe_val.is_lui = de_val.is_lui;
        exe_val.is_jal = de_val.is_jal;
        exe_val.is_jalr = de_val.is_jalr;
        exe_val.is_branch = de_val.is_branch;

        exe_val.pc = de_val.pc;
        exe_val.pc_plus_four = de_val.pc_plus_four;

        let is_register_op = (de_val.opcode >> 5) & 1 == 1;
        let is_alternate = (de_val.imm11_0 >> 10) & 1 == 1;

        let right_operant = if is_register_op {
            de_val.rs2
        } else {
            de_val.imm11_0
        };

        // ALU
        let add_result = if exe_val.is_jalr {
            (de_val.rs1 as i32 + de_val.imm32) as u32
        } else if exe_val.is_jal || exe_val.is_branch {
            (exe_val.pc as i32 + exe_val.imm32) as u32
        } else if is_register_op {
            if is_alternate {
                de_val.rs1 - de_val.rs2
            } else {
                de_val.rs1 + de_val.rs2
            }
        } else {
            (de_val.rs1 as i32 + de_val.imm32) as u32
        };

        let sll_result = if is_register_op {
            de_val.rs1 << (de_val.rs2 & 0x1f)
        } else {
            de_val.rs1 << (de_val.shamt)
        };

        let srl_result = {
            let shamt = if is_register_op {
                de_val.rs2
            } else {
                de_val.shamt
            };

            if is_alternate {
                ((de_val.rs1 as i32) >> (shamt & 0x1f)) as u32 // SRA
            } else {
                de_val.rs1 >> (shamt & 0x1F) // SRL
            }
        };

        let sltu_result = if de_val.rs1 < right_operant {
            1_u32
        } else {
            0_u32
        };

        let slt_result =
            if (de_val.rs1 as i32) < (right_operant as i32) {
                1_u32
            } else {
                0_u32
            };

        let and_result = de_val.rs1 & right_operant;
        let or_result = de_val.rs1 | right_operant;
        let xor_result = de_val.rs1 ^ right_operant;

        exe_val.alu_result =
            match ALUOperation::try_from(de_val.funct3) {
                Ok(ALUOperation::ADD) => add_result,
                Ok(ALUOperation::SLL) => sll_result,
                Ok(ALUOperation::SRL) => srl_result,
                Ok(ALUOperation::SLTU) => sltu_result,
                Ok(ALUOperation::SLT) => slt_result,
                Ok(ALUOperation::AND) => and_result,
                Ok(ALUOperation::OR) => or_result,
                Ok(ALUOperation::XOR) => xor_result,
                _ => {
                    // println!("Unimplemented! funct3 = {:#05b}, instruction = {:#010x}",
                    //     de_val.funct3,
                    //     de_val.instruction);
                    0_u32
                }
            };

        if exe_val.is_branch {
            let _hook = 1;
        }

        let beq_result = de_val.rs1 == de_val.rs2;
        let slt_result = slt_result == 1;
        let sltu_result = sltu_result == 1;
        let branch_condition_met = exe_val.is_branch
            && match BranchType::try_from(de_val.funct3) {
                Ok(BranchType::BEQ) => beq_result,
                Ok(BranchType::BNE) => !beq_result,
                Ok(BranchType::BLT) => slt_result,
                Ok(BranchType::BGE) => !slt_result,
                Ok(BranchType::BLTU) => sltu_result,
                Ok(BranchType::BGEU) => !sltu_result,
                _ => false,
            };

        pc_update_info.should_update = exe_val.is_jal
            || exe_val.is_jalr
            || branch_condition_met;
        pc_update_info.pc_new = add_result;
    }

    fn should_stall(&self) -> bool {
        !matches!(self.stage.borrow().to_owned(), Stage::EXE)
    }

    fn latch_next(&self) {
        self.exe_val_ready
            .replace(self.exe_val.borrow().to_owned());
    }

    fn get_values_out(&self) -> ExecutionValues {
        self.exe_val_ready.borrow().to_owned()
    }
}
