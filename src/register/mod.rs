use std::cell::RefCell;

pub const NUM_REGISTER: usize = 32;

#[derive(Debug, Clone, Copy)]
pub struct Register32(pub u32);

pub type RegFile = RefCell<[Register32; NUM_REGISTER]>;