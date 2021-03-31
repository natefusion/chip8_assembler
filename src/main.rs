#![allow(non_camel_case_types, non_snake_case)]
use crate::{Register::*, Mnemonic::*};
use std::{fs::File, io::{BufRead, BufReader}, env};

#[derive(Copy,Clone)]
enum Mnemonic {
    CLEAR, END, JUMP, JUMP0, BEGIN, NEQ,
    EQ, SET, ADD, OR, AND, XOR, SUB, SHR,
    SUBR, SHL, RAND, DRAW, WRITEBCD, WRITE,
    READ, UNKNOWN
}

enum Register {
    V, I, D, S, K
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
        "clear"    => CLEAR,
        "end"      => END,
        "jump"     => JUMP,
        "jump0"    => JUMP0,
        "begin"    => BEGIN,
        "neq"      => NEQ,
        "eq"       => EQ,
        "set"      => SET,
        "add"      => ADD,
        "or"       => OR,
        "and"      => AND,
        "xor"      => XOR,
        "sub"      => SUB,
        "shr"      => SHR,
        "subr"     => SUBR,
        "shl"      => SHL,
        "rand"     => RAND,
        "draw"     => DRAW,
        "writebcd" => WRITEBCD,
        "write"    => WRITE,
        "read"     => READ,
        _ => {
            errors.push(format!("{}\tUnknown mnemonic found ('{}')", line,&line[..end_of_mnemonic]));
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
                    'V' | 'v' => order.push(V),
                    'I' | 'i' => order.push(I),
                    'D' | 'd' => order.push(D),
                    'S' | 's' => order.push(S),
                    'K' | 'k' => order.push(K),
                    _ => errors.push(format!("{}\tInvalid register ({})",line,val)),
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
            _ => errors.push(format!("{}\tUnknown symbol ({})",line, val)),
        }
    }

    Line { mnemonic, arguments, order }
}

fn eval(info: &Line, line: &str, errors: &mut Vec<String>) -> u16 {
    let mut register = info.order.iter();
    let (mut shell, extra) = match info.mnemonic {
        EQ => {
            match (register.next(), register.next()) {
                (Some(V), None) => (0x4000, 0x82),
                (Some(K), None) => (0xE0A1, 0x81),
                _ => (0xF, 0xF) }},
        
        NEQ => {
            match (register.next(), register.next()) {
                (Some(K), None)    => (0xE09E, 0x81),
                (Some(V), Some(V)) => (0x5000, 0x482),
                (Some(V), None)    => (0x3000, 0x82),
                _ => (0xF, 0xF) }},
        
        SET => {
            match (register.next(), register.next()) {
                (Some(V), None)    => (0x6000, 0x82),
                (Some(V), Some(V)) => (0x8000, 0x482),
                (Some(I), None)    => (0xA000, 0x1),
                (Some(V), Some(D)) => (0xF007, 0x81),
                (Some(D), Some(V)) => (0xF015, 0x81),
                (Some(V), Some(S)) => (0xF018, 0x81),
                (Some(I), Some(V)) => (0xF029, 0x81),
                (Some(V), Some(K)) => (0xF00A, 0x81),
                _ => (0xF, 0xF) }},
        
        ADD => {
            match (register.next(), register.next()) {
                (Some(V), None)    => (0x7000, 0x82),
                (Some(V), Some(V)) => (0x8004, 0x482),
                (Some(I), Some(V)) => (0xF01E, 0x81),
                _ => (0xF, 0xF) }},

        CLEAR    => (0x00E0, 0x0),
        END      => (0x00EE, 0x0),
        BEGIN    => (0x2000, 0x1),
        OR       => (0x8001, 0x482),
        AND      => (0x8002, 0x482),
        XOR      => (0x8003, 0x482),
        SUB      => (0x8005, 0x482),
        SHR      => (0x8006, 0x482),
        SUBR     => (0x8007, 0x482),
        SHL      => (0x800E, 0x482),
        RAND     => (0xC000, 0x82),
        DRAW     => (0xD000, 0x483),
        WRITEBCD => (0xF033, 0x81),
        WRITE    => (0xF055, 0x81),
        READ     => (0xF065, 0x81),
        JUMP     => (0x1000, 0x1),
        JUMP0    => (0xB000, 0x1),
        UNKNOWN  => return 0,

    };
    
    if info.arguments.len() != extra & 0xF {
        errors.push(format!("{}\t{} arguments were given. There should be {}", line, info.arguments.len(), extra & 0xF));
        return 0;
    }
        
    for (i, val) in info.arguments.iter().enumerate() {
        let shift = (extra >> (4 + (i * 4))) & 0xF;
        let max = if shift == 0 { 0xFFFF >> ((extra & 0xF) * 4) } else { 0xF };
        
        if *val > max {
            errors.push(format!("{}\tArgument {} (0x{:X}) is too big. The max is 0x{:X}", line, i+1, val, max));
            return 0;
        }
        
        shell |= val << shift;
    }
    
    shell
}

fn main() {
    let file = {
        match env::args().nth(1) {
            Some(filename) => {
                match File::open(filename) {
                    Ok(file) => file,
                    Err(_) => { eprintln!("Cannot read file"); std::process::exit(1); }
                }},
            None => { eprintln!("Please enter a file"); std::process::exit(1); }}
    };

    let reader = BufReader::new(file).lines();
    let mut errors = vec![];
      
    for line in reader.map(|l| l.unwrap()) {
        let err = errors.len();
        let info = scan(&line, &mut errors);
        if err == errors.len() {
            println!("{:X}", eval(&info, &line, &mut errors));
        }
    }

    if errors.len() != 0 {
        eprintln!("ERROR(S):");
        for i in errors.iter() {
            eprintln!("{}", i);

        }
        //std::process::exit(1);
    }
}
