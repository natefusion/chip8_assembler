pub enum Mnemonic {
    Clear, End, Jump, Jump0, Begin, Neq,
    Eq, Set, Add, Or, And, Xor, Sub, Shr,
    Subr, Shl, Rand, Draw, Writebcd, Write,
    Read
}

pub enum Register {
    V, I, Dt, St, Key, Unknown
}
