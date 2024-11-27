use emulator::Emulator;

mod data_registers;
mod display;
mod emulator;
mod font;
mod keypad;
mod stack;
mod timer;

fn main() {
    let mut emulator = Emulator::new();
    emulator.run();
}
