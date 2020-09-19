use x86_64::instructions::port::Port;


fn read(bus: u8, slot: u8, func: u8, offset: u8) -> u32 {
  let bus = bus as u32;
  let slot = slot as u32;
  let func = func as u32;
  let offset = offset as u32;

  let address = (bus << 16) | (slot << 11) | (func << 8) | (offset & 0xfc) | 0x80000000;

  let mut control_port: Port<u32> = Port::new(0xCF8);
  let mut data_port: Port<u32> = Port::new(0xCFC);

  unsafe { control_port.write(address); }
  let data = unsafe { data_port.read() };

  (data >> ((offset & 2) * 8)) & 0xFFFF
}

pub fn init() {


}