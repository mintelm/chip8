use minifb::{Window, WindowOptions};

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

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

    pub fn draw_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) {
        let mut idx = (x % DISPLAY_WIDTH) + ((y % DISPLAY_HEIGHT) * DISPLAY_WIDTH);

        for byte in sprite.iter() {
            for i in 0..u8::BITS {
                let bit_mask = 0b1000_0000 >> i;
                if byte & bit_mask != 0 {
                    // Pixels are xor'd into the buffer.
                    if self.buffer[idx] ^ 1 == 1 {
                        self.buffer[idx] = u32::MAX;
                    } else {
                        self.buffer[idx] = 0;
                    }
                }
                idx += 1;
            }
            // Skip to next row for the next byte of the sprite.
            idx += DISPLAY_WIDTH - u8::BITS as usize;
        }
    }
}
