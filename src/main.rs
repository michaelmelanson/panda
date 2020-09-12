#![no_std]
#![no_main]

extern crate rlibc;

#[macro_use]
mod log;
mod panic;
mod vga;

use bootloader::BootInfo;

#[no_mangle]
pub extern "C" fn _start(bootinfo: &'static BootInfo) -> ! {
    println!("Panda");
    println!("-----");
    println!();

    println!("Memory map:");
    for region in bootinfo.memory_map.into_iter() {
        println!(" - {:#016X}-{:#016X} {:?}", region.range.start_addr(), region.range.end_addr(), region.region_type);
    }

    println!("Physical memory is at {:#016X}", bootinfo.physical_memory_offset);

    panic!("Testing panic!");
    //loop {}
}
