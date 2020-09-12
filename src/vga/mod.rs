mod buffer;
mod colour;
mod colour_code;
mod writer;
mod write_impl;

use buffer::Buffer;
use colour::Colour;
use colour_code::ColourCode;
use writer::Writer;

pub(crate) struct Vga {
    pub writer: Writer,
}

impl Vga {
    pub fn new() -> Self {
        Vga {
            writer: Writer::new(0, ColourCode::new(Colour::White, Colour::Black), unsafe {
                &mut *(0xb8000 as *mut Buffer)
            }),
        }
    }
}
