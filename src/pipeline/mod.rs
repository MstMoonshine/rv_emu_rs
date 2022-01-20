pub mod instruction_fetch;

pub enum State {
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