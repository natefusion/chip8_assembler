use std::{slice::Iter, iter::Peekable, collections::HashMap, fs::File, io::Write, fs, process, env};
use Exp::*;
use Token::*;

#[allow(non_camel_case_types, non_snake_case)]
#[derive(PartialEq,Eq,Hash,Clone)]
enum Token {
    V(u8),
    I, KEY, ST, DT,

    A_ADD, A_SUB, A_DIV, A_MUL,

    CLEAR, RETURN, JUMP, JUMP0, CALL, NEQ,
    EQ, SET, ADD, OR, AND, XOR, SUB, SHR,
    SUBR, SHL, RAND, DRAW, BCD, WRITE,
    READ,

    DEF, LOOP, INCLUDE, MAIN,

    User(String),
}

impl Token {
    fn new(raw: &str) -> Self {
        match raw {
            "clear"    => CLEAR,
            "return"   => RETURN,
            "jump"     => JUMP,
            "jump0"    => JUMP0,
            "call"     => CALL,
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
            "bcd"      => BCD,
            "write"    => WRITE,
            "read"     => READ,
            "v0" => V(0),
            "v1" => V(1),
            "v2" => V(2),
            "v3" => V(3),
            "v4" => V(4),
            "v5" => V(5),
            "v6" => V(6),
            "v7" => V(7),
            "v8" => V(8),
            "v9" => V(9),
            "va" => V(0xA),
            "vb" => V(0xB),
            "vc" => V(0xC),
            "vd" => V(0xD),
            "ve" => V(0xE),
            "vf" => V(0xF),
            "i"  => I,
            "dt" => DT,
            "st" => ST,
            "key" => KEY,
            "def" => DEF,
            "+" => A_ADD,
            "-" => A_SUB,
            "*" => A_MUL,
            "/" => A_DIV,
            "loop" => LOOP,
            "include" => INCLUDE,
            "main" => MAIN,
            
            _ => User(raw.to_string()),
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            CLEAR  => "Clear",
            RETURN => "Return",
            JUMP   => "Jump",
            JUMP0  => "Jump0",
            CALL   => "Call",
            NEQ    => "Neq",
            EQ     => "Eq",
            SET    => "Set",
            ADD    => "Add",
            OR     => "Or",
            AND    => "And",
            XOR    => "Xor",
            SUB    => "Sub",
            SHR    => "Shr",
            SUBR   => "Subr",
            SHL    => "Shl",
            RAND   => "Rand",
            DRAW   => "Draw",
            BCD    => "bcd",
            WRITE  => "Write",
            READ   => "Read",
            V(_) => "V",
            I => "I",
            DT => "DT",
            ST => "ST",
            KEY => "Key",
            DEF => "def",
            A_ADD => "+",
            A_MUL => "*",
            A_SUB => "-",
            A_DIV => "/",
            LOOP => "loop",
            INCLUDE => "include",
            MAIN => "main",
            User(x) => x,
        })
    }
}

#[derive(Clone)]
enum Exp {
    Symbol(Token),
    Number(u16),
    Proc(fn(&[Exp], &mut Env) -> Exp),
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
                print!("Number({}) ", x);
                //print!("Atom({}) ", x);
            },

            Symbol(x) => {
                print!("Symbol({}) ", x);
            },

            Proc(_) => {
                print!("Proc ");
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
    defs: HashMap<Token, Exp>,
    pc: u16,
    main: u16, //holds 'main' label
}

