use core::fmt::UpperHex;

use crate::acpi;
use ::acpi::PciConfigRegions;
use spin::Once;
use x86_64::PhysAddr;

use crate::memory;

static PCI_CONFIG_REGIONS: Once<Option<&'static PciConfigRegions>> = Once::new();

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PciDeviceAddress {
    pub segment: u16,
    pub bus: u8,
    pub slot: u8,
    pub function: u8,
}

impl PciDeviceAddress {
    pub fn new(segment: u16, bus: u8, slot: u8, function: u8) -> Self {
        PciDeviceAddress {
            segment,
            bus,
            slot,
            function,
        }
    }
}

impl core::fmt::Display for PciDeviceAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{:04X}-{:02X}:{:02X}:{:02X}",
            self.segment, self.bus, self.slot, self.function
        )
    }
}

pub fn read<T: UpperHex + Copy>(device: &PciDeviceAddress, offset: u8) -> Option<T> {
    let segment = device.segment;
    let bus = device.bus;
    let slot = device.slot;
    let function = device.function;

    let phys_addr = PCI_CONFIG_REGIONS
        .wait()
        .unwrap()
        .as_ref()
        .unwrap()
        .physical_address(segment, bus, slot, function)?;
    let phys_addr = phys_addr + (offset as u64);
    let virt_addr = memory::physical_to_virtual_address(PhysAddr::new(phys_addr));
    let ptr = virt_addr.as_ptr::<T>();

    let value = unsafe { *ptr };
    // debug!("PCI: {:X}-{:X}:{:X}+{:X} = {:X}", segment, bus, slot, offset, value);
    Some(value)
}

pub fn init() {
    PCI_CONFIG_REGIONS.call_once(|| acpi::pci_config_regions());
}
