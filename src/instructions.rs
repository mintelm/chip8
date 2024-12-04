pub enum Instruction {
    CLS,
    JP,
    LD,
    ADDC,
    LDI,
    DSPR,
    NOOP,
}

pub fn instruction_matcher(instruction: u16) -> Instruction {
    match instruction & 0xF000 {
        0x0000 => match instruction & 0x0FFF {
            0x00E0 => Instruction::CLS,
            _ => Instruction::NOOP,
        },
        0x1000 => Instruction::JP,
        0x6000 => Instruction::LD,
        0x7000 => Instruction::ADDC,
        0xA000 => Instruction::LDI,
        0xD000 => Instruction::DSPR,
        _ => Instruction::NOOP,
    }
}
