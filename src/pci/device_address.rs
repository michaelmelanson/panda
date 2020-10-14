use core::fmt::UpperHex;

use bit_field::BitField;
use pci::{PciDeviceKind, PciDisplaySubclassKind, PciStorageSubclassKind};
use x86_64::PhysAddr;

use crate::{memory, pci};

pub enum PciBridgeType {
    PciToPciBridge,
    PciToCardbusBridge,
}

#[derive(Debug, Copy, Clone)]
pub enum PciDeviceRegister {
    // Common fields
    VendorID,
    DeviceID,
    Command,
    Status,
    RevisionID,
    ProgIF,
    Subclass,
    ClassCode,
    CacheLineSize,
    LatencyTimer,
    HeaderType,
    BuiltInSelfTest,

    // 0x00 Standard header
    BaseRegister0,
    BaseRegister1,
    BaseRegister2,
    BaseRegister3,
    BaseRegister4,
    BaseRegister5,
    CardbusCISPointer,
    SubsystemID,
    SubsystemVendorID,
    ExpansionROMBaseAddress,
    CapabilitiesPointer,
    InterruptLine,
    InterruptPIN,
    MinGrant,
    MaxLatency,

    // 0x01 PCI-to-PCI bridge
    // BaseRegister0, // already defined
    // BaseRegister1, // already defined
    PrimaryBusNumber,
    SecondaryBusNumber,
    SubordinateBusNumber,
    SecondaryLatencyTimer,
    IOBase,
    IOLimit,
    SecondaryStatus,
    MemoryBase,
    MemoryLimit,
    PrefetchableMemoryBase,
    PrefetchableMemoryLimit,
    PrefetchableBaseUpper32Bits,
    PrefetchableLimitUpper32Bits,
    IOBaseUpper16Bits,
    IOLimitUpper16Bits,
    // CapabilitiesPointer, // already defined
    // ExpansionROMBaseAddress, // already defined
    // InterruptLine, // already defined
    // InterruptPIN, // already defined
    BridgeControl,
    // 0x02 PCI-to-Cardbus
    // TODO
}

impl PciDeviceRegister {
    const fn offset(&self) -> u8 {
        match self {
            // Common fields
            PciDeviceRegister::VendorID => 0x00,
            PciDeviceRegister::DeviceID => 0x02,
            PciDeviceRegister::Command => 0x04,
            PciDeviceRegister::Status => 0x06,
            PciDeviceRegister::RevisionID => 0x08,
            PciDeviceRegister::ProgIF => 0x09,
            PciDeviceRegister::Subclass => 0x0A,
            PciDeviceRegister::ClassCode => 0x0B,
            PciDeviceRegister::CacheLineSize => 0x0C,
            PciDeviceRegister::LatencyTimer => 0x0D,
            PciDeviceRegister::HeaderType => 0x0E,
            PciDeviceRegister::BuiltInSelfTest => 0x0F,

            // 0x00 Standard header
            PciDeviceRegister::BaseRegister0 => 0x10,
            PciDeviceRegister::BaseRegister1 => 0x14,
            PciDeviceRegister::BaseRegister2 => 0x18,
            PciDeviceRegister::BaseRegister3 => 0x1C,
            PciDeviceRegister::BaseRegister4 => 0x20,
            PciDeviceRegister::BaseRegister5 => 0x24,
            PciDeviceRegister::CardbusCISPointer => 0x28,
            PciDeviceRegister::SubsystemID => 0x2C,
            PciDeviceRegister::SubsystemVendorID => 0x2E,
            PciDeviceRegister::ExpansionROMBaseAddress => 0x30,
            PciDeviceRegister::CapabilitiesPointer => 0x34,
            PciDeviceRegister::InterruptLine => 0x3C,
            PciDeviceRegister::InterruptPIN => 0x3D,
            PciDeviceRegister::MinGrant => 0x3E,
            PciDeviceRegister::MaxLatency => 0x3F,

            // 0x01 PCI-to-PCI bridge
            // BaseRegister0, // already defined
            // BaseRegister1, // already defined
            PciDeviceRegister::PrimaryBusNumber => 0x18,
            PciDeviceRegister::SecondaryBusNumber => 0x19,
            PciDeviceRegister::SubordinateBusNumber => 0x1A,
            PciDeviceRegister::SecondaryLatencyTimer => 0x1B,
            PciDeviceRegister::IOBase => 0x1C,
            PciDeviceRegister::IOLimit => 0x1D,
            PciDeviceRegister::SecondaryStatus => 0x1E,
            PciDeviceRegister::MemoryBase => 0x20,
            PciDeviceRegister::MemoryLimit => 0x22,
            PciDeviceRegister::PrefetchableMemoryBase => 0x24,
            PciDeviceRegister::PrefetchableMemoryLimit => 0x26,
            PciDeviceRegister::PrefetchableBaseUpper32Bits => 0x28,
            PciDeviceRegister::PrefetchableLimitUpper32Bits => 0x2C,
            PciDeviceRegister::IOBaseUpper16Bits => 0x30,
            PciDeviceRegister::IOLimitUpper16Bits => 0x32,
            // CapabilitiesPointer, // already defined
            // ExpansionROMBaseAddress, // already defined
            // InterruptLine, // already defined
            // InterruptPIN, // already defined
            PciDeviceRegister::BridgeControl => 0x3C,
            // 0x02 PCI-to-Cardbus
            // TODO
        }
    }

