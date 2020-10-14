#[derive(Debug)]
pub enum DeviceKind {
    Unknown,
    PciBus,
    PciDevice(PciDeviceKind),
    PcKeyboard,
}

#[derive(Debug)]
pub enum PciDeviceKind {
    Storage(PciStorageSubclassKind),
    Network,
    Display(PciDisplaySubclassKind),
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
}

#[derive(Debug)]
pub enum PciStorageSubclassKind {
    SCSI,
    IDE,
    Floppy,
    IPI,
    RAID,
    ATA,
    SerialATA,
    SerialAttachedSCSI,
    NVMem,
    Other,
}

#[derive(Debug)]
pub enum PciDisplaySubclassKind {
    VGA, // VGA-Compatible
    XGA,
    ThreeD,
    Other,
}
