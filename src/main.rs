use std::env;
use std::fs;
use std::io::Read;

fn main() {
    let rom_name = env::args().nth(1).unwrap();
    
    let mut rom = fs::File::open(&rom_name).unwrap();
    let mut rom_buffer = Vec::new();
    rom.read_to_end(&mut rom_buffer).unwrap();
    
    let rom_buffer = rom_buffer;
}
