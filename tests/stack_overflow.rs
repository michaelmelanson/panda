#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(panda::test_runner)]

use core::sync::atomic::Ordering;

use panda::interrupts::double_fault::DOUBLE_FAULT_COUNT;
use panda::*;

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    volatile::Volatile::new(0).read(); // prevent tail recursion
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    serial_print!("testing that kernel stack overflow triggers double fault... ");

    panda::gdt::init();
    panda::interrupts::init();
    stack_overflow();

    panic!("no double fault exception");
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let double_fault_count = DOUBLE_FAULT_COUNT.load(Ordering::Acquire);

    if double_fault_count == 1 {
        serial_println!("[ok]");
        qemu::exit_success();
    } else {
        serial_println!(
            "[failed] (panic but {} double fault exceptions",
            double_fault_count
        );
        serial_println!("{}", info);
        qemu::exit_failure();
    }

    loop {}
}
