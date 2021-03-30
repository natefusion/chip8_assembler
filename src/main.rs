#![allow(non_camel_case_types, non_snake_case)]
use crate::{Register::*, Mnemonic::*};
use std::{fs::File, io::{BufRead, BufReader, Write}, env};

#[derive(Copy,Clone)]
enum Token {
    CLEAR, END, NEQ, EQ, SET,
    ADD, OR, AND, XOR, SUB,
    SUBR, SHL, SHR, JUMP0, RAND,
    DRAW, WRITEBCD, READ, WRITE,
    
    V(usize), I, DT, ST,
    KEY,
    LITERAL(usize),
}

fn tokenize(chunk: &str, a: usize, b: usize) -> Token {
    /* Design
     * Separate the string by a delim (a space is what I'm thinking)
     * Parse each chunk separately and generate and error or a token
     */

    match chunk[a..b] {
        "[v" => {
            if let Some(num) = tokenize(chunk, a+1, a+2) {
                if let 
                V(num)
            }
        },
        "[i]" => I,
        "[dt]" => DT,
        "[st]" => ST,
        "[key]" =>, KEY,
        
        "clear" => CLEAR,
        "end"  => END,
        "neq" => NEQ,
        "eq"  => EQ,
        "set" => SET,
        "add" => ADD,
        "or" => OR,
        "and" => AND,
        "xor" => XOR,
        "sub" => SUB,
        "subr" => SUBR,
        "shl" => SHL,
        "shr" => SHR,
        "jump" => JUMP,
        "jump0" => JUMP0,
        "rand" => RAND,
        "draw" => DRAW,
        "writebcd" => WRITEBCD,
        "read" => READ,
        "write" => WRITE,
    }
}

fn main() {
    // remember that the max size is 4096 bytes!!
    let input = {
        match env::args().nth(1) {
            Some(filename) => {
                match File::open(filename) {
                    Ok(input) => input,
                    Err(_) => { eprintln!("Cannot read file"); std::process::exit(1); },
                }
            },
            None => { eprintln!("Please enter a file"); std::process::exit(1); },
        }
    };

    let reader = new BufReader::new(file).lines();

    for line in reader.map(|l| l.unwrap()) {
        let blocks = line.split_whitespace();
        let tokens = tokenize(blocks);
    }

    //let mut output = File::create("a.bin").unwrap();
    //output.write_all(&opcodes).unwrap();
}
