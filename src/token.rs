#[allow(non_camel_case_types)]

pub enum Token<'a> {

    COLON,

    /* Maybe later
    LEFT_CURLY, RIGHT_CURLY,
    OPERATOR(Operator),
    PLUS, MINUS, SLASH,
    STAR, LEFT_PAREN, RIGHT_PAREN
     */
  
    MNEMONIC(Mnemonic),

    LABEL(&'a str), // [start index, end index]
    NUM(usize),

    REGISTER(Register),
    ALIAS,
}

pub enum Mnemonic {
    CLEAR, END, JUMP, JUMP0, BEGIN, NEQ,
    EQ, SET, ADD, OR, AND, XOR, SUB, SHR,
    SUBR, SHL, RAND, DRAW, WRITEBCD, WRITE,
    READ
}

pub enum Register {
       V(usize), I, DT, ST, KEY
}
