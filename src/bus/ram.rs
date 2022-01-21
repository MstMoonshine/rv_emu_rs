use super::mmio_device::MMIODevice;
use std::cell::RefCell;

pub struct RAMDevice {
    ram: RefCell<Vec<u32>>,
    size: usize,
}

impl RAMDevice {
    pub fn new(size: usize) -> Self {
        Self {
            ram: RefCell::new(vec![0_u32; size]),
            size,
        }
    }
}

impl MMIODevice for RAMDevice {
    fn read(&self, location: usize) -> u32 {
        if location < self.size {
            self.ram.borrow()[location]
        } else {
            0_u32
        }
    }

    fn write(&self, location: usize, val: u32) {
        self.ram.borrow_mut()[location] = val;
    }
}

#[cfg(test)]
#[test]
fn test() {
    let ram = RAMDevice::new(0x1000_0000);

    ram.write(0x0, 0x1122_3344);

    ram.write(0x4, 0xdead_beef);

    let mut val = ram.read(0x0);
    println!("{:#x}", val);

    val = ram.read(0x4);
    println!("{:#x}", val);
}
