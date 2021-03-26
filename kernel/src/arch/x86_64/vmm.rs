//**************************************************************************************************
// vmm.rs                                                                                          *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::frame::Frame;
use crate::pmm;
use crate::spinlock::Spinlock;
use core::convert::TryFrom;
use units;
use x86::control_registers::size_64::cr4;
use x86::paging::size_64 as paging;
use x86::paging::size_64::{MapType, MapValue, MapperAllocator};
use x86::{cpuid, PhysicalAddress52, VirtualAddress48, VirtualAddress57, VirtualAddress64};

pub const KERNEL_VIRTUAL_START: u64 = 0xffffffff80000000;

pub const HEAP_VIRTUAL_START: u64 = 0xffffffff80000000;

pub const PHYSICAL_MAP_VIRTUAL_START: u64 = 0xffff000000000000;

pub const LEVEL_4_PHYSICAL_MAP_SIZE: u64 = units::Information::from_tebibyte(256).bytes() as u64;

pub const LEVEL_5_PHYSICAL_MAP_SIZE: u64 = units::Information::from_pebibyte(128).bytes() as u64;

static STATE: Spinlock<Option<State>> = Spinlock::new(None);

pub unsafe fn init() {
    let mut state = STATE.lock();

    assert!(state.is_none(), "VMM has already been initialized.");

    let linear_address_57 = cr4::read().la57();
    let pages_1gib = cpuid::leaf_80000001::read().pages_1gib();

    let mut allocator = PageTableAllocator;
    let mut mapper = paging::Mapper::new(&mut allocator);

    let root_table;
    let root_table_address = allocator.alloc_4_kib_table();

    let map_type = {
        if pages_1gib {
            MapType::Page1Gib
        } else {
            MapType::Page2Mib
        }
    };

    let page_size = map_type.page_size_in_bytes();

    // Just check and see if 5 level paging is already enabled in case UEFI ever enables it. In
    // the future it will either be enabled here or in the bootloader. It is likely that it will
    // still need to be enabled by the kernel or bootloader sometimes.

    if linear_address_57 {
        let page_len = LEVEL_5_PHYSICAL_MAP_SIZE / page_size;

        let pml5 = root_table_address.as_mut_ptr::<paging::Pml5Table>();
        let virtual_address = VirtualAddress57::try_from(PHYSICAL_MAP_VIRTUAL_START).unwrap();

        mapper.map_level_5(
            pml5,
            virtual_address,
            PhysicalAddress52::null(),
            map_type,
            page_len,
        );
    } else {
        let page_len = LEVEL_4_PHYSICAL_MAP_SIZE / page_size;

        let pml4 = root_table_address.as_mut_ptr::<paging::Pml4Table>();
        let mut virtual_address = VirtualAddress48::try_from(PHYSICAL_MAP_VIRTUAL_START).unwrap();

        mapper.map_level_4(
            pml4,
            virtual_address,
            PhysicalAddress52::null(),
            map_type,
            page_len,
        );
    }

    println!("VMM initialized.");
}

pub const fn convert_physical_address_mut<T>(ptr: *mut T) -> *mut T {
    ((ptr as usize) + (PHYSICAL_MAP_VIRTUAL_START as usize)) as *mut T
}

pub const fn convert_physical_address<T>(ptr: *const T) -> *const T {
    ((ptr as usize) + (PHYSICAL_MAP_VIRTUAL_START as usize)) as *const T
}

#[derive(Debug)]
struct State {}

enum Table {}

struct PageTableAllocator;

impl paging::MapperAllocator for PageTableAllocator {
    unsafe fn alloc_4_kib_table(&mut self) -> PhysicalAddress52 {
        pmm::allocate_frame().segment().into()
    }

    unsafe fn dealloc_4_kib_table(&mut self, address: PhysicalAddress52) {
        let frame = Frame::from_address(address.into());
        pmm::free_frame(frame);
    }
}
