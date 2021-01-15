//**************************************************************************************************
// paging.rs                                                                                       *
// Copyright (c) 2019-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use core::convert::TryFrom;
use core::mem;
use uefi_core::memory::MemoryPages;
use uefi_core::Error;
use x86::paging::size_64::MapperAllocator;
use x86::PhysicalAddress52;

pub struct PagingAllocator;

impl MapperAllocator for PagingAllocator {
    unsafe fn alloc_4_kib_table(&mut self) -> PhysicalAddress52 {
        MemoryPages::allocate(1)
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
