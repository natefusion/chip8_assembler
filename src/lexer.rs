use crate::token::{TokenInfo, Token::*, TokenType, TokenType::*};

macro_rules! jump_to {
    ($pattern:expr, $index:expr, $data:expr) => {
        match $data[$index..].find($pattern) {
            Some(x) => $index+x,
            None => $data.len(), }}
}

pub struct Lexer {
    data: &'static str,
    line: usize,
    index: usize,
    delim: usize,
    error_occured: bool,
    pub tokens: Vec<TokenInfo>,
}

impl Lexer {
    pub fn new(data: &'static str) -> Self {
        Self {
            data,
            line: 1,
            index: 0,
            delim: 0,
            error_occured: false,
            tokens: vec![], }
    }

    pub fn tokenize(&mut self) {
        while self.index < self.data.len() {
            self.delim = jump_to!(char::is_whitespace, self.index, self.data);
            
            match self.current() {
                ':' => self.add_token(Some((Other(Colon), self.chunk()))),
                '%' => self.register(),
                '0'..='9' => self.number(),
                'a'..='z' => self.identifier(),
                
                ' ' | '\t' | '\r' => {},
                '\n' => { self.line += 1; }
                ';' => { self.index = jump_to!('\n', self.index, self.data)-1; },
                
                _ => self.add_token(None),
            }

            self.index += 1;
        }
    }

    fn number(&mut self) {
        let radix = if let Some('x') = self.peek() {
            self.index += 2;
            16
        } else {
            10
        };

        self.add_token(match usize::from_str_radix(self.chunk(), radix) {
            Ok(num) => Some((Number(num), self.chunk())),
            Err(_) => None,
        });

        self.index = self.delim-1;
    }
    
    fn register(&mut self) {
        let lexeme = self.chunk();
        self.index += 1;
        
        self.add_token(
            if 'v' == self.current() {
                Some((Register(V), lexeme))
            } else {            
                match self.chunk() {
                    "i" => Some((Register(I), lexeme)),
                    "dt" => Some((Register(DT), lexeme)),
                    "st" => Some((Register(ST), lexeme)),
                    "key" => Some((Register(Key), lexeme)),
                    _ => None,
                }
            }
        );

        self.index = self.delim-1;
    }

    fn identifier(&mut self) {
        self.add_token(Some((
            match self.chunk() {
                "clear"    => Mnemonic(Clear),
                "end"      => Mnemonic(End),
                "jump"     => Mnemonic(Jump),
                "jump0"    => Mnemonic(Jump0),
                "begin"    => Mnemonic(Begin),
                "neq"      => Mnemonic(Neq),
                "eq"       => Mnemonic(Eq),
                "set"      => Mnemonic(Set),
                "add"      => Mnemonic(Add),
                "or"       => Mnemonic(Or),
                "and"      => Mnemonic(And),
                "xor"      => Mnemonic(Xor),
                "sub"      => Mnemonic(Sub),
                "shr"      => Mnemonic(Shr),
                "subr"     => Mnemonic(Subr),
                "shl"      => Mnemonic(Shl),
                "rand"     => Mnemonic(Rand),
                "draw"     => Mnemonic(Draw),
                "writebcd" => Mnemonic(Writebcd),
                "write"    => Mnemonic(Write),
                "read"     => Mnemonic(Read),
                "alias"    => Other(Alias),
                "const"    => Other(Const),
                _          => Other(Identifier),
            }, self.chunk()))
        );

        self.index = self.delim-1;
    }

    fn chunk(&self)   -> &'static str { &self.data[self.index..self.delim] }
    fn current(&self) -> char         { self.data.chars().nth(self.index).unwrap() }
    fn peek(&self)    -> Option<char> { self.data.chars().nth(self.index+1) }
        
    fn add_token(&mut self, info: Option<(TokenType, &'static str)>) {
        match info {
            Some((token, lexeme)) if !self.error_occured => { self.tokens.push(TokenInfo { token, lexeme, line: self.line, }); },

            None => {
                eprintln!("Error on line {} here: {}", self.line, self.chunk());
                self.error_occured = true;
            },

            _ => {},
        }
    }
}
