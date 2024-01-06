mod swapper;
mod cart;
mod io;
mod joker_execution_unit;
mod joker_execution_function;

use io::Reader;

use crate::io::{ForwardFileReader, ForwardFileWriter, Writer};
use std::{fs::File, io::Write};

fn main() {
    let mut ffr = ForwardFileReader::new(&String::from("in.txt"), 3);
    let mut ffw = ForwardFileWriter::new(&String::from("out.txt"), 2);

    while ffr.is_readable() {
        let a = ffr.read();

        println!("{}", a as char);
        ffw.write(a);
    }
}
