pub mod keyboard;

use keyboard::task::keyboard_task;

use crate::task::{Executor, Task};

use super::{Device, DeviceKind};


pub fn start_device_driver(executor: &mut Executor, device: &Device) {
  match device.kind() {
    DeviceKind::PcKeyboard => executor.spawn(Task::new(keyboard_task(device.id))),
    DeviceKind::PciBus => {}
    DeviceKind::PciDevice(_) => {}
    DeviceKind::Unknown => {},
  }
}
