use super::{mmio_device::MMIODevice, ADDR_ALIGN};
use std::cell::RefCell;

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
}

impl MMIODevice for ROMDevice {
    fn read(&self, offset: usize) -> u32 {
        if offset < self.size {
            self.rom.borrow()[offset]
        } else {
            0_u32
        }
    }

    fn write(&self, _offset: usize, _val: u32) {}
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
