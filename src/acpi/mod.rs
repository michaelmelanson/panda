mod context_handler;
mod device_iterator;
mod mapping_handler;

use core::{fmt::Display, hash::Hash};

use crate::{acpi, pci};
use ::acpi::{Acpi, PciConfigRegions};
use alloc::boxed::Box;
use alloc::vec::Vec;
use context_handler::AmlContextHandler;
use device_iterator::DeviceIterator;
use mapping_handler::AcpiMappingHandler;
use spin::Once;

use aml::{AmlContext, AmlError, AmlName, AmlValue, DebugVerbosity};
use x86_64::PhysAddr;

use crate::{memory, pci::PciDeviceAddress};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcpiDeviceAddress(aml::AmlName);

impl Hash for AcpiDeviceAddress {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.as_string().hash(state)
    }
}

impl AcpiDeviceAddress {
    pub fn aml_name(&self) -> &aml::AmlName {
        &self.0
    }

    pub fn pci_address(&self) -> Option<PciDeviceAddress> {
        pci_address_for_acpi_address(self)
    }

    pub fn parent(&self) -> Option<AcpiDeviceAddress> {
        self.aml_name().parent().ok().map(|aml_name| AcpiDeviceAddress(aml_name))
    }
}

impl Display for AcpiDeviceAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0.as_string())
    }
}

static RSDP: Once<Acpi> = Once::new();
static DSDT_AML: Once<AmlContext> = Once::new();

fn rsdp() -> &'static Acpi {
    RSDP.call_once(|| unsafe { ::acpi::search_for_rsdp_bios(&mut AcpiMappingHandler).unwrap() })
}

fn dsdt_aml() -> &'static AmlContext {
    DSDT_AML.call_once(|| {
        let aml_table = rsdp().dsdt.as_ref().expect("No DSDT");

        println!("Parsing AML...");
        let mut context =
            aml::AmlContext::new(Box::new(AmlContextHandler), false, DebugVerbosity::All);

        let aml_table_addr =
            memory::physical_to_virtual_address(PhysAddr::new(aml_table.address as u64));
        let aml_slice = unsafe {
            core::slice::from_raw_parts::<u8>(aml_table_addr.as_ptr(), aml_table.length as usize)
        };
        context.parse_table(aml_slice).expect("Could not parse AML");

        context
    })
}

pub fn init() {
    let rsdp = rsdp();
    println!("ACPI: {:#?}", rsdp);
}

pub fn pci_config_regions() -> Option<&'static PciConfigRegions> {
    rsdp().pci_config_regions.as_ref()
}

pub fn search(start: &AmlName, name: &str) -> Result<AmlName, AmlError> {
    let name = AmlName::from_str(name)?;
    let name = name.resolve(start)?;
    let dsdt = dsdt_aml();
    let (name, _) = dsdt.namespace.search(&name, start)?;
    Ok(name)
}

pub fn get(name: &AmlName) -> Result<AmlValue, AmlError> {
    let dsdt = dsdt_aml();
    let value = dsdt.namespace.get_by_path(name);
    // debug!("ACPI: {} = {:X?}", name.as_string(), value);
    Ok(value?.clone())
}

pub fn get_as_integer(name: &AmlName) -> Result<u64, AmlError> {
    let value = get(name)?;
    value_as_integer(&value)
}

pub fn value_as_integer(value: &AmlValue) -> Result<u64, AmlError> {
    let dsdt = dsdt_aml();
    Ok(value.as_integer(&*dsdt)?)
}

pub fn devices() -> DeviceIterator {
    let mut device_names = Vec::new();
    let dsdt = dsdt_aml();

    dsdt.namespace
        .traverse(|name, _level| {
            device_names.push(name.clone());
            Ok(true)
        })
        .unwrap();

    DeviceIterator::new(device_names)
}

pub fn pci_address_for_acpi_address(acpi_address: &AcpiDeviceAddress) -> Option<PciDeviceAddress> {
    let aml_name = acpi_address.aml_name();

    println!("Finding PCI address for ACPI device {}", aml_name.as_string());

    let segment = acpi::search(aml_name, "_SEG")
        .map(|segment| acpi::get_as_integer(&segment).expect("failed to get segment number"))
        .unwrap_or(0) as u16;

    let bus = acpi::search(aml_name, "_BBN")
        .map(|bus| acpi::get_as_integer(&bus).expect("failed to get bus number"))
        .unwrap_or(0) as u8;

    let adr =
        acpi::get_as_integer(&aml_name.child(&AmlName::from_str("_ADR").unwrap())).ok()? as u32;
    let slot = (adr >> 16) as u8;
    let function = (adr & 0xff) as u8;

    let base_address = pci::base_address_for_segment(segment)
        .expect("Could not find PCI base address");

    let pci_address = PciDeviceAddress::new(
        base_address,
        segment,
        bus,
        slot,
        function,
    );

    println!(" -> PCI address is {}", pci_address);
    Some(pci_address)
}
