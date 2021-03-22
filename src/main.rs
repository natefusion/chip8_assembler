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
        None => line.len(),
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
    let (mut shell, extra) = match info.mnemonic {
        Mnemonic::CLEAR  => (0x00E0, 0x0),
        Mnemonic::RETURN => (0x00EE, 0x0),
        Mnemonic::JUMP => {
            match info.order[0] {
                Register::NUM => (0x1000, 0x1),
                Register::V   => (0xB000, 0x1),
                _ => panic!("unknown arguments")}},
        
        Mnemonic::CALL => (0x2000, 0x1),
        Mnemonic::SKIP_E => {
            match (&info.order[0], &info.order[1]) {
                (Register::V, Register::NUM) => (0x3000, 0x82),
                (Register::V, Register::V)   => (0x5000, 0x482),
                _ => panic!("unkown arguments")}},
        
        Mnemonic::SKIP_NE => (0x4000, 0x82),
        Mnemonic::LOAD => {
            match (&info.order[0], &info.order[1]) {
                (Register::V, Register::NUM) => (0x6000, 0x82),
                (Register::V, Register::V)   => (0x8000, 0x482),
                (Register::I, Register::NUM) => (0xA000, 0x1),
                (Register::V, Register::D)   => (0xF007, 0x81),
                (Register::D, Register::V)   => (0xF015, 0x81),
                (Register::V, Register::S)   => (0xF018, 0x81),
                (Register::I, Register::V)   => (0xF029, 0x81),
                _ => panic!("unkown arguments")}},
        
        Mnemonic::ADD => {
            match (&info.order[0], &info.order[1]) {
                (Register::V, Register::NUM) => (0x7000, 0x82),
                (Register::V, Register::V)   => (0x8004, 0x482),
                (Register::I, Register::V)   => (0xF01E, 0x81),
                _ => panic!("unkown arguments")}},
        
        Mnemonic::OR        => (0x8001, 0x482),   Mnemonic::AND    => (0x8002, 0x482),
        Mnemonic::XOR       => (0x8003, 0x482),   Mnemonic::SUB    => (0x8005, 0x482),
        Mnemonic::SHIFT_R   => (0x8006, 0x482),   Mnemonic::SUB_N  => (0x8007, 0x482),
        Mnemonic::SHIFT_L   => (0x800E, 0x482),   Mnemonic::RAND   => (0xC000, 0x82),
        Mnemonic::DRAW      => (0xD000, 0x483),   Mnemonic::SKIP_P => (0xE09E, 0x81),
        Mnemonic::SKIP_NP   => (0xE0A1, 0x81),    Mnemonic::WAIT   => (0xF00A, 0x81),
        Mnemonic::STORE_BCD => (0xF033, 0x81),    Mnemonic::STORE  => (0xF055, 0x81),
        Mnemonic::READ      => (0xF065, 0x81),
    };

    if info.arguments.len() != extra & 0xF {
        panic!("Incorrect amount of arguments");
    }

    for (i, val) in info.arguments.iter().enumerate() {
        let shift = (extra >> (4 + (i<<2))) & 0xF;
        let max = match shift {
            8 | 4 => 0xF,
            _ => 0xFFFF >> ((extra & 0xF) * 4)
        };

        if *val > max {
            panic!("num too big!");
        }
        
        shell |= val << shift;
    }

    shell
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
    let code = "load %VA, FF";
    let info = scan(code, &mut errors);
    println!("{:X}",evaluate(&info));

    println!("{:?}",info.arguments);
    print_order(&info.order);

    if errors.len() != 0 {
        eprintln!("ERROR(S):");
        for i in errors.iter() {
            eprintln!("{}",i);
        }
        std::process::exit(1);
    }
}
