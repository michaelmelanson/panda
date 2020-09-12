use core::sync::atomic::{AtomicUsize, Ordering};

use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

pub static PAGE_FAULT_COUNT: AtomicUsize = AtomicUsize::new(0);

pub extern "x86-interrupt" fn page_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    PAGE_FAULT_COUNT.fetch_add(1, Ordering::SeqCst);

    panic!(
        "EXCEPTION: PAGE FAULT (error code {:?})\n{:#?}",
        error_code, stack_frame
    );
}
