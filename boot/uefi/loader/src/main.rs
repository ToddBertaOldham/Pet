//**************************************************************************************************
// main.rs                                                                                         *
// Copyright (c) 2018-2019 Todd Berta-Oldham                                                       *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#![no_std]
#![no_main]

#[macro_use]
extern crate uefi_core;
extern crate alloc;

mod paging;

use self::paging::UefiPagingAllocator;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::convert::TryFrom;
use core::mem;
use elf;
use kernel_init::{KernelArgs, KernelMainFunction, MemoryInfo, MemoryMapEntry};
use uefi_core::graphics;
use uefi_core::io::storage;
use uefi_core::memory::{MemoryMap, MemoryMapKey, MemoryPages, MemoryType};
use uefi_core::system;
use uefi_core::{Error, Handle, Status, SystemTable};
use x86::control_registers::cr3;
use x86::paging::size_64::{operations as paging_operations, PageTable, VirtualAddress};

//TODO Temp fix. Remove later.
#[used]
#[no_mangle]
pub static _fltused: i32 = 0;

#[no_mangle]
pub unsafe extern "C" fn efi_main(image_handle: Handle, system_table: *mut SystemTable) -> Status {
    system::init(image_handle, system_table).expect("Failed to initialize UEFI system.");
    main();
}

fn main() -> ! {
    initialize_graphics_and_console();

    let mut memory_info = MemoryInfo::default();
    let mut args = KernelArgs::default();

    let entry_address = load_kernel();

    printrln!("Kernel main at {:#X}.", entry_address);
    printrln!("Preparing to create memory map and then jump...");

    let (key, mut memory_map) = obtain_memory_map();
    memory_info.set_memory_map(Some(&mut memory_map));

    system::exit(key).expect("Failed to exit boot.");

    args.set_memory_info(memory_info);

    unsafe {
        let kernel_main: KernelMainFunction = mem::transmute(entry_address);
        (kernel_main)(&args);
    }
}

fn initialize_graphics_and_console() {
    let buffer =
        graphics::OutputBuffer::locate().expect("Failed to locate graphics output buffer.");

    let mut output = buffer.open(0).expect("Failed to open graphics output.");
    output
        .set_closest_mode_from_resolution(1280, 720, true)
        .expect("Failed to set graphics output resolution.");

    printrln!("Pet UEFI Boot Loader");
    printrln!("Copyright (c) 2018-2019 Todd Berta-Oldham");

    if cfg!(debug_assertions) {
        printrln!("This is a debug build.");
    }

    match output.framebuffer_address() {
        Some(address) => {
            printrln!(
                "Graphics output initialized at address {:#X} with {}x{} resolution.",
                address,
                output.width(),
                output.height()
            );
        }
        None => panic!("Graphics output could not be initialized with a linear framebuffer."),
    }
}

fn obtain_memory_map() -> (MemoryMapKey, Box<[MemoryMapEntry]>) {
    let mut uefi_map = MemoryMap::get().expect("Failed to get memory map.");
    let mut memory_map_buffer = Vec::<kernel_init::MemoryMapEntry>::new();

    while memory_map_buffer.capacity() < uefi_map.len() {
        memory_map_buffer.reserve(uefi_map.len() - memory_map_buffer.capacity());
        uefi_map = MemoryMap::get().expect("Failed to get memory map.");
    }

    memory_map_buffer.resize_with(uefi_map.len(), kernel_init::MemoryMapEntry::default);

    let key = uefi_map.key();
    let mut memory_map = memory_map_buffer.into_boxed_slice();

    for (index, uefi_entry) in uefi_map.iter().enumerate() {
        let entry = &mut memory_map[index];
        entry.set_start(uefi_entry.physical_start() as usize);
        entry.set_end(uefi_entry.physical_end() as usize);
        entry.set_entry_type(convert_memory_type(uefi_entry.region_type()));
    }

    mem::forget(uefi_map);

    (key, memory_map)
}

