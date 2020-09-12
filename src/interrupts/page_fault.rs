use core::sync::atomic::{AtomicUsize, Ordering};

use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

pub static PAGE_FAULT_COUNT: AtomicUsize = AtomicUsize::new(0);

pub extern "x86-interrupt" fn page_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    PAGE_FAULT_COUNT.fetch_add(1, Ordering::SeqCst);

    use x86_64::registers::control::Cr2;

    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);

    panic!("Invalid page fault");
}
