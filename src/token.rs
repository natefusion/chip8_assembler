use {Keyword::*, Category::*};

pub struct Token {
    pub category: Category,
    pub raw: &'static str,
    pub line: usize,
    pub ch: usize,
}


pub enum Keyword {
    V, I, Dt, St, Key,
    
    Clear, Return, Jump, Jump0, Call, Neq,
    Eq, Set, Add, Or, And, Xor, Sub, Shr,
    Subr, Shl, Rand, Draw, Writebcd, Write,
    Read,
    
    Colon, Define,
}

pub enum Category {
    Function(Keyword),
    Definition(Keyword),
    Register(Keyword),
    Number,
    Identifier,
}

impl Token {
    pub fn new(raw: &'static str, line: usize, ch: usize) -> Self {
        Token { category: Self::tokenize(raw), raw, line, ch }
    }

    fn tokenize(raw: &str) -> Category {
        match raw.chars().nth(0).unwrap() {
            '0'..='9' => Number,
            
            'v' => Register(V),
            'i' => Register(I),
            ':' => Definition(Colon),
            
            'a'..='z' |
            'A'..='Z' => match raw {
                "clear"    => Function(Clear),
                "return"   => Function(Return),
                "jump"     => Function(Jump),
                "jump0"    => Function(Jump0),
                "call"     => Function(Call),
                "neq"      => Function(Neq),
                "eq"       => Function(Eq),
                "set"      => Function(Set),
                "add"      => Function(Add),
                "or"       => Function(Or),
                "and"      => Function(And),
                "xor"      => Function(Xor),
                "sub"      => Function(Sub),
                "shr"      => Function(Shr),
                "subr"     => Function(Subr),
                "shl"      => Function(Shl),
                "rand"     => Function(Rand),
                "draw"     => Function(Draw),
                "writebcd" => Function(Writebcd),
                "write"    => Function(Write),
                "read"     => Function(Read),
                
                "define" => Definition(Define),

                "dt"  => Register(Dt),
                "st"  => Register(St),
                "key" => Register(Key),

                _ => Identifier,
            }

            _ => Identifier,
        }
    }
}
