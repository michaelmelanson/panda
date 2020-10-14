mod device_children_iterator;
pub mod drivers;

use alloc::vec::Vec;
use core::{fmt::Display, sync::atomic::AtomicUsize, sync::atomic::Ordering};
use hashbrown::{HashMap, HashSet};

use crate::acpi::AcpiDeviceAddress;
use crate::task::Executor;

use aml::{AmlName, AmlValue};
use device_children_iterator::DeviceChildrenIterator;
use drivers::start_device_driver;
use lazy_static::lazy_static;
use pci::{DeviceKind, PciDeviceAddress, PciDeviceKind};
use spin::{Once, RwLock, RwLockUpgradeableGuard};

use crate::{acpi, pci};

lazy_static! {
    static ref PCI_ROOT: AmlName = AmlName::from_str("\\_SB_.PCI0").unwrap();
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Debug)]
pub struct DeviceId(usize);

impl DeviceId {
    pub fn new() -> Self {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        DeviceId(id)
    }
}

impl Display for DeviceId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Device #{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Device {
    id: DeviceId,
    parent_id: Option<DeviceId>,
    acpi_address: Option<AcpiDeviceAddress>,
    pci_address: Option<PciDeviceAddress>,
}

impl Device {
    fn kind(&self) -> DeviceKind {
        if let Some(acpi_address) = &self.acpi_address {
            let name = acpi_address.aml_name();
            let hid = name.child(&AmlName::from_str("_HID").unwrap());
            let cid = name.child(&AmlName::from_str("_CID").unwrap());
            let sub = name.child(&AmlName::from_str("_SUB").unwrap());

            let hid = acpi::get(&hid).ok();
            let cid = acpi::get(&cid).ok();
            let sub = acpi::get(&sub).ok();

            match (name.as_string().as_str(), hid, cid, sub) {
                ("\\_SB_.PCI0", _, _, _) => return DeviceKind::PciBus,
                (_, Some(AmlValue::Integer(0x303D041)), _, _) => return DeviceKind::PcKeyboard,
                _ => {}
            }
        }

        if let Some(pci_device_address) = &self.pci_address {
            return DeviceKind::PciDevice(pci_device_address.kind());
        }

        DeviceKind::Unknown
    }

    fn children(&self) -> DeviceChildrenIterator {
        match self.kind() {
            DeviceKind::PciBus => {
                let parent_pci_address = self.pci_address.expect("PCI bus with no PCI address?");

                DeviceChildrenIterator::PCI {
                    parent_pci_address,
                    next_slot: 1,
                }
            }

            _ => DeviceChildrenIterator::Empty,
        }
    }
}

impl Display for Device {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.id)?;
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
    devices: HashSet<DeviceId>,
    parent_id: HashMap<DeviceId, DeviceId>,
    pci_addresses: HashMap<DeviceId, PciDeviceAddress>,
    id_by_pci_address: HashMap<PciDeviceAddress, DeviceId>,
    acpi_addresses: HashMap<DeviceId, AcpiDeviceAddress>,
    id_by_acpi_address: HashMap<AcpiDeviceAddress, DeviceId>,
}

impl DeviceManager {
    pub fn add_pci_device(
        &mut self,
        pci_address: PciDeviceAddress,
        parent_id: Option<DeviceId>,
    ) -> DeviceId {
        if let Some(device_id) = self.id_by_pci_address.get(&pci_address) {
            return *device_id;
        }

        let device_id = DeviceId::new();
        self.devices.insert(device_id);
        self.id_by_pci_address.insert(pci_address, device_id);
        self.pci_addresses.insert(device_id, pci_address);
        device_id
    }

    pub fn add_acpi_device(
        &mut self,
        acpi_address: AcpiDeviceAddress,
        parent_id: Option<DeviceId>,
    ) -> DeviceId {
        let device_id = if let Some(pci_address) = acpi_address.pci_address() {
            if let Some(device_id) = self.id_by_pci_address.get(&pci_address) {
                return *device_id;
            }

            let device_id = DeviceId::new();
            self.pci_addresses.insert(device_id, pci_address);
            self.id_by_pci_address.insert(pci_address, device_id);

            device_id
        } else {
            DeviceId::new()
        };

        self.devices.insert(device_id);

        if let Some(parent_acpi_address) = acpi_address.parent() {
            if let Some(parent_device_id) = self.id_by_acpi_address.get(&parent_acpi_address) {
                self.parent_id.insert(device_id, *parent_device_id);
            } else {
                self.parent_id.remove(&device_id);
            }
        }

        self.acpi_addresses.insert(device_id, acpi_address.clone());
        self.id_by_acpi_address.insert(acpi_address.clone(), device_id);

        if let Some(parent_id) = parent_id {
            self.parent_id.insert(device_id, parent_id);
        }

        device_id
    }

    fn get(&self, id: &DeviceId) -> Option<Device> {
        if !self.devices.contains(&id) {
            return None;
        }

        let parent_id = self.parent_id.get(&id).map(|x| *x);
        let pci_address = self.pci_addresses.get(&id).map(|x| *x);
        let acpi_address = self.acpi_addresses.get(&id).map(|x| x.clone());

        Some(Device {
            id: *id,
            parent_id,
            acpi_address,
            pci_address,
        })
    }

    fn insert_or_update_device(&mut self, new_device: &Device) {
        self.devices.insert(new_device.id);

        new_device
            .parent_id
            .map(|parent_id| self.parent_id.insert(new_device.id, parent_id))
            .unwrap_or_else(|| self.parent_id.remove(&new_device.id));

        new_device
            .acpi_address
            .clone()
            .map(|acpi_address| self.acpi_addresses.insert(new_device.id, acpi_address))
            .unwrap_or_else(|| self.acpi_addresses.remove(&new_device.id));

        new_device
            .pci_address
            .map(|pci_address| self.pci_addresses.insert(new_device.id, pci_address))
            .unwrap_or_else(|| self.pci_addresses.remove(&new_device.id));
    }

    fn find_by_acpi_address(&mut self, acpi_address: &AcpiDeviceAddress) -> Option<Device> {
        if let Some(device_id) = self.id_by_acpi_address.get(acpi_address) {
            return self.get(device_id);
        }

        None
    }

    fn find_by_pci_address(&mut self, pci_address: &PciDeviceAddress) -> Option<Device> {
        if let Some(device_id) = self.id_by_pci_address.get(pci_address) {
            return self.get(device_id);
        }

        None
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

pub fn start_all_devices(executor: &mut Executor) {
    println!("Devices:");

    let device_manager = device_manager();

    for device_id in device_manager.devices.iter() {
        let device = device_manager.get(device_id).unwrap();
        println!("  - {} ({:?})", device, device.kind());
        start_device_driver(executor, &device);
    }
}
