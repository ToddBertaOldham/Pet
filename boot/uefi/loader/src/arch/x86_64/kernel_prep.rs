//**************************************************************************************************
// kernel_prep.rs                                                                                  *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use super::paging::UefiPagingAllocator;
use core::convert::TryFrom;
use elf;
use uefi_core::memory;
use x86::control::size_64::register_3 as cr3;
use x86::paging::level_4::{operations as paging_operations, PageTable, VirtualAddress};

pub fn assert_headers_match(identity_header: &elf::IdentityHeader, header: &elf::Header) {
    assert_eq!(header.machine, elf::Machine::X86_64, "Kernel is not x86_64.");
}

pub fn finish_loading_kernel(
    loaded_memory: &mut [u8],
    page_count: usize,
    loaded_memory_segment: memory::Segment,
) {
    unsafe {
        let page_allocator = UefiPagingAllocator;

        let cr3_value = cr3::read();
        let page_table = &mut *(cr3_value.physical_address() as *mut PageTable);

        for i in 0..page_count {
            let offset = i * 4096;

            let physical_address = loaded_memory.as_ptr().add(offset);
            let adjust_memory_range = loaded_memory_segment.start() + offset;
            let virtual_address = VirtualAddress::try_from(adjust_memory_range as u64)
                .expect("Invalid virtual address.");

            paging_operations::map(
                page_table,
                physical_address,
                virtual_address,
                Some(&page_allocator),
            )
            .expect("Mapping operation failed.");
        }

        printrln!(
            "Successfully mapped kernel to {:#X}.",
            loaded_memory_segment.start()
        );
    }
}
