// *************************************************************************
// paging.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use x86::paging::size_64::{ PagingAllocator, PageTable, PagingError };
use core::mem;
use uefi_core::memory::MemoryPages;

pub struct UefiPagingAllocator;

impl PagingAllocator for UefiPagingAllocator {
    fn allocate_page_table(&self) -> Result<*mut PageTable, PagingError> {
        let pages = MemoryPages::allocate(1).expect("Failed to allocate page for new page table.");
        let page_table = pages.as_mut_slice().as_mut_ptr() as *mut PageTable;
        mem::forget(pages);
        Ok(page_table)
    }
}