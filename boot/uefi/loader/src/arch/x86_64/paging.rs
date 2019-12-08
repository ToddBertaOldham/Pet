//**************************************************************************************************
// paging.rs                                                                                       *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use core::mem;
use uefi_core::memory::MemoryPages;
use x86::paging::size_64::{PageTable, PagingAllocator, PagingError};

pub struct UefiPagingAllocator;

impl PagingAllocator for UefiPagingAllocator {
    fn allocate_page_table(&self) -> Result<*mut PageTable, PagingError> {
        let mut pages =
            MemoryPages::allocate(1).expect("Failed to allocate page for new page table.");
        let page_table = pages.as_mut_slice().as_mut_ptr() as *mut PageTable;
        mem::forget(pages);
        Ok(page_table)
    }
}
