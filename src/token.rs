use std::fmt;

pub struct TokenInfo {
    pub token: TokenType,
    pub lexeme: &'static str,
    pub line: usize,
}

#[derive(Copy,Clone)]
pub enum Token {
    Colon, Alias, Const, Identifier,

    // Mnemonics
    Clear, End, Jump, Jump0, Begin, Neq,
    Eq, Set, Add, Or, And, Xor, Sub, Shr,
    Subr, Shl, Rand, Draw, Writebcd, Write,
    Read,

    // Registers
    V, I, DT, ST, Key
}

pub enum TokenType {
    Mnemonic(Token), Register(Token), Number(usize), Other(Token)
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            match &self {
                Token::Colon => "Colon",
                Token::Alias => "Alias",
                Token::Const => "Const",
                Token::Identifier => "Identifier",
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
                Token::V => "V",
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
            TokenType::Other(x) => f.write_fmt(format_args!("{:?}",x)),
        }
    }
}
