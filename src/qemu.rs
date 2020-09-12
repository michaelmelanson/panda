#[allow(dead_code)]
pub fn exit_success() {
    exit(0x10)
}
#[allow(dead_code)]
pub fn exit_failure() {
    exit(0x11)
}

fn exit(exit_code: u8) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
