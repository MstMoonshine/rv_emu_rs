use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::{cell::RefCell, sync::Arc};

use super::{decode::Decode, PipelineStage, Stage};

#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
enum ALUOperation {
    ADD = 0b000,
    SLL = 0b001,
    SLT = 0b010,
    XOR = 0b100,
    SRL = 0b101,
    OR = 0b110,
    AND = 0b111,
}

pub struct Execute<'a> {
    stage: Arc<RefCell<Stage>>,
    prev_stage: &'a Decode<'a>,

    alu_result: RefCell<u32>,
    alu_result_ready: RefCell<u32>,
}

impl<'a> Execute<'a> {
    pub fn new(stage: Arc<RefCell<Stage>>, prev_stage: &'a Decode) -> Self {
        Self {
            stage: stage.clone(),
            prev_stage,

            alu_result: RefCell::new(0_u32),
            alu_result_ready: RefCell::new(0_u32),
        }
    }

    pub fn get_alu_result_out(&self) -> u32 {
        *self.alu_result_ready.borrow()
    }
}

impl<'a> PipelineStage for Execute<'a> {
    fn compute(&self) {
        if self.should_stall() {
            return;
        }

        let de_val = self.prev_stage.get_decoded_values_out();

        let is_register_op = (de_val.opcode >> 5) & 1 == 1;
        let is_alternate = (de_val.imm11_0 >> 10) & 1 == 1;
        let imm32 = (de_val.imm11_0 << 21) as i32 >> 21;

        match ALUOperation::try_from(de_val.funt3) {
            Ok(ALUOperation::ADD) => {
                if is_register_op {
                    let result = if is_alternate {
                        de_val.rs1 - de_val.rs2
                    } else {
                        de_val.rs1 + de_val.rs2
                    };
                    self.alu_result.replace(result);
                } else {
                    let result = (de_val.rs1 as i32 + imm32) as u32;
                    self.alu_result.replace(result);
                }
            }

            _ => {
                println!("Unimplemented! func3 = {:#050b}", de_val.funt3);
            }
        }
    }

    fn latch_next(&self) {
        self.alu_result_ready.replace(*self.alu_result.borrow());
    }

    fn should_stall(&self) -> bool {
        !matches!(self.stage.borrow().to_owned(), Stage::EXE)
    }
}
