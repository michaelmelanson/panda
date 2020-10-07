mod device_children_iterator;
use core::fmt::Display;

use crate::acpi::AcpiDeviceAddress;
use alloc::vec::Vec;
use aml::AmlName;
use device_children_iterator::DeviceChildrenIterator;
use lazy_static::lazy_static;
use pci::PciDeviceAddress;
use spin::{Mutex, MutexGuard, Once, RwLock, RwLockReadGuard, RwLockUpgradeableGuard};

use crate::{acpi, pci};

lazy_static! {
    static ref PCI_ROOT: AmlName = AmlName::from_str("\\_SB_.PCI0").unwrap();
}

type DeviceId = usize;


#[derive(Debug)]
enum DeviceKind {
    PciBus,
    Storage(StorageSubclassKind),
    Network,
    Display(DisplaySubclassKind),
    Multimedia,
    Memory,
    Bridge,
    SimpleComms,
    BasePeripheral,
    Input,
    Dock,
    Processor,
    SerialBusController,
    Wireless,
    Unknown,
}

#[derive(Debug)]
enum StorageSubclassKind {
    SCSI,
    IDE,
    Floppy,
    IPI,
    RAID,
    ATA,
    SerialATA,
    SerialAttachedSCSI,
    NVMem,
    Other
}

#[derive(Debug)]
enum DisplaySubclassKind {
    VGA, // VGA-Compatible
    XGA,
    ThreeD,
    Other
}

#[derive(Debug, Clone)]
pub struct Device {
    id: DeviceId,
    parent_id: Option<DeviceId>,
    acpi_address: Option<AcpiDeviceAddress>,
    pci_address: Option<PciDeviceAddress>
}

impl Device {
    fn kind(&self) -> DeviceKind {
        if let Some(acpi_address) = &self.acpi_address {
            match acpi_address.aml_name().as_string().as_str() {
                "\\_SB_.PCI0" => return DeviceKind::PciBus,
                _ => {},
            }
        }

        if let Some(pci_device_address) = &self.pci_address {
            let vendor_id: u16 =
                pci::read(pci_device_address, 0).expect("invalid PCI device");
            if vendor_id != 0xffff {
                let device_id: u16 = pci::read(pci_device_address, 2).expect("invalid PCI device");
                let class = pci::read::<u8>(pci_device_address, 11).expect("failed to read PCI class");
                let subclass = pci::read::<u8>(pci_device_address, 10).expect("failed to read PCI subclass");

                let kind = match (class, subclass) {
                    (0x00, 0x00) => DeviceKind::Unknown,
                    (0x01, subclass) => {
                        let subclass = match subclass {
                            0x00 => StorageSubclassKind::SCSI,
                            0x01 => StorageSubclassKind::IDE,
                            0x02 => StorageSubclassKind::Floppy,
                            0x03 => StorageSubclassKind::IPI,
                            0x04 => StorageSubclassKind::RAID,
                            0x05 => StorageSubclassKind::ATA,
                            0x06 => StorageSubclassKind::SerialATA,
                            0x07 => StorageSubclassKind::SerialAttachedSCSI,
                            0x08 => StorageSubclassKind::NVMem,
                            0x80 => StorageSubclassKind::Other,
                            subclass => unimplemented!("Storage subclass {:2X}", subclass)
                        };
                        DeviceKind::Storage(subclass)
                    },
                    (0x02, _) => DeviceKind::Network,
                    (0x03, subclass) => {
                        let subclass = match subclass {
                            0x00 => DisplaySubclassKind::VGA,
                            0x01 => DisplaySubclassKind::XGA,
                            0x02 => DisplaySubclassKind::ThreeD,
                            0x80 => DisplaySubclassKind::Other,
                            subclass => unimplemented!("Display subclass {:2X}", subclass)
                        };

                        DeviceKind::Display(subclass)
                    },
                    (0x04, _) => DeviceKind::Multimedia,
                    (0x05, _) => DeviceKind::Memory,
                    (0x06, _) => DeviceKind::Bridge,
                    (0x07, _) => DeviceKind::SimpleComms,
                    (0x08, _) => DeviceKind::BasePeripheral,
                    (0x09, _) => DeviceKind::Input,
                    (0x0A, _) => DeviceKind::Dock,
                    (0x0B, _) => DeviceKind::Processor,
                    (0x0C, _) => DeviceKind::SerialBusController,
                    (0x0D, _) => DeviceKind::Wireless,
                    (_, _)    => DeviceKind::Unknown,
                };

                return kind;
            } else {
                println!("Invalid PCI device -- no vendor found");
            }
        }

        DeviceKind::Unknown
    }

