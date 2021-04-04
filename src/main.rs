mod scanner;
mod parser;
mod tokens;
use std::{fs::File, io::{Write, BufReader}, env};
use crate::{scanner::Scanner, parser::parse};

fn main() {
    let (filename, file) = {
        match env::args().nth(1) {
            Some(filename) => {
                match File::open(&filename) {
                    Ok(file) => (filename.to_string(), file),
                    Err(_) => { eprintln!("Cannot read file"); std::process::exit(1); }
                }},
            None => { eprintln!("Please enter a file"); std::process::exit(1); }}
    };

    let mut reader = BufReader::new(file);
    let mut scanner = Scanner::new();
    let mut opcodes = vec![];

    scanner.scan_file(&mut reader);

    for instruction in scanner.instructions.iter() {
        let (mnemonic, registers, arguments) = instruction;
        let opcode = parse(&mnemonic, &registers, &arguments);
        //println!("{:X}", opcode);
        let h2 = (opcode & 0x00FF) as u8;
        let h1 = ((opcode & 0xFF00) >> 8) as u8;
        opcodes.append(&mut vec![h1,h2]);
    }

    let mut compiled_file = File::create(filename + ".bin").unwrap();

    compiled_file.write(&opcodes).unwrap();
}
