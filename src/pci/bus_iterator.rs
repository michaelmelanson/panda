use super::PciDeviceAddress;

pub struct PciBusIterator {
    base_address: u64,
    segment: u16,
    bus: u8,
    slot: u8,
    function: u8,
}

impl PciBusIterator {
    pub fn new(base_address: u64, segment: u16, bus: u8) -> Self {
        Self {
            base_address,
            segment,
            bus,
            slot: 0,
            function: 0,
        }
    }
}

impl PciBusIterator {
    fn current(&self) -> PciDeviceAddress {
        PciDeviceAddress::new(
            self.base_address,
            self.segment,
            self.bus,
            self.slot,
            self.function,
        )
    }
}

impl Iterator for PciBusIterator {
    type Item = PciDeviceAddress;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let max_function = if self.current().is_multifunction() {
                7
            } else {
                0
            };

            if self.function < max_function {
                self.function += 1;
            } else if self.slot < 255 {
                self.slot += 1;
                self.function = 0;
            } else {
                return None;
            }

            let current = self.current();
            if current.is_valid_device() {
                return Some(current);
            }
        }
    }
}
