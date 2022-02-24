#[allow(non_camel_case_types, non_snake_case)]
mod token;

use std::{fs, io::Write, fs::File, env, process, collections::HashMap, slice::Iter};
use crate::token::{Token, Instruction, Category::*, Keyword::*};
use lazy_static::lazy_static;

fn lex(tokenstream: &'static str) -> Vec<Token> {
    // Tokens are separated by whitespace
    tokenstream
        .lines()
        .enumerate()
        .flat_map(|(line, string)| {
            let string = string.trim();
            // Comments go until the end of the line and are ignored.
            let delim = match string.find('#') {
                Some(x) => x,
                _ => string.len(),
            };

            let mut tokens = vec![];
            let mut beg = 0;
            let code = &string[..delim];

            // gay
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

            Def(Defvar) => if let (Some(x), Some(y)) = (env.iter.next(), env.iter.next()) {
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
                    (Some(_), Some(_)) => { eprintln!("Identifier '{}' was defined as a label and as a definition", token.raw); },
                    (Some(x), None   ) => { env.instructions.last_mut().unwrap().arguments.push(Num(*x)); },
                    (None,    Some(x)) => { parse_token(x, env); },
                    (None,    None)    => { eprintln!("Unknown identifier '{}' ({}, {})", token.raw, token.line, token.ch); },
                }
            }

            Reg(_) | Num(_) => { env.instructions.last_mut().unwrap().arguments.push(token.category); },

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

fn eval_ins(instruction: &Instruction) -> Result<usize, String> {
    let func = &instruction.function;
    let args = &instruction.arguments;
    let line = instruction.line;
    let ch = instruction.ch;

    let nums =
        args
        .iter()
        .filter(|x| matches!(x, Num(_) | Reg(V(_))))
        .map(|x| match x {
            Num(val) | Reg(V(val)) => *val,
            _ => 0, // this will never occur
        }).collect::<Vec<usize>>();

    /*                  v- number of arguments
     * opcode_info: 0x482
     *                 ^- first argument is shifted 8 bits to the right,
     *                ^-- second argument is shifted 4 bits to the right
     *
     * The arguments are shifted so that they can be bitwise-ored into opcode_shell with ease
     * This solution also keeps the match statement from getting ugly
     */

    // ugleh
    let (mut opcode_shell, opcode_info) = match (func, &args[..]) {
        (Eq, [Reg(V(_)), Reg(V(_))]) => (0x9000, 0x482),
        (Eq, [Reg(V(_)), Num(_)]) |
        (Eq, [Num(_), Reg(V(_))]) => (0x4000, 0x82),
        (Eq, [Reg(V(_)), Reg(Key)]) |
        (Eq, [Reg(Key), Reg(V(_))]) => (0xE0A1, 0x81),

        (Neq, [Reg(V(_)), Reg(Key)]) |
        (Neq, [Reg(Key), Reg(V(_))]) => (0xE09E, 0x81),
        (Neq, [Reg(V(_)), Reg(V(_))]) => (0x5000, 0x482),
        (Neq, [Reg(V(_)), Num(_)]) => (0x3000, 0x82),

        (Set, [Reg(V(_)), Num(_)]) => (0x6000, 0x82),
        (Set, [Reg(V(_)), Reg(V(_))]) => (0x8000, 0x482),
        (Set, [Reg(I), Num(_)]) => (0xA000, 0x1),
        (Set, [Reg(V(_)), Reg(Dt )]) => (0xF007, 0x81),
        (Set, [Reg(Dt), Reg(V(_))]) => (0xF015, 0x81),
        (Set, [Reg(V(_)), Reg(St)]) => (0xF018, 0x81),
        (Set, [Reg(I), Reg(V(_))]) => (0xF029, 0x81),
        (Set, [Reg(V(_)), Reg(Key)]) => (0xF00A, 0x81),

        (Add, [Reg(V(_)), Num(_)]) => (0x7000, 0x82),
        (Add, [Reg(V(_)), Reg(V(_))]) => (0x8004, 0x482),
        (Add, [Reg(I), Reg(V(_))]) => (0xF01E, 0x81),

        (Or, [Reg(V(_)), Reg(V(_))]) => (0x8001, 0x482),
        (And, [Reg(V(_)), Reg(V(_))]) => (0x8002, 0x482),
        (Xor, [Reg(V(_)), Reg(V(_))]) => (0x8003, 0x482),
        (Sub, [Reg(V(_)), Reg(V(_))]) => (0x8005, 0x482),
        (Shr, [Reg(V(_)), Reg(V(_))]) => (0x8006, 0x482),
        (Subr, [Reg(V(_)), Reg(V(_))]) => (0x8007, 0x482),
        (Shl, [Reg(V(_)), Reg(V(_))]) => (0x800E, 0x482),

        (Rand, [Reg(V(_)), Num(_)]) => (0xC000, 0x82),
        (Draw, [Reg(V(_)), Reg(V(_)), Num(_)]) => (0xD000, 0x483),

        (Bcd,    [Reg(V(_))]) => (0xF033, 0x81),
        (Write,  [Reg(V(_))]) => (0xF055, 0x81),
        (Read,   [Reg(V(_))]) => (0xF065, 0x81),

        (Clear, []) => (0x00E0, 0x0),
        (Return, []) => (0x00EE, 0x0),
        (Call, [Reg(V(_))]) => (0x2000, 0x1),
        (Jump, [Num(_)]) => (0x1000, 0x1),
        (Jump0, [Num(_)]) => (0xB000, 0x1),

        //speshul
        (Include, _) => { return Ok(nums.iter().fold(0, |acc, x| (acc << 4) | x)); },
        
        _ => return Err(format!("Malformed instruction found at ({}, {})", line, ch)),
    };

    let opcode_args = opcode_info & 0xF;
    let args_shift = opcode_info >> 4;

    if nums.len() != opcode_args {
        return Err(format!("Expected {} arguments, found {} at ({}, {})", opcode_args, args.len(), line, ch));
    }

    for (i, val) in nums.iter().enumerate() {
        let shift = (args_shift >> (i << 2)) & 0xF;
        let max = if shift == 0 { 0xFFFF >> (opcode_args << 2) } else { 0xF };
        
        if *val > max {
            return Err(format!("0x{:X} ({}) is bigger than the max of 0x{:X} ({}) for opcode 0x{:X} at ({}, {})", val, val, max, max, opcode_shell, line, ch));
        }
        
        opcode_shell |= val << shift;
    }

    Ok(opcode_shell)
}

fn eval(instructions: &[Instruction]) -> Result<Vec<u8>, Vec<String>> {
    let mut opcodes = vec![];
    let mut errs = vec![];
    
    for instruction in instructions.iter() {
        match eval_ins(instruction) {
            Ok(op) => {
                let op1 = ((op & 0xFF00) >> 8) as u8;
                let op2 = (op & 0xFF) as u8;
                
                opcodes.push(op1);
                opcodes.push(op2);
            },
            Err(e) => { errs.push(e); },
        }
    }

    if errs.len() == 0 {
        Ok(opcodes)
    } else {
        Err(errs)
    }
}

fn load(path: Option<String>) -> (String, String) {
    match path {
        Some(x) => match fs::read_to_string(&x) {
            Ok(file) => (file.trim().to_string(), x),
            Err(_) => { eprintln!("Cannot read file"); process::exit(1); }},
        None => { eprintln!("Please enter a file"); process::exit(1); }}
}

fn main() {
    // remember that the max size is 4096 bytes!!
    lazy_static! { static ref FILE: (String, String) = load(env::args().nth(1)); }
    let opcodes = eval(&parse(&lex(&FILE.0)));

    match opcodes {
        Ok(ops) => {
            let mut buffer = File::create(FILE.1.clone() + &String::from(".bin")).unwrap();

            buffer.write_all(&ops).unwrap();
        },
        
        Err(e) => {
            for err in e.iter() {
                println!("{err}");
            }
        },
    }
}
