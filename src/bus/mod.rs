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

pub struct MemoryLayout {
	pub rom_start: usize,
	pub rom_size: usize,
	pub ram_start: usize,
	pub ram_size: usize,
}

pub struct Bus {
	ram: RAMDevice,
	rom: ROMDevice,
	pub memory_layout: MemoryLayout
}

impl Bus {
	pub fn new(file: &[u32]) -> Self {
		Self {
			ram: RAMDevice::new(RAM_SIZE),
			rom: ROMDevice::new(file),
			memory_layout: MemoryLayout {
				rom_start: ROM_START,
				rom_size: file.len() * ADDR_ALIGN,
				ram_start: RAM_START,
				ram_size: RAM_SIZE,
			}
		}
	}

	fn get_offset(addr: usize, start: usize) -> usize {
		(addr - start) / ADDR_ALIGN 
	}

	pub fn read(&self, addr: usize) -> Result<u32, BusError> {
		if addr % ADDR_ALIGN != 0 {
			return Err(BusError::LoadAddrMisaligned(addr));
		}

		if addr >= self.memory_layout.rom_start
		&& addr < self.memory_layout.rom_start + self.memory_layout.rom_size {
			Ok(self.rom.read(Self::get_offset(addr, self.memory_layout.rom_start)))
		} else if addr >= self.memory_layout.ram_start
		&& addr < self.memory_layout.ram_start + self.memory_layout.ram_size {
			Ok(self.ram.read(Self::get_offset(addr, self.memory_layout.ram_start)))
		} else {
			Ok(0_u32)
		}
	}

	pub fn write(&self, addr: usize, val: u32) -> Result<(), BusError> {
		if addr % ADDR_ALIGN != 0 {
			return Err(BusError::StoreAddrMisaligned(addr, val));
		}

		if addr >= self.memory_layout.rom_start
		&& addr < self.memory_layout.rom_start + self.memory_layout.rom_size {
			self.rom.write(Self::get_offset(addr, self.memory_layout.rom_start), val);
		} else if addr >= self.memory_layout.ram_start
		&& addr < self.memory_layout.ram_start + self.memory_layout.ram_size {
			self.ram.write(Self::get_offset(addr, self.memory_layout.ram_start), val);
		}

		Ok(())
	}
}

#[cfg(test)]
#[test]
fn test() {
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
	test_read(&bus, 0x4000_0008);
	test_read(&bus, 0x4000_000c);
	test_read(&bus, 0x4000_0010);

	// test_write(&bus, 0x4000_0001, 1);
	// test_read(&bus, 0x4000_0001);
}
