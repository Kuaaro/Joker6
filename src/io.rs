pub trait Reader {
    fn read(&self) -> u8;
    fn is_readable(&self) -> bool;
}

pub trait Writer {
    fn write(&self, input: u8);
}