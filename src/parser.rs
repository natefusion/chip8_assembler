use crate::tokens::{Mnemonic, Mnemonic::*, Register, Register::*};

pub fn parse(mnemonic: &Mnemonic, registers: &[Register], arguments: &[usize]) -> Option<Vec<u8>> {
    /*                  v- number of arguments
     * opcode_info: 0x482
     *                ^^- first argument is shifted 8 bits to the left,
     *                ^-- second argument is shifted 4 bits to the left
     *
     * The arguments are shifted so that they can be bitwise-ored into opcode_shell with ease
     */
    let mut register = registers.iter();
    let (mut opcode_shell, opcode_info) = match mnemonic {
        Eq => {
            match (register.next(), register.next()) {
                (Some(V), Some(V)) => (0x9000, 0x482),
                (Some(V),   None)  => (0x4000, 0x82),
                (Some(Key), None)  => (0xE0A1, 0x81),
                _ => (0xF, 0xF) }},

        Neq => {
            match (register.next(), register.next()) {
                (Some(Key), None)   => (0xE09E, 0x81),
                (Some(V),   Some(V)) => (0x5000, 0x482),
                (Some(V),   None)    => (0x3000, 0x82),
                _ => (0xF, 0xF) }},

        Set => {
            match (register.next(), register.next()) {
                (Some(V),  None)      => (0x6000, 0x82),
                (Some(V),  Some(V))   => (0x8000, 0x482),
                (Some(I),  None)      => (0xA000, 0x1),
                (Some(V),  Some(Dt))  => (0xF007, 0x81),
                (Some(Dt), Some(V))   => (0xF015, 0x81),
                (Some(V),  Some(St))  => (0xF018, 0x81),
                (Some(I),  Some(V))   => (0xF029, 0x81),
                (Some(V),  Some(Key)) => (0xF00A, 0x81),
                _ => (0xF, 0xF) }},

        Add => {
            match (register.next(), register.next()) {
                (Some(V), None)    => (0x7000, 0x82),
                (Some(V), Some(V)) => (0x8004, 0x482),
                (Some(I), Some(V)) => (0xF01E, 0x81),
                _ => (0xF, 0xF) }},

        Clear    => (0x00E0, 0x0),
        End      => (0x00EE, 0x0),
        Begin    => (0x2000, 0x1),
        Or       => (0x8001, 0x482),
        And      => (0x8002, 0x482),
        Xor      => (0x8003, 0x482),
        Sub      => (0x8005, 0x482),
        Shr      => (0x8006, 0x482),
        Subr     => (0x8007, 0x482),
        Shl      => (0x800E, 0x482),
        Rand     => (0xC000, 0x82),
        Draw     => (0xD000, 0x483),
        Writebcd => (0xF033, 0x81),
        Write    => (0xF055, 0x81),
        Read     => (0xF065, 0x81),
        Jump     => (0x1000, 0x1),
        Jump0    => (0xB000, 0x1)
    };

    if arguments.len() != opcode_info & 0xF {
        return None;
    }

    for (i, val) in arguments.iter().enumerate() {
        let shift = (opcode_info >> (4 + (i * 4))) & 0xF;
        let max = if shift == 0 { 0xFFFF >> ((opcode_info & 0xF) * 4) } else { 0xF };

        if *val > max {
            return None;
        }

        opcode_shell |= val << shift;
    }

    Some(vec![((opcode_shell & 0xFF00) >> 8) as u8, (opcode_shell & 0x00FF) as u8])
}
