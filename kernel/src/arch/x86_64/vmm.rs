//**************************************************************************************************
// vmm.rs                                                                                          *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::frame::Frame;
use crate::pmm;
use crate::spinlock::Spinlock;
use core::convert::TryInto;
use kernel_interface::init::{Args, MemoryType};
use units;
use x86::control_registers::size_64::cr3::FlagsValue;
use x86::control_registers::size_64::{cr3, cr4};
use x86::paging::size_64 as paging;
use x86::paging::size_64::{MapType, MapValue, MapperAllocator, RootTable};
use x86::{cpuid, PhysicalAddress52};

pub use kernel_interface::init::{
    BP_STACK_VIRTUAL_BOTTOM, BP_STACK_VIRTUAL_TOP, KERNEL_VIRTUAL_START,
};

pub const HEAP_VIRTUAL_START: u64 = 0xffffffff80000000;

pub const PHYSICAL_MAP_VIRTUAL_START: u64 = 0xffff800000000000;

pub const LEVEL_4_PHYSICAL_MAP_SIZE: u64 = units::Information::from_tebibyte(64).bytes() as u64;

pub const LEVEL_5_PHYSICAL_MAP_SIZE: u64 = units::Information::from_pebibyte(32).bytes() as u64;

static STATE: Spinlock<Option<State>> = Spinlock::new(None);

pub unsafe fn init(args: &Args) {
    let mut state = STATE.lock();

    assert!(state.is_none(), "VMM has already been initialized.");

    let linear_address_57 = cr4::read().la57();
    let pages_1gib = cpuid::leaf_80000001::read().pages_1gib();

    let mut allocator = PageTableAllocator;

    let root_table;
    let root_table_address = allocator.alloc_4_kib_table();

    println!(
        "Created kernel root page table at {:#X}.",
        root_table_address
    );

    let mut mapper = paging::Mapper::new(&mut allocator);

    let map_type = {
        if pages_1gib {
            println!("1 Gib is the max supported page size.");
            MapType::Page1Gib
        } else {
            println!("2 Mib is the max supported page size.");
            MapType::Page2Mib
        }
    };

    let page_size = map_type.page_size_in_bytes();

    // Just check and see if 5 level paging is already enabled in case UEFI ever enables it. In
    // the future it will either be enabled here or in the bootloader. It is likely that it will
    // still need to be enabled by the kernel or bootloader sometimes.

    let physical_map_page_count;

    if linear_address_57 {
        println!("Level 5 paging is active.");
        root_table = RootTable::Pml5(root_table_address.as_mut_ptr());
        physical_map_page_count = LEVEL_5_PHYSICAL_MAP_SIZE / page_size;
    } else {
        root_table = RootTable::Pml4(root_table_address.as_mut_ptr());
        println!("Level 4 paging is active.");
        physical_map_page_count = LEVEL_4_PHYSICAL_MAP_SIZE / page_size;
    }

    // Map physical memory into the higher part of virtual memory.

    mapper
        .map(
            root_table,
            PHYSICAL_MAP_VIRTUAL_START,
            PhysicalAddress52::null(),
            map_type,
            physical_map_page_count,
        )
        .expect("Failed to create physical memory mapping.");

    println!(
        "Created physical memory mapping using {} large pages.",
        physical_map_page_count
    );

    // Map kernel sections of memory. The boot loader may use non-contiguous memory sections
    // as long as the binary data is stored in order of the virtual mapping.

    let mut kernel_virtual = KERNEL_VIRTUAL_START;

    for entry in args
        .memory_map
        .as_slice()
        .iter()
        .filter(|section| section.memory_type == MemoryType::KERNEL)
    {
        let segment = entry.as_segment();
        let page_count = (segment.len() as u64) / MapType::Page4Kib.page_size_in_bytes();

        mapper
            .map(
                root_table,
                kernel_virtual,
                segment.start(),
                MapType::Page4Kib,
                page_count,
            )
            .expect("Failed to map kernel.");

        println!(
            "Created kernel mapping for section at {:#X} using {} pages.",
            segment.start(),
            page_count,
        );

        kernel_virtual += segment.len() as u64;
    }

    println!("Created all kernel mappings.");

    // Map kernel stack.

    let mut kernel_stack_virtual = BP_STACK_VIRTUAL_BOTTOM;

    for entry in args
        .memory_map
        .as_slice()
        .iter()
        .filter(|section| section.memory_type == MemoryType::KERNEL_STACK)
    {
        let segment = entry.as_segment();
        let page_count = (segment.len() as u64) / MapType::Page4Kib.page_size_in_bytes();

        mapper
            .map(
                root_table,
                kernel_stack_virtual,
                segment.start(),
                MapType::Page4Kib,
                page_count,
            )
            .expect("Failed to map kernel.");

        println!(
            "Created kernel stack mapping for section at {:#X} using {} pages.",
            segment.start(),
            page_count,
        );

        kernel_stack_virtual += segment.len() as u64;
    }

    println!("Created all kernel stack mappings.");

    // Update CR3 with kernel page table.

    cr3::write(cr3::FlagsValue::new(root_table_address, false, false).unwrap());

    println!("Wrote new root page table to CR3.");

    // Finish initialization.

    *state = Some(State {
        kernel_table: root_table,
    });

    println!("VMM initialized.");
}

pub unsafe fn convert_physical_address_mut<T>(ptr: *mut T) -> *mut T {
    let working_ptr = ptr as *mut u8;
    working_ptr.add(PHYSICAL_MAP_VIRTUAL_START as usize) as *mut T
}

pub unsafe fn convert_physical_address<T>(ptr: *const T) -> *const T {
    let working_ptr = ptr as *const u8;
    working_ptr.add(PHYSICAL_MAP_VIRTUAL_START as usize) as *const T
}

#[derive(Debug)]
struct State {
    kernel_table: RootTable,
}

unsafe impl Send for State {}

struct PageTableAllocator;

impl paging::MapperAllocator for PageTableAllocator {
    unsafe fn alloc_4_kib_table(&mut self) -> PhysicalAddress52 {
        pmm::allocate_frame()
            .segment()
            .start()
            .try_into()
            .unwrap_or(PhysicalAddress52::null())
    }

    unsafe fn dealloc_4_kib_table(&mut self, address: PhysicalAddress52) {
        let frame = Frame::from_address(u64::from(address) as usize);
        pmm::free_frame(frame);
    }
}
