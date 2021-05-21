//**************************************************************************************************
// vmm.rs                                                                                          *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::frame::Frame;
use crate::pmm;
use crate::spinlock::Spinlock;
use core::convert::TryInto;
use core::ptr;
use kernel_interface::init::{Args, MemoryType};
use units;
use x86::control_registers::size_64::{cr3, cr4};
use x86::paging::size_64 as paging;
use x86::paging::size_64::{MapType, MapValue, MapperInterface, RootTable};
use x86::{cpuid, PhysicalAddress52, VirtualAddress48, VirtualAddress57};

use core::fmt::Debug;
pub use kernel_interface::init::{
    BP_STACK_VIRTUAL_BOTTOM, BP_STACK_VIRTUAL_TOP, KERNEL_VIRTUAL_START,
};
use memory::Address64;

pub const HEAP_VIRTUAL_START: u64 = PHYSICAL_MAP_VIRTUAL_START + LEVEL_4_PHYSICAL_MAP_SIZE;

pub const PHYSICAL_MAP_VIRTUAL_START: u64 = 0xffff800000000000;

pub const LEVEL_4_PHYSICAL_MAP_SIZE: u64 = units::Information::from_tebibyte(64).bytes() as u64;

pub const LEVEL_5_PHYSICAL_MAP_SIZE: u64 = units::Information::from_pebibyte(32).bytes() as u64;

static STATE: Spinlock<Option<State>> = Spinlock::new(None);

pub unsafe fn init(args: &Args) {
    let mut state = STATE.lock();

    assert!(state.is_none(), "VMM has already been initialized.");

    let linear_address_57 = cr4::read().la57();
    let pages_1gib = cpuid::leaf_80000001::read().pages_1gib();

    let mut allocator = IdentityMapperInterface;

    let root_table;
    let final_root_table;
    let root_table_address = allocator.alloc_table();

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

    let page_size = map_type.size_in_bytes();

    // Just check and see if 5 level paging is already enabled in case UEFI ever enables it. In
    // the future it will either be enabled here or in the bootloader. It is likely that it will
    // still need to be enabled by the kernel or bootloader sometimes.

    let physical_map_page_count;

    if linear_address_57 {
        println!("Level 5 paging is active.");

        let root_table_ptr = root_table_address.as_mut_ptr();

        root_table = RootTable::Pml5(root_table_ptr);
        final_root_table = RootTable::Pml5(convert_physical_address_mut(root_table_ptr));

        physical_map_page_count = LEVEL_5_PHYSICAL_MAP_SIZE / page_size;
    } else {
        println!("Level 4 paging is active.");

        let root_table_ptr = root_table_address.as_mut_ptr();

        root_table = RootTable::Pml4(root_table_ptr);
        final_root_table = RootTable::Pml4(convert_physical_address_mut(root_table_ptr));

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
        let page_count = (segment.len() as u64) / MapType::Page4Kib.size_in_bytes();

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
        let page_count = (segment.len() as u64) / MapType::Page4Kib.size_in_bytes();

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
        kernel_table: final_root_table,
    });

    println!("VMM initialized.");
}

pub unsafe fn allocate_pages<TVirtualAddress: TryInto<u64>>(
    virtual_address: TVirtualAddress,
    len: usize,
) {
    let mut state_lock = STATE.lock();

    let mut state = state_lock.as_mut().expect("VMM not initialized.");

    let mut allocator = KernelSpaceMapperInterface;
    let mut mapper = paging::Mapper::new(&mut allocator);

    //TODO Consider handling this differently. Maybe return a result instead.

    let converted_virtual_address = virtual_address
        .try_into()
        .ok()
        .expect("Invalid virtual address for mapping.");

    for i in 0..len {
        // TODO Consider option to have mapper allocate memory for page.

        let page = pmm::allocate_frame();

        let next_virtual_address = converted_virtual_address + (4096 * i as u64);
        let next_physical_address = page.segment().start();

        mapper
            .map(
                state.kernel_table,
                next_virtual_address,
                next_physical_address,
                MapType::Page4Kib,
                1,
            )
            .expect("Failed to map page.");

        ptr::write_bytes(
            next_virtual_address as *mut u8,
            0,
            MapType::Page4Kib.size_in_bytes() as usize,
        );
    }
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

// This interface is for use before physical memory is mapped into kernel space. Its virtual
// pointer conversion method (default impl) expects that each physical address is mapped to the
// virtual address of the same value (0x1000 physical = 0x1000 virtual).

struct IdentityMapperInterface;

impl paging::MapperInterface for IdentityMapperInterface {
    unsafe fn alloc_table(&mut self) -> PhysicalAddress52 {
        pmm::allocate_frame()
            .segment()
            .start()
            .try_into()
            .expect("Failed to allocate page table.")
    }

    unsafe fn dealloc_table(&mut self, address: PhysicalAddress52) {
        let frame = Frame::from_address(u64::from(address) as usize);
        pmm::free_frame(frame);
    }
}

// This interface is for use after physical memory is mapped into kernel space. Its virtual pointer
// conversion will offset the physical address by the kernel physical mapping start address. Other
// methods will just use the identity mapper interface implementation.

struct KernelSpaceMapperInterface;

impl paging::MapperInterface for KernelSpaceMapperInterface {
    unsafe fn alloc_table(&mut self) -> PhysicalAddress52 {
        let mut allocator = IdentityMapperInterface;
        allocator.alloc_table()
    }

    unsafe fn dealloc_table(&mut self, address: PhysicalAddress52) {
        let mut allocator = IdentityMapperInterface;
        allocator.dealloc_table(address)
    }

    unsafe fn convert_to_virtual_ptr<T>(&mut self, address: PhysicalAddress52) -> *mut T {
        convert_physical_address_mut(address.as_mut_ptr())
    }
}
