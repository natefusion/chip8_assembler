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
    pub registers: Vec<Keyword>,
    pub arguments: Vec<usize>,

    pub line: usize,
    pub ch: usize,
}

#[derive(Copy, Clone)]
pub enum Keyword {
    V, I, Dt, St, Key,
    
    Clear, Return, Jump, Jump0, Call, Neq,
    Eq, Set, Add, Or, And, Xor, Sub, Shr,
    Subr, Shl, Rand, Draw, Bcd, Write,
    Read,
    
    Colon, Define, Unk,
}

#[derive(Copy, Clone)]
pub enum Category {
    Func(Keyword),
    Def(Keyword),
    Reg(Keyword),
    Num,
    Ident,
}

impl Token {
    pub fn new(raw: &'static str, line: usize, ch: usize) -> Self {
        Token { category: Self::tokenize(raw), raw, line, ch }
    }

    fn tokenize(raw: &str) -> Category {
        match raw.chars().nth(0).unwrap() {
            '0'..='9' => Num,
            
            'v' => Reg(V),
            'i' => Reg(I),
            ':' => Def(Colon),
            
            'a'..='z' |
            'A'..='Z' => match raw {
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
                
                "define" => Def(Define),

                "dt"  => Reg(Dt),
                "st"  => Reg(St),
                "key" => Reg(Key),

                _ => Ident,
            }

            _ => Ident,
        }
    }
}

impl Instruction {
    pub fn new(token: &Token, function: Keyword) -> Self {
        Self {
            function,
            registers: vec![],
            arguments: vec![],
            line: token.line,
            ch: token.ch,
        }
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
