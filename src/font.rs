use std::ops::Range;

macro_rules! create_char_range {
    ($offset:literal) => {
        FONT_START_ADDR + CHAR_SIZE * $offset..FONT_START_ADDR + CHAR_SIZE + CHAR_SIZE * $offset
    };
}

const CHAR_COUNT: usize = 16;
const CHAR_SIZE: usize = 5;

pub const FONT_START_ADDR: usize = 0x50;
pub const FONT_END_ADDR: usize = FONT_START_ADDR + CHAR_COUNT * CHAR_SIZE;

pub const CHAR_0_RANGE: Range<usize> = create_char_range!(0x0);
pub const CHAR_1_RANGE: Range<usize> = create_char_range!(0x1);
pub const CHAR_2_RANGE: Range<usize> = create_char_range!(0x2);
pub const CHAR_3_RANGE: Range<usize> = create_char_range!(0x3);
pub const CHAR_4_RANGE: Range<usize> = create_char_range!(0x4);
pub const CHAR_5_RANGE: Range<usize> = create_char_range!(0x5);
pub const CHAR_6_RANGE: Range<usize> = create_char_range!(0x6);
pub const CHAR_7_RANGE: Range<usize> = create_char_range!(0x7);
pub const CHAR_8_RANGE: Range<usize> = create_char_range!(0x8);
pub const CHAR_9_RANGE: Range<usize> = create_char_range!(0x9);
pub const CHAR_A_RANGE: Range<usize> = create_char_range!(0xA);
pub const CHAR_B_RANGE: Range<usize> = create_char_range!(0xB);
pub const CHAR_C_RANGE: Range<usize> = create_char_range!(0xC);
pub const CHAR_D_RANGE: Range<usize> = create_char_range!(0xD);
pub const CHAR_E_RANGE: Range<usize> = create_char_range!(0xE);
pub const CHAR_F_RANGE: Range<usize> = create_char_range!(0xF);

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
