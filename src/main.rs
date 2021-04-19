mod lexer;
mod scanner;
mod token;
mod parser;
use std::{fs::File, io::Write,  env};
use crate::{lexer::Lexer, scanner::Scanner};

fn main() {
    let (filename,mut file) = {
        match env::args().nth(1) {
            Some(filename) => {
                match std::fs::read_to_string(&filename) {
                    Ok(file) => (filename, file),
                    Err(_) => { eprintln!("Cannot read file"); std::process::exit(1); }
                }},
            None => { eprintln!("Please enter a file"); std::process::exit(1); }}
    };
    file.make_ascii_lowercase();

    let mut lexer = Lexer::new(&file);
    lexer.lex();

    let mut iter = lexer.tokens.iter().peekable();
    let mut scanner = Scanner::new(&mut iter);
    scanner.scan();

    let mut opcodes = vec![];
    let mut error_occured = false;
    
    for (mnemonic, registers, arguments) in scanner.instructions {
        match parser::parse(&mnemonic, &registers, &arguments) {
            Ok(mut x) if !error_occured => opcodes.append(&mut x),
            Err(err) => {
                error_occured = true;
                eprintln!("Error: {}", err);
                eprintln!("{:X?}", arguments);
            },
            _ => {},
        }
    }

    if error_occured {
        std::process::exit(1);
    }

    File::create(filename + ".bin").unwrap()
        .write_all(&opcodes).unwrap();
}
