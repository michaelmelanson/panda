use core::sync::atomic::{AtomicUsize, Ordering};

use x86_64::structures::idt::InterruptStackFrame;

pub static DOUBLE_FAULT_COUNT: AtomicUsize = AtomicUsize::new(0);

pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: u64,
) -> ! {
    DOUBLE_FAULT_COUNT.fetch_add(1, Ordering::Relaxed);

    panic!(
        "EXCEPTION: DOUBLE FAULT (error code {})\n{:#?}",
        error_code, stack_frame
    );
}
