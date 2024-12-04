use std::{process, thread, time};

use crate::{
    display::Display,
    font::*,
    instructions::{instruction_matcher, Instruction},
    keypad::KeyPad,
    stack::Stack,
    timer::Timer,
};

const MEMORY_BYTES: usize = 4096;
const FREQUENCY_HZ: usize = 700;
const SLEEP_US: u64 = (1000 * 1000 / FREQUENCY_HZ) as u64;
const REG_COUNT: usize = 16;

pub struct Emulator {
    display: Display,
    memory: [u8; MEMORY_BYTES],
    pc: u16,
    stack: Stack,
    d_regs: [u8; REG_COUNT],
    i_reg: u16,
    instruction: u16,
    delay_t: Timer,
    sound_t: Timer,
    keypad: KeyPad,
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            display: Display::new(),
            pc: 0x200, // Typical chip8 program start.
            d_regs: [0; REG_COUNT],
            memory: {
                let mut data = [0; MEMORY_BYTES];
                data[FONT_START_ADDR..FONT_END_ADDR].copy_from_slice(&FONT_RAW_DATA);

                data
            },
            i_reg: 0,
            instruction: 0,
            stack: Stack::new(),
            keypad: KeyPad::new(),
            delay_t: Timer::new(),
            sound_t: Timer::new(),
        }
    }

    pub fn load_rom(&mut self, file: String) {
        let rom = std::fs::read("./roms/".to_owned() + &file).unwrap();
        let begin = self.pc as usize;
        let end = self.pc as usize + rom.len();
        self.memory[begin..end].copy_from_slice(&rom);
    }

    pub fn run(&mut self) {
        loop {
            self.fetch();
            self.execute();
            self.display.update();
            thread::sleep(time::Duration::from_micros(SLEEP_US));
        }
    }

    fn fetch(&mut self) {
        let pc: usize = self.pc as usize;
        self.instruction = ((self.memory[pc] as u16) << 8) | self.memory[pc + 1] as u16;
        self.pc += 2;
    }

    fn execute(&mut self) {
        match instruction_matcher(self.instruction) {
            Instruction::CLS => self.clear_screen(),
            Instruction::JP => self.jump(),
            Instruction::LD => self.load(),
            Instruction::ADDC => self.add(),
            Instruction::LDI => self.load_i(),
            Instruction::DSPR => self.display_sprite(),
            Instruction::NOOP => {
                eprintln!("[Error]: Unknown instruction.");
                process::exit(1);
            }
        }
    }

    /// Clear the display.
    fn clear_screen(&mut self) {
        self.display.clear();
    }

    /// Jump to address.
    fn jump(&mut self) {
        // Address is in bytes 2-4.
        self.pc = self.instruction & 0x0FFF;
    }

    /// Load value into register.
    fn load(&mut self) {
        // Register index is in byte 2.
        let idx = ((self.instruction & 0x0F00) >> 8) as usize;
        // Value is in bytes 3-4.
        let value = (self.instruction & 0x00FF) as u8;

        self.d_regs[idx] = value;
    }

    /// Add value to the value of register.
    fn add(&mut self) {
        // Register index is in byte 2.
        let idx = ((self.instruction & 0x0F00) >> 8) as usize;
        // Value is in bytes 3-4.
        let value = (self.instruction & 0x00FF) as u8;

        self.d_regs[idx] += value;
    }

    /// Load value into i register.
    fn load_i(&mut self) {
        // Value is in bytes 2-4.
        self.i_reg = self.instruction & 0x0FFF;
    }

    /// Display sprite.
    fn display_sprite(&mut self) {
        let x_idx = ((self.instruction & 0x0F00) >> 8) as usize;
        let y_idx = ((self.instruction & 0x00F0) >> 4) as usize;
        let n = self.instruction & 0x000F;
        let begin = (self.i_reg & 0x0FFF) as usize;
        let end = ((self.i_reg & 0x0FFF) + n) as usize;
        let sprite = &self.memory[begin..end];

        self.display.draw_sprite(
            self.d_regs[x_idx].into(),
            self.d_regs[y_idx].into(),
            &sprite,
        );
    }
}
