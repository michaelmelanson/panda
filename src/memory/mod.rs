use bootloader::BootInfo;
use frame_allocator::PhysicalFrameAllocator;
use linked_list_allocator::LockedHeap;
use x86_64::{
    structures::paging::mapper::MapToError, structures::paging::FrameAllocator,
    structures::paging::Mapper, structures::paging::OffsetPageTable, structures::paging::Page,
    structures::paging::PageTable, structures::paging::PageTableFlags,
    structures::paging::Size4KiB, VirtAddr,
};

pub mod allocator;
pub mod frame_allocator;

static HEAP_START: usize = 0x_4444_4444_0000;
static HEAP_SIZE: usize = 100 * 1024;
// static FRAME_ALLOCATOR: Mutex<PhysicalFrameAllocator> = Mutex::new(PhysicalFrameAllocator::new());

#[global_allocator]
static mut GLOBAL_ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init(boot_info: &'static BootInfo) {
    let mut frame_allocator = PhysicalFrameAllocator::new(&boot_info.memory_map);

    println!("Memory map:");
    for region in boot_info.memory_map.into_iter() {
        println!(
            " - {:#016X}-{:#016X} ({:?} KiB) {:?}",
            region.range.start_addr(),
            region.range.end_addr(),
            (region.range.end_frame_number - region.range.start_frame_number) * 4,
            region.region_type
        );
    }

    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(physical_memory_offset) };

    let mut mapper = unsafe { OffsetPageTable::new(l4_table, physical_memory_offset) };

    init_heap(&mut mapper, &mut frame_allocator).expect("Failed to init heap");

    println!("Done!");
}

unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let physical_addr = level_4_table_frame.start_address();
    let virtual_addr = physical_memory_offset + physical_addr.as_u64();
    let page_table_ptr = virtual_addr.as_mut_ptr();

    &mut *page_table_ptr
}

fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() };
    }

    println!("Intializing kernel heap...");
    unsafe {
        GLOBAL_ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}