    fn children(&self) -> DeviceChildrenIterator {
        match self.kind() {
            DeviceKind::PciBus => {
                let parent_pci_address = self.pci_address.expect("PCI bus with no PCI address?");

                DeviceChildrenIterator::PCI {
                    parent_pci_address,
                    next_slot: 0
                }
            }

            _ => DeviceChildrenIterator::Empty,
        }
    }
}

impl Display for Device {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Device #{}", self.id)?;
        if let Some(parent_id) = self.parent_id {
            write!(f, ", child of {}", parent_id)?;
        }
        if let Some(acpi_address) = &self.acpi_address {
            write!(f, ", ACPI address {}", acpi_address)?;
        }
        if let Some(pci_address) = &self.pci_address {
            write!(f, ", PCI address: {}", pci_address)?;
        }

        Ok(())
    }
}

#[derive(Default)]
pub struct DeviceManager {
    next_device_id: DeviceId,
    devices: Vec<Device>,
}

impl DeviceManager {
    pub fn add_device(&mut self, acpi_address: Option<AcpiDeviceAddress>, pci_address: Option<PciDeviceAddress>, parent_id: Option<DeviceId>) -> Device {
        // println!(
        //     "DEVICE: Adding device at ACPI address {:?}, PCI address {:?}, child of {:?}",
        //     acpi_address, pci_address, parent_id
        // );


        let pci_address = pci_address.or_else(|| {
            acpi_address.clone().and_then(|ref acpi_address| {
                acpi::pci_address_for_acpi_address(acpi_address)
            })
        });

        let mut device = if let Some(mut device) = acpi_address.clone().and_then(|ref acpi_address| self.find_by_acpi_address(acpi_address)) {
            if pci_address.is_some() {
                device.pci_address = pci_address;
            }

            device.clone()
        } else if let Some(mut device) = pci_address.and_then(|ref pci_address| self.find_by_pci_address(pci_address)) {
            if acpi_address.is_some() {
                device.acpi_address = acpi_address;
            }

            device.clone()
        } else {
            let device = Device {
                id: self.next_device_id,
                parent_id,
                acpi_address,
                pci_address,
            };
            
            self.next_device_id += 1;            
            device
        };

        self.insert_or_update_device(&device);
        device
    }

    fn insert_or_update_device(&mut self, new_device: &Device) {
        for device in self.devices.iter_mut() {
            if device.id == new_device.id {
                *device = new_device.clone();
                return;
            }
        }

        self.devices.push(new_device.clone());
    }

    fn find_by_acpi_address(&mut self, acpi_address: &AcpiDeviceAddress) -> Option<&mut Device> {
        for device in self.devices.iter_mut() {
            if let Some(ref device_acpi_address) = device.acpi_address {
                if device_acpi_address == acpi_address {
                   return Some(device);
                }
            }
        }

        None
    }

    fn find_by_pci_address(&mut self, pci_address: &PciDeviceAddress) -> Option<&mut Device> {
        for device in self.devices.iter_mut () {
            if device.pci_address == Some(*pci_address) {
                return Some(device);
            }
        }

        None
    }

    pub fn discover_child_devices(&mut self, parent: &Device) {
        for (acpi_address, pci_address) in parent.children() {
            self.add_device(acpi_address, pci_address, Some(parent.id));
        }
    }
}

static mut DEVICE_MANAGER: Once<RwLock<DeviceManager>> = Once::new();

pub fn init() {
    device_manager();
}

pub fn device_manager() -> RwLockUpgradeableGuard<'static, DeviceManager> {
    unsafe {
        DEVICE_MANAGER
            .call_once(|| RwLock::new(DeviceManager::default()))
            .upgradeable_read()
    }
}

pub fn print_all_devices() {
    println!("Devices:");

    let device_manager = device_manager();

    for device in device_manager.devices.iter() {
        println!("  - {} ({:?})", device, device.kind());
    }
}
