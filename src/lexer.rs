use crate::token::{Token, Token::*, Mnemonic::*, Register::*};

pub struct Lexer<'a> {
    data: &'a str,
    line: (usize, usize), // (line #, index at line #)
    index: usize,
    delim: usize,
    pub tokens: Vec<Token<'a>>,
    error_occured: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(data: &'a str) -> Self {
        Self {
            data,
            line: (1, 1),
            index: 0,
            delim: 0,
            tokens: vec![],
            error_occured: false,
        }
    }
    
    pub fn lex(&mut self) {
        while self.index < self.data.len() {
            self.analyze();
        }

        if self.error_occured {
            std::process::exit(1);
        }
    }
    
    fn analyze(&mut self) {
        let character = self.peek().unwrap();
        match character {
            ':' => self.add_token(Ok(COLON)),
            '%' => self.register(),
            '0'..='9' => self.number(),
            'a'..='z' => self.word(),
            
            ' ' | '\t' | '\r' => {},
            '\n' => { self.line.0 += 1; self.line.1 = self.index },
            ';' => {
                self.index = match self.data[self.index..].find('\n') {
                    Some(x) => self.index + x,
                    None => self.data.len(),
                };
            },
            
            _ => self.add_token(self.error()),
        }

        self.advance();
    }

    fn number(&mut self) {
        let radix = {
            if let Some('x') = self.peek_next() {
                self.advance();
                self.advance();
                16
            } else {
                10
            }
        };

        self.delim = match self.data[self.index..].find(|c: char| !c.is_ascii_digit()) {
            Some(x) => self.index + x,
            None => self.data.len()
        };

        let number = match usize::from_str_radix(self.chunk(), radix) {
            Ok(num) => Ok(NUM(num)),
            Err(_) => self.error(),
        };

        self.index = self.delim-1;

        self.add_token(number);
    }

    fn register(&mut self) {
        self.advance();
        
        if let Some('v') = self.peek() {
            self.advance();
        }

        self.delim = match self.data[self.index..].find(char::is_whitespace) {
            Some(x) => self.index + x,
            None => self.data.len()
        };

        let register = match self.chunk() {
            "i" => Ok(REGISTER(I)),
            "dt" => Ok(REGISTER(DT)),
            "st" => Ok(REGISTER(ST)),
            "key" => Ok(REGISTER(KEY)),
            _ => {
                match usize::from_str_radix(self.chunk(), 16) {
                    Ok(num) => {
                        self.advance();
                        Ok(REGISTER(V(num)))
                    },
                    Err(_) => self.error(),
                }
            },
        };

        self.index = self.delim-1;

        self.add_token(register);
    }

    fn word(&mut self) {
        self.delim = match self.data[self.index..].find(|c: char| !c.is_ascii_lowercase()) {
            Some(x) => self.index + x,
            _ => self.data.len()
        };

        let word = Ok(
            match self.chunk() {
                "clear"    => MNEMONIC(CLEAR),
                "end"      => MNEMONIC(END),
                "jump"     => MNEMONIC(JUMP),
                "jump0"    => MNEMONIC(JUMP0),
                "begin"    => MNEMONIC(BEGIN),
                "neq"      => MNEMONIC(NEQ),
                "eq"       => MNEMONIC(EQ),
                "set"      => MNEMONIC(SET),
                "add"      => MNEMONIC(ADD),
                "or"       => MNEMONIC(OR),
                "and"      => MNEMONIC(AND),
                "xor"      => MNEMONIC(XOR),
                "sub"      => MNEMONIC(SUB),
                "shr"      => MNEMONIC(SHR),
                "subr"     => MNEMONIC(SUBR),
                "shl"      => MNEMONIC(SHL),
                "rand"     => MNEMONIC(RAND),
                "draw"     => MNEMONIC(DRAW),
                "writebcd" => MNEMONIC(WRITEBCD),
                "write"    => MNEMONIC(WRITE),
                "read"     => MNEMONIC(READ),
                "alias"    => ALIAS,
                _ => LABEL(&self.data[self.index..self.delim]),
        });

        self.index = self.delim-1;

        self.add_token(word);
    }

    fn chunk(&mut self) -> &str {
        &self.data[self.index..self.delim]
    }

    fn peek(&self) -> Option<char> {
        self.data.chars().nth(self.index)
    }

    fn peek_next(&self) -> Option<char> {
        self.data.chars().nth(self.index + 1)
    }

    fn advance(&mut self) {
        self.index += 1;
    }

    fn add_token(&mut self, token: Result<Token<'a>, ((usize, usize), usize)>) {
        match token {
            Err((x,y)) => {
                eprintln!("Error on line {} here: {}", x.0, &self.data[(x.1)..y]);
                self.error_occured = true;
            },
            
            Ok(x) if !self.error_occured => self.tokens.push(x),
            _ => {},
        }
    }

    fn error(&self) -> Result<Token<'a>, ((usize, usize), usize)> {
        Err((self.line, self.index))
    }
}
