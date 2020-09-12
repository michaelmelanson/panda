use core::fmt::Write;

use lazy_static::lazy_static;
use spin::Mutex;

use crate::{vga::Vga, serial_print};

pub(crate) enum LogTarget {
    Vga(Vga),
}

impl Write for LogTarget {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        serial_print!("{}", s);
        
        match self {
            LogTarget::Vga(vga) => vga.writer.write_str(s),
        }
    }
}

lazy_static! {
    pub(crate) static ref TARGET: Mutex<LogTarget> = Mutex::new(LogTarget::Vga(Vga::new()));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::log::_print(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    TARGET.lock().write_fmt(args).unwrap();
}
