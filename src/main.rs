use std::{slice::Iter, collections::HashMap};
use Exp::*;

#[derive(Clone)]
enum Exp {
    Atom(String),
    List(Vec<Exp>),
}

impl Exp {
    fn append(&mut self, item: Exp) {
        match self {
            List(x) => { x.push(item); },
            Atom(_) => {},
        }
    }
    
    fn prepend(&mut self, item: Exp) {
        match self {
            List(x) => { x.insert(0, item); },
            Atom(_) => {},
        }
    }

    fn print(&self) {
        match self {
            Atom(x) => {
                print!("{} ", x);
            },
            
            List(x) => {
                print!(" [ ");
                
                for a in x {
                    a.print();
                }
                
                print!("]");
            }
        }
    }
}

struct Env<'a> {
    iter: Iter<'a, String>,
    labels: HashMap<String, usize>,
    defs: HashMap<String, String>,
}

fn tokenize(code: &'static str) -> Vec<String> {
    code.replace("(", " ( ")
        .replace(")", " ) ")
        .replace("}", " } ")
        .replace("{", " { ")
        .replace("/*", " /* ")
        .replace("*/", " */ ")
        .lines()
        .flat_map(|x| {
            // splits line with whitespace as a deliminator into a vec of strings
            // inserts a "" to the end of the vec to signify a new line has occurred
            let mut y = x.split_whitespace()
                 .map(|x| x.to_string())
                 .collect::<Vec<String>>();

            //y.push("".to_string());
            y
        }).collect()
}

fn is_ins(token: &str) -> bool {
    match token {
        "def"      |
        
        "clear"    |
        "return"   |
        "jump"     |
        "jump0"    |
        "call"     |
        "neq"      |
        "eq"       |
        "set"      |
        "add"      |
        "or"       |
        "and"      |
        "xor"      |
        "sub"      |
        "shr"      |
        "subr"     |
        "shl"      |
        "rand"     |
        "draw"     |
        "bcd"      |
        "write"    |
        "read"     => true,
        _ => false,
    }
}

fn is_keyword(token: &str) -> bool {
    is_ins(token) || match token {
        "loop" |
        "macro" |
        "/*" |
        "*/"
            => true,
        _ => false,
    }
}

/*
fn is_math(token: &str) -> bool {
    match token {
        "+" |
        "*" |
        "/" |
        "-" => true,
        _ => false,
    }
}
*/

fn parse(iter: &mut Iter<String>, end: Option<&str>) -> Exp {
    let mut ast = List(vec![]);

    while let Some(token) = iter.next() {
        match token as &str {
            "loop" => {
                if let Some(mut x) = eval_loop(iter) {
                    x.prepend(Atom(token.to_string()));
                    ast.append(x);
                }
            },

            i if is_ins(&token) => {
                if let Some(mut x) = eval_ins(iter) {
                    x.prepend(Atom(i.to_string()));
                    ast.append(x);
                }
            },

            "(" => {
                let mut math = List(vec![]);

                while let Some(t) = iter.next() {
                    if t as &str == ")" { break; }
                    math.append(Atom(t.to_string()));
                }

                
                ast.append(math);
            },
            
            "/*" => { if !skip_comment(iter) { println!("comment end bad"); break; } },

            // tells parse where to stop parsing.
            // this is used for anything wrapped in braces
            _ => match end {
                Some(x) => if x == token { break; }
                None => {
                    return Atom(token.to_string());
                },
            },
        }
    }

    //println!("{:?}", ast);
    ast
}

fn eval_loop(iter: &mut Iter<String>) -> Option<Exp> {
    if let Some(token) = iter.next() {
        if token as &str != "{" {
            return None;
        }
    }

    Some(parse(iter, Some("}")))
}

/*
fn eval_math(iter: &mut Iter<String>) -> Option<Exp> {
    let mut ins = List(vec![]);
    
    while let Some(token) = iter.next() {
        if token as &str == ")" {
            break;
        }

        ins.append(Atom(token.clone()));
    }

    Some(ins)
}
*/

fn eval_ins(iter: &mut Iter<String>) -> Option<Exp> {
    let mut ins = List(vec![]);
    
    while let Some(token) = iter.next() {
        if is_keyword(&token) { break; }

        ins.append(Atom(token.clone()));
    }

    Some(ins)
}

fn skip_comment(iter: &mut Iter<String>) -> bool {
    while let Some(token) = iter.next() {
        if token as &str == "*/" {
            //println!("comment end gud");
            return true;
        }
    }

    false
}


fn main() {
    let code = include_str!("test.ch8");

    let tokens = tokenize(code);
    //println!("{:?}",tokens);
    let mut iter = tokens.iter();
    let ast = parse(&mut iter, None);

    ast.print();
}
