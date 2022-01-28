use super::{mmio_device::MMIODevice, ADDR_ALIGN};
use std::{cell::RefCell, fmt};

#[derive(Debug)]
pub enum ROMError {
    _LoadError,
}
impl std::error::Error for ROMError {}

impl fmt::Display for ROMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ROMError::_LoadError => {
                write!(f, "ROM Load Error")
            }
        }
    }
}

pub struct ROMDevice {
    rom: RefCell<Vec<u32>>,
    size: usize,
}

impl ROMDevice {
    pub fn new(file: &[u32]) -> Self {
        Self {
            rom: RefCell::new(Vec::from(file)),
            size: file.len() * ADDR_ALIGN,
        }
    }

    pub fn _load(&self, file: &[u32]) -> Result<(), ROMError> {
        if file.len() * 4 > self.size {
            Err(ROMError::_LoadError)
        } else {
            self.rom.replace(Vec::from(file));
            Ok(())
        }
    }
}

impl MMIODevice for ROMDevice {
    fn read(&self, location: usize) -> u32 {
        if location < self.size {
            self.rom.borrow()[location]
        } else {
            0_u32
        }
    }

    fn write(&self, _location: usize, _val: u32) {}
}

#[cfg(test)]
#[test]
fn test() {
    let file = [1_u32, 2, 3, 4, 5];

    let ram = ROMDevice::new(&file);

    ram.write(0x0, 0x1122_3344); // invalid

    ram.write(0x4, 0xdead_beef); // invalid

    let mut val = ram.read(0x0);
    println!("{:#x}", val);

    val = ram.read(0x4);
    println!("{:#x}", val);
}
