#[allow(non_camel_case_types, non_snake_case)]
mod token;

use std::{fs, env, process, collections::HashMap, slice::Iter};
use crate::token::{Token, Instruction, Category::*, Keyword::*};
use lazy_static::lazy_static;

fn lex(tokenstream: &'static str) -> Vec<Token> {
    // Tokens are separated by whitespace
    tokenstream
        .lines()
        .enumerate()
        .flat_map(|(line, string)| {
            // Comments go until the end of the line and are ignored.
            let delim = match string.find(';') {
                Some(x) => x,
                _ => string.len(),
            };

            let mut tokens = vec![];
            let mut beg = 0;
            let code = &string[..delim];

            while beg < code.len() {
                let end = if let Some(x) = code[beg..].find(|x:char| x.is_whitespace()) { x + beg } else { code.len() };
                tokens.push(Token::new(&code[beg..end], line+1, beg+1));
                beg = if let Some(x) = code[end..].find(|x:char| !x.is_whitespace()) { x + end } else { break };
            }

            tokens
        }).collect()
}

fn parse(tokenlist: &[Token]) -> Vec<Instruction> {
    // This is just an object embedded in a function
    struct Env<'a> {
        instructions: Vec<Instruction>,
        labels:       HashMap<&'a Token, usize>,
        definitions:  HashMap<&'a Token, &'a Token>,
        iter:         Iter<'a, Token>,
    }
    
    fn parse_token(token: &Token, env: &mut Env) {
        match token.category {
            Func(function) => { env.instructions.push(Instruction::new(function, token.line, token.ch)); },

            // Def(Colon) and Def(Define) do almost exactly the same thing. Should I try to combine them? (Doing so would require a peekable iterator)
            Def(Colon) => if let Some(x) = env.iter.next() {
                match x.category {
                    Ident => if let Some(_) = env.labels.insert(&x, env.instructions.len() * 2 + 0x200) {
                        eprintln!("Warning: label '{}' was redefined at ({}, {})", x.raw, x.line, x.ch);
                    },
                    _     => { eprintln!("Malformed label. '{}' is not a valid identifier. ({}, {})", x.raw, x.line, x.ch);           },
                }
            },

            Def(Define) => if let (Some(x), Some(y)) = (env.iter.next(), env.iter.next()) {
                match (&x.category, &y.category) {
                    (Ident, Reg(_)) |
                    (Ident, Num(_)) => if let Some(_) = env.definitions.insert(&x, &y) {
                        eprintln!("Warning: definition '{}' was redefined at ({}, {})", x.raw, x.line, x.ch);
                    },
                    _ => { eprintln!("Malformed definition. '{}' is not a valid identifier. ({}, {})", x.raw, x.line, x.ch); },
                }
            },

            Ident => {
                let label = env.labels.get(&token);
                let definition = env.definitions.get(&token);

                match (label, definition) {
                    (Some(_), Some(_)) => { eprintln!("Identifier '{}' was defined as a label and as a definition", token.raw);   },
                    (Some(x), None   ) => { env.instructions.last_mut().unwrap().arguments.push(*x);       },
                    (None,    Some(x)) => { parse_token(x, env);                                           },
                    (None,    None   ) => { eprintln!("Unknown identifier ({}) at ({}, {})", token.raw, token.line, token.ch); },
                }
            }

            Reg(x) => {
                if let V = x {
                    let num = usize::from_str_radix(&token.raw[1..], 16).unwrap();
                    env.instructions.last_mut().unwrap().arguments.push(num);
                }

                env.instructions.last_mut().unwrap().registers.push(x);
            },
            
            Num(num) => { env.instructions.last_mut().unwrap().arguments.push(num); },

            _ => {},
        }
    }

    let mut env = Env {
        instructions: vec![],
        labels:       HashMap::new(),
        definitions:  HashMap::new(),
        iter:         tokenlist.iter(),
    };

    while let Some(token) = env.iter.next() {
        parse_token(&token, &mut env);
    }

    env.instructions
}

fn evaluate(instruction: &Instruction) -> Result<usize, String> {
    let function = instruction.function;
    let mut register = instruction.registers.iter();
    let arguments = instruction.arguments.iter().enumerate();
    let line = instruction.line;
    let ch = instruction.ch;
    
    // Makes next match statement look pretty
    let (arg0, arg1) = match (register.next(), register.next()) {
        (Some(x), Some(y)) => (*x,  *y),
        (None,    Some(x)) => (Unk, *x),
        (Some(x), None   ) => (*x,  Unk),
        (None,    None   ) => (Unk, Unk),
    };

    /*                  v- number of arguments
     * opcode_info: 0x482
     *                 ^- first argument is shifted 8 bits to the right,
     *                ^-- second argument is shifted 4 bits to the right
     *
     * The arguments are shifted so that they can be bitwise-ored into opcode_shell with ease
     * This solution also keeps the match statement from getting ugly
     */
    let (mut opcode_shell, opcode_info) = match (function, arg0, arg1) {
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

         _ => return Err(format!("Malformed instruction found at ({}, {})", line, ch)),
    };

    let opcode_args = opcode_info & 0xF;
    let args_shift = opcode_info >> 4;
    
    if arguments.len() != opcode_args {
        return Err(format!("Expected {} arguments, found {} at ({}, {})", opcode_args, arguments.len(), line, ch));
    }

    for (i, val) in arguments {
        let shift = (args_shift >> (i << 2)) & 0xF;
        let max = if shift == 0 { 0xFFFF >> (opcode_args << 2) } else { 0xF };

        if *val > max {
            return Err(format!("0x{:X} ({}) is bigger than the max of 0x{:X} ({}) for opcode 0x{:X} at ({}, {})", val, val, max, max, opcode_shell, line, ch));
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
    // remember that the max size is 4096 bytes!!
    lazy_static! { static ref FILE: String = load(env::args().nth(1)); }

    let tokenlist = lex(&FILE);

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
