use std::fmt;

#[derive(Debug)]
pub enum BusError {
    LoadAddrMisaligned(usize),
    StoreAddrMisaligned(usize, u32),
}

impl std::error::Error for BusError {}

impl fmt::Display for BusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BusError::LoadAddrMisaligned(addr) => {
                write!(
                    f,
                    "Read Misaligned at {:#010x}",
                    addr
                )
            }
            BusError::StoreAddrMisaligned(addr, val) => {
                write!(
                    f,
                    "Write Misaligned at {:#010x} with value {}",
                    addr, val
                )
            }
        }
    }
}
