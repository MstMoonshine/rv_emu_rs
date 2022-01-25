pub mod instruction_fetch;
pub mod decode;
pub mod execute;
pub mod memory_access;
pub mod write_back;

#[derive(Debug, Clone, Copy)]
pub enum Stage {
    IF,
    DE,
    EXE,
    MEM,
    WB,
}

pub trait PipelineStage {
    fn compute(&self);
    fn latch_next(&self);
    fn should_stall(&self) -> bool;
}