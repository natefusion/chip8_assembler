#[allow(non_camel_case_types, non_snake_case)]
mod token;

use std::{fs, env, process, collections::HashMap, slice::Iter};
use crate::token::{Token, Instruction, Category::*, Keyword::*};
use lazy_static::lazy_static;

fn tokenize(tokenstream: &'static str) -> Vec<Token> {
    tokenstream
        .lines()
        .enumerate()
        .flat_map(|(line, string)| {
            // Comments go until the end of the line and are ignored.
            let delim = match string.find(';') {
                Some(x) => x,
                _ => string.len(),
            };

            string[..delim]
                .split_ascii_whitespace()
                .enumerate()
                .map(|(ch, raw)| Token::new(raw, line+1, ch))
                .collect::<Vec<Token>>()
                
        }).collect()
}

fn parse(tokenlist: &[Token]) -> Vec<Instruction> {
    struct Env<'a> {
        instructions: Vec<Instruction>,
        labels: HashMap<&'a Token, usize>,
        definitions: HashMap<&'a Token, &'a Token>,
        iter: Iter<'a, Token>,
    }

    fn parse_token(token: &Token, env: &mut Env) {
        let mut push_number = |token: &Token| {
            let (x, radix) = if let Some("0x") = token.raw.get(0..2) {
                (2, 16)
            } else if let Some("v") = token.raw.get(0..1) {
                (1, 16)
            } else {
                (0, 10)
            };
            
            match usize::from_str_radix(&token.raw[x..], radix) {
                Ok(num) => { env.instructions.last_mut().unwrap().arguments.push(num); },
                Err(_) => { eprintln!("Identifiers cannot start with numbers"); },
            }
        };
        
        match token.category {
            Func(function) => { env.instructions.push(Instruction::new(token, function)); },

            Def(Colon) => {
                if let Some(x @ Token { category: Ident, .. }) = env.iter.next() {
                    env.labels.insert(&x, env.instructions.len() * 2 + 0x200);
                }
            },

            Def(Define) => {
                if let (Some(x), Some(y)) = (env.iter.next(), env.iter.next()) {
                    match (x.category, y.category) {
                        (Ident, Reg(_)) |
                        (Ident, Num) => { env.definitions.insert(&x, &y); },
                        _ => {},
                    }
                }
            },

            Ident => {
                // check for intersections pls
                if let Some(&x) = env.definitions.get(&token) {
                    parse_token(&x, env);
                } else if let Some(x) = env.labels.get(&token) {
                    env.instructions.last_mut().unwrap().arguments.push(*x);
                } else {
                    eprintln!("Error, unknown identifier, or identifier defined as a label and definition on line {}", token.line);
                }
            }

            Reg(x) => {
                if let V = x { push_number(token); }
                env.instructions.last_mut().unwrap().registers.push(x);
            },
            
            Num => { push_number(token); },

            _ => {},
        }
    }

    let mut env = Env {
        instructions: vec![],
        labels: HashMap::new(),
        definitions: HashMap::new(),
        iter: tokenlist.iter()
    };

    while let Some(token) = env.iter.next() {
        parse_token(&token, &mut env);
    }

    env.instructions
}

fn evaluate(instruction: &Instruction) -> Result<usize, String> {
    let mnemonic = instruction.function;
    let mut register = instruction.registers.iter();
    let arguments = instruction.arguments.iter().enumerate();
    let line = instruction.line;
    let ch = instruction.ch;
    /*                  v- number of arguments
     * opcode_info: 0x482
     *                ^^- first argument is shifted 8 bits to the left,
     *                ^-- second argument is shifted 4 bits to the left
     *
     * The arguments are shifted so that they can be bitwise-ored into opcode_shell with ease
     */
    let (arg0, arg1) = match (register.next(), register.next()) {
        (Some(x), Some(y)) => (*x,*y),
        (None,    Some(x)) => (Unk, *x),
        (Some(x), None) => (*x, Unk),
        (None, None) => (Unk, Unk),
    };
    
    let (mut opcode_shell, opcode_info) = match (mnemonic, arg0, arg1) {
        (Eq,     V,   V  ) => (0x9000, 0x482),
        (Eq,     V,   Unk) => (0x4000, 0x82),
        (Eq,     V,   Key) => (0xE0A1, 0x81),
        (Eq,     Key, V  ) => (0xE0A1, 0x81),

        (Neq,    V,   Key) => (0xE09E, 0x81),
        (Neq,    Key, V  ) => (0xE09E, 0x81),
        (Neq,    V,   V  ) => (0x5000, 0x482),
        (Neq,    V,   Unk) => (0x3000, 0x82),

        (Set,    V,   Unk) => (0x6000, 0x82),
        (Set,    V,   V  ) => (0x8000, 0x482),
        (Set,    I,   Unk) => (0xA000, 0x1),
        (Set,    V,   Dt ) => (0xF007, 0x81),
        (Set,    Dt,  V  ) => (0xF015, 0x81),
        (Set,    V,   St ) => (0xF018, 0x81),
        (Set,    I,   V  ) => (0xF029, 0x81),
        (Set,    V,   Key) => (0xF00A, 0x81),

        (Add,    V,   Unk) => (0x7000, 0x82),
        (Add,    V,   V  ) => (0x8004, 0x482),
        (Add,    I,   V  ) => (0xF01E, 0x81),

        (Or,     V,   V  ) => (0x8001, 0x482),
        (And,    V,   V  ) => (0x8002, 0x482),
        (Xor,    V,   V  ) => (0x8003, 0x482),
        (Sub,    V,   V  ) => (0x8005, 0x482),
        (Shr,    V,   V  ) => (0x8006, 0x482),
        (Subr,   V,   V  ) => (0x8007, 0x482),
        (Shl,    V,   V  ) => (0x800E, 0x482),

        (Rand,   V,   Unk) => (0xC000, 0x82),
        (Draw,   V,   V  ) => (0xD000, 0x483),

        (Bcd,    V,   Unk) => (0xF033, 0x81),
        (Write,  V,   Unk) => (0xF055, 0x81),
        (Read,   V,   Unk) => (0xF065, 0x81),

        (Clear,  Unk, Unk) => (0x00E0, 0x0),
        (Return, Unk, Unk) => (0x00EE, 0x0),
        (Call,   Unk, Unk) => (0x2000, 0x1),
        (Jump,   Unk, Unk) => (0x1000, 0x1),
        (Jump0,  Unk, Unk) => (0xB000, 0x1),

         _ => return Err(format!("Unknown instruction found on line {} at character {}", line, ch)),
    };
    
    if instruction.arguments.len() != opcode_info & 0xF {
        return Err(format!("opcode: {:X}, Expected {} arguments, found {} on line {} at character {}", opcode_shell, opcode_info & 0xF, arguments.len(), line, ch));
    }

    for (i, val) in arguments {
        let shift = (opcode_info >> (4 + (i * 4))) & 0xF;
        let max = if shift == 0 { 0xFFFF >> ((opcode_info & 0xF) * 4) } else { 0xF };

        if *val > max {
            return Err(format!("0x{:X} ({}) is bigger than the max of 0x{:X} ({}) on line {} at character {}", val, val, max, max, line, ch));
        }

        opcode_shell |= val << shift;
    }

    Ok(opcode_shell)
}

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

    let tokenlist = tokenize(&FILE);

    /*
    for token in tokenlist.iter() {
        println!("{}",token.raw);
    }
     */
    let instructions = parse(&tokenlist);
    for instruction in instructions.iter() {
        println!("{:X?}",evaluate(instruction));
        //evaluate(instruction);
    }
}
