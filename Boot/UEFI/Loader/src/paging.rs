// *************************************************************************
// paging.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use x86_64::paging::{ PagingManager, PagingError, PageTable };
use alloc::boxed::Box;

pub struct UefiPagingManager;

impl PagingManager for UefiPagingManager {
    fn allocate_page_table(&self) -> Result<*mut PageTable, PagingError> {
        // Temporary implementation.
        let page_table_box = Box::from(PageTable::new());
        Ok(Box::leak(page_table_box))
    }
}