use std::{process, thread, time};

use crate::{
    data_registers::DataRegisters, display::Display, font::*, keypad::KeyPad, stack::Stack,
    timer::Timer,
};

const MEMORY_BYTES: usize = 4096;
const FREQUENCY_HZ: usize = 700;
const SLEEP_US: u64 = (1000 * 1000 / FREQUENCY_HZ) as u64;

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
        self.i_reg = ((self.memory[pc] as u16) << 8) | self.memory[pc + 1] as u16;
        self.pc += 2;
    }

    fn execute(&self) {
        match self.i_reg {
            0 => println!("Executing 0."),
            _ => {
                eprintln!("[Error]: Unknown instruction.");
                process::exit(1);
            }
        }
    }
}
