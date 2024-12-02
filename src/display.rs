use minifb::{Window, WindowOptions};

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

pub struct Display {
    buffer: [u32; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    window: Window,
}

impl Display {
    pub fn new() -> Display {
        Display {
            buffer: [0; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            window: Window::new(
                "Chip8 Emulator",
                DISPLAY_WIDTH,
                DISPLAY_HEIGHT,
                WindowOptions::default(),
            )
            .expect("Unable to create the window."),
        }
    }

    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, DISPLAY_WIDTH, DISPLAY_HEIGHT)
            .unwrap();
    }
}
