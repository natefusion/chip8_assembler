#![allow(non_camel_case_types, non_snake_case)]
use std::{iter::Peekable, slice::Iter, collections::HashMap};
use crate::token::{Token, Token::*, Mnemonic, Register};

pub struct Scanner<'a> {
    tokens: &'a mut Peekable<Iter<'a, Token<'a>>>,
    labels: HashMap<&'a str, usize>,
    aliases: HashMap<&'a str, &'a Register>,
    variables: HashMap<&'a str, usize>,
    pub instructions: Vec<(&'a Mnemonic, Vec<&'a Register>, Vec<usize>)>,
}

impl<'a> Scanner<'a> {
    pub fn new(tokens: &'a mut Peekable<Iter<'a, Token<'a>>>) -> Self {
        Self {
            tokens,
            labels: HashMap::new(),
            aliases: HashMap::new(),
            variables: HashMap::new(),
            instructions: vec![]
        }
    }

    pub fn scan(&mut self) {
        while let Some(token) = self.tokens.next() {
            let line = self.instructions.len();
            match token {
                MNEMONIC(x) => {
                    self.push_mnemonic(x);

                    if let Mnemonic::BEGIN = x {
                        if let Some(NUM(_)) = self.tokens.peek() {
                        } else {
                            self.push_argument(line * 2 + 0x200);
                        }
                    }
                },
                
                REGISTER(x) => self.push_register(x),
                NUM(x)      => self.push_argument(*x),
                LABEL(label) => self.label(line, label),
                CONST | ALIAS => self.substitute(),
                _ => {},
            }
        }
    }

    // Problems occur if there is a const and alias set to the same thing, pls fix
    fn label(&mut self, line: usize, label: &'a str) {
        if let Some(COLON) = self.tokens.peek() {
            self.labels.insert(label, line);
        } else if let Some(&l) = self.labels.get(&label) {
            self.push_argument(l * 2 + 0x200);
        } else if let Some(&r) = self.aliases.get(&label) {
            self.push_register(r);
        } else if let Some(&a) = self.variables.get(&label) {
            self.push_argument(a);
        } else {
            self.error(format!("unknown label found '{}'", label));
        }
    }

    fn substitute(&mut self) {
        match (self.tokens.next(), self.tokens.next()) {
            (Some(LABEL(name)), Some(REGISTER(register))) => { self.aliases.insert(name, register); },
            (Some(LABEL(name)), Some(NUM(num))) => { self.variables.insert(name, *num); },
            _ => self.error("That's not how you make an alias. TRY AGAIN".to_string()),
        }
    }

    fn error(&self, msg: String) {
        eprintln!("Error: {}", msg);
        std::process::exit(1);
    }

    fn push_mnemonic(&mut self, mnemonic: &'a Mnemonic) { self.instructions.push((mnemonic, vec![], vec![])); }
    
    fn push_register(&mut self, register: &'a Register) {
        if let Register::V(x) = register { self.push_argument(*x); }
        self.instructions.last_mut().unwrap().1.push(register);
    }
    
    fn push_argument(&mut self, argument: usize) { self.instructions.last_mut().unwrap().2.push(argument); }
}
