use minifb::{Window, WindowOptions};

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const WHITE: u32 = u32::MAX;
const BLACK: u32 = 0;

pub struct Screen {
    buffer: [u32; SCREEN_WIDTH * SCREEN_HEIGHT],
    window: Window,
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            buffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
            window: Window::new(
                "Chip8 Emulator",
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                WindowOptions::default(),
            )
            .expect("Unable to create the window."),
        }
    }

    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();
    }

    pub fn draw_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) {
        let mut idx = (x % SCREEN_WIDTH) + ((y % SCREEN_HEIGHT) * SCREEN_WIDTH);

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
            idx += SCREEN_WIDTH - u8::BITS as usize;
        }
    }

    pub fn clear(&mut self) {
        self.buffer = [0; SCREEN_WIDTH * SCREEN_HEIGHT];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_sprite(screen: &Screen, x: usize, y: usize, sprite: &[u8]) {
        let mut idx = (x % SCREEN_WIDTH) + ((y % SCREEN_HEIGHT) * SCREEN_WIDTH);

        for byte in sprite.iter() {
            for i in 0..u8::BITS {
                let bit_mask = 0b1000_0000 >> i;
                if byte & bit_mask != 0 {
                    assert_eq!(screen.buffer[idx], WHITE);
                } else {
                    assert_eq!(screen.buffer[idx], BLACK);
                }
                idx += 1;
            }
            // Skip to next row for the next byte of the sprite.
            idx += SCREEN_WIDTH - u8::BITS as usize;
        }
    }

    #[test]
    fn draw_sprite() {
        let x = 0;
        let y = 0;
        let sprite = [0b1010_0101];
        let mut screen = Screen::new();

        screen.draw_sprite(x, y, &sprite);

        assert_sprite(&screen, x, y, &sprite);
        for i in 8..screen.buffer.len() {
            assert_eq!(screen.buffer[i], BLACK);
        }
    }

    #[test]
    fn draw_sprite_wraps() {
        let x = SCREEN_WIDTH;
        let y = SCREEN_HEIGHT;
        let sprite = [0b1010_0101];
        let mut screen = Screen::new();

        screen.draw_sprite(x, y, &sprite);

        assert_sprite(&screen, x, y, &sprite);
        for i in 8..screen.buffer.len() {
            assert_eq!(screen.buffer[i], BLACK);
        }
    }

    #[test]
    fn draw_sprite_xors() {
        let x = 0;
        let y = 0;
        let mut screen = Screen::new();
        let sprite1 = [0b1111_0010];
        let sprite2 = [0b1000_0101];
        let mut idx = (x % SCREEN_WIDTH) + ((y % SCREEN_HEIGHT) * SCREEN_WIDTH);

        screen.draw_sprite(x, y, &sprite1);
        screen.draw_sprite(x, y, &sprite2);

        let xored_sprite = sprite1[0] ^ sprite2[0];
        for i in 0..u8::BITS {
            let bit_mask = 0b1000_0000 >> i;
            if xored_sprite & bit_mask as u8 != 0 {
                assert_eq!(screen.buffer[idx], WHITE);
            } else {
                assert_eq!(screen.buffer[idx], BLACK);
            }
            idx += 1;
        }
        for i in 8..screen.buffer.len() {
            assert_eq!(screen.buffer[i], BLACK);
        }
    }

    #[test]
    fn clear() {
        let mut screen = Screen::new();

        for i in 0..screen.buffer.len() {
            screen.buffer[i] = 0xCAFE;
        }

        screen.clear();

        for i in 0..screen.buffer.len() {
            assert_eq!(screen.buffer[i], BLACK);
        }
    }
}
