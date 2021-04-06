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
    line_index: usize,
    annotations: Vec<(String, usize)>,
    pub instructions: Vec<(Mnemonic, Vec<Register>, Vec<usize>)>,
    pub errors: Vec<(String, String, usize, usize)>,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            line: String::new(),
            delim: 0,
            index: 0,
            line_index: 0,
            annotations: vec![],
            instructions: vec![],
            errors: vec![],
        }
    }

    pub fn scan_file(&mut self, file: &mut BufReader<File>) {
        for (index, line) in file.lines().map(|l| l.unwrap().trim().to_ascii_lowercase()).enumerate() {
            self.line = line;
            self.delim = 0;
            self.index = 0;
            self.line_index = index;
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
            '%' => self.scan_register(),
            '0'..='9' => {
                let radix = match self.peek(1) {
                    Some('x') => { self.index += 2; 16 },
                    _ => 10,
                };
                self.scan_number(radix);
            },
            
            ';' => self.delim = self.line.len(), // The rest of a line is skipped if there is a comment
            
            _ => {
                if !self.scan_annotation() && !self.scan_mnemonic() {
                    // Replaces an annotation with a number if found
                    if let Some(annotation) = self.annotations.iter().find(|&x| x.0 == self.chunk()) {
                        self.instructions.last_mut().unwrap().2.push(annotation.1);
                        
                    } else { self.push_error("Unknown chunk".to_string()); }
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
                "i"   => I,
                "dt"  => Dt,
                "st"  => St,
                "key" => Key,
                _ => {
                    if self.peek(0).unwrap() == 'v' {
                        self.index += 1;
                        self.scan_number(16);
                        V
                    } else { self.push_error("Not a register".to_string()); return; }
                },
            }
        };
        
        self.instructions.last_mut().unwrap().1.push(register);
    }
        
    fn scan_number(&mut self, radix: u32) {
        match usize::from_str_radix(self.chunk(), radix) {
            Ok(argument) => self.instructions.last_mut().unwrap().2.push(argument),
            Err(_) => self.push_error("Not a number".to_string()),
        }
    }

    fn scan_annotation(&mut self) -> bool {
        if self.delim-1 - self.index > 0 &&
            &self.line[self.delim-1..self.delim] == ":" {
                
            let annotation = self.line[self.index..self.delim-1].to_string();
            let pc         = self.instructions.len() * 2 + 0x200;
            
            self.annotations.push((annotation, pc));

            true
        } else {
            false
        }
    }

    fn scan_mnemonic(&mut self) -> bool {
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
                _ => return false,
            },
            vec![],
            vec![]
        ));

        true
    }
    
    fn push_error(&mut self, msg: String) {
        self.errors.push((
            msg,
            self.line.clone(),
            self.line_index,
            self.index,
        ));
    }

    fn chunk(&self) -> &str {
        &self.line[self.index..self.delim]
    }
    
    fn peek(&self, i: usize) -> Option<char> {
        self.line.chars().nth(self.index+i)
    }
}
