use core::sync::atomic::{AtomicUsize, Ordering};

use x86_64::structures::idt::InterruptStackFrame;

pub static BREAKPOINT_COUNT: AtomicUsize = AtomicUsize::new(0);

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) -> () {
    BREAKPOINT_COUNT.fetch_add(1, Ordering::SeqCst);
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
