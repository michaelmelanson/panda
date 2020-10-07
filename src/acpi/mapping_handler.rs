use core::ptr::NonNull;

use x86_64::PhysAddr;

use crate::memory;

#[derive(Debug, Copy, Clone)]
pub struct AcpiMappingHandler;

impl acpi::AcpiHandler for AcpiMappingHandler {
    unsafe fn map_physical_region<T>(
        &mut self,
        physical_address: usize,
        size: usize,
    ) -> acpi::PhysicalMapping<T> {
        let virtual_address =
            memory::physical_to_virtual_address(PhysAddr::new(physical_address as u64));
        let virtual_address =
            NonNull::new(virtual_address.as_mut_ptr()).expect("Could not map physical address");

        acpi::PhysicalMapping {
            physical_start: physical_address,
            virtual_start: virtual_address,
            mapped_length: size,
            region_length: size,
        }
    }

    fn unmap_physical_region<T>(&mut self, _region: acpi::PhysicalMapping<T>) {
        // nothing to do
    }
}
