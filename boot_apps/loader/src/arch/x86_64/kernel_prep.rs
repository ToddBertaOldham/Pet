//**************************************************************************************************
// kernel_prep.rs                                                                                  *
// Copyright (c) 2019-2021 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use super::paging::PagingAllocator;
use core::convert::TryFrom;
use elf;
use uefi_core::memory;
use x86::control_registers::size_64::cr3;
use x86::paging::size_64::{MapType, Mapper, Pml4Table};
use x86::{PhysicalAddress52, VirtualAddress48, VirtualAddress64};

pub fn check_headers(_: &elf::IdentityHeader, header: &elf::Header) {
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

        let mut mapper = Mapper::new(allocator);

        let table = cr3::read().physical_address().as_mut_ptr::<Pml4Table>();

        let count = u64::try_from(page_count).unwrap();

        for page in 0..count {
            mapper
                .map_level_4(
                    table,
                    virtual_address.add_table_index(page, true).unwrap(),
                    MapType::Page4Kib(physical_address.add_page_4_kib(page, true).unwrap()),
                )
                .expect("Failed to map kernel");
        }

        printrln!(
            "Successfully mapped kernel to {:#X}.",
            loaded_memory_segment.start()
        );
    }
}
