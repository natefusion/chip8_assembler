#![allow(non_camel_case_types, non_snake_case)]
use std::{fs::File, io::{BufRead, BufReader}};
use crate::tokens::{Mnemonic, Mnemonic::*, Register, Register::*};

/*
; TODO
; implement macros for things like calculation:
; EX: :math { 1 + 2 + 3 }
; parsing process
*/

pub struct Scanner {
    line: String,
    delim: usize,
    index: usize,
    pub instructions: Vec<(Mnemonic, Vec<Register>, Vec<usize>)>,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            line: String::new(),
            delim: 0,
            index: 0,
            instructions: vec![],
        }
    }

    pub fn scan_file<'a>(&mut self, file: &mut BufReader<File>) {
        for line in file.lines().map(|l| l.unwrap().trim().to_string()) {
            self.line = line;
            self.delim = 0;
            self.index = 0;
            self.scan_line();
        }
    }

    // Tokenizes the given string
    fn scan_line(&mut self) {
        while self.delim < self.line.len() {
            self.delim = {
                match self.line[self.index..].find(char::is_whitespace) {
                    Some(x) => self.index + x,
                    None => self.line.len(),
                }
            };
            self.scan_chunk();
        }
    }

    /* Algorithm:
     * Check first character to determine the type of token
     * match the chunk (from the current index to the next space character) to a token
     * Add the newly formed token to the list of instructions
     */
    fn scan_chunk(&mut self) {
        match self.peek(0) {
            '%'        => self.scan_register(),
            '0'..='9' => self.scan_number(),         
            ';'         => self.delim = self.line.len(), // The rest of a line is skipped if there is a comment
            _           => self.scan_mnemonic(),
        }

        // Moves index to the next chunk
        self.index = match self.line[self.delim..].find(|c: char| c.is_ascii_graphic()) {
            Some(x) => self.delim + x,
            None => self.line.len(),
        };
    }
    
    fn scan_register(&mut self) {
        let register = {
            match self.chunk() {
                _ if self.peek(1) == 'v' => {
                    self.index += 2;
                    let argument = usize::from_str_radix(self.chunk(), 16).unwrap();
                    self.instructions.last_mut().unwrap().2.push(argument);
                    V
                },
                "i"   => I,
                "dt"  => DT,
                "st"  => ST,
                "key" => KEY,
                _     => Register::UNKNOWN,
            }
        };
        
        self.instructions.last_mut().unwrap().1.push(register);
    }
        
    fn scan_number(&mut self) {
        let radix = {
            if self.peek(1) == 'x' {
                self.index += 2;
                16 
            } else {
                10 
            }
        };

        let argument = usize::from_str_radix(self.chunk(), radix).unwrap();
        self.instructions.last_mut().unwrap().2.push(argument);
    }
    
    fn scan_mnemonic(&mut self) {
        let mnemonic = (
            match self.chunk() {
                "clear"    => CLEAR,
                "end"      => END,
                "jump"     => JUMP,
                "jump0"    => JUMP0,
                "begin"    => BEGIN,
                "neq"      => NEQ,
                "eq"       => EQ,
                "set"      => SET,
                "add"      => ADD,
                "or"       => OR,
                "and"      => AND,
                "xor"      => XOR,
                "sub"      => SUB,
                "shr"      => SHR,
                "subr"     => SUBR,
                "shl"      => SHL,
                "rand"     => RAND,
                "draw"     => DRAW,
                "writebcd" => WRITEBCD,
                "write"    => WRITE,
                "read"     => READ,
                _ => Mnemonic::UNKNOWN,
            },
            vec![],
            vec![]
        );
        
        self.instructions.push(mnemonic);
    }
    
    fn chunk(&self) -> &str {
        &self.line[self.index..self.delim]
    }
    
    fn peek(&self, i: usize) -> char {
        self.line.chars().nth(self.index+i).unwrap()
    }
}
