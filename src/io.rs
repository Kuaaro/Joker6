use std::{fs::File, io::{Read, Write}};

pub trait Reader {
    fn read(&mut self) -> u8;
    fn is_readable(&self) -> bool;
}

pub trait Writer {
    fn write(&mut self, input: u8);
}

pub struct ForwardFileReader {
    file: File,
    buffer: Vec<u8>,
    index: usize,
    file_size: u64,
    buffer_capacity: usize
}

impl ForwardFileReader {
    pub fn new(path: &String, buffer_capacity: usize) -> Self {
        let file: File = File::open(path).unwrap();
        let mut buffer: Vec<u8> = Vec::new();
        buffer.resize(buffer_capacity, 0);

        return ForwardFileReader{buffer, index: 0, file_size: file.metadata().unwrap().len(), buffer_capacity, file}; // elements added in different order because if file is passed first then file_size tries to access moved value
    }
}

impl Reader for ForwardFileReader {
    fn read(&mut self) -> u8 {
        if self.index % self.buffer_capacity == 0 {
            self.file.read(&mut self.buffer).unwrap();
        }

        let out = self.buffer[self.index % self.buffer_capacity];

        self.index += 1;
        
        return out;
    }

    fn is_readable(&self) -> bool {
        return self.index as u64 != self.file_size;
    }
}

pub struct ForwardFileWriter {
    file: File,
    buffer: Vec<u8>,
    index: usize,
    buffer_capacity: usize
}

impl ForwardFileWriter {
    pub fn new(path: &String, buffer_capacity: usize) -> Self{
        let file: File = File::create(path).unwrap();
        let mut buffer: Vec<u8> = Vec::new();
        buffer.resize(buffer_capacity, 0);

        return ForwardFileWriter{file, buffer, index: 0, buffer_capacity};
    }
}

impl Writer for ForwardFileWriter {
    fn write(&mut self, input: u8) {
        self.buffer[self.index] = input;
        self.index += 1;

        if self.index == self.buffer_capacity {
            self.file.write(self.buffer.as_slice()).unwrap();
            self.index = 0;
        }
    }
}

impl Drop for ForwardFileWriter {
    fn drop(&mut self) {
        self.file.write(&self.buffer.as_slice()[0..self.index]).unwrap();
    }
}

pub struct BackwardFileReader {
    file: File,
    buffer: Vec<u8>,
    index: usize,
    file_size: u64,
    buffer_capacity: usize
}

impl BackwardFileReader { //todo
    pub fn new(path: &String, buffer_capacity: usize) -> Self {
        let file: File = File::open(path).unwrap();
        let mut buffer: Vec<u8> = Vec::new();
        buffer.resize(buffer_capacity, 0);

    }
}

pub struct BackwardFileWriter {

}