
pub struct AcpiMappingHandler {
  physical_memory_base: u64
}

impl acpi::AcpiHandler for AcpiMappingHandler {
    unsafe fn map_physical_region<T>(&mut self, physical_address: usize, size: usize) -> acpi::PhysicalMapping<T> {
        self.physical_memory_base + physical_address
    }

    fn unmap_physical_region<T>(&mut self, region: acpi::PhysicalMapping<T>) {
        // nothing to do
    }
}

pub fn init(physical_memory_base: u64) {
  let acpi = unsafe { acpi::search_for_rsdp_bios(AcpiMappingHandler { physical_memory_base }).unwrap() };
  println!("ACPI: {:?}", acpi);
}
