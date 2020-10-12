#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(panda::test_runner)]

extern crate alloc;
extern crate panda;
extern crate rlibc;

use panda::*;

use core::panic::PanicInfo;

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
    log::set_log_target(log::LogTarget::Vga(vga::Vga::new(
        bootinfo.physical_memory_offset,
    )));

    println!("Panda");
    println!();

    gdt::init();
    interrupts::init();
    pic::init();
    memory::init(&bootinfo);
    device::init();
    acpi::init();
    pci::init();

    let mut executor = task::init();

    for acpi_address in acpi::devices() {
        let mut device_manager = device::device_manager().upgrade();
        let device = device_manager.add_device(Some(acpi_address), None, None);
        device_manager.discover_child_devices(&device);
    }

    device::start_all_devices(&mut executor);

    println!("All done initializing");
    executor.run();
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
