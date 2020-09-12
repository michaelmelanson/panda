use super::{
    buffer::{Buffer, ScreenChar},
    colour_code::ColourCode,
};

pub struct Writer {
    column_position: usize,
    colour_code: ColourCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn new(
        column_position: usize,
        colour_code: ColourCode,
        buffer: &'static mut Buffer,
    ) -> Self {
        Self {
            column_position,
            colour_code,
            buffer,
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            ascii_character => {
                if self.column_position >= Buffer::width() {
                    self.new_line();
                }

                let row = Buffer::height() - 1;
                let col = self.column_position;

                let colour_code = self.colour_code;
                self.buffer
                    .set(row, col, ScreenChar::new(ascii_character, colour_code));
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..Buffer::height() {
            for col in 0..Buffer::width() {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(Buffer::height() - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar::new(b' ', self.colour_code);
        for col in 0..Buffer::width() {
            self.buffer.chars[row][col].write(blank);
        }
    }
}
