use std::{collections::HashMap, fmt};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref MNEMONICS: HashMap<&'static str, Token> = [
        ("clear",    Token::Clear),
        ("end",      Token::End),
        ("jump",     Token::Jump),
        ("jump0",    Token::Jump0),
        ("begin",    Token::Begin),
        ("neq",      Token::Neq),
        ("eq",       Token::Eq),
        ("set",      Token::Set),
        ("add",      Token::Add),
        ("or",       Token::Or),
        ("and",      Token::And),
        ("xor",      Token::Xor),
        ("sub",      Token::Sub),
        ("shr",      Token::Shr),
        ("subr",     Token::Subr),
        ("shl",      Token::Shl),
        ("rand",     Token::Rand),
        ("draw",     Token::Draw),
        ("writebcd", Token::Writebcd),
        ("write",    Token::Write),
        ("read",     Token::Read)
    ].iter().cloned().collect();
}

lazy_static! {
    pub static ref REGISTERS: HashMap<&'static str, Token> = [
        ("%v0",  Token::V(0x0)),
        ("%v1",  Token::V(0x1)),
        ("%v2",  Token::V(0x2)),
        ("%v3",  Token::V(0x3)),
        ("%v4",  Token::V(0x4)),
        ("%v5",  Token::V(0x5)),
        ("%v6",  Token::V(0x6)),
        ("%v7",  Token::V(0x7)),
        ("%v8",  Token::V(0x8)),
        ("%v9",  Token::V(0x9)),
        ("%va",  Token::V(0xA)),
        ("%vb",  Token::V(0xB)),
        ("%vc",  Token::V(0xC)),
        ("%vd",  Token::V(0xD)),
        ("%ve",  Token::V(0xE)),
        ("%vf",  Token::V(0xF)),
        ("%i",   Token::I),
        ("%dt",  Token::DT),
        ("%st",  Token::ST),
        ("%key", Token::Key)
    ].iter().cloned().collect();
}

lazy_static! {
    pub static ref MACROS: HashMap<&'static str, Token> = [
        ("alias", Token::Alias),
        ("const", Token::Const),
        (":",     Token::Colon)
    ].iter().cloned().collect();
}

pub struct TokenInfo {
    pub token: TokenType,
    pub lexeme: String,
    pub index: usize,
}

#[derive(Copy, Clone)]
pub enum Token {
    // Registers
    V(usize), I, DT, ST, Key,

    // Mnemonics
    Clear, End, Jump, Jump0, Begin, Neq,
    Eq, Set, Add, Or, And, Xor, Sub, Shr,
    Subr, Shl, Rand, Draw, Writebcd, Write,
    Read,

    // Macros
    Colon, Alias, Const,
}

#[derive(Copy, Clone)]
pub enum TokenType {
    Identifier(u64),
    Mnemonic(Token),
    Register(Token),
    Macro(Token),
    Number(usize),
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            match &self {
                Token::Colon => "Colon",
                Token::Alias => "Alias",
                Token::Const => "Const",
                Token::Clear => "Clear",
                Token::End => "End",
                Token::Jump => "Jump",
                Token::Jump0 => "Jump0",
                Token::Begin => "Begin",
                Token::Neq => "Neq",
                Token::Eq => "Eq",
                Token::Set => "Set",
                Token::Add => "Add",
                Token::Or => "Or",
                Token::And => "And",
                Token::Xor => "Xor",
                Token::Sub => "Sub",
                Token::Shr => "Shr",
                Token::Subr => "Subr",
                Token::Shl => "Shl",
                Token::Rand => "Rand",
                Token::Draw => "Draw",
                Token::Writebcd => "Writebcd",
                Token::Write => "Write",
                Token::Read => "Read",
                Token::V(_) => "V",
                Token::I => "I",
                Token::DT => "DT",
                Token::ST => "ST",
                Token::Key => "Key",
            }
        )
    }
}

impl fmt::Debug for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            TokenType::Number(x) => f.write_fmt(format_args!("Number:{:?}",x)),
            TokenType::Mnemonic(x) => f.write_fmt(format_args!("Mnemonic:{:?}",x)),
            TokenType::Register(x) => f.write_fmt(format_args!("Register:{:?}",x)),
            TokenType::Macro(x) => f.write_fmt(format_args!("{:?}",x)),
            TokenType::Identifier(x) => f.write_fmt(format_args!("Identifier: {}",x)),
        }
    }
}
