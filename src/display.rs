pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

pub struct Display {
    pixels: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT],
}

impl Display {
    pub fn new() -> Display {
        Display {
            pixels: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
        }
    }
}
