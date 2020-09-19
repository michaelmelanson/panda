use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::{
    structures::paging::{FrameAllocator, PhysFrame, Size4KiB},
    PhysAddr,
};

pub struct PhysicalFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl PhysicalFrameAllocator {
    pub const fn new(memory_map: &'static MemoryMap) -> Self {
        Self {
            memory_map,
            next: 0,
        }
    }

    pub fn frames<'a>(&'a self) -> impl Iterator<Item = PhysFrame> {
        let regions = self
            .memory_map
            .iter()
            .filter(|r| r.region_type == MemoryRegionType::Usable);

        let ranges = regions.map(|r| r.range.start_addr()..r.range.end_addr());
        let frame_addresses = ranges.flat_map(|r| r.step_by(4096));

        frame_addresses.map(|address| PhysFrame::containing_address(PhysAddr::new(address)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for PhysicalFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let frame = self.frames().nth(self.next);
        self.next += 1;
        frame
    }
}
