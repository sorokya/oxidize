pub mod rom;
pub mod utils;

use std::env;
use std::fs::File;
use std::io::Read;

use rom::Rom;

fn main() {
    // read the buffer
    let rom_name = env::args().nth(1).unwrap();
    let rom = Rom::load(&mut File::open(&rom_name).unwrap());

    println!("{}", rom.header);
}
