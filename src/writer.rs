use std::ops::Deref;

trait Writer {
    fn write_byte(&mut self, input: u8);
    fn write_bytes(&mut self, input: &[u8]);
    fn move_index(&mut self, index: usize);
    fn get_size(&self) -> usize;
}

pub struct RamWriter {
    bytes: Vec<u8>,
    index_modifier: usize,
    index: usize
}

impl Writer for RamWriter {
    fn write_byte(&mut self, input: u8) {
        self.bytes[self.index] = input;
        self.index += self.index_modifier;
    }

    fn write_bytes(&mut self, input: &[u8]) {
        for i in input {
            self.write_byte(*i);
        }
    }

    fn move_index(&mut self, index: usize) {
        self.index = index;
    }

    fn get_size(&self) -> usize{
        return self.bytes.len();
    }
}

impl RamWriter {
    pub fn new(index: usize, index_modifier: usize) -> Self {
        return RamWriter{bytes: Vec::new(), index_modifier, index};
    }
}

pub struct DiskWriter {

}