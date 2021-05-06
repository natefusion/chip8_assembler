use std::fmt;
use Token::*;

pub fn mnemonic(lexeme: &str) -> Result<(TokenType, Token), String> {
    match lexeme {
        "clear"    => Ok((Mnemonic, Clear)),
        "end"      => Ok((Mnemonic, End)),
        "jump"     => Ok((Mnemonic, Jump)),
        "jump0"    => Ok((Mnemonic, Jump0)),
        "begin"    => Ok((Mnemonic, Begin)),
        "neq"      => Ok((Mnemonic, Neq)),
        "eq"       => Ok((Mnemonic, Eq)),
        "set"      => Ok((Mnemonic, Set)),
        "add"      => Ok((Mnemonic, Add)),
        "or"       => Ok((Mnemonic, Or)),
        "and"      => Ok((Mnemonic, And)),
        "xor"      => Ok((Mnemonic, Xor)),
        "sub"      => Ok((Mnemonic, Sub)),
        "shr"      => Ok((Mnemonic, Shr)),
        "subr"     => Ok((Mnemonic, Subr)),
        "shl"      => Ok((Mnemonic, Shl)),
        "rand"     => Ok((Mnemonic, Rand)),
        "draw"     => Ok((Mnemonic, Draw)),
        "writebcd" => Ok((Mnemonic, Writebcd)),
        "write"    => Ok((Mnemonic, Write)),
        "read"     => Ok((Mnemonic, Read)),
        x          => Err(format!("Expected a mnemonic, found: {}", x)),
    }
}

pub fn register(lexeme: &str) -> Result<(TokenType, Token), String> {
    match lexeme {
        "%v0"  => Ok((Register, V(0x0))),
        "%v1"  => Ok((Register, V(0x1))),
        "%v2"  => Ok((Register, V(0x2))),
        "%v3"  => Ok((Register, V(0x3))),
        "%v4"  => Ok((Register, V(0x4))),
        "%v5"  => Ok((Register, V(0x5))),
        "%v6"  => Ok((Register, V(0x6))),
        "%v7"  => Ok((Register, V(0x7))),
        "%v8"  => Ok((Register, V(0x8))),
        "%v9"  => Ok((Register, V(0x9))),
        "%va"  => Ok((Register, V(0xA))),
        "%vb"  => Ok((Register, V(0xB))),
        "%vc"  => Ok((Register, V(0xC))),
        "%vd"  => Ok((Register, V(0xD))),
        "%ve"  => Ok((Register, V(0xE))),
        "%vf"  => Ok((Register, V(0xF))),
        "%i"   => Ok((Register, I)),
        "%dt"  => Ok((Register, DT)),
        "%st"  => Ok((Register, ST)),
        "%key" => Ok((Register, Key)),
        x      => Err(format!("Expected a register, found: {}", x)),
    }
}

pub fn macro(lexeme: &str) -> Result<(TokenType, Token), String> {
    match lexeme {
        "alias" => Ok((Macro, Alias)),
        "const" => Ok((Macro, Const)),
        ":"     => Ok((Macro, Colon)),
        x       => Err(format!("Expeceted a macro, found: {}", x)),
    }
}

pub struct TokenInfo {
    pub tt: TokenType,
    pub t: Token,
    pub l: String,
    pub i: usize,
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
