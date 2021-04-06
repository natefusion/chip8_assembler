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
    annotations: Vec<(String, usize)>,
    pub instructions: Vec<(Mnemonic, Vec<Register>, Vec<usize>)>,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            line: String::new(),
            delim: 0,
            index: 0,
            annotations: vec![],
            instructions: vec![],
        }
    }

    pub fn scan_file(&mut self, file: &mut BufReader<File>) {
        for line in file.lines().map(|l| l.unwrap().trim().to_ascii_lowercase()) {
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
        match self.peek(0).unwrap() {
            '%'       => self.scan_register(),
            '0'..='9' => self.scan_number(),
            ';'       => self.delim = self.line.len(), // The rest of a line is skipped if there is a comment
            _ => {
                // Maybe merge these two methods into one?
                // Maybe have annotations work like a stack maybe?
                self.scan_annotation();
                self.scan_mnemonic();

                // Replaces an annotation with a number if found
                if let Some(annotation) = self.annotations.iter().find(|&x| x.0 == self.chunk()) {
                    self.instructions.last_mut().unwrap().2.push(annotation.1);
                }
            },
        }

        // Moves index to the next chunk
        self.index = match self.line[self.delim..].find(|c: char| c.is_ascii_graphic()) {
            Some(x) => self.delim + x,
            None => self.line.len(),
        };
    }
    
    fn scan_register(&mut self) {
        self.index += 1;
        let register = {
            match self.chunk() {
                _ if self.peek(0).unwrap() == 'v' => {
                    self.index += 1;
                    let argument = usize::from_str_radix(self.chunk(), 16).unwrap();
                    self.instructions.last_mut().unwrap().2.push(argument);
                    V
                },
                "i"   => I,
                "dt"  => Dt,
                "st"  => St,
                "key" => Key,
                _     => Register::Unknown,
            }
        };
        
        self.instructions.last_mut().unwrap().1.push(register);
    }
        
    fn scan_number(&mut self) {
        let radix = {
            if let Some('x') = self.peek(1) {
                self.index += 2;
                16 
            } else {
                10 
            }
        };

        let argument = usize::from_str_radix(self.chunk(), radix).unwrap();
        self.instructions.last_mut().unwrap().2.push(argument);
    }

    fn scan_annotation(&mut self) {
        if &self.line[self.delim-1..self.delim] == ":" {
            let annotation = self.line[self.index..self.delim-1].to_string();
            let pc         = self.instructions.len() * 2 + 0x200;

            self.annotations.push((annotation, pc));
        }
    }
    
    fn scan_mnemonic(&mut self) {
        self.instructions.push((
            match self.chunk() {
                "clear"    => Clear,
                "end"      => End,
                "jump"     => Jump,
                "jump0"    => Jump0,
                "begin"    => Begin,
                "neq"      => Neq,
                "eq"       => Eq,
                "set"      => Set,
                "add"      => Add,
                "or"       => Or,
                "and"      => And,
                "xor"      => Xor,
                "sub"      => Sub,
                "shr"      => Shr,
                "subr"     => Subr,
                "shl"      => Shl,
                "rand"     => Rand,
                "draw"     => Draw,
                "writebcd" => Writebcd,
                "write"    => Write,
                "read"     => Read,
                _ => return,
            },
            vec![],
            vec![]
        ));
    }
    
    fn chunk(&self) -> &str {
        &self.line[self.index..self.delim]
    }
    
    fn peek(&self, i: usize) -> Option<char> {
        self.line.chars().nth(self.index+i)
    }
}
