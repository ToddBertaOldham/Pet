//**************************************************************************************************
// paging.rs                                                                                       *
// Copyright (c) 2019-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use core::convert::TryFrom;
use core::mem;
use uefi_core::memory::MemoryPages;
use x86::paging::level_4::MapperAllocator;
use x86::PhysicalAddress52;

pub struct PagingAllocator;

impl MapperAllocator for PagingAllocator {
    unsafe fn alloc_table(&mut self) -> PhysicalAddress52 {
        let mut pages = MemoryPages::allocate(1).expect("Failed to allocate page for new table.");
        let page_table = pages.as_mut_slice().as_mut_ptr();
        mem::forget(pages);
        PhysicalAddress52::try_from(page_table)
            .expect("Allocated invalid physical address for page table.")
    }

    unsafe fn dealloc_table(&mut self, address: PhysicalAddress52) {
        unimplemented!()
    }
}
