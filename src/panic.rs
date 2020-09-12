use core::panic::PanicInfo;

pub fn panic_handler(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    use crate::qemu;
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    qemu::exit_failure();
    loop {}
}