fn tokenize(code: &str) -> Vec<String> {
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

    match u16::from_str_radix(&atom[x..], radix) {
        Ok(n) => Number(n),
        Err(_) => Symbol(Token::new(atom)),
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
    let mut a = vec![emit_op(JUMP, &[Number(env.main)], env)];
    if let List(l) = ast {
        for exp in l {
            a.push(eval(&exp, env));
        }
    }

    a[0] = emit_op(JUMP, &[Number(env.main)], env);
    a
}

fn eval_args(args: &[Exp], env: &mut Env) -> Vec<Exp> {
    args.iter().map(|a| eval(&a, env)).collect()
}

fn eval_partial(args: &[Exp], env: &mut Env) -> Vec<Exp> {
    // There are times to eval the 'v' registers and times to not
    args
        .iter()
        .map(|a| if !matches!(a, Symbol(V(_))) {
            eval(&a, env)
        } else {
            a.clone()
        })
        .collect()
}

fn emit_op(proc: Token, args: &[Exp], env: &mut Env) -> Exp {
    env.pc += 2;
    let (mut shell, info) = match (proc, &args[..]) {
        (EQ, [Symbol(V(_)), Symbol(V(_))]) => (0x9000, 0x482),
        (EQ, [Symbol(V(_)), Number(_)]) |
        (EQ, [Number(_), Symbol(V(_))]) => (0x4000, 0x82),
        (EQ, [Symbol(V(_)), Symbol(KEY)]) |
        (EQ, [Symbol(KEY), Symbol(V(_))]) => (0xE0A1, 0x81),
        
        (NEQ, [Symbol(V(_)), Symbol(KEY)]) |
        (NEQ, [Symbol(KEY), Symbol(V(_))]) => (0xE09E, 0x81),
        (NEQ, [Symbol(V(_)), Symbol(V(_))]) => (0x5000, 0x482),
        (NEQ, [Symbol(V(_)), Number(_)]) => (0x3000, 0x82),

        (SET, [Symbol(V(_)), Number(_)]) => (0x6000, 0x82),
        (SET, [Symbol(V(_)), Symbol(V(_))]) => (0x8000, 0x482),
        (SET, [Symbol(I), Number(_)]) => (0xA000, 0x1),
        (SET, [Symbol(V(_)), Symbol(DT )]) => (0xF007, 0x81),
        (SET, [Symbol(DT), Symbol(V(_))]) => (0xF015, 0x81),
        (SET, [Symbol(V(_)), Symbol(ST)]) => (0xF018, 0x81),
        (SET, [Symbol(I), Symbol(V(_))]) => (0xF029, 0x81),
        (SET, [Symbol(V(_)), Symbol(KEY)]) => (0xF00A, 0x81),

        (ADD, [Symbol(V(_)), Number(_)]) => (0x7000, 0x82),
        (ADD, [Symbol(V(_)), Symbol(V(_))]) => (0x8004, 0x482),
        (ADD, [Symbol(I), Symbol(V(_))]) => (0xF01E, 0x81),

        (OR, [Symbol(V(_)), Symbol(V(_))]) => (0x8001, 0x482),
        (AND, [Symbol(V(_)), Symbol(V(_))]) => (0x8002, 0x482),
        (XOR, [Symbol(V(_)), Symbol(V(_))]) => (0x8003, 0x482),
        (SUB, [Symbol(V(_)), Symbol(V(_))]) => (0x8005, 0x482),
        (SHR, [Symbol(V(_)), Symbol(V(_))]) => (0x8006, 0x482),
        (SUBR, [Symbol(V(_)), Symbol(V(_))]) => (0x8007, 0x482),
        (SHL, [Symbol(V(_)), Symbol(V(_))]) => (0x800E, 0x482),

        (RAND, [Symbol(V(_)), Number(_)]) => (0xC000, 0x82),
        (DRAW, [Symbol(V(_)), Symbol(V(_)), Number(_)]) => (0xD000, 0x483),

        (BCD,    [Symbol(V(_))]) => (0xF033, 0x81),
        (WRITE,  [Symbol(V(_))]) => (0xF055, 0x81),
        (READ,   [Symbol(V(_))]) => (0xF065, 0x81),

        (CLEAR, []) => (0x00E0, 0x0),
        (RETURN, []) => (0x00EE, 0x0),
        (CALL, [Symbol(V(_))]) => (0x2000, 0x1),
        (JUMP, [Number(_)]) => (0x1000, 0x1),
        (JUMP0, [Number(_)]) => (0xB000, 0x1),
        
        _ => (0,0),
    };
    
    let args_shift = info >> 4;

    // i variable gets messed up if the elements are not numbers
    let args = eval_args(&args, env).iter().filter(|x| matches!(x, Symbol(V(_)) | Number(_))).map(|x| x.clone()).collect::<Vec<Exp>>();
    for (i, x) in args.iter().enumerate() {
        if let Number(val) = x {
            let val = *val as u16;
            let shift = (args_shift >> (i << 2)) & 0xF;

            shell |= val << shift;
        }
    }
    List(vec![Number((shell as u16 & 0xFF00) >> 8),
              Number(shell as u16 & 0xFF)])
}

fn eval(ast: &Exp, env: &mut Env) -> Exp {
    match ast {
        List(_) => {
            let first = eval(&ast.first(), env);
            let rest = if let List(l) = ast.rest() {
                l
            } else {
                vec![ast.rest()]
            };

            if let Proc(p) = first {
                p(&rest, env)
            } else {
                let mut r = eval_args(&rest, env);
                r.insert(0, first);
                env.pc += r.len() as u16;
                List(r)
            }
        },

        Symbol(s) => {
            if let Some(e) = env.defs.get(s) {
                (*e).clone()
            } else {
                // unknown symbols eval to themselves???
                ast.clone()
            }
        },

        Number(_) => {
            ast.clone()
        },
        
        _ => { ast.clone() },
    }
}

fn compile(opcodes: &[Exp]) -> Vec<u8> {
    let mut binary = vec![];
    for op in opcodes {
        match op {
            Number(n) => { binary.push(*n as u8); }
            List(l) => { binary.append(&mut compile(&l)); }
            _ => { op.print(); },
        }
    }

    binary
}

fn load(path: Option<String>) -> (String, String) {
    match path {
        Some(x) => match fs::read_to_string(&x) {
            Ok(file) => (file.trim().to_string(), x),
            Err(_) => { eprintln!("Cannot read file"); process::exit(1); }},
        None => { eprintln!("Please enter a file"); process::exit(1); }}
}

fn main() {
    let (code, filename) = load(env::args().nth(1));

    let tokens = tokenize(&code);
    //println!("{:?}",tokens);
    let mut iter = tokens.iter().peekable();

    let ast = parse(&mut iter);
    //ast.print();

    let mut env = Env {
        defs: {
            let mut builtin = HashMap::new();
            // does this work????????? no, it does not
            builtin.insert(V(0), Number(0x0));
            builtin.insert(V(1), Number(0x1));
            builtin.insert(V(2), Number(0x2));
            builtin.insert(V(3), Number(0x3));
            builtin.insert(V(4), Number(0x4));
            builtin.insert(V(5), Number(0x5));
            builtin.insert(V(6), Number(0x6));
            builtin.insert(V(7), Number(0x7));
            builtin.insert(V(8), Number(0x8));
            builtin.insert(V(9), Number(0x9));
            builtin.insert(V(0xA), Number(0xa));
            builtin.insert(V(0xB), Number(0xb));
            builtin.insert(V(0xC), Number(0xc));
            builtin.insert(V(0xD), Number(0xd));
            builtin.insert(V(0xE), Number(0xe));
            builtin.insert(V(0xF), Number(0xf));

            builtin.insert(I, Symbol(I));
            builtin.insert(KEY, Symbol(KEY));
            builtin.insert(DT, Symbol(DT));
            builtin.insert(ST, Symbol(ST));
            builtin.insert(MAIN, Symbol(MAIN));

            builtin.insert(A_ADD, Proc(|args: &[Exp], env: &mut Env| -> Exp {
                let args = eval_partial(&args, env);
                Number(eval_args(&args, env).iter().map(|x| match x {
                    Number(n) => *n,
                    _ => 0, //gay
                }).collect::<Vec<u16>>().iter().fold(0, |sum, a| sum + a))}));

            builtin.insert(DEF, Proc(|args: &[Exp], env: &mut Env| -> Exp {
                let args = eval_partial(&args, env);
                if let Symbol(key) = args[0].clone() {
                    let val = match args.len() {
                        1 => {
                            if matches!(eval(&args[0], env), Symbol(MAIN)) {
                                env.main = env.pc;
                            }
                            
                            Number(env.pc)
                        },
                        
                        2 => args[1].first(),
                        _ => Symbol(Token::new("roh ruh")),
                    };

                    env.defs.insert(key, val.clone());
                    Symbol(DEF)
                } else {
                    Symbol(Token::new("ruh roh"))
                }
            }));

            builtin.insert(LOOP, Proc(|args: &[Exp], env: &mut Env| -> Exp {
                let label = env.pc ; // catch pc before it changes
                let mut args = args.to_vec();
                args.push(emit_op(JUMP, &[Number(label)], env));
                List(eval_partial(&args, env))
            }));

            builtin.insert(EQ, Proc(|args: &[Exp], env: &mut Env| -> Exp {
                let args = eval_partial(&args, env);
                emit_op(EQ, &args, env)
            }));

            builtin.insert(NEQ, Proc(|args: &[Exp], env: &mut Env| -> Exp {
                let args = eval_partial(&args, env);
                emit_op(NEQ, &args, env)
            }));

            builtin.insert(SET, Proc(|args: &[Exp], env: &mut Env| -> Exp {
                let args = eval_partial(&args, env);
                emit_op(SET, &args, env)
            }));

            builtin.insert(ADD, Proc(|args: &[Exp], env: &mut Env| -> Exp {
                let args = eval_partial(&args, env);
                emit_op(ADD, &args, env)
            }));

             builtin.insert(OR, Proc(|args: &[Exp], env: &mut Env| -> Exp {
                let args = eval_partial(&args, env);
                emit_op(OR, &args, env)
             }));

            builtin.insert(AND, Proc(|args: &[Exp], env: &mut Env| -> Exp {
                let args = eval_partial(&args, env);
                emit_op(AND, &args, env)
            }));

            builtin.insert(XOR, Proc(|args: &[Exp], env: &mut Env| -> Exp {
                let args = eval_partial(&args, env);
                emit_op(XOR, &args, env)
            }));

             builtin.insert(SUB, Proc(|args: &[Exp], env: &mut Env| -> Exp {
                let args = eval_partial(&args, env);
                emit_op(SUB, &args, env)
             }));

             builtin.insert(SHR, Proc(|args: &[Exp], env: &mut Env| -> Exp {
                let args = eval_partial(&args, env);
                emit_op(SHR, &args, env)
             }));

             builtin.insert(SUBR, Proc(|args: &[Exp], env: &mut Env| -> Exp {
                let args = eval_partial(&args, env);
                emit_op(SUBR, &args, env)
             }));

             builtin.insert(SHL, Proc(|args: &[Exp], env: &mut Env| -> Exp {
                let args = eval_partial(&args, env);
                emit_op(SHL, &args, env)
             }));

             builtin.insert(RAND, Proc(|args: &[Exp], env: &mut Env| -> Exp {
                let args = eval_partial(&args, env);
                emit_op(RAND, &args, env)
             }));

             builtin.insert(DRAW, Proc(|args: &[Exp], env: &mut Env| -> Exp {
                let args = eval_partial(&args, env);
                emit_op(DRAW, &args, env)
             }));

             builtin.insert(BCD, Proc(|args: &[Exp], env: &mut Env| -> Exp {
                let args = eval_partial(&args, env);
                emit_op(BCD, &args, env)
             }));

             builtin.insert(WRITE, Proc(|args: &[Exp], env: &mut Env| -> Exp {
                let args = eval_partial(&args, env);
                emit_op(WRITE, &args, env)
             }));

             builtin.insert(READ, Proc(|args: &[Exp], env: &mut Env| -> Exp {
                let args = eval_partial(&args, env);
                emit_op(READ, &args, env)
             }));

            builtin.insert(CLEAR, Proc(|args: &[Exp], env: &mut Env| -> Exp {
                let args = eval_partial(&args, env);
                emit_op(CLEAR, &args, env)
            }));

            builtin.insert(RETURN, Proc(|args: &[Exp], env: &mut Env| -> Exp {
                let args = eval_partial(&args, env);
                emit_op(RETURN, &args, env)
             }));


            builtin.insert(CALL, Proc(|args: &[Exp], env: &mut Env| -> Exp {
                let args = eval_partial(&args, env);
                emit_op(CALL, &args, env)
            }));

            builtin.insert(JUMP, Proc(|args: &[Exp], env: &mut Env| -> Exp {
                let args = eval_partial(&args, env);
                emit_op(JUMP, &args, env)
            }));

            builtin.insert(JUMP0, Proc(|args: &[Exp], env: &mut Env| -> Exp {
                let args = eval_partial(&args, env);
                emit_op(JUMP0, &args, env)
             }));
            
            builtin
        },
        pc: 0x200,
        main: 0x200,
    };

     //eval_top_level(&ast, &mut env).iter().for_each(|x| x.print());
    let opcodes = compile(&eval_top_level(&ast, &mut env));
    //println!("{:?}", opcodes);

    let mut buffer = File::create(filename.clone() + &String::from(".bin")).unwrap();
    buffer.write_all(&opcodes).unwrap();
}
