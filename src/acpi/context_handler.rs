use x86_64::PhysAddr;

use crate::memory;

#[derive(Debug)]
pub struct AmlContextHandler;

impl aml::Handler for AmlContextHandler {
    fn read_u8(&self, address: usize) -> u8 {
        let virt_addr = memory::physical_to_virtual_address(PhysAddr::new(address as u64));
        let ptr = virt_addr.as_ptr::<u8>();
        let value = unsafe { *ptr };
        debug!("ACPI: read byte {:02X} from {:?}", value, virt_addr);
        value
    }

    fn read_u16(&self, address: usize) -> u16 {
        let virt_addr = memory::physical_to_virtual_address(PhysAddr::new(address as u64));
        let ptr = virt_addr.as_ptr::<u16>();
        let value = unsafe { *ptr };
        debug!("ACPI: read word {:04X} from {:?}", value, virt_addr);
        value
    }

    fn read_u32(&self, address: usize) -> u32 {
        let virt_addr = memory::physical_to_virtual_address(PhysAddr::new(address as u64));
        let ptr = virt_addr.as_ptr::<u32>();
        let value = unsafe { *ptr };
        debug!("ACPI: read dword {:08X} from {:?}", value, virt_addr);
        value
    }

    fn read_u64(&self, address: usize) -> u64 {
        let virt_addr = memory::physical_to_virtual_address(PhysAddr::new(address as u64));
        let ptr = virt_addr.as_ptr::<u64>();
        let value = unsafe { *ptr };
        debug!("ACPI: read qword {:016X} from {:?}", value, virt_addr);
        value
    }

    fn write_u8(&mut self, address: usize, value: u8) {
        let virt_addr = memory::physical_to_virtual_address(PhysAddr::new(address as u64));
        let ptr = virt_addr.as_mut_ptr::<u8>();
        unsafe {
            *ptr = value;
        }
        debug!("ACPI: wrote byte {:02X} to {:?}", value, virt_addr);
    }

    fn write_u16(&mut self, address: usize, value: u16) {
        let virt_addr = memory::physical_to_virtual_address(PhysAddr::new(address as u64));
        let ptr = virt_addr.as_mut_ptr::<u16>();
        unsafe {
            *ptr = value;
        }
        debug!("ACPI: wrote word {:04X} to {:?}", value, virt_addr);
    }

    fn write_u32(&mut self, address: usize, value: u32) {
        let virt_addr = memory::physical_to_virtual_address(PhysAddr::new(address as u64));
        let ptr = virt_addr.as_mut_ptr::<u32>();
        unsafe {
            *ptr = value;
        }
        debug!("ACPI: wrote dword {:08X} to {:?}", value, virt_addr);
    }

    fn write_u64(&mut self, address: usize, value: u64) {
        let virt_addr = memory::physical_to_virtual_address(PhysAddr::new(address as u64));
        let ptr = virt_addr.as_mut_ptr::<u64>();
        unsafe {
            *ptr = value;
        }
        debug!("ACPI: wrote qword {:016X} to {:?}", value, virt_addr);
    }

    fn read_io_u8(&self, _port: u16) -> u8 {
        todo!();
    }

    fn read_io_u16(&self, _port: u16) -> u16 {
        todo!();
    }

    fn read_io_u32(&self, _port: u16) -> u32 {
        todo!();
    }

    fn write_io_u8(&self, _port: u16, _value: u8) {
        todo!();
    }

    fn write_io_u16(&self, _port: u16, _value: u16) {
        todo!();
    }

    fn write_io_u32(&self, _port: u16, _value: u32) {
        todo!();
    }

    fn read_pci_u8(&self, _segment: u16, _bus: u8, _device: u8, _function: u8, _offset: u16) -> u8 {
        todo!();
    }

    fn read_pci_u16(
        &self,
        _segment: u16,
        _bus: u8,
        _device: u8,
        _function: u8,
        _offset: u16,
    ) -> u16 {
        todo!();
    }

    fn read_pci_u32(
        &self,
        _segment: u16,
        _bus: u8,
        _device: u8,
        _function: u8,
        _offset: u16,
    ) -> u32 {
        todo!();
    }

    fn write_pci_u8(
        &self,
        _segment: u16,
        _bus: u8,
        _device: u8,
        _function: u8,
        _offset: u16,
        _value: u8,
    ) {
        todo!()
    }

    fn write_pci_u16(
        &self,
        _segment: u16,
        _bus: u8,
        _device: u8,
        _function: u8,
        _offset: u16,
        _value: u16,
    ) {
        todo!()
    }

    fn write_pci_u32(
        &self,
        _segment: u16,
        _bus: u8,
        _device: u8,
        _function: u8,
        _offset: u16,
        _value: u32,
    ) {
        todo!()
    }
}
