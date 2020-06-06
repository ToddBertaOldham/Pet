//**************************************************************************************************
// kernel_prep.rs                                                                                  *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use super::paging::PagingAllocator;
use core::convert::TryFrom;
use elf;
use uefi_core::memory;
use x86::paging::level_4::{MapType, Mapper};
use x86::{PhysicalAddress52, VirtualAddress48};

pub fn check_headers_match(identity_header: &elf::IdentityHeader, header: &elf::Header) {
    assert_eq!(
        header.machine,
        elf::Machine::X86_64,
        "Kernel is not x86_64."
    );
}

pub fn map_pages(
    loaded_memory: &mut [u8],
    page_count: usize,
    loaded_memory_segment: memory::Segment,
) {
    unsafe {
        let virtual_address = VirtualAddress48::try_from(loaded_memory_segment.start() as u64)
            .expect("Invalid virtual address.");
        let physical_address = PhysicalAddress52::try_from(loaded_memory.as_ptr() as u64)
            .expect("Invalid physical address.");

        let allocator = &mut PagingAllocator;

        let mut mapper =
            Mapper::with_control_table(allocator).expect("Failed to create mapper.");

        mapper
            .map_multiple(
                virtual_address,
                MapType::Page4Kb(physical_address),
                page_count,
            )
            .expect("Failed to map kernel");

        printrln!(
            "Successfully mapped kernel to {:#X}.",
            loaded_memory_segment.start()
        );
    }
}
