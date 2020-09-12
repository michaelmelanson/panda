use volatile::Volatile;

use super::colour_code::ColourCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub(crate) struct ScreenChar {
    ascii_character: u8,
    colour_code: ColourCode,
}

impl ScreenChar {
    pub fn new(ascii_character: u8, colour_code: ColourCode) -> Self {
        ScreenChar {
            ascii_character,
            colour_code,
        }
    }
}
#[repr(transparent)]
pub(crate) struct Buffer {
    pub chars: [[Volatile<ScreenChar>; Buffer::width()]; Buffer::height()],
}

impl Buffer {
    pub const fn height() -> usize {
        25
    }
    pub const fn width() -> usize {
        80
    }

    pub fn set(&mut self, row: usize, col: usize, c: ScreenChar) {
        self.chars[row][col] = Volatile::new(c);
    }
}
