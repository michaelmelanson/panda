mod bus_iterator;
mod device_address;
mod types;

use core::fmt::UpperHex;

use crate::{acpi, device};
use ::acpi::PciConfigRegions;
use crate::device::DeviceId;
use bus_iterator::PciBusIterator;
pub use device_address::PciDeviceAddress;
use device_address::{PciDeviceRegister, PciHeaderType};
use spin::Once;
pub use types::*;
use x86_64::PhysAddr;

use crate::memory;

static PCI_CONFIG_REGIONS: Once<Option<&'static PciConfigRegions>> = Once::new();

pub fn init() {
    PCI_CONFIG_REGIONS.call_once(|| acpi::pci_config_regions());
}

pub fn discover() {
    if let Some(Some(config_regions)) = PCI_CONFIG_REGIONS.wait() {
        for region in &config_regions.regions {
            discover_pci_bus(region.base_address, region.pci_segment_group, 0, None);
        }
    }
}

pub fn base_address_for_segment(segment: u16) -> Option<u64> {
    if let Some(Some(config_regions)) = PCI_CONFIG_REGIONS.wait() {
        for region in &config_regions.regions {
            if region.pci_segment_group == segment {
                return Some(region.base_address);
            }
        }
    }

    None
}

pub fn discover_pci_bus(
    base_address: u64,
    segment: u16,
    bus: u8,
    parent_device_id: Option<DeviceId>,
) {
    for pci_address in PciBusIterator::new(base_address, segment, bus) {
        println!(" - PCI device: {} ({:?})", pci_address, pci_address.kind());
        let device_id = device::device_manager().upgrade().add_pci_device(pci_address, parent_device_id);
        if pci_address.header_type() == PciHeaderType::PciToPciBridge {
            let secondary_bus = pci_address.read::<u8>(PciDeviceRegister::SecondaryBusNumber);
            discover_pci_bus(base_address, segment, secondary_bus, Some(device_id));
        }
    }
}
