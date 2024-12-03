use minifb::{Window, WindowOptions};

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const WHITE: u32 = u32::MAX;
const BLACK: u32 = 0;

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
                        self.buffer[idx] = WHITE;
                    } else {
                        self.buffer[idx] = BLACK;
                    }
                }
                idx += 1;
            }
            // Skip to next row for the next byte of the sprite.
            idx += DISPLAY_WIDTH - u8::BITS as usize;
        }
    }

    pub fn clear(&mut self) {
        self.buffer = [0; DISPLAY_WIDTH * DISPLAY_HEIGHT];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_sprite(display: &Display, x: usize, y: usize, sprite: &[u8]) {
        let mut idx = (x % DISPLAY_WIDTH) + ((y % DISPLAY_HEIGHT) * DISPLAY_WIDTH);

        for byte in sprite.iter() {
            for i in 0..u8::BITS {
                let bit_mask = 0b1000_0000 >> i;
                if byte & bit_mask != 0 {
                    assert_eq!(display.buffer[idx], WHITE);
                } else {
                    assert_eq!(display.buffer[idx], BLACK);
                }
                idx += 1;
            }
            // Skip to next row for the next byte of the sprite.
            idx += DISPLAY_WIDTH - u8::BITS as usize;
        }
    }

    #[test]
    fn draw_sprite() {
        let x = 0;
        let y = 0;
        let sprite = [0b1010_0101];
        let mut display = Display::new();

        display.draw_sprite(x, y, &sprite);

        assert_sprite(&display, x, y, &sprite);
        for i in 8..display.buffer.len() {
            assert_eq!(display.buffer[i], BLACK);
        }
    }

    #[test]
    fn draw_sprite_wraps() {
        let x = DISPLAY_WIDTH;
        let y = DISPLAY_HEIGHT;
        let sprite = [0b1010_0101];
        let mut display = Display::new();

        display.draw_sprite(x, y, &sprite);

        assert_sprite(&display, x, y, &sprite);
        for i in 8..display.buffer.len() {
            assert_eq!(display.buffer[i], BLACK);
        }
    }

    #[test]
    fn draw_sprite_xors() {
        let x = 0;
        let y = 0;
        let mut display = Display::new();
        let sprite1 = [0b1111_0010];
        let sprite2 = [0b1000_0101];
        let mut idx = (x % DISPLAY_WIDTH) + ((y % DISPLAY_HEIGHT) * DISPLAY_WIDTH);

        display.draw_sprite(x, y, &sprite1);
        display.draw_sprite(x, y, &sprite2);

        let xored_sprite = sprite1[0] ^ sprite2[0];
        for i in 0..u8::BITS {
            let bit_mask = 0b1000_0000 >> i;
            if xored_sprite & bit_mask as u8 != 0 {
                assert_eq!(display.buffer[idx], WHITE);
            } else {
                assert_eq!(display.buffer[idx], BLACK);
            }
            idx += 1;
        }
        for i in 8..display.buffer.len() {
            assert_eq!(display.buffer[i], BLACK);
        }
    }

    #[test]
    fn clear() {
        let mut display = Display::new();

        for i in 0..display.buffer.len() {
            display.buffer[i] = 0xCAFE;
        }

        display.clear();

        for i in 0..display.buffer.len() {
            assert_eq!(display.buffer[i], BLACK);
        }
    }
}
