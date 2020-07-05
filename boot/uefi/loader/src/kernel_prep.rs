//**************************************************************************************************
// kernel_prep.rs                                                                                  *
// Copyright (c) 2019-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::arch;
use alloc::vec::Vec;
use core::convert::{TryFrom, TryInto};
use core::mem;
use kernel_init;
use uefi_core::io::storage::Volume;
use uefi_core::io::Endian;
use uefi_core::memory::{MemoryMap, MemoryMapKey, MemoryPages, MemoryType};
use uefi_core::system;

pub fn run_and_jump() -> ! {
    let mut args = kernel_init::Args::default();

    printrln!("Starting kernel prep.");

    let mut volume = Volume::containing_current_image().expect("Failed to obtain storage volume");

    let entry_address = load_kernel(&mut volume, &mut args);

    load_initial(&mut volume, &mut args);

    printrln!("Preparing to create memory map and then jump...");

    let key = obtain_memory_map(&mut args);

    system::exit(key).expect("Failed to exit boot services.");

    unsafe {
        let entry: kernel_init::EntryFunction = mem::transmute(entry_address);
        (entry)(&args);
    }

    panic!("Kernel returned from entry.");
}

fn load_kernel(volume: &mut Volume, args: &mut kernel_init::Args) -> usize {
    // This code is fine for now but in the future this (and a lot of other code) should
    // avoid panicking and instead provide a more helpful error screen.

    let mut kernel_buffer = Vec::new();

    volume
        .open_node("boot\\system\\kernel", true, false)
        .expect("Failed to open kernel.")
        .read_to_end(&mut kernel_buffer)
        .expect("Failed to read kernel.");

    printrln!("Read kernel from disk.");

    let kernel_file = elf::File::new(kernel_buffer.as_ref());

    let identity_header = kernel_file
        .read_identity_header()
        .expect("Failed to read kernel identity header.");

    assert!(identity_header.is_valid(), "Kernel binary is not valid.");

    assert_eq!(
        Endian::CURRENT,
        identity_header
            .data
            .try_into()
            .expect("Kernel endian is unknown."),
        "Kernel is the wrong endian."
    );

    let header = kernel_file
        .read_header()
        .expect("Failed to read kernel header.");

    assert_eq!(
        header.object_type,
        elf::ObjectType::EXECUTABLE,
        "Kernel is not an executable."
    );

    arch::kernel_prep::check_headers(&identity_header, &header);

    printrln!("Kernel is valid.");
    printrln!("Kernel entry at {:#X}.", header.entry);

    let load_memory_segment = kernel_file
        .load_memory_segment()
        .expect("Failed to get kernel load memory segment.");

    printrln!(
        "Kernel requires {} byte(s) of memory.",
        load_memory_segment.len()
    );

    let mut pages = MemoryPages::allocate_for(load_memory_segment.len())
        .expect("Failed to allocate pages for kernel.");

    let page_count = pages.len();

    printrln!("Allocated {} page(s) for kernel.", page_count);

    let pages_slice = pages.as_mut_slice();
    kernel_file
        .load_to(pages_slice)
        .expect("Failed to load kernel to paged memory.");

    args.memory_info.kernel_physical_start = pages_slice.as_ptr() as usize;
    args.memory_info.kernel_length = pages_slice.len();

    printrln!("Loaded kernel at {:#X}.", pages_slice.as_ptr() as usize);

    arch::kernel_prep::map_pages(pages_slice, page_count, load_memory_segment);

    mem::forget(pages);

    usize::try_from(header.entry).expect("Kernel entry address is too large.")
}

fn load_initial(volume: &mut Volume, args: &mut kernel_init::Args) {
    let mut initial_buffer = Vec::new();

    volume
        .open_node("boot\\initial", true, false)
        .expect("Failed to open initial.")
        .read_to_end(&mut initial_buffer)
        .expect("Failed to read initial.");

    printrln!("Read initial from disk.");


}

fn obtain_memory_map(args: &mut kernel_init::Args) -> MemoryMapKey {
    let mut uefi_map = MemoryMap::get().expect("Failed to get memory map.");
    let mut kernel_map = Vec::<kernel_init::MemoryMapEntry>::new();

    while kernel_map.capacity() < uefi_map.len() {
        kernel_map.reserve(uefi_map.len() - kernel_map.capacity());
        uefi_map = MemoryMap::get().expect("Failed to get memory map.");
    }

    kernel_map.resize_with(uefi_map.len(), kernel_init::MemoryMapEntry::default);

    let key = uefi_map.key();
    let mut boxed_kernel_map = kernel_map.into_boxed_slice();

    for (index, uefi_entry) in uefi_map.iter().enumerate() {
        boxed_kernel_map[index] = kernel_init::MemoryMapEntry::new(
            uefi_entry.physical_segment(),
            convert_memory_type(uefi_entry.region_type()),
        );
    }

    args.memory_info.memory_map = boxed_kernel_map.as_mut_ptr();
    args.memory_info.memory_map_count = boxed_kernel_map.len();

    mem::forget(uefi_map);
    mem::forget(boxed_kernel_map);

    key
}

fn convert_memory_type(memory_type: MemoryType) -> kernel_init::MemoryMapEntryType {
    match memory_type {
        MemoryType::LOADER_CODE
        | MemoryType::LOADER_DATA
        | MemoryType::BOOT_SERVICES_CODE
        | MemoryType::BOOT_SERVICES_DATA
        | MemoryType::CONVENTIONAL => kernel_init::MemoryMapEntryType::Conventional,
        MemoryType::MEMORY_MAPPED_IO | MemoryType::MEMORY_MAPPED_IO_PORT_SPACE => {
            kernel_init::MemoryMapEntryType::MemoryMappedIo
        }
        MemoryType::RUNTIME_SERVICES_CODE | MemoryType::RUNTIME_SERVICES_DATA => {
            kernel_init::MemoryMapEntryType::Firmware
        }
        MemoryType::UNUSABLE => kernel_init::MemoryMapEntryType::Unusable,
        MemoryType::PERSISTENT_MEMORY => kernel_init::MemoryMapEntryType::Persistent,
        MemoryType::ACPI_RECLAIM => kernel_init::MemoryMapEntryType::AcpiReclaim,
        MemoryType::ACPI_MEMORY_NVS => kernel_init::MemoryMapEntryType::AcpiNvs,
        _ => kernel_init::MemoryMapEntryType::ReservedOther,
    }
}
