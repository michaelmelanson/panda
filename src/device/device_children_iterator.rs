use crate::{acpi::AcpiDeviceAddress, pci::{self, PciDeviceAddress}};


pub enum DeviceChildrenIterator {
  Empty,

  PCI {
      parent_pci_address: PciDeviceAddress,
      next_slot: u16
  },
}

impl Iterator for DeviceChildrenIterator {
  type Item = (Option<AcpiDeviceAddress>, Option<PciDeviceAddress>);

  fn next(&mut self) -> Option<Self::Item> {
      match self {
          DeviceChildrenIterator::Empty => None,
          DeviceChildrenIterator::PCI {
              parent_pci_address,
              ref mut next_slot
          } => {
              if *next_slot > 0xff {
                  return None;
              }

              loop {
                  let address = PciDeviceAddress::new(parent_pci_address.segment, parent_pci_address.bus, *next_slot as u8, 0);
                  *next_slot += 1;

                  if let Some(vender_id) = pci::read::<u16>(&address, 0) {
                      if vender_id != 0xffff {
                          return Some((None, Some(address)))
                      }
                  }
              }
          }
      }
  }
}
