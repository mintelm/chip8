use crate::{
    data_registers::DataRegisters, display::Display, font::*, keypad::KeyPad, stack::Stack,
    timer::Timer,
};

const INSTRUCTIONS_PER_SECOND: usize = 700;
const MEMORY_BYTES: usize = 4096;

pub struct Emulator {
    display: Display,
    memory: [u8; MEMORY_BYTES],
    pc: u16,
    stack: Stack,
    d_regs: DataRegisters,
    i_reg: u16,
    delay_t: Timer,
    sound_t: Timer,
    keypad: KeyPad,
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            display: Display::new(),
            pc: 0,
            d_regs: DataRegisters::new(),
            memory: {
                let mut data = [0; MEMORY_BYTES];
                data[FONT_START_ADDR..FONT_END_ADDR].copy_from_slice(&FONT_RAW_DATA);

                data
            },
            i_reg: 0,
            stack: Stack::new(),
            keypad: KeyPad::new(),
            delay_t: Timer::new(),
            sound_t: Timer::new(),
        }
    }

    pub fn run(&mut self) {}

    fn fetch(&mut self) -> u16 {
        0
    }

    fn execute(&self, _instruction: u16) {}
}
