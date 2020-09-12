use core::sync::atomic::{AtomicUsize, Ordering};

use pic::Irq;
use x86_64::structures::idt::InterruptStackFrame;

use crate::pic;

pub static KEYBOARD_COUNT: AtomicUsize = AtomicUsize::new(0);

pub extern "x86-interrupt" fn keyboard_handler(_stack_frame: &mut InterruptStackFrame) -> () {
    KEYBOARD_COUNT.fetch_add(1, Ordering::SeqCst);

    use x86_64::instructions::port::Port;

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    print!("{}", scancode);

    pic::notify_end_of_interrupt(Irq::Keyboard);
}
