mod buffer;
mod colour;
mod colour_code;
mod write_impl;
mod writer;

use buffer::Buffer;
use colour::Colour;
use colour_code::ColourCode;
use writer::Writer;

pub struct Vga {
    pub writer: Writer,
}

impl Vga {
    pub fn new(physical_base: u64) -> Self {
        Vga {
            writer: Writer::new(0, ColourCode::new(Colour::White, Colour::Black), unsafe {
                &mut *((physical_base + 0xb8000) as *mut Buffer)
            }),
        }
    }
}
