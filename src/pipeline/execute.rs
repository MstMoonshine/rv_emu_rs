use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::{cell::RefCell, sync::Arc};

use super::{decode::Decode, PipelineStage, Stage};

#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(u32)]
enum ALUOperation {
    ADD = 0b000, // also SUB
    SLL = 0b001,
    SLT = 0b010,
    SLTU = 0b011,
    XOR = 0b100,
    SRL = 0b101, // also SRA
    OR  = 0b110,
    AND = 0b111,
}


#[derive(Debug, Clone, Copy)]
pub struct ExecutionValues {
    pub rd:             u32,
    pub funct3:         u32,
    pub rs1:            u32,
    pub rs2:            u32,
    pub imm11_0:        u32,
    pub shamt:          u32,
    pub imm32:          i32,

    pub alu_result:     u32,
    pub is_alu_operation: bool,
    pub is_store:   bool,
    pub is_load:    bool,
    pub is_lui:  bool,
}

impl ExecutionValues {
    pub fn new() -> Self {
        Self {
            rd:                 0,
            funct3:             0,
            rs1:                0,
            rs2:                0,
            imm11_0:            0,
            shamt:              0,
            imm32:              0,

            alu_result:         0,
            is_alu_operation:   false,
            is_store:           false, 
            is_load:            false, 
            is_lui:          false, 
        }
    }
}

pub struct Execute {
    stage: Arc<RefCell<Stage>>,
    prev_stage: Arc<Decode>,

    exe_val: RefCell<ExecutionValues>,
    exe_val_ready: RefCell<ExecutionValues>
}

impl Execute {
    pub fn new(stage: Arc<RefCell<Stage>>, prev_stage: Arc<Decode>) -> Self {
        Self {
            stage,
            prev_stage,

            exe_val: RefCell::new(ExecutionValues::new()),
            exe_val_ready: RefCell::new(ExecutionValues::new()),
        }
    }

    pub fn get_execution_values_out(&self) -> ExecutionValues {
        self.exe_val_ready.borrow().to_owned()
    }
}

impl PipelineStage for Execute {
    fn compute(&self) {
        if self.should_stall() { return; }

        let de_val = self.prev_stage.get_decoded_values_out();
        let mut exe_val = self.exe_val.borrow_mut();

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

        let is_register_op = (de_val.opcode >> 5) & 1 == 1;
        let is_alternate = (de_val.imm11_0 >> 10) & 1 == 1;

        let right_number = if is_register_op { de_val.rs2 } else { de_val.imm11_0 };

        exe_val.alu_result = match ALUOperation::try_from(de_val.funct3) {
            Ok(ALUOperation::ADD) => {
                if is_register_op {
                    if is_alternate {
                        de_val.rs1 - de_val.rs2
                    } else {
                        de_val.rs1 + de_val.rs2
                    }
                } else {
                    (de_val.rs1 as i32 + de_val.imm32) as u32
                }
            },

            Ok(ALUOperation::SLL) => if is_register_op {
                de_val.rs1 << (de_val.rs2 & 0x1f)
            } else {
                de_val.rs1 << (de_val.shamt)
            },

            Ok(ALUOperation::SRL) => {
                let shamt = if is_register_op {
                    de_val.rs2
                } else {
                    de_val.shamt
                };

                if is_alternate {
                    ((de_val.rs1 as i32) >> (shamt & 0x1f)) as u32 // SRA
                } else {
                    de_val.rs1 >> (shamt & 0x1F)                   // SRL
                }
            },
            Ok(ALUOperation::SLTU) => if de_val.rs1 < right_number { 1_u32 } else { 0_u32 },
            Ok(ALUOperation::SLT) => if (de_val.rs1 as i32) < (right_number as i32) { 1_u32 } else { 0_u32 },

            Ok(ALUOperation::AND) => de_val.rs1 & right_number,
            Ok(ALUOperation::OR) => de_val.rs1 | right_number,
            Ok(ALUOperation::XOR) => de_val.rs1 ^ right_number,

            _ => {
                // println!("Unimplemented! funct3 = {:#05b}, instruction = {:#010x}",
                //     de_val.funct3,
                //     de_val.instruction);
                0_u32
            }
        };
    }

    fn latch_next(&self) {
        self.exe_val_ready.replace(self.exe_val.borrow().to_owned());
    }

    fn should_stall(&self) -> bool {
        !matches!(self.stage.borrow().to_owned(), Stage::EXE)
    }
}
