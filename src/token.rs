use {Keyword::*, Category::*};
use std::{cmp::{Eq, PartialEq}, hash::{Hash, Hasher}};

pub struct Token {
    pub category: Category,
    pub raw: &'static str,
    pub line: usize,
    pub ch: usize,
}

pub struct Instruction {
    pub function: Keyword,
    pub arguments: Vec<Category>,
    //pub registers: Vec<Keyword>,
    //pub arguments: Vec<usize>,

    pub line: usize,
    pub ch: usize,
}

#[derive(Copy, Clone)]
pub enum Keyword {
    V(usize), I, Dt, St, Key,
    
    Clear, Return, Jump, Jump0, Call, Neq,
    Eq, Set, Add, Or, And, Xor, Sub, Shr,
    Subr, Shl, Rand, Draw, Bcd, Write,
    Read,
    
    Colon, Defvar, Defmacro, Include
}

#[derive(Copy,Clone)]
pub enum Category {
    Func(Keyword),
    Def(Keyword),
    Reg(Keyword),
    Num(usize),
    Ident,
}

impl Token {
    pub fn new(raw: &'static str, line: usize, ch: usize) -> Self {
        Token { category: Self::tokenize(raw), raw, line, ch }
    }

    fn number(raw: &'static str, x: usize, radix: u32) -> Category {
        match usize::from_str_radix(&raw[x..], radix) {
            Ok(num) => Num(num),
            Err(_)  => Ident,
        }
    }

    fn tokenize(raw: &'static str) -> Category {
        match raw.chars().next().unwrap() {
            '0'..='9' => {
                let (x, radix) = match raw.get(0..2) {
                    Some("0x") => (2, 16),
                    Some("0b") => (2, 2),
                    _ => (0, 10),
                };

                Self::number(raw, x, radix)
            },
            
            'v' => match (raw.len(), Self::number(raw, 1, 16)) {
                (2, Num(num)) => Reg(V(num)),
                (_, _) => Ident,
            },

            _ => match raw {
                "clear"    => Func(Clear),
                "return"   => Func(Return),
                "jump"     => Func(Jump),
                "jump0"    => Func(Jump0),
                "call"     => Func(Call),
                "neq"      => Func(Neq),
                "eq"       => Func(Eq),
                "set"      => Func(Set),
                "add"      => Func(Add),
                "or"       => Func(Or),
                "and"      => Func(And),
                "xor"      => Func(Xor),
                "sub"      => Func(Sub),
                "shr"      => Func(Shr),
                "subr"     => Func(Subr),
                "shl"      => Func(Shl),
                "rand"     => Func(Rand),
                "draw"     => Func(Draw),
                "bcd"      => Func(Bcd),
                "write"    => Func(Write),
                "read"     => Func(Read),
                "include" => Func(Include),
                
                
                "defvar" => Def(Defvar),
                "defmacro" => Def(Defmacro),

                "dt"  => Reg(Dt),
                "st"  => Reg(St),
                "key" => Reg(Key),
                "i"   => Reg(I),
                ":"   => Def(Colon),

                _ => Ident,
            }
        }
    }
}

impl Instruction {
    pub fn new(function: Keyword, line: usize, ch: usize) -> Self {
        Self { function, arguments: vec![], line, ch, }
    }
}

impl Hash for Token {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw.hash(state);
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl Eq for Token {}

impl std::fmt::Debug for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                Keyword::Colon => "Colon",
                Keyword::Clear => "Clear",
                Keyword::Return => "Return",
                Keyword::Jump => "Jump",
                Keyword::Jump0 => "Jump0",
                Keyword::Call => "Call",
                Keyword::Neq => "Neq",
                Keyword::Eq => "Eq",
                Keyword::Set => "Set",
                Keyword::Add => "Add",
                Keyword::Or => "Or",
                Keyword::And => "And",
                Keyword::Xor => "Xor",
                Keyword::Sub => "Sub",
                Keyword::Shr => "Shr",
                Keyword::Subr => "Subr",
                Keyword::Shl => "Shl",
                Keyword::Rand => "Rand",
                Keyword::Draw => "Draw",
                Keyword::Bcd => "bcd",
                Keyword::Write => "Write",
                Keyword::Read => "Read",
                Keyword::V(_) => "V",
                Keyword::I => "I",
                Keyword::Dt => "DT",
                Keyword::St => "ST",
                Keyword::Key => "Key",
                Keyword::Include => "include",
                Keyword::Defvar => "defvar",
                Keyword::Defmacro => "defmacro",
            }
        )
    }
}
