use emulator::Emulator;

mod display;
mod emulator;
mod font;
mod instructions;
mod keypad;
mod stack;
mod timer;

fn main() {
    let mut emulator = Emulator::new();
    emulator.load_rom("ibm.ch8".to_owned());
    emulator.run();
}
