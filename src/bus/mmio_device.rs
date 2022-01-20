pub trait MMIODevice {
    fn read(&self, offset: usize) -> u32;
    fn write(&self, offset: usize, val: u32);
}
