use crate::token::{Mnemonic, Mnemonic::*, Register, Register::*};

pub fn parse(mnemonic: &Mnemonic, registers: &[&Register], arguments: &[usize]) -> Result<Vec<u8>, String> {
    /*                  v- number of arguments
     * opcode_info: 0x482
     *                ^^- first argument is shifted 8 bits to the left,
     *                ^-- second argument is shifted 4 bits to the left
     *
     * The arguments are shifted so that they can be bitwise-ored into opcode_shell with ease
     */

    // maybe remove all embedded match statements and instead to this:
    // match (mnemonic, registers.next(), registers.next(), registers.next())
    let mut register = registers.iter();
    let (mut opcode_shell, opcode_info) = match mnemonic {
        EQ => {
            match (register.next(), register.next()) {
                (Some(V(_)), Some(V(_))) => (0x9000, 0x482),
                (Some(V(_)), None)       => (0x4000, 0x82),
                (Some(V(_)), Some(KEY))  => (0xE0A1, 0x81),
                (Some(KEY),  Some(V(_))) => (0xE0A1, 0x81),
                _ => return Err(format!("Incorrect arguments given")) }},

        NEQ => {
            match (register.next(), register.next()) {
                (Some(V(_)), Some(KEY))  => (0xE09E, 0x81),
                (Some(KEY),  Some(V(_))) => (0xE09E, 0x81),
                (Some(V(_)), Some(V(_))) => (0x5000, 0x482),
                (Some(V(_)), None)       => (0x3000, 0x82),
                _ => return Err(format!("Incorrect arguments given")) }},

        SET => {
            match (register.next(), register.next()) {
                (Some(V(_)), None)       => (0x6000, 0x82),
                (Some(V(_)), Some(V(_))) => (0x8000, 0x482),
                (Some(I),    None)       => (0xA000, 0x1),
                (Some(V(_)), Some(DT))   => (0xF007, 0x81),
                (Some(DT),   Some(V(_))) => (0xF015, 0x81),
                (Some(V(_)), Some(ST))   => (0xF018, 0x81),
                (Some(I),    Some(V(_))) => (0xF029, 0x81),
                (Some(V(_)), Some(KEY))  => (0xF00A, 0x81),
                _ => return Err(format!("Incorrect arguments given")) }},

        ADD => {
            match (register.next(), register.next()) {
                (Some(V(_)), None)       => (0x7000, 0x82),
                (Some(V(_)), Some(V(_))) => (0x8004, 0x482),
                (Some(I),    Some(V(_))) => (0xF01E, 0x81),
                _ => return Err(format!("Incorrect arguments given")) }},

        // these all accept '1' as an argument where it should technically be a '%v1'. pls fix
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
        JUMP0    => (0xB000, 0x1)
    };

    if arguments.len() != opcode_info & 0xF {
        return Err(format!("Expected {} arguments, found {}", opcode_info & 0xF, arguments.len()));
    }

    for (i, val) in arguments.iter().enumerate() {
        let shift = (opcode_info >> (4 + (i * 4))) & 0xF;
        let max = if shift == 0 { 0xFFFF >> ((opcode_info & 0xF) * 4) } else { 0xF };

        if *val > max {
            return Err(format!("0x{:X} ({}) is bigger than the max of 0x{:X} ({})", val, val, max, max));
        }

        opcode_shell |= val << shift;
    }

    Ok(vec![((opcode_shell & 0xFF00) >> 8) as u8, (opcode_shell & 0x00FF) as u8])
}
