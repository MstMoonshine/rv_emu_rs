use crate::bus::mmio_device::MMIODevice;

use self::{ram::RAMDevice, rom::ROMDevice, bus_error::BusError};

pub mod mmio_device;
pub mod ram;
pub mod rom;
pub mod bus_error;

const ADDR_ALIGN: usize = 4;

const ROM_START: usize = 0x4000_0000;
const RAM_START: usize = 0x8000_0000;

const RAM_SIZE: usize = 0x4000_0000;

fn get_offset(addr: usize, start: usize) -> usize {
	(addr - start) / ADDR_ALIGN 
}


pub struct Bus {
	ram: RAMDevice,
	rom: ROMDevice,
}

impl Bus {
	pub fn new(file: &[u32]) -> Self {
		Self {
			ram: RAMDevice::new(RAM_SIZE),
			rom: ROMDevice::new(file),
		}
	}

	pub fn read(&self, addr: usize) -> Result<u32, BusError> {
		if addr % ADDR_ALIGN != 0 {
			return Err(BusError::LoadAddrMisaligned(addr));
		}

		if addr >= ROM_START && addr < ROM_START + self.rom.size {
			Ok(self.rom.read(get_offset(addr, ROM_START)))
		} else if addr >= RAM_START && addr < RAM_START + self.ram.size {
			Ok(self.ram.read(get_offset(addr, RAM_START)))
		} else {
			Ok(0_u32)
		}
	}

	pub fn write(&self, addr: usize, val: u32) -> Result<(), BusError> {
		if addr % ADDR_ALIGN != 0 {
			return Err(BusError::StoreAddrMisaligned(addr, val));
		}

		if addr >= ROM_START && addr < ROM_START + self.rom.size {
			self.rom.write(get_offset(addr, ROM_START), val);
		} else if addr >= RAM_START && addr < RAM_START + self.ram.size {
			self.ram.write(get_offset(addr, RAM_START), val);
		}

		Ok(())
	}
}

// ----- test -----
fn test_read(bus: &Bus, addr: usize) {
	let val = bus.read(addr)
	.expect("read error");
	println!("{:#010x}: {:#010x}", addr, val);
}

fn test_write(bus: &Bus, addr: usize, val: u32) {
	let ret = bus.write(addr, val);
	match ret {
		Err(e) => println!("{}", e),
		_ => (),
	}
}

#[cfg(test)]
#[test]
fn test() {
    let rom_file = [0x1122_3344_u32, 0xdead_beef, 1, 2, 3, 4, 5, 0xaabbccdd, 0x44332211];

	let bus = Bus::new(&rom_file);

	test_write(&bus, 0x8000_0000, 0x1010_1010);
	test_write(&bus, 0x8000_0004, 0x2020_2020);

	test_read(&bus, 0x8000_0000);
	test_read(&bus, 0x8000_0004);

	test_write(&bus, 0x4000_0000, 0x1010_1010);
	test_write(&bus, 0x4000_0004, 0x2020_2020);

	test_read(&bus, 0x4000_0000);
	test_read(&bus, 0x4000_0004);

	test_write(&bus, 0x4000_0001, 1);
	test_read(&bus, 0x4000_0001);
}
