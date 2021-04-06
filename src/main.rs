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
                    Ok(file) => (filename, file),
                    Err(_) => { eprintln!("Cannot read file"); std::process::exit(1); }
                }},
            None => { eprintln!("Please enter a file"); std::process::exit(1); }}
    };

    let mut reader = BufReader::new(file);
    let mut scanner = Scanner::new();
    let mut opcodes = vec![];

    scanner.scan_file(&mut reader);

    if scanner.errors.len() == 0 {
        let mut parse_error = false;
        for instruction in scanner.instructions {
            let (mnemonic, registers, arguments) = instruction;
            let opcode = parse(&mnemonic, &registers, &arguments);
            match opcode {
                Ok(mut x) => opcodes.append(&mut x),
                Err(error) => { parse_error = true; eprintln!("{}",error); },
            }
        }
        
        if !parse_error {
            File::create(filename + ".bin").unwrap()
                .write_all(&opcodes).unwrap();
        }
    } else {
        for error in scanner.errors {
            let (msg, line, line_index, index) = error;
            
            eprintln!("{} found on line {} at character {}\n{}\n",
                      msg, line_index, index, line
            );
        }
        std::process::exit(1);
    }
}
