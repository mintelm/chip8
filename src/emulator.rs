use std::{process, thread, time};

use crate::{font::*, keypad::KeyPad, screen::Screen, stack::Stack, timer::Timer};

/// x is the second nibble. Used to lookup registers.
macro_rules! get_x {
    ($val:expr) => {
        (($val & 0x0F00) >> 8) as usize
    };
}

/// y is the third nibble. Used to lookup registers.
macro_rules! get_y {
    ($val:expr) => {
        (($val & 0x00F0) >> 4) as usize
    };
}

/// n is the fourth nibble. A 4 bit number.
macro_rules! get_n {
    ($val:expr) => {
        $val & 0x000F
    };
}

/// nn is the second byte. A 8 bit number.
macro_rules! get_nn {
    ($val:expr) => {
        ($val & 0x00FF) as u8
    };
}

/// nnn is the second nibble and the second byte. Typically a 12 bit memory address.
macro_rules! get_nnn {
    ($val:expr) => {
        $val & 0x0FFF
    };
}

const MEMORY_BYTES: usize = 4096;
const FREQUENCY_HZ: usize = 700;
const SLEEP_US: u64 = (1000 * 1000 / FREQUENCY_HZ) as u64;
const REG_COUNT: usize = 16;

enum Instruction {
    CLS,
    RET,
    JP,
    CALL,
    LD,
    ADD,
    LDI,
    DSPR,
    NOOP,
}

pub struct Emulator {
    screen: Screen,
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
            screen: Screen::new(),
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

    pub fn load_rom(&mut self, file: &str) {
        let rom = std::fs::read(&file).unwrap();
        let begin = self.pc as usize;
        let end = self.pc as usize + rom.len();
        self.memory[begin..end].copy_from_slice(&rom);
    }

    pub fn run(&mut self) {
        loop {
            self.fetch();
            self.execute();
            self.screen.update();
            thread::sleep(time::Duration::from_micros(SLEEP_US));
        }
    }

    fn get_instruction(&self) -> Instruction {
        match self.instruction & 0xF000 {
            0x0000 => match self.instruction {
                0x00E0 => Instruction::CLS,
                0x00EE => Instruction::RET,
                _ => Instruction::NOOP,
            },
            0x1000 => Instruction::JP,
            0x2000 => Instruction::CALL,
            0x6000 => Instruction::LD,
            0x7000 => Instruction::ADD,
            0xA000 => Instruction::LDI,
            0xD000 => Instruction::DSPR,
            _ => Instruction::NOOP,
        }
    }

    fn fetch(&mut self) {
        let pc: usize = self.pc as usize;
        self.instruction = ((self.memory[pc] as u16) << 8) | self.memory[pc + 1] as u16;
        self.pc += 2;
    }

    fn execute(&mut self) {
        match self.get_instruction() {
            Instruction::CLS => self.clear_screen(),
            Instruction::RET => self.return_subroutine(),
            Instruction::JP => self.jump(),
            Instruction::CALL => self.call_subroutine(),
            Instruction::LD => self.load(),
            Instruction::ADD => self.add(),
            Instruction::LDI => self.load_i(),
            Instruction::DSPR => self.draw_sprite(),
            Instruction::NOOP => {
                eprintln!("[Error]: Unknown instruction.");
                process::exit(1);
            }
        }
    }

    /// Clear the screen.
    fn clear_screen(&mut self) {
        self.screen.clear();
    }

    // Return from a subroutine.
    fn return_subroutine(&mut self) {
        self.pc = self.stack.pop();
    }

    /// Jump to nnn.
    fn jump(&mut self) {
        self.pc = get_nnn!(self.instruction);
    }

    /// Call subroutine from nnn.
    fn call_subroutine(&mut self) {
        self.stack.push(self.pc);
        self.pc = get_nnn!(self.instruction);
    }

    /// Load nn into register x.
    fn load(&mut self) {
        let x = get_x!(self.instruction);
        let nn = get_nn!(self.instruction);

        self.d_regs[x] = nn;
    }

    /// Add nn to register x.
    fn add(&mut self) {
        let x = get_x!(self.instruction);
        let nn = get_nn!(self.instruction);

        self.d_regs[x] += nn;
    }

    /// Load nnn into register i.
    fn load_i(&mut self) {
        self.i_reg = get_nnn!(self.instruction);
    }

    /// Draw n pixels tall sprite from register i at coordinates of register x, register y.
    fn draw_sprite(&mut self) {
        let x = get_x!(self.instruction);
        let y = get_y!(self.instruction);
        let n = get_n!(self.instruction);
        let begin = self.i_reg as usize;
        let end = (self.i_reg + n) as usize;
        let sprite = &self.memory[begin..end];

        self.screen
            .draw_sprite(self.d_regs[x].into(), self.d_regs[y].into(), &sprite);
    }
}
