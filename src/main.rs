#![allow(non_camel_case_types, non_snake_case)]
use crate::{Register::*, Mnemonic::*};
use std::{fs::File, io::{BufRead, BufReader}, env};

#[derive(Copy,Clone)]
enum Mnemonic {
    CLEAR, RETURN, JUMP, CALL, SKIP_E,
    SKIP_NE, LOAD, ADD, OR, AND,
    XOR, SUB, SHIFT_R, SUB_N, SHIFT_L,
    RAND, DRAW, SKIP_P,SKIP_NP,
    WAIT, STORE_BCD, STORE, READ,
    UNKNOWN
}

enum Register {
    V, I, D, S,
}

struct Line {
    mnemonic: Mnemonic,
    arguments: Vec<u16>,
    order: Vec<Register>,
}

// Maybe read in chunks, rather than character by character?
fn scan(line: &str, errors: &mut Vec<String>) -> Line {
    let line = line.trim();

    // Only one mnemonic per line.
    // Mnemonics must be at the beginning of the line (ignoring whitespace)
    let end_of_mnemonic = match line.find(' ') {
        Some(num) => num,
        None => line.len(),
    };

    let mnemonic = match &line[..end_of_mnemonic] {
        "clear"     => CLEAR,     "return"  => RETURN,
        "jump"      => JUMP,      "call"    => CALL,
        "skip_e"    => SKIP_E,    "skip_ne" => SKIP_NE,
        "load"      => LOAD,      "add"     => ADD,
        "or"        => OR,        "and"     => AND,
        "xor"       => XOR,       "sub"     => SUB,
        "shift_r"   => SHIFT_R,   "sub_n"   => SUB_N,
        "shift_l"   => SHIFT_L,   "rand"    => RAND,
        "draw"      => DRAW,      "skip_p"  => SKIP_P,
        "skip_np"   => SKIP_NP,   "wait"    => WAIT,
        "store_bcd" => STORE_BCD, "store"   => STORE,
        "read"      => READ,
        _ => {
            errors.push(format!("Unknown mnemonic found: '{}'", &line[..end_of_mnemonic]));
            return Line { mnemonic: UNKNOWN, arguments: vec![], order: vec![] };
        },
    };
        
    let mut arguments = vec![];
    let mut order = vec![];

    let mut iter = line[end_of_mnemonic..].char_indices();
    while let Some((mut i, val)) = iter.next() {
        i = i + end_of_mnemonic;
        
        match val {
            ';' => break, // ignore the rest of line if the comment marker (;) is found
            '%' => {
                match iter.next().unwrap().1 {
                    'V' => order.push(V),
                    'I' => order.push(I),
                    'D' => order.push(D),
                    'S' => order.push(S),
                    _ => errors.push(format!("Invalid register ({}) @ character {}",val,i+i)),
                }
            },
            // Is there a better way to do this?
            '0'..='9' | 'A'..='F' | 'a'..='f' => {
                let mut j = i+1;
                while j < line.len() && iter.next().unwrap().1.is_ascii_hexdigit() {
                    j += 1;
                }

                arguments.push(u16::from_str_radix(&line[i..j], 16).unwrap());
            },
            ' ' | '\n' | ',' => {}, // spaces are ignored (but what about tabs?????)
            _ => errors.push(format!("Unknown symbol ({}) @ character {}",val,i+1)),
        }
    }

    Line {
        mnemonic: mnemonic,
        arguments: arguments,
        order: order,
    }
}

fn eval(info: &Line, errors: &mut Vec<String>) -> u16 {
    let mut register = info.order.iter();
    let (mut shell, extra) = match info.mnemonic {
        JUMP => {
            match register.next() {
                None    => (0x1000, 0x1),
                Some(V) => (0xB000, 0x1),
                _ => (0xF, 0xF) }},
        
        SKIP_E => {
            match (register.next(), register.next()) {
                (Some(V), None)    => (0x3000, 0x82),
                (Some(V), Some(V)) => (0x5000, 0x482),
                _ => (0xF, 0xF) }},
        
        LOAD => {
            match (register.next(), register.next()) {
                (Some(V), None)    => (0x6000, 0x82),
                (Some(V), Some(V)) => (0x8000, 0x482),
                (Some(I), None)    => (0xA000, 0x1),
                (Some(V), Some(D)) => (0xF007, 0x81),
                (Some(D), Some(V)) => (0xF015, 0x81),
                (Some(V), Some(S)) => (0xF018, 0x81),
                (Some(I), Some(V)) => (0xF029, 0x81),
                _ => (0xF, 0xF) }},
        
        ADD => {
            match (register.next(), register.next()) {
                (Some(V), None)    => (0x7000, 0x82),
                (Some(V), Some(V)) => (0x8004, 0x482),
                (Some(I), Some(V)) => (0xF01E, 0x81),
                _ => (0xF, 0xF) }},

        CLEAR     => (0x00E0, 0x0),   RETURN  => (0x00EE, 0x0),
        CALL      => (0x2000, 0x1),   SKIP_NE => (0x4000, 0x82),
        OR        => (0x8001, 0x482), AND     => (0x8002, 0x482),
        XOR       => (0x8003, 0x482), SUB     => (0x8005, 0x482),
        SHIFT_R   => (0x8006, 0x482), SUB_N   => (0x8007, 0x482),
        SHIFT_L   => (0x800E, 0x482), RAND    => (0xC000, 0x82),
        DRAW      => (0xD000, 0x483), SKIP_P  => (0xE09E, 0x81),
        SKIP_NP   => (0xE0A1, 0x81),  WAIT    => (0xF00A, 0x81),
        STORE_BCD => (0xF033, 0x81),  STORE   => (0xF055, 0x81),
        READ      => (0xF065, 0x81),
        UNKNOWN   => return 0,
    };
    if let (0xF, 0xF) = (shell, extra) {
        errors.push(format!("unknown arguments"));
        0
    } else {
        if info.arguments.len() != extra & 0xF {
            errors.push(format!("{} arguments were supplied when {} arguments were requested",extra & 0xF, info.arguments.len()));
            return 0;
        }
        
        for (i, val) in info.arguments.iter().enumerate() {
            let shift = (extra >> (4 + (i * 4))) & 0xF;
            let max = if shift == 0 { 0xFFFF >> ((extra & 0xF) * 4) } else { 0xF };
            
            if *val > max {
                errors.push(format!("0x{:X} was greater than the max value of 0x{:X} for the supplied argument", val, max));
                return 0;
            }
            
            shell |= val << shift;
        }
        
        shell
    }
}

fn print_order(order: &Vec<Register>) {
    for i in order.iter() {
        print!("{}",(match i {
            I => "I",
            D => "D",
            V => "V",
            S => "S",
        }));
        print!(" ");
    }
}

fn main() {
    let file = {
        match env::args().nth(1) {
            Some(file) => {
                match File::open(file) {
                    Ok(file) => file,
                    Err(_) => { eprintln!("Cannot read file"); std::process::exit(1); },
                }
            },
            None => { eprintln!("Please enter a file"); std::process::exit(1); },
        }
    };

    let reader = BufReader::new(file).lines();
    let mut errors = vec![];
    
    for line in reader.map(|l| l.unwrap()) {
        let info = scan(&line, &mut errors);
        let opcode = eval(&info, &mut errors);
        println!("{:X}", opcode);
    }

    if errors.len() != 0 {
        eprintln!("ERROR(S):");
        for i in errors.iter() {
            eprintln!("{}",i);
        }
        std::process::exit(1);
    }
}
