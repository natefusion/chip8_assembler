use crate::tokens::{Mnemonic, Mnemonic::*, Register, Register::*};

pub fn parse(mnemonic: &Mnemonic, registers: &Vec<Register>, arguments: &Vec<usize>) -> usize {
    /*                  v- number of arguments
     * opcode_info: 0x482
     *                ^^- first argument is shifted 8 bits to the left,
     *                ^-- second argument is shifted 4 bits to the left
     *
     * The arguments are shifted so that they can be bitwise-ored into opcode_shell with ease
     */
    let mut register = registers.iter();
    let (mut opcode_shell, opcode_info) = match mnemonic {
        EQ => {
            match (register.next(), register.next()) {
                (Some(V),   None) => (0x4000, 0x82),
                (Some(KEY), None) => (0xE0A1, 0x81),
                _ => (0xF, 0xF) }},

        NEQ => {
            match (register.next(), register.next()) {
                (Some(KEY), None)    => (0xE09E, 0x81),
                (Some(V),   Some(V)) => (0x5000, 0x482),
                (Some(V),   None)    => (0x3000, 0x82),
                _ => (0xF, 0xF) }},

        SET => {
            match (register.next(), register.next()) {
                (Some(V),  None)      => (0x6000, 0x82),
                (Some(V),  Some(V))   => (0x8000, 0x482),
                (Some(I),  None)      => (0xA000, 0x1),
                (Some(V),  Some(DT))  => (0xF007, 0x81),
                (Some(DT), Some(V))   => (0xF015, 0x81),
                (Some(V),  Some(ST))  => (0xF018, 0x81),
                (Some(I),  Some(V))   => (0xF029, 0x81),
                (Some(V),  Some(KEY)) => (0xF00A, 0x81),
                _ => (0xF, 0xF) }},

        ADD => {
            match (register.next(), register.next()) {
                (Some(V), None)    => (0x7000, 0x82),
                (Some(V), Some(V)) => (0x8004, 0x482),
                (Some(I), Some(V)) => (0xF01E, 0x81),
                _ => (0xF, 0xF) }},

        CLEAR    => (0x00E0, 0x0),
        END      => (0x00EE, 0x0),
        BEGIN    => (0x2000, 0x1),
        OR       => (0x8001, 0x482),
        AND      => (0x8002, 0x482),
        XOR      => (0x8003, 0x482),
        SUB      => (0x8005, 0x482),
        SHR      => (0x8006, 0x482),
        SUBR     => (0x8007, 0x482),
        SHL      => (0x800E, 0x482),
        RAND     => (0xC000, 0x82),
        DRAW     => (0xD000, 0x483),
        WRITEBCD => (0xF033, 0x81),
        WRITE    => (0xF055, 0x81),
        READ     => (0xF065, 0x81),
        JUMP     => (0x1000, 0x1),
        JUMP0    => (0xB000, 0x1),
        Mnemonic::UNKNOWN => return 0,

    };

    if arguments.len() != opcode_info & 0xF {
        return 0;
    }

    for (i, val) in arguments.iter().enumerate() {
        let shift = (opcode_info >> (4 + (i * 4))) & 0xF;
        let max = if shift == 0 { 0xFFFF >> ((opcode_info & 0xF) * 4) } else { 0xF };

        if *val > max {
            return 0;
        }

        opcode_shell |= val << shift;
    }

    opcode_shell
}
