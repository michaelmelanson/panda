#![no_std]
#![cfg_attr(test, no_main)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![feature(alloc_error_handler)]
#![feature(const_in_array_repeat_expressions)]
#![feature(abi_x86_interrupt)]
#![feature(wake_trait)]

extern crate alloc;
extern crate rlibc;

#[macro_use]
pub mod log;
#[macro_use]
pub mod serial;

#[cfg(test)]
pub mod test_runner;

pub mod acpi;
pub mod device;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod panic;
pub mod pci;
pub mod pic;
pub mod qemu;
pub mod task;
pub mod vga;

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    qemu::exit_success();
}

/// Entry point for `cargo test`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    panic::test_panic_handler(info)
}
