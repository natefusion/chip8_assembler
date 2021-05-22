#[allow(non_camel_case_types)]
mod token;

use std::{fs, env, process};
use crate::token::{Token, Category::*, Keyword::*};
use lazy_static::lazy_static;



fn lexer(tokenstream: &'static str) -> Vec<Token> {
    tokenstream
        .lines()
        .enumerate()
        .flat_map(|(line, string)| {
            // Comments go until the end of the line and are ignored by the parser
            let delim = match string.find(';') {
                Some(x) => x,
                _ => string.len(),
            };

            string[..delim]
                .split_ascii_whitespace()
                .enumerate()
                .map(|(ch, raw)| Token::new(raw, line, ch))
                .collect::<Vec<Token>>()
                
        }).collect()
}

/*
fn parser(tokenlist: &[Token]) -> Vec<Instruction> {
}
*/

fn load(path: Option<String>) -> String {
    match path {
        Some(x) => match fs::read_to_string(x) {
            Ok(file) => file.trim().to_string(),
            Err(_) => { eprintln!("Cannot read file"); process::exit(1); }},
        None => { eprintln!("Please enter a file"); process::exit(1); }}
}

fn main() {
    // Implement variable handling as part of parser as a variable in the parser function
    // remember that the max size is 4096 bytes!!
    lazy_static! { static ref FILE: String = load(env::args().nth(1)); }

    let tokenlist = lexer(&FILE);

    for token in tokenlist.iter() {
        println!("{}",token.raw);
    }

    /*
    let mut instructions = parse(&tokens);

    for ins in instructions.iter() {
        println!("Instruction {{");
        println!("\tidentifier: {}", ins.identifier.lexeme);
        println!("\targuments: {}", ins.arguments.iter().fold(String::new(), |state, x| {
            format!("{} {} ",state, x.lexeme)
                
        }));
        println!("}}");
    }
     */
}
