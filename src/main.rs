use std::{slice::Iter, iter::Peekable, collections::HashMap};
use Exp::*;

#[derive(Clone)]
enum Exp {
    Atom(String),
    List(Vec<Exp>),
}

// woohoo shitty lisp here I come
impl Exp {
    fn push(&mut self, item: Exp) {
        if let List(x) = self { x.push(item); }
    }
    
    fn prepend(&mut self, item: Exp) {
        if let List(x) = self { x.insert(0, item); }
    }

    fn is_empty(&self) -> bool {
        match self {
            Atom(x) => x.is_empty(),
            List(x) => x.is_empty(),
        }
    }

    fn first(&self) -> Exp {
        match self {
            List(x) => x[0].clone(),
            Atom(_) => self.clone(),
        }
    }

    fn rest(&self) -> Exp {
        match self {
            // who needs efficiency
            List(x) => List(x[1..].to_vec()),
            Atom(_) => self.clone(),
        }
    }
    
    fn print(&self) {
        match self {
            Atom(x) => {
                print!("\"{}\" ", x);
                //print!("Atom({}) ", x);
            },
            
            List(x) => {
                print!("[ ");
                //print!("List([");
                
                for a in x {
                    a.print();
                }
                
                print!("]\n");
            }
        }
    }
}

struct Env {
    labels: HashMap<String, usize>,
    defs: HashMap<String, String>,
    pc: usize,
}

fn tokenize(code: &'static str) -> Vec<String> {
    code.lines()
        .flat_map(|x| {
            if let Some("#") = x.get(..1) {
                return vec![];
            }
            
            let mut y = x.split_whitespace()
                .map(|x| x.to_string())
                .collect::<Vec<String>>();

            match y.iter().next() {
                Some(x) => if x != "(" && x != ")" {
                    y.insert(0, "(".to_string());
                    y.push(")".to_string());
                },
                None => {},
            }
            y
        }).collect()
}

fn parse(iter: &mut Peekable<Iter<String>>) -> Exp {
    let mut ast = List(vec![]);

    while let Ok(x) = parse_math(iter) {
        ast.push(x);
    }

    ast
}

fn parse_math(iter: &mut Peekable<Iter<String>>) -> Result<Exp, String> {
    let token = iter.next();

    if let Some(x) = token { 
        if *x == "(" {
            let mut l = List(vec![]);

            while let Some(x) = iter.peek() {
                if **x == ")" { break; }
                match parse_math(iter) {
                    Ok(x) => l.push(x),
                    Err(x) => {},
                }
            }
            iter.next();
            Ok(l)
        } else if *x == ")" {
            Err("bad math".to_string())
        } else {
            Ok(Atom(x.to_string()))
        }
    } else {
        Err("bad math".to_string())
    }
}

//fn eval(ast: Exp, env: Env) {
//}

fn main() {
    let code = include_str!("test.ch8");

    let tokens = tokenize(code);
    //println!("{:?}",tokens);
    let mut iter = tokens.iter().peekable();

    let ast = parse(&mut iter);
    //ast.print();
    //if let Ok(x) = ast {
    //    x.print();
    //}

    /*
    let mut env = Env {
        ast,
        labels: HashMap::new(),
        defs: HashMap::new(),
        pc: 0x200,
    };
     */

}
