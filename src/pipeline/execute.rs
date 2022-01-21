use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::{cell::RefCell, sync::Arc};

use super::{decode::Decode, PipelineStage, Stage};

#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(u32)]
enum ALUOperation {
    ADD = 0b000,
    SLL = 0b001,
    SLT = 0b010,
    XOR = 0b100,
    SRL = 0b101,
    OR  = 0b110,
    AND = 0b111,
}


#[derive(Debug, Clone, Copy)]
pub struct ExecutionValues {
    pub rd:             u32,
    pub funct3:          u32,
    pub rs1:            u32,
    pub rs2:            u32,
    pub imm32:          i32,

    pub alu_result:     u32,
    pub is_alu_operation: bool,
    pub is_store:   bool,
    pub is_load:    bool,
}

impl ExecutionValues {
    pub fn new() -> Self {
        Self {
            rd:                 0,
            funct3:              0,
            rs1:                0,
            rs2:                0,
            imm32:              0,

            alu_result:         0,
            is_alu_operation:   false,
            is_store:           false, 
            is_load:            false, 
        }
    }
}

pub struct Execute<'a> {
    stage: Arc<RefCell<Stage>>,
    prev_stage: &'a Decode<'a>,

    exe_val: RefCell<ExecutionValues>,
    exe_val_ready: RefCell<ExecutionValues>
}

impl<'a> Execute<'a> {
    pub fn new(stage: Arc<RefCell<Stage>>, prev_stage: &'a Decode) -> Self {
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

impl<'a> PipelineStage for Execute<'a> {
    fn compute(&self) {
        if self.should_stall() { return; }

        let de_val = self.prev_stage.get_decoded_values_out();
        let mut exe_val = self.exe_val.borrow_mut();

        exe_val.rd = de_val.rd;
        exe_val.funct3 = de_val.funct3;
        exe_val.rs1 = de_val.rs1;
        exe_val.rs2 = de_val.rs2;
        exe_val.imm32 = de_val.imm32;
        exe_val.is_alu_operation = de_val.is_alu_operation;
        exe_val.is_store = de_val.is_store;
        exe_val.is_load = de_val.is_load;

        let is_register_op = (de_val.opcode >> 5) & 1 == 1;
        let is_alternate = (de_val.imm11_0 >> 10) & 1 == 1;

        match ALUOperation::try_from(de_val.funct3) {
            Ok(ALUOperation::ADD) => {
                if is_register_op {
                    let result = if is_alternate {
                        de_val.rs1 - de_val.rs2
                    } else {
                        de_val.rs1 + de_val.rs2
                    };
                    exe_val.alu_result = result;
                } else {
                    let result = (de_val.rs1 as i32 + de_val.imm32) as u32;
                    exe_val.alu_result = result;
                }
            }

            _ => {
                println!("Unimplemented! funct3 = {:#05b}", de_val.funct3);
            }
        }
    }

    fn latch_next(&self) {
        self.exe_val_ready.replace(self.exe_val.borrow().to_owned());
    }

    fn should_stall(&self) -> bool {
        !matches!(self.stage.borrow().to_owned(), Stage::EXE)
    }
}