fn convert_memory_type(memory_type: MemoryType) -> kernel_init::MemoryMapEntryType {
    match memory_type {
        MemoryType::LOADER_CODE
        | MemoryType::LOADER_DATA
        | MemoryType::BOOT_SERVICES_CODE
        | MemoryType::BOOT_SERVICES_DATA
        | MemoryType::CONVENTIONAL => kernel_init::MemoryMapEntryType::Conventional,
        MemoryType::RUNTIME_SERVICES_CODE | MemoryType::RUNTIME_SERVICES_DATA => {
            kernel_init::MemoryMapEntryType::Firmware
        }
        MemoryType::UNUSABLE => kernel_init::MemoryMapEntryType::Unusable,
        MemoryType::PERSISTENT_MEMORY => kernel_init::MemoryMapEntryType::Persistent,
        MemoryType::ACPI_RECLAIM => kernel_init::MemoryMapEntryType::AcpiReclaim,
        MemoryType::ACPI_MEMORY_NVS => kernel_init::MemoryMapEntryType::AcpiNvs,
        MemoryType::MEMORY_MAPPED_IO | MemoryType::MEMORY_MAPPED_IO_PORT_SPACE => {
            kernel_init::MemoryMapEntryType::MemoryMappedIo
        }
        _ => kernel_init::MemoryMapEntryType::ReservedOther,
    }
}

fn load_kernel() -> u64 {
    let kernel_buffer = read_kernel_from_disk().into_boxed_slice();
    let kernel_file = elf::File::new(kernel_buffer.as_ref());

    let id_header = kernel_file
        .read_identity_header()
        .expect("Failed to read kernel identification header.");

    if !id_header.is_valid() {
        panic!("Kernel is not a valid ELF file.");
    }

    if !id_header.is_64bit() {
        panic!("Kernel is not 64 bit.");
    }

    if !id_header.is_little_endian() {
        panic!("Kernel is not little endian.");
    }

    printrln!("Kernel is valid.");

    let header = kernel_file
        .read_header()
        .expect("Failed to read kernel header.");

    let memory_range = kernel_file
        .memory_range()
        .expect("Failed to read kernel memory range.");

    printrln!("Kernel requires {} byte(s) of memory.", memory_range.len());

    let mut pages = MemoryPages::allocate_for(memory_range.len())
        .expect("Failed to allocate pages for kernel.");

    let page_count = pages.len();

    printrln!("Allocated {} page(s) for kernel.", page_count);

    let pages_slice = pages.as_mut_slice();
    kernel_file
        .load_to(pages_slice)
        .expect("Failed to load kernel to paged memory.");

    printrln!("Loaded kernel at {:#X}.", pages_slice.as_ptr() as usize);

    unsafe {
        let page_allocator = UefiPagingAllocator;

        let cr3_value = cr3::read();
        let page_table = &mut *(cr3_value.physical_address() as *mut PageTable);

        for i in 0..page_count {
            let offset = i * 4096;

            let physical_address = pages_slice.as_ptr().add(offset);
            let adjust_memory_range = memory_range.start_address() + offset;
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
    }

    printrln!(
        "Successfully mapped kernel to {:#X}.",
        memory_range.start_address()
    );

    mem::forget(pages);

    header.entry()
}

fn read_kernel_from_disk() -> Vec<u8> {
    printrln!("Searching for kernel...");

    let provider =
        storage::VolumeBuffer::locate().expect("Failed to locate volume provider buffer.");

    let mut kernel_buffer = Vec::new();

    for (index, volume_result) in provider.iter().enumerate() {
        printrln!("Checking volume {}...", index);

        match volume_result.and_then(|volume| volume.root_node()) {
            Ok(root) => match root.open_node("boot\\kernel", true, false) {
                Ok(kernel_node) => {
                    printrln!("Kernel found. Reading...");
                    kernel_node
                        .read_to_end(&mut kernel_buffer)
                        .expect("Failed to read kernel from disk.");
                    printrln!("Read {} bytes from disk.", kernel_buffer.len());
                    return kernel_buffer;
                }
                Err(error) => {
                    if let Error::PathNonExistent(_) = &error {
                        printrln!("Kernel not found.");
                        continue;
                    }
                    panic!(
                        "The error \"{}\" occurred while trying to find and open kernel.",
                        &error
                    );
                }
            },
            Err(error) => printrln!(
                "The error \"{}\" occurred while trying to open volume {}. Skipping...",
                error,
                index
            ),
        }
    }

    panic!("Failed to find and read kernel.")
}
