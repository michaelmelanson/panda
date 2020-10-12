use alloc::vec::Vec;
use aml::AmlName;

use super::AcpiDeviceAddress;

pub struct DeviceIterator {
    device_names: Vec<AmlName>,
    index: usize,
}

impl DeviceIterator {
    pub fn new(device_names: Vec<AmlName>) -> Self {
        DeviceIterator {
            device_names,
            index: 0,
        }
    }
}

impl Iterator for DeviceIterator {
    type Item = AcpiDeviceAddress;

    fn next(&mut self) -> Option<Self::Item> {
        let name = self.device_names.get(self.index);
        self.index += 1;
        name.map(|name| AcpiDeviceAddress(name.clone()))
    }
}
