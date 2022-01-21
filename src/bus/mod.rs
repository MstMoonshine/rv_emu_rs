use self::{bus_error::BusError, ram::RAMDevice, rom::ROMDevice};
use crate::{bus::mmio_device::MMIODevice, pipeline::memory_access::MemoryAccessWidth};

pub mod bus_error;
pub mod mmio_device;
pub mod ram;
pub mod rom;

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
    pub memory_layout: MemoryLayout,
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
            },
        }
    }

    fn read_chunk(&self, addr: usize) -> u32 {
        if addr >= self.memory_layout.rom_start
            && addr < self.memory_layout.rom_start + self.memory_layout.rom_size
        {
            self.rom
                .read((addr - self.memory_layout.rom_start) >> 2)
        } else if addr >= self.memory_layout.ram_start
            && addr < self.memory_layout.ram_start + self.memory_layout.ram_size
        {
            self.ram
                .read((addr - self.memory_layout.ram_start) >> 2)
        } else {
            0_u32
        }
    }

    fn write_chunk(&self, addr: usize, val: u32) {
        if addr >= self.memory_layout.rom_start
            && addr < self.memory_layout.rom_start + self.memory_layout.rom_size
        {
            self.rom
                .write((addr - self.memory_layout.rom_start) >> 2, val);
        } else if addr >= self.memory_layout.ram_start
            && addr < self.memory_layout.ram_start + self.memory_layout.ram_size
        {
            self.ram
                .write((addr - self.memory_layout.ram_start) >> 2, val);
        }
    }

    pub fn read(&self, addr: usize, width: MemoryAccessWidth) -> Result<u32, BusError> {
        let val = self.read_chunk(addr);
        let offset = addr & 0b11;

        let read_result = match width {
            MemoryAccessWidth::Byte => {
                match offset {
                    0b00 => val & 0x0000_00FF,
                    0b01 => (val & 0x0000_FF00) >> 8,
                    0b10 => (val & 0x00FF_0000) >> 16,
                    0b11 => (val & 0xFF00_0000) >> 24,
                    _ => { 0_u32 } // should never happen
                }
            },
            MemoryAccessWidth::HalfWord => {
                match offset {
                    0b00 => val & 0x0000_FFFF,
                    0b01 => (val & 0xFFFF_0000) >> 16,
                    _ => {
                        return Err(BusError::LoadAddrMisaligned(addr));
                    }
                }
            },
            MemoryAccessWidth::Word => {
                match offset {
                    0b00 => val,
                    _ => {
                        return Err(BusError::LoadAddrMisaligned(addr));
                    }
                }
            },
        };

        Ok(read_result)
    }

    pub fn write(&self, addr: usize, val: u32, width: MemoryAccessWidth) -> Result<(), BusError> {
        let chunk = self.read_chunk(addr);
        let offset = addr & 0b11;

        let write_val = match width {
            MemoryAccessWidth::Byte => {
                match offset {
                    0b00 => (chunk & 0xFFFF_FF00) | (val & 0xFF),
                    0b01 => (chunk & 0xFFFF_00FF) | ((val & 0xFF) << 8),
                    0b10 => (chunk & 0xFF00_FFFF) | ((val & 0xFF) << 16),
                    0b11 => (chunk & 0x00FF_FFFF) | ((val & 0xFF) << 24),
                    _ => { 0_u32 } // should never happen
                }
            },
            MemoryAccessWidth::HalfWord => {
                match offset {
                    0b00 => (chunk & 0xFFFF_0000) | (val & 0xFFFF),
                    0b01 => (chunk & 0x0000_FFFF) | ((val & 0xFFFF) << 16),
                    _ => {
                        return Err(BusError::StoreAddrMisaligned(addr, val));
                    }
                }
            },
            MemoryAccessWidth::Word => {
                match offset {
                    0b00 => val,
                    _ => {
                        return Err(BusError::StoreAddrMisaligned(addr, val));
                    }
                }
            },
        };

        self.write_chunk(addr, write_val);

        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test() {
    fn test_read(bus: &Bus, addr: usize, width: MemoryAccessWidth) {
        let val = bus.read(addr, width).expect("read error");
        println!("{:#010x}: {:#010x}", addr, val);
    }

    fn test_write(bus: &Bus, addr: usize, val: u32, width: MemoryAccessWidth) {
        let ret = bus.write(addr, val, width);
        match ret {
            Err(e) => println!("{}", e),
            _ => (),
        }
    }

    let rom_file = [
        0x1122_3344_u32,
        0xdead_beef,
        1,
        2,
        3,
        4,
        5,
        0xaabbccdd,
        0x44332211,
    ];

    let bus = Bus::new(&rom_file);

    let width = MemoryAccessWidth::Word;
    test_write(&bus, 0x8000_0000, 0xdead_beef, width);
    test_write(&bus, 0x8000_0004, 0x2020_2020, width);

    test_read(&bus, 0x8000_0000, width);
    test_read(&bus, 0x8000_0004, width);

    test_write(&bus, 0x4000_0000, 0x1010_1010, width);
    test_write(&bus, 0x4000_0004, 0x2020_2020, width);

    test_read(&bus, 0x4000_0000, width);
    test_read(&bus, 0x4000_0004, width);
    test_read(&bus, 0x4000_0008, width);
    test_read(&bus, 0x4000_000c, width);
    test_read(&bus, 0x4000_0010, width);

    test_read(&bus, 0x8000_0001, MemoryAccessWidth::Byte);
    test_write(&bus, 0x8000_0001, 1, MemoryAccessWidth::Byte);
    test_read(&bus, 0x8000_0001, MemoryAccessWidth::Byte);
}
