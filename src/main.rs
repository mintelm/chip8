use emulator::Emulator;

mod emulator;
mod font;
mod keypad;
mod screen;
mod stack;
mod timer;

fn main() {
    let mut emulator = Emulator::new();
    emulator.load_rom("./roms/ibm.ch8");
    emulator.run();
}
