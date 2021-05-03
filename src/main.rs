#[allow(non_camel_case_types)]
mod token;
mod lexer;
mod scanner;

use lazy_static::lazy_static;

fn main() {
    // remember that the max size is 4096 bytes!!
    // No more stupid little 'a annotations. yay!!
    lazy_static! {
        static ref FILE: String = {
            match std::env::args().nth(1) {
                Some(filename) => {
                    match std::fs::read_to_string(filename) {
                        Ok(file) => file.to_ascii_lowercase(),
                        Err(_) => { eprintln!("Cannot read file"); std::process::exit(1); },
                    }
                },
                None => { eprintln!("Please enter a file"); std::process::exit(1); }}};}

    let mut lexer = lexer::Lexer::new(&FILE);
    lexer.tokenize();

    for token in lexer.tokens.iter() {
        println!("type: '{:?}'\nlexeme: '{}'\nline: '{}'\n",
                 token.token, token.lexeme, token.line
        );
    }

    let mut iter = lexer.tokens.iter().peekable();
    let mut scanner = scanner::Scanner::new(&mut iter);
    scanner.scan();

    for (mnemonic, registers, arguments) in scanner.instructions.iter() {
        print!("{:?} ", mnemonic);
        registers.iter().for_each(|r| print!("{:?} ", r));
        arguments.iter().for_each(|a| print!("{} ", a));
        println!();
    }
}
