use core::sync::atomic::{AtomicUsize, Ordering};

use x86_64::structures::{
    idt::{InterruptStackFrame, PageFaultErrorCode},
    paging::Page,
    paging::PageTableFlags,
};

use crate::memory::{self, HEAP_SIZE, HEAP_START};

pub static PAGE_FAULT_COUNT: AtomicUsize = AtomicUsize::new(0);

pub extern "x86-interrupt" fn page_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    PAGE_FAULT_COUNT.fetch_add(1, Ordering::SeqCst);

    use x86_64::registers::control::Cr2;

    let address = Cr2::read();

    if address.as_u64() >= HEAP_START && address.as_u64() - HEAP_START <= HEAP_SIZE {
        unsafe {
            memory::map_page(
                Page::containing_address(address),
                PageTableFlags::GLOBAL
                    | PageTableFlags::PRESENT
                    | PageTableFlags::NO_EXECUTE
                    | PageTableFlags::WRITABLE,
            );
        }
        return;
    }

    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", address);
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);

    panic!("Invalid page fault");
}
