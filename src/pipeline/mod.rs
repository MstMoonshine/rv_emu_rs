pub mod instruction_fetch;
pub mod decode;

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

#[cfg(test)]
#[test]
fn test() {
    use std::{sync::Arc, cell::RefCell};

    use crate::{bus::Bus, pipeline::{instruction_fetch::InstructionFetch, decode::Decode}};

	fn show_de(stage_de: &Decode) {
		println!("Decoded values: {:?}", stage_de.get_decoded_values_out());
	}

	let bus = Arc::new(Bus::new(
		&[0x00100093, 0x00200113, 0x002081b3])
	);
	let stage = Arc::new(RefCell::new(Stage::IF));

	let stage_if = InstructionFetch::new(
		bus.clone(), 
		stage.clone(),
	);

	let stage_de = Decode::new(
		stage.clone(),
		&stage_if,
	);

	for _ in 0..10 {

		stage_if.compute();
		stage_de.compute();

		show_de(&stage_de);

		stage_if.latch_next();
		stage_de.latch_next();


		let current_stage = stage.borrow().to_owned();
		let next_stage = match current_stage {
			Stage::IF => Stage::DE,
			Stage::DE => Stage::IF,
			_ => Stage::IF,
		};

		stage.replace(next_stage);
	}
}