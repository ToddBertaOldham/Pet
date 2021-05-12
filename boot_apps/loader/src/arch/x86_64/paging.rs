//**************************************************************************************************
// paging.rs                                                                                       *
// Copyright (c) 2019-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use core::convert::TryFrom;
use core::mem;
use uefi::memory;
use uefi::memory::{MemoryPages, MemoryType};
use x86::control_registers::size_64::cr3;
use x86::control_registers::size_64::cr3::FlagsValue;
use x86::paging::size_64::{MapType, Mapper, MapperAllocator, Pml4Table};
use x86::PhysicalAddress52;

pub fn map(physical_memory: &mut [u8], virtual_memory: memory::Segment, page_count: usize) {
    unsafe {
        let allocator = &mut PagingAllocator;

        let mut mapper = Mapper::new(allocator);

        let table = cr3::read::<FlagsValue>()
            .physical_address()
            .as_mut_ptr::<Pml4Table>();

        let count = u64::try_from(page_count).unwrap();

        mapper
            .map_level_4(
                table,
                virtual_memory.start(),
                physical_memory.as_ptr(),
                MapType::Page4Kib,
                count,
            )
            .expect("Failed to map memory.");
    }
}

struct PagingAllocator;

impl MapperAllocator for PagingAllocator {
    unsafe fn alloc_4_kib_table(&mut self) -> PhysicalAddress52 {
        MemoryPages::with_len(1, MemoryType::LOADER_DATA)
            .ok()
            .and_then(|mut pages| {
                let page_table = pages.as_mut_slice().as_mut_ptr();
                mem::forget(pages);
                PhysicalAddress52::try_from(page_table).ok()
            })
            .unwrap_or(PhysicalAddress52::null())
    }

    unsafe fn dealloc_4_kib_table(&mut self, _: PhysicalAddress52) {
        unimplemented!("dealloc_4_kib_table is not supported.")
    }
}
