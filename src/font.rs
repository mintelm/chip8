const CHAR_COUNT: usize = 16;
const CHAR_SIZE: usize = 5;

pub const FONT_START_ADDR: usize = 0x50;
pub const FONT_END_ADDR: usize = FONT_START_ADDR + CHAR_COUNT * CHAR_SIZE;

pub const CHAR_0_ADDR: usize = FONT_START_ADDR;
pub const CHAR_1_ADDR: usize = CHAR_0_ADDR + CHAR_SIZE;
pub const CHAR_2_ADDR: usize = CHAR_1_ADDR + CHAR_SIZE;
pub const CHAR_3_ADDR: usize = CHAR_2_ADDR + CHAR_SIZE;
pub const CHAR_4_ADDR: usize = CHAR_3_ADDR + CHAR_SIZE;
pub const CHAR_5_ADDR: usize = CHAR_4_ADDR + CHAR_SIZE;
pub const CHAR_6_ADDR: usize = CHAR_5_ADDR + CHAR_SIZE;
pub const CHAR_7_ADDR: usize = CHAR_6_ADDR + CHAR_SIZE;
pub const CHAR_8_ADDR: usize = CHAR_7_ADDR + CHAR_SIZE;
pub const CHAR_9_ADDR: usize = CHAR_8_ADDR + CHAR_SIZE;
pub const CHAR_A_ADDR: usize = CHAR_9_ADDR + CHAR_SIZE;
pub const CHAR_B_ADDR: usize = CHAR_A_ADDR + CHAR_SIZE;
pub const CHAR_C_ADDR: usize = CHAR_B_ADDR + CHAR_SIZE;
pub const CHAR_D_ADDR: usize = CHAR_C_ADDR + CHAR_SIZE;
pub const CHAR_E_ADDR: usize = CHAR_D_ADDR + CHAR_SIZE;
pub const CHAR_F_ADDR: usize = CHAR_E_ADDR + CHAR_SIZE;

#[rustfmt::skip]
pub const FONT_RAW_DATA: [u8; CHAR_COUNT * CHAR_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
