use std::{slice::Iter, iter::Peekable, collections::HashMap};
use Exp::*;

#[derive(Clone)]
enum Exp {
    Symbol(String),
    Number(isize),
    Proc(fn(&[Exp]) -> Exp),
    List(Vec<Exp>),
}

// woohoo shitty lisp here I come
impl Exp {
    fn push(&mut self, item: Exp) {
        if let List(x) = self { x.push(item); }
    }
    
    fn first(&self) -> Exp {
        match self {
            List(x) => x[0].clone(),
            _ => self.clone(),
        }
    }

    fn rest(&self) -> Exp {
        match self {
            // who needs efficiency
            List(x) => List(x[1..].to_vec()),
            _ => self.clone(),
        }
    }
    
    fn print(&self) {
        match self {
            Number(x) => {
                print!("\"{}\" ", x);
                //print!("Atom({}) ", x);
            },

            Symbol(x) => {
                print!("\"{}\" ", x);
            },

            Proc(_) => {
                print!("proc here ");
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
    defs: HashMap<String, Exp>,
    pc: isize,
}

fn tokenize(code: &'static str) -> Vec<String> {
    code.lines()
        .flat_map(|x| {
            if let Some(";") = x.get(..1) {
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

    while let Ok(x) = parse_exp(iter) {
        ast.push(x);
    }

    ast
}

fn parse_atom(atom: &str) -> Exp {
    let (x, radix) = match atom.get(0..2) {
        Some("#b") => (2, 2),
        Some("#x") => (2, 16),
        _ => (0, 10),
    };

    match isize::from_str_radix(&atom[x..], radix) {
        Ok(n) => Number(n),
        Err(_) => Symbol(atom.to_string()),
    }
}

fn parse_exp(iter: &mut Peekable<Iter<String>>) -> Result<Exp, String> {
    let token = iter.next();

    if let Some(x) = token { 
        if *x == "(" {
            let mut l = List(vec![]);

            while let Some(x) = iter.peek() {
                if **x == ")" { break; }
                match parse_exp(iter) {
                    Ok(x) => l.push(x),
                    Err(_) => {},
                }
            }
            iter.next();
            Ok(l)
        } else if *x == ")" {
            Err("unexpected closing parenthesis".to_string())
        } else {
            Ok(parse_atom(x))
        }
    } else {
        Err("wut".to_string())
    }
}

fn eval_top_level(ast: &Exp, env: &mut Env) -> Vec<Exp> {
    let mut a = vec![];
    if let List(l) = ast {
        for exp in l {
            a.push(eval(&exp, env));
        }
    }

    a
}

fn eval_builtin(first: &Exp, rest: &Exp, env: &mut Env) -> Option<Exp> {
    match first {
        Symbol(x) => match x.as_ref() {
            "def" => {
                if let List(l) = rest {
                    if let Symbol(z) = rest.first() {
                        match l.len() {
                            1 => { env.defs.insert(z.clone(), Number(env.pc)); },
                            2 => {
                                let val = eval(&rest.rest().first(), env);
                                env.defs.insert(z.clone(), val);
                            },

                            _ => {
                                let val = eval(&rest.rest(), env);
                                env.defs.insert(z.clone(), val);
                            },
                        }
                    }

                    Some(rest.first().clone())
                } else {
                    None
                }
            },
            _ => { None },
        },
        _ => { None },
    }
}

fn eval(ast: &Exp, env: &mut Env) -> Exp {
    match ast {
        List(_) => {
            let first = ast.first();
            let rest = ast.rest();
            match eval_builtin(&first, &rest, env) {
                Some(x) => {
                    x
                },
                
                None => { // not builtin proc
                    let first = eval(&ast.first(), env);
                    let rest = if let List(l) = ast.rest() {
                        l.iter().map(|a| eval(&a, env)).collect()
                    } else {
                        vec![Symbol("aaa".to_string())]
                    };
                    
                    if let Proc(p) = first {
                        p(&rest)
                    } else {
                        Symbol("aa".to_string())
                    }
                },
            }
        },

        Symbol(s) => {
            if let Some(e) = env.defs.get(s) {
                (*e).clone()
            } else {
                Symbol("unknown".to_string())
            }
        },

        Number(_) => {
            ast.clone()
        },
        
        _ => { ast.clone() },
    }
}

fn main() {
    let code = include_str!("test2.ch8");

    let tokens = tokenize(code);
    //println!("{:?}",tokens);
    let mut iter = tokens.iter().peekable();

    let ast = parse(&mut iter);
    //ast.first().print();

    let mut env = Env {
        defs: {
            let mut builtin = HashMap::new();
            // does this work?????????
            builtin.insert("v0".to_string(), Number(0x0));
            builtin.insert("v1".to_string(), Number(0x1));
            builtin.insert("v2".to_string(), Number(0x2));
            builtin.insert("v3".to_string(), Number(0x3));
            builtin.insert("v4".to_string(), Number(0x4));
            builtin.insert("v5".to_string(), Number(0x5));
            builtin.insert("v6".to_string(), Number(0x6));
            builtin.insert("v7".to_string(), Number(0x7));
            builtin.insert("v8".to_string(), Number(0x8));
            builtin.insert("v9".to_string(), Number(0x9));
            builtin.insert("va".to_string(), Number(0xa));
            builtin.insert("vb".to_string(), Number(0xb));
            builtin.insert("vc".to_string(), Number(0xc));
            builtin.insert("vd".to_string(), Number(0xd));
            builtin.insert("ve".to_string(), Number(0xe));
            builtin.insert("vf".to_string(), Number(0xf));

            builtin.insert("i".to_string(), Symbol("i".to_string()));
            builtin.insert("key".to_string(), Symbol("key".to_string()));
            builtin.insert("dt".to_string(), Symbol("dt".to_string()));
            builtin.insert("st".to_string(), Symbol("st".to_string()));

            builtin.insert("+".to_string(), Proc(|args: &[Exp]| -> Exp {
                Number(args.iter().map(|x| match x {
                    Number(n) => *n,
                    _ => 0isize, //gay
                }).collect::<Vec<isize>>().iter().fold(0, |sum, a| sum + a))}));

            builtin
        },
        pc: 0x200,
    };

    eval_top_level(&ast, &mut env).iter().for_each(|x| x.print());

}
