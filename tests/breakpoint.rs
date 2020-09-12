#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(panda::test_runner)]

use core::sync::atomic::Ordering;

use panda::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("testing that breakpoints invoke the handler... ");

    panda::interrupts::init();
    x86_64::instructions::interrupts::int3();

    let breakpoint_count = panda::interrupts::breakpoint::BREAKPOINT_COUNT.load(Ordering::Acquire);
    assert_eq!(breakpoint_count, 1);

    serial_println!("[ok]");

    qemu::exit_success();
    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    serial_println!("[failed]");
    serial_println!("{}", info);
    qemu::exit_failure();
    loop {}
}
