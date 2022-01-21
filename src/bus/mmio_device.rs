pub trait MMIODevice {
    fn read(&self, location: usize) -> u32;
    fn write(&self, location: usize, val: u32);
}
