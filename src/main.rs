#[allow(non_camel_case_types)]
mod token;
//mod lexer;
//mod scanner;

use std::{fs::File, io::{/*Write,*/ BufReader, BufRead}, env, process, result::Result, collections::HashMap};
use crate::token::{Token, Token::*, TokenType, TokenType::*};

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn tokenize(lexeme: &str) -> Result<TokenType, String> {
    let c = |x| lexeme.chars().nth(x);

    match c(0).unwrap() {
        '%' => match token::registers(lexeme) {
            Some(x) => Ok(TokenType::Register(*x)),
            None    => Err(format!("Expected a Register, found: {}", lexeme)),
        },

        '0'..='9' => {
            let (index, radix) = if let Some('x') = c(1) { (2, 16) } else { (0, 10) };

            match usize::from_str_radix(&lexeme[index..], radix) {
                Ok(n) => Ok(TokenType::Number(n)),
                Err(_) => Err(format!("Expected a number, found: {}", lexeme)),
            }
        },

        _ => {
            Ok(if let Some(x) = token::MNEMONICS.get(&lexeme) {
                TokenType::Mnemonic(*x)
            } else if let Some(x) = token::MACROS.get(&lexeme) {
                TokenType::Macro(*x)
            } else {
                TokenType::Identifier({
                    let mut s = DefaultHasher::new();
                    lexeme.hash(&mut s);
                    s.finish()
                })
            })
        },
    }
}

fn scan(token: TokenType) {
    match token {
        Mnemonic(x) => { instructions.push((*x, vec![], vec![])); },
        
        Register(x) => {
            if let V(n) = x { instructions.last_mut().unwrap().2.push(*n); }
            instructions.last_mut().unwrap().1.push(*x);
        },
            
        Number(x) => { instructions.last_mut().unwrap().2.push(*x); },

        _ => {},
    }
}

fn main() {
    // remember that the max size is 4096 bytes!!
    // No more stupid little 'a annotations. yay!!
    let (_filename, file) = {
        match env::args().nth(1) {
            Some(filename) => {
                match File::open(&filename) {
                    Ok(file) => (filename, BufReader::new(file)),
                    Err(_) => { eprintln!("Cannot read file"); process::exit(1); }}},

            None => { eprintln!("Please enter a file"); process::exit(1); }}
    };

    let mut tokens: Vec<TokenType> = vec![];

    // Iterates through the file by line
    // Splits each line by whitespace into chunks
    // Tokenizes each chunk
    for (index, line) in file.lines().enumerate().map(|(i,l)| (i, l.unwrap())) {
        tokens.append(
            &mut line[..(if let Some(x) = line.find(';') { x } else { line.len() })] // ';' represents the start of a comment
            .split_ascii_whitespace()
            .map(|l| l.to_ascii_lowercase())
            .map(|lexeme| match tokenize(&lexeme) {
                Ok(token) => token,
                Err(msg)  => { eprintln!("{} on line {}", msg, index+1); process::exit(1); },
            }).collect()
        );
    }

    let mut iter = tokens.iter().peekable();
    let mut variables = HashMap::new();
    let mut instructions: Vec<(Token, Vec<Token>, Vec<usize>)> = vec![];
    
    while let Some(token) = iter.next() {
        match token {
            Identifier(x) => if let Some(t) = variables.get(x) { scan(token); },
            
            Macro(x) => {
                if let Some(Identifier(i)) = iter.next() {
                    match (x, iter.peek()) {
                        (Alias | Const, Some(Register(_)) | Some(Number(_))) => { variables.insert(i, *iter.next().unwrap()); },
                        (Colon, _) => { variables.insert(i, Number((instructions.len()) * 2 + 0x200)); },
                        _ => {},
                    }
                }
            },

            _ => scan(token),

        }
    }

    /*
    for token in tokens.iter() {
        println!("type: '{:?}'\nlexeme: '{}'\nline: '{}'\n",
                 token.token, token.lexeme, token.index
        );
    }
    */

    for (mnemonic, registers, arguments) in instructions.iter() {
        print!("{:?} ", mnemonic);
        registers.iter().for_each(|r| print!("{:?} ", r));
        arguments.iter().for_each(|a| print!("{} ", a));
        println!();
    }
}
