use bootloader::BootInfo;
use frame_allocator::PhysicalFrameAllocator;
use linked_list_allocator::LockedHeap;
use spin::{Mutex, Once};
use x86_64::{
    structures::paging::FrameAllocator, structures::paging::Mapper,
    structures::paging::OffsetPageTable, structures::paging::Page, structures::paging::PageTable,
    structures::paging::PageTableFlags, VirtAddr,
};

pub mod frame_allocator;

pub static HEAP_START: u64 = 0x_4444_4444_0000;
pub static HEAP_SIZE: u64 = 100 * 1024;

static mut FRAME_ALLOCATOR: Once<Mutex<PhysicalFrameAllocator>> = Once::new();
static mut MAPPER: Once<Mutex<OffsetPageTable>> = Once::new();

#[global_allocator]
static mut GLOBAL_ALLOCATOR: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

pub fn init(boot_info: &'static BootInfo) {
    unsafe {
        FRAME_ALLOCATOR
            .call_once(|| Mutex::new(PhysicalFrameAllocator::new(&boot_info.memory_map)));
    }

    // println!("Memory map:");
    // for region in boot_info.memory_map.into_iter() {
    //     println!(
    //         " - {:#016X}-{:#016X} ({:?} KiB) {:?}",
    //         region.range.start_addr(),
    //         region.range.end_addr(),
    //         (region.range.end_frame_number - region.range.start_frame_number) * 4,
    //         region.region_type
    //     );
    // }

    unsafe {
        MAPPER.call_once(|| {
            let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
            let l4_table = active_level_4_table(physical_memory_offset);

            Mutex::new(OffsetPageTable::new(l4_table, physical_memory_offset))
        });
    }

    println!("Intializing kernel heap...");
    unsafe {
        GLOBAL_ALLOCATOR
            .lock()
            .init(HEAP_START as usize, HEAP_SIZE as usize);
    }

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

pub unsafe fn map_page(page: Page, flags: PageTableFlags) {
    println!("Mapping page {:?} with flags {:?}", page, flags);

    let mut mapper = MAPPER.wait().unwrap().lock();
    let mut frame_allocator = FRAME_ALLOCATOR.wait().unwrap().lock();

    let frame = frame_allocator
        .allocate_frame()
        .expect("Failed to allocate frame");

    let mapping = mapper
        .map_to(page, frame, flags, &mut *frame_allocator)
        .expect("Failed to map page");

    println!(" -> {:?}", mapping);

    mapping.flush()
}
