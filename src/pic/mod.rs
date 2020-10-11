use pic8259_simple::ChainedPics;
use x86_64::instructions::port::Port;

pub const PIC_1_OFFSET: u8 = 0x20;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[derive(Copy, Clone)]
pub enum Irq {
    Timer = 0,
    Keyboard = 1,
}

impl Irq {
    pub fn vector(&self) -> u8 {
        *self as u8
    }

    pub fn interrupt_id(&self) -> u8 {
        PIC_1_OFFSET + self.vector()
    }
}

pub fn init() {
    unsafe {
        PICS.lock().initialize();
        x86_64::instructions::interrupts::enable();
    }
}

pub fn notify_end_of_interrupt(interrupt_id: u8) {
    unsafe {
        PICS.lock().notify_end_of_interrupt(interrupt_id);
    }
}

pub fn notify_end_of_irq(irq: u8) {
    notify_end_of_interrupt(0x20 + irq);
}

pub fn isr() -> u16 {
    let mut pic1_cmd: Port<u8> = Port::new(0x20);
    let mut pic2_cmd: Port<u8> = Port::new(0xA0);

    const READ_ISR: u8 = 0x0b;
    
    unsafe { 
        pic1_cmd.write(READ_ISR); 
        pic2_cmd.write(READ_ISR);

        ((pic2_cmd.read() as u16) << 8) | (pic1_cmd.read() as u16)
    }
}