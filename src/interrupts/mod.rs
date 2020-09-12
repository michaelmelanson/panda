pub mod breakpoint;
pub mod double_fault;
pub mod page_fault;

use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

use crate::gdt;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint
            .set_handler_fn(breakpoint::breakpoint_handler);

        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault::double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt.page_fault
            .set_handler_fn(page_fault::page_fault_handler);
        idt
    };
}

pub fn init() {
    IDT.load();
}
