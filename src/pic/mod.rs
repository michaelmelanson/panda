use pic8259_simple::ChainedPics;

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

pub fn notify_end_of_interrupt(irq: Irq) {
    unsafe {
        PICS.lock().notify_end_of_interrupt(irq.interrupt_id());
    }
}
