mod scanner;
mod parser;
mod tokens;
use std::{fs::File, io::BufReader, env};
use crate::{scanner::Scanner, parser::parse};

fn main() {
    let file = {
        match env::args().nth(1) {
            Some(filename) => {
                match File::open(filename) {
                    Ok(file) => file,
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
        println!("{:X}", opcode);

        opcodes.push(opcode);
    }
}
