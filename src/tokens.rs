#[derive(Clone)]
pub enum Mnemonic {
    CLEAR, END, JUMP, JUMP0, BEGIN, NEQ,
    EQ, SET, ADD, OR, AND, XOR, SUB, SHR,
    SUBR, SHL, RAND, DRAW, WRITEBCD, WRITE,
    READ, UNKNOWN
}

#[derive(Clone)]
pub enum Register {
    V, I, DT, ST, KEY, UNKNOWN
}
