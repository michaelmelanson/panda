use alloc::vec::Vec;
use aml::{resource::Resource, AmlName};
use pc_keyboard::DecodedKey;
use x86_64::instructions::port::Port;

use crate::{
  acpi, 
    device::{device_manager, DeviceId},
    interrupts::irq::wait_irq,
};

#[derive(Debug)]
pub struct Scancode(u8);

pub async fn keyboard_task(device_id: DeviceId) {
  println!("Keyboard task started");

  let device_manager = device_manager();
  let device = device_manager.get(device_id).expect("could not find keyboard device");
  let acpi_address = device.acpi_address.as_ref().expect("Keyboard does not have ACPI address");
  let crs_name = acpi_address.aml_name().child(&AmlName::from_str("_CRS").unwrap());
  let crs = acpi::get(&crs_name).expect("Could not get keyboard CRS");
  let resources = aml::resource::resource_descriptor_list(&crs).expect("Could not parse keyboard CRS");


  let mut ports = Vec::with_capacity(2);
  let mut irq = None;

  for resource in resources {
    match resource {
        Resource::Irq(irq_descriptor) => irq = Some(irq_descriptor.irq),
        Resource::IOPort(io_descriptor) => ports.push(io_descriptor.memory_range.0),
        other => println!("Unexpected resource in CRS: {:?}", other)
    }
  }

  let ports = ports;
  let mut command_port: Port<u8> = Port::new(ports[0]);
  let irq = irq.expect("No IRQ given for keyboard") as u8;


  let mut keyboard = pc_keyboard::Keyboard::new(
    pc_keyboard::layouts::Dvorak104Key,
    pc_keyboard::ScancodeSet1,
    pc_keyboard::HandleControl::Ignore
  );

  loop {
    wait_irq(irq).await;

    let scancode: u8 = unsafe { command_port.read() };

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
      if let Some(key) = keyboard.process_keyevent(key_event) {
        match key {
          DecodedKey::Unicode(character) => print!("{}", character),
          DecodedKey::RawKey(key) => print!("{:?}", key)
        }
      }
    }
  }
}
