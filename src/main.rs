#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(panda::test_runner)]

extern crate rlibc;

extern crate panda;

use panda::*;
use panda::{log::LogTarget, vga::Vga};

use core::panic::PanicInfo;

pub fn halt_loop() -> ! {
    loop {
        x86_64::instructions::hlt()
    }
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    qemu::exit_success();
    halt_loop()
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start(bootinfo: &'static bootloader::BootInfo) -> ! {
    log::set_log_target(LogTarget::Vga(Vga::new(bootinfo.physical_memory_offset)));

    println!("Panda");
    println!();

    println!("Memory map:");
    for region in bootinfo.memory_map.into_iter() {
        println!(
            " - {:#016X}-{:#016X} {:?}",
            region.range.start_addr(),
            region.range.end_addr(),
            region.region_type
        );
    }

    println!(
        "Physical memory is at {:#016X}",
        bootinfo.physical_memory_offset
    );

    gdt::load();
    interrupts::init();
    pic::init();

    println!("All done initializing");

    halt_loop()
}

#[test_case]
pub fn test_trivial() {
    assert_eq!(1, 1);
}

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    panic::panic_handler(info)
}

#[cfg(test)]
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    qemu::exit_failure();
    halt_loop()
}
