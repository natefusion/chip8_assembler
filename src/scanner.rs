use std::{iter::Peekable, slice::Iter, collections::HashMap};
use crate::token::{TokenInfo, Token, Token::*, TokenType::*};

pub struct Scanner<'a> {
    tokens: &'a mut Peekable<Iter<'a, TokenInfo>>,
    labels: HashMap<&'static str, usize>,
    variables: HashMap<&'static str, &'a TokenInfo>,
    pub instructions: Vec<(Token, Vec<Token>, Vec<usize>)>, // [(Mnemonic, [Register], [usize])]
}

impl<'a> Scanner<'a> {
    pub fn new(tokens: &'a mut Peekable<Iter<'a, TokenInfo>>) -> Self {
        Self {
            tokens,
            labels: HashMap::new(),
            variables: HashMap::new(),
            instructions: vec![],
        }
    }

    pub fn scan(&mut self) {
        while let Some(token) = self.tokens.next() {
            self.push_token(&token);
        }
    }

    fn push_token(&mut self, token: &'a TokenInfo) {
        match token.token {
            Register(x) => {
                if let V = x {
                    let number = usize::from_str_radix(&token.lexeme[2..], 16).unwrap();
                    self.instructions.last_mut().unwrap().2.push(number);
                }
                
                self.instructions.last_mut().unwrap().1.push(x);
            },
            
            Mnemonic(x) => {
                self.instructions.push((x, vec![], vec![]));
            },
            
            Number(x) => { self.instructions.last_mut().unwrap().2.push(x); },
            
            Other(t) => match t {
                Colon => {
                    if let Some(key) = self.tokens.next() {
                        let pc = self.instructions.len();
                        self.labels.insert(key.lexeme, pc);
                    }
                },
                
                Const | Alias => {
                    if let (Some(key), Some(value)) = (self.tokens.next(), self.tokens.next()) {
                        if let Other(Identifier) = &key.token {
                            match &value.token {
                                Register(_) | Number(_) => { self.variables.insert(key.lexeme, value); },
                                _ => {},
                            }
                        }
                    }
                },
                
                Identifier => {
                    if let Some(&t) = self.labels.get(&token.lexeme) { self.instructions.last_mut().unwrap().2.push(t * 2 + 0x200); }
                    if let Some(&t) = self.variables.get(&token.lexeme) { self.push_token(t); }
                },

                _ => {},
            },
        }
    }
}
