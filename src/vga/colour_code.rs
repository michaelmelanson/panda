use super::colour::Colour;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColourCode(u8);

impl ColourCode {
    pub fn new(foreground: Colour, background: Colour) -> Self {
        ColourCode((background as u8) << 4 | (foreground as u8))
    }
}
