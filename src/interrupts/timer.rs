use core::sync::atomic::{AtomicUsize, Ordering};

use pic::Irq;
use x86_64::structures::idt::InterruptStackFrame;

use crate::pic;

pub static TIMER_COUNT: AtomicUsize = AtomicUsize::new(0);

pub extern "x86-interrupt" fn timer_handler(_stack_frame: &mut InterruptStackFrame) -> () {
    TIMER_COUNT.fetch_add(1, Ordering::SeqCst);
    print!(".");

    pic::notify_end_of_interrupt(Irq::Timer);
}