    const fn width(&self) -> u8 {
        match self {
            // Common fields
            PciDeviceRegister::VendorID => 2,
            PciDeviceRegister::DeviceID => 3,
            PciDeviceRegister::Command => 2,
            PciDeviceRegister::Status => 2,
            PciDeviceRegister::RevisionID => 1,
            PciDeviceRegister::ProgIF => 1,
            PciDeviceRegister::Subclass => 1,
            PciDeviceRegister::ClassCode => 1,
            PciDeviceRegister::CacheLineSize => 1,
            PciDeviceRegister::LatencyTimer => 1,
            PciDeviceRegister::HeaderType => 1,
            PciDeviceRegister::BuiltInSelfTest => 1,

            // 0x00 Standard header
            PciDeviceRegister::BaseRegister0 => 4,
            PciDeviceRegister::BaseRegister1 => 4,
            PciDeviceRegister::BaseRegister2 => 4,
            PciDeviceRegister::BaseRegister3 => 4,
            PciDeviceRegister::BaseRegister4 => 4,
            PciDeviceRegister::BaseRegister5 => 4,
            PciDeviceRegister::CardbusCISPointer => 4,
            PciDeviceRegister::SubsystemID => 2,
            PciDeviceRegister::SubsystemVendorID => 2,
            PciDeviceRegister::ExpansionROMBaseAddress => 4,
            PciDeviceRegister::CapabilitiesPointer => 1,
            PciDeviceRegister::InterruptLine => 1,
            PciDeviceRegister::InterruptPIN => 1,
            PciDeviceRegister::MinGrant => 1,
            PciDeviceRegister::MaxLatency => 1,

            // 0x01 PCI-to-PCI bridge
            // BaseRegister0, // already defined
            // BaseRegister1, // already defined
            PciDeviceRegister::PrimaryBusNumber => 1,
            PciDeviceRegister::SecondaryBusNumber => 1,
            PciDeviceRegister::SubordinateBusNumber => 1,
            PciDeviceRegister::SecondaryLatencyTimer => 1,
            PciDeviceRegister::IOBase => 1,
            PciDeviceRegister::IOLimit => 1,
            PciDeviceRegister::SecondaryStatus => 2,
            PciDeviceRegister::MemoryBase => 2,
            PciDeviceRegister::MemoryLimit => 2,
            PciDeviceRegister::PrefetchableMemoryBase => 2,
            PciDeviceRegister::PrefetchableMemoryLimit => 2,
            PciDeviceRegister::PrefetchableBaseUpper32Bits => 4,
            PciDeviceRegister::PrefetchableLimitUpper32Bits => 4,
            PciDeviceRegister::IOBaseUpper16Bits => 2,
            PciDeviceRegister::IOLimitUpper16Bits => 2,
            // CapabilitiesPointer, // already defined
            // ExpansionROMBaseAddress, // already defined
            // InterruptLine, // already defined
            // InterruptPIN, // already defined
            PciDeviceRegister::BridgeControl => 2,
            // 0x02 PCI-to-Cardbus
            // TODO
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum PciHeaderType {
    Standard,
    PciToPciBridge,
    PciToCardbusBridge,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PciDeviceAddress {
    pub base_address: u64,
    pub segment: u16,
    pub bus: u8,
    pub slot: u8,
    pub function: u8,
}

impl PciDeviceAddress {
    pub fn new(base_address: u64, segment: u16, bus: u8, slot: u8, function: u8) -> Self {
        PciDeviceAddress {
            base_address,
            segment,
            bus,
            slot,
            function,
        }
    }

    pub fn read<T: UpperHex + Copy>(&self, register: PciDeviceRegister) -> T {
        if core::mem::size_of::<T>() != register.width().into() {
            panic!(
                "Tried to read {} bytes PCI register {:?} but it's {} bytes wide",
                core::mem::size_of::<T>(),
                register,
                register.width()
            );
        }

        let phys_addr = PhysAddr::new(
            self.base_address
                + ((self.bus as u64) << 20)
                + ((self.slot as u64) << 15)
                + ((self.function as u64) << 12)
                + (register.offset() as u64),
        );

        let virt_addr = memory::physical_to_virtual_address(phys_addr);
        let ptr = virt_addr.as_ptr::<T>();

        let value = unsafe { *ptr };

        value
    }

    fn slot_root(&self) -> PciDeviceAddress {
        PciDeviceAddress::new(self.base_address, self.segment, self.bus, self.slot, 0)
    }

    pub fn is_valid_device(&self) -> bool {
        self.read::<u16>(PciDeviceRegister::VendorID) != 0xFFFF
    }

    pub fn header_type(&self) -> PciHeaderType {
        let header_type: u8 = self.read(PciDeviceRegister::HeaderType);
        match header_type.get_bits(0..1) {
            0x00 => PciHeaderType::Standard,
            0x01 => PciHeaderType::PciToPciBridge,
            0x02 => PciHeaderType::PciToCardbusBridge,
            header_type => unimplemented!("PCI header type {}", header_type),
        }
    }

    pub fn is_multifunction(&self) -> bool {
        self.read::<u8>(PciDeviceRegister::HeaderType).get_bit(7)
    }

    pub fn kind(&self) -> PciDeviceKind {
        let class = self.read::<u8>(PciDeviceRegister::ClassCode);
        let subclass = self.read::<u8>(PciDeviceRegister::Subclass);

        match (class, subclass) {
            (0x00, 0x00) => unimplemented!(),
            (0x01, subclass) => {
                let subclass = match subclass {
                    0x00 => PciStorageSubclassKind::SCSI,
                    0x01 => PciStorageSubclassKind::IDE,
                    0x02 => PciStorageSubclassKind::Floppy,
                    0x03 => PciStorageSubclassKind::IPI,
                    0x04 => PciStorageSubclassKind::RAID,
                    0x05 => PciStorageSubclassKind::ATA,
                    0x06 => PciStorageSubclassKind::SerialATA,
                    0x07 => PciStorageSubclassKind::SerialAttachedSCSI,
                    0x08 => PciStorageSubclassKind::NVMem,
                    0x80 => PciStorageSubclassKind::Other,
                    subclass => unimplemented!("Storage subclass {:2X}", subclass),
                };
                PciDeviceKind::Storage(subclass)
            }
            (0x02, _) => PciDeviceKind::Network,
            (0x03, subclass) => {
                let subclass = match subclass {
                    0x00 => PciDisplaySubclassKind::VGA,
                    0x01 => PciDisplaySubclassKind::XGA,
                    0x02 => PciDisplaySubclassKind::ThreeD,
                    0x80 => PciDisplaySubclassKind::Other,
                    subclass => unimplemented!("Display subclass {:2X}", subclass),
                };

                PciDeviceKind::Display(subclass)
            }
            (0x04, _) => PciDeviceKind::Multimedia,
            (0x05, _) => PciDeviceKind::Memory,
            (0x06, _) => PciDeviceKind::Bridge,
            (0x07, _) => PciDeviceKind::SimpleComms,
            (0x08, _) => PciDeviceKind::BasePeripheral,
            (0x09, _) => PciDeviceKind::Input,
            (0x0A, _) => PciDeviceKind::Dock,
            (0x0B, _) => PciDeviceKind::Processor,
            (0x0C, _) => PciDeviceKind::SerialBusController,
            (0x0D, _) => PciDeviceKind::Wireless,
            (_, _) => unimplemented!(),
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
