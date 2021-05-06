use crate::token::{TokenInfo, TokenType, TokenType::*};
use crate::token;

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

        self.add_token(
            match usize::from_str_radix(self.chunk(), radix) {
                Ok(num) => Some(Number(num)),
                Err(_) => None,
            }
        );

        self.index = self.delim-1;
    }
    
    fn register(&mut self) {
        let register = self.chunk();

        self.add_token(
            match token::REGISTERS.get(&register) {
                Some(x) => Some(Register(*x)),
                None => None,
            }
        );

        self.index = self.delim-1;
    }

    fn identifier(&mut self) {
        let ident = self.chunk();

        self.add_token(Some(
            if let Some(x) = token::MNEMONICS.get(&ident) {
                Mnemonic(*x)
            } else if let Some(x) = token::MACROS.get(&ident) {
                Macro(*x)
            } else {
                Identifier
            }
        ));

        self.index = self.delim-1;
    }

    fn chunk(&self)   -> &'static str { &self.data[self.index..self.delim] }
    fn current(&self) -> char         { self.data.chars().nth(self.index).unwrap() }
    fn peek(&self)    -> Option<char> { self.data.chars().nth(self.index+1) }
        
    fn add_token(&mut self, info: Option<TokenType>) {
        match info {
            Some(token) if !self.error_occured => { self.tokens.push(TokenInfo { token, lexeme: self.chunk(), line: self.line, }); },

            None => {
                eprintln!("Error on line {} here: {}", self.line, self.chunk());
                self.error_occured = true;
            },

            _ => {},
        }
    }
}
