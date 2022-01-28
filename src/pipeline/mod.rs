pub mod decode;
pub mod execute;
pub mod instruction_fetch;
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

pub trait PipelineStage<TI, TO>
where
    TI: Clone + Copy,
    TO: Clone + Copy,
{
    fn compute(&self, values: TI);
    fn should_stall(&self) -> bool;
    fn get_values_out(&self) -> TO;
    fn latch_next(&self);
}
