#![allow(non_camel_case_types, non_snake_case)]

#[derive(Copy,Clone)]
enum Mnemonic {
    CLEAR, RETURN, JUMP, CALL, SKIP_E,
    SKIP_NE, LOAD, ADD, OR, AND,
    XOR, SUB, SHIFT_R, SUB_N, SHIFT_L,
    RAND, DRAW, SKIP_P,SKIP_NP,
    WAIT, STORE_BCD, STORE, READ,
}

enum Register {
    V, I, D, S,
    NUM,
}

struct Line {
    mnemonic: Mnemonic,
    arguments: Vec<u16>,
    order: Vec<Register>,
}

fn scan(line: &str, errors: &mut Vec<String>) -> Line {
    let line = line.trim();

    // Only one mnemonic per line.
    // Mnemonics must be at the beginning of the line (ignoring whitespace)
    let end_of_mnemonic = match line.find(' ') {
        Some(num) => num,
        None => panic!("Unkown mnemonic found '{}'", line),
    };

    let mnemonic = match &line[..end_of_mnemonic] {
        "clear"     => Mnemonic::CLEAR,     "return"  => Mnemonic::RETURN,
        "jump"      => Mnemonic::JUMP,      "call"    => Mnemonic::CALL,
        "skip_e"    => Mnemonic::SKIP_E,    "skip_ne" => Mnemonic::SKIP_NE,
        "load"      => Mnemonic::LOAD,      "add"     => Mnemonic::ADD,
        "or"        => Mnemonic::OR,        "and"     => Mnemonic::AND,
        "xor"       => Mnemonic::XOR,       "sub"     => Mnemonic::SUB,
        "shift_r"   => Mnemonic::SHIFT_R,   "sub_n"   => Mnemonic::SUB_N,
        "shift_l"   => Mnemonic::SHIFT_L,   "rand"    => Mnemonic::RAND,
        "draw"      => Mnemonic::DRAW,      "skip_p"  => Mnemonic::SKIP_P,
        "skip_np"   => Mnemonic::SKIP_NP,   "wait"    => Mnemonic::WAIT,
        "store_bcd" => Mnemonic::STORE_BCD, "store"   => Mnemonic::STORE,
        "read"      => Mnemonic::READ,
        _ => panic!(format!("Unknown mnemonic found: '{}'", &line[..end_of_mnemonic])),
    };
        
    let mut arguments = vec![];
    let mut order = vec![];

    let mut iter = line[end_of_mnemonic..].char_indices();
    let mut is_v = false;
    while let Some((i, val)) = iter.next() {
        let i = i + end_of_mnemonic;

        match val {
            ';' => break, // ignore the rest of line if the comment marker (;) is found
            '%' => {
                match iter.next().unwrap().1 {
                    'V' => {
                        is_v = true;
                        order.push(Register::V);
                    },
                    'I' => order.push(Register::I),
                    'D' => order.push(Register::D),
                    'S' => order.push(Register::S),
                    // v-- extra +1 because character pointer is moved, but 'i' is not incremented
                    _ => errors.push(format!("Invalid register ({}) @ character {}",val,i+2)),
                }
            },
            // Is there a better way to do this?
            '0'..='9' | 'A'..='F' | 'a'..='f' => {
                let mut j = i+1;
                while j < line.len() && iter.next().unwrap().1.is_ascii_hexdigit() {
                    j += 1;
                }

                arguments.push(u16::from_str_radix(&line[i..j], 16).unwrap());
                if !is_v { order.push(Register::NUM); }
                is_v = false;
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

fn evaluate(info: &Line) -> u16 {
    let (mut opcode, argsnumber) = match info.mnemonic {
        Mnemonic::CLEAR  => (0x00E0, 0),
        Mnemonic::RETURN => (0x00EE, 0),
        Mnemonic::JUMP => {
            match info.order[0] {
                Register::NUM => (0x1000, 1),
                Register::V   => (0xB000, 1)}},
        
        Mnemonic::CALL => (0x2000, 1),
        Mnemonic::SKIP_E => {
            match (&info.order[0], &info.order[1]) {
                (Register::V, Register::NUM) => (0x3000, 2),
                (Register::V, Register::V)   => (0x5000, 2),
                _ => panic!("unkown arguments")}},
        
        Mnemonic::SKIP_NE => (0x4000, 2),
        Mnemonic::LOAD => {
            match (&info.order[0], &info.order[1]) {
                (Register::V, Register::NUM) => (0x6000, 2),
                (Register::V, Register::V)   => (0x8000, 2),
                (Register::I, Register::NUM) => (0xA000, 1),
                (Register::V, Register::D)   => (0xF007, 1),
                (Register::D, Register::V)   => (0xF015, 1),
                (Register::V, Register::S)   => (0xF018, 1),
                (Register::I, Register::V)   => (0xF029, 1),
                _ => panic!("unkown arguments")}},
        
        Mnemonic::ADD => {
            match (&info.order[0], &info.order[1]) {
                (Register::V, Register::NUM) => (0x7000, 2),
                (Register::V, Register::V)   => (0x8004, 2),
                (Register::I, Register::V)   => (0xF01E, 1),
                _ => panic!("unkown arguments")}},
        
        Mnemonic::OR        => (0x8001, 2),  Mnemonic::AND    => (0x8002, 2),
        Mnemonic::XOR       => (0x8003, 2),  Mnemonic::SUB    => (0x8005, 2),
        Mnemonic::SHIFT_R   => (0x8006, 2),  Mnemonic::SUB_N  => (0x8007, 2),
        Mnemonic::SHIFT_L   => (0x800E, 2),  Mnemonic::RAND   => (0xC000, 2),
        Mnemonic::DRAW      => (0xD000, 3),  Mnemonic::SKIP_P => (0xE09E, 1),
        Mnemonic::SKIP_NP   => (0xE0A1, 1),  Mnemonic::WAIT   => (0xF00A, 1),
        Mnemonic::STORE_BCD => (0xF033, 1),  Mnemonic::STORE  => (0xF055, 1),
        Mnemonic::READ      => (0xF065, 1),
    };

    if info.arguments.len() != argsnumber {
        panic!("Incorrect amount of arguments");
    }

    // this sure better get rewritten
    match argsnumber {
        1 => {
            if opcode & 0xFF == 0 {
                if info.arguments[0] <= 0xFFF {
                    opcode |= info.arguments[0];
                } else {
                    panic!("Your number is too big!");
                }
            } else {
                if info.arguments[0] <= 0xF {
                    opcode |= info.arguments[0] << 8;
                } else {
                    panic!("Your number is too big!");
                }
        },
        2 => {
            if info.arguments[0] <= 0xF {
                opcode |= info.arguments[0] << 8;
            } else {
                panic!("Your number is too big!");
            }

            if info.arguments[1] <= 0xFF {
                if opcode & 0xFF == 0 {
                    opcode |= info.arguments[1];
                } else {
                    opcode |= info.arguments[1] << 4;
                }
            } else {
                panic!("Your number is too big!");
            }
        },
        3 => {
            for (i, val) in info.arguments.iter().enumerate() {
                if *val > 0xF {
                    panic!("Your number is too big!");
                }
                opcode |= val << (8 - (i * 4));
            }
        }
        _ => panic!("This will never happen (I think)"),
    }

    opcode
}

fn print_order(order: &Vec<Register>) {
    for i in order.iter() {
        match i {
            Register::I => print!("I"),
            Register::D => print!("D"),
            Register::V => print!("V"),
            Register::S => print!("S"),
            Register::NUM => print!("NUM"),
        }
        print!(" ");
    }
}

fn main() {
    let mut errors = vec![];
    let code = include_str!("test.ch8");
    let info = scan(code, &mut errors);
    println!("{:X}",evaluate(&info));

    println!("{:?}",info.arguments);
    print_order(&info.order);

    if errors.len() != 0 {
        println!("ERROR(S):");
        for i in errors.iter() {
            println!("{}",i);
        }
        std::process::exit(1);
    }
}
