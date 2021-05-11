//**************************************************************************************************
// kernel_prep.rs                                                                                  *
// Copyright (c) 2019-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::arch;
use alloc::vec::Vec;
use core::convert::{TryFrom, TryInto};
use core::mem;
use kernel_interface::init;
use uefi::configuration::Table;
use uefi::io::storage::Volume;
use uefi::io::Endian;
use uefi::memory::{MemoryMap, MemoryMapKey, MemoryPages, Segment};
use uefi::system;

pub fn run_and_jump() -> ! {
    let mut args = init::Args::default();

    let mut memory_modifiers = Vec::<init::MemorySection>::new();

    con_out_println!("Starting kernel prep.");

    let mut volume = Volume::containing_current_image().expect("Failed to obtain storage volume");

    let entry_address = load_kernel(&mut volume, &mut memory_modifiers);

    create_kernel_stack(&mut memory_modifiers);

    //load_initial(&mut volume, &mut args);

    obtain_configuration_tables(&mut args);

    con_out_println!("Obtaining the memory map and then jumping to kernel.");

    let key = obtain_memory_map(&mut args, memory_modifiers);

    system::exit(key).expect("Failed to exit boot services.");

    unsafe {
        arch::kernel_prep::enter_kernel(entry_address, args);
    }

    panic!("Kernel returned from entry.");
}

fn load_kernel(volume: &mut Volume, memory_modifiers: &mut Vec<init::MemorySection>) -> usize {
    // This code is fine for now but in the future this (and a lot of other code) should
    // avoid panicking and instead provide a more helpful error screen.

    let mut kernel_buffer = Vec::new();

    volume
        .open_node("boot\\system\\kernel", true, false)
        .expect("Failed to open kernel.")
        .read_to_end(&mut kernel_buffer)
        .expect("Failed to read kernel.");

    con_out_println!("Read kernel from disk.");

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

    con_out_println!("Kernel is valid.");
    con_out_println!("Kernel entry at {:#X}.", header.entry);

    let load_memory_segment = kernel_file
        .load_memory_segment()
        .expect("Failed to get kernel load memory segment.");

    con_out_println!(
        "Kernel requires {} byte(s) of memory.",
        load_memory_segment.len()
    );

    let mut pages = MemoryPages::allocate_for(load_memory_segment.len())
        .expect("Failed to allocate pages for kernel.");

    let page_count = pages.len();

    con_out_println!("Allocated {} page(s) for kernel.", page_count);

    let pages_slice = pages.as_mut_slice();

    kernel_file
        .load_to(pages_slice)
        .expect("Failed to load kernel to paged memory.");

    memory_modifiers.push(init::MemorySection {
        start: pages_slice.as_ptr() as usize,
        len: load_memory_segment.len(),
        memory_type: init::MemoryType::KERNEL,
    });

    con_out_println!("Loaded kernel at {:#X}.", pages_slice.as_ptr() as usize);

    arch::paging::map(pages_slice, load_memory_segment, page_count);

    con_out_println!("Mapped kernel to {:#X}.", load_memory_segment.start());

    mem::forget(pages);

    usize::try_from(header.entry).expect("Kernel entry address is too large.")
}

fn create_kernel_stack(memory_modifiers: &mut Vec<init::MemorySection>) {
    // Allocate memory for the kernel stack.

    let mut pages = MemoryPages::allocate(init::STACK_PAGES as usize)
        .expect("Failed to allocate pages for kernel stack.");

    let pages_slice = pages.as_mut_slice();

    con_out_println!("Allocated {} page(s) for kernel stack.", init::STACK_PAGES);

    memory_modifiers.push(init::MemorySection {
        start: pages_slice.as_ptr() as usize,
        len: pages_slice.len(),
        memory_type: init::MemoryType::KERNEL_STACK,
    });

    arch::paging::map(
        pages_slice,
        Segment::with_len(init::BP_STACK_VIRTUAL_BOTTOM as usize, pages_slice.len()),
        init::STACK_PAGES as usize,
    );

    mem::forget(pages);

    con_out_println!(
        "Mapped kernel stack bottom to {:#X}.",
        init::BP_STACK_VIRTUAL_BOTTOM
    );
}

fn load_initial(volume: &mut Volume, args: &mut init::Args) {
    let mut initial_buffer = Vec::new();

    volume
        .open_node("boot\\initial", true, false)
        .expect("Failed to open initial.")
        .read_to_end(&mut initial_buffer)
        .expect("Failed to read initial.");

    con_out_println!("Read initial from disk.");
}

fn obtain_configuration_tables(args: &mut init::Args) {
    unsafe {
        for table in uefi::configuration::iter_tables().unwrap() {
            match table {
                Table::Acpi1(rsdp1_ptr) => {
                    let rsdp1 = &*rsdp1_ptr;

                    // Prefer ACPI 2.0 pointer if available.

                    if args.system_info.rsdt.is_null() {
                        args.system_info.rsdt = rsdp1.rsdt_address;
                    }

                    con_out_println!("Found APIC 1 table with RSDT at {:#X}.", rsdp1.rsdt_address);
                }
                Table::Acpi2(rsdp2_ptr) => {
                    let rsdp2 = &*rsdp2_ptr;

                    if args.system_info.rsdt.is_null() {
                        args.system_info.rsdt = rsdp2.rsdt_address;
                    }

                    if args.system_info.xsdt.is_null() {
                        args.system_info.xsdt = rsdp2.xsdt_address;
                    }

                    con_out_println!(
                        "Found APIC 2 table with RSDT at {:#X} and XSDT at {:#X}.",
                        rsdp2.rsdt_address,
                        rsdp2.xsdt_address
                    );
                }
                Table::Sal(_) => {}
                Table::Mps(_) => {}
                Table::Smbios(_) => {}
                Table::Smbios3(_) => {}
                Table::Unknown(_) => {}
            }
        }
    }

    con_out_println!("Obtained configuration tables.");
}

fn obtain_memory_map(
    args: &mut init::Args,
    memory_modifiers: Vec<init::MemorySection>,
) -> MemoryMapKey {
    let mut uefi_map = MemoryMap::get().expect("Failed to get memory map.");
    let mut kernel_map = Vec::<init::MemorySection>::new();

    let modifier_capacity = memory_modifiers.len() * 2;
    let mut required_capacity = uefi_map.len() + modifier_capacity;

    loop {
        if kernel_map.capacity() < required_capacity {
            kernel_map.reserve(required_capacity - kernel_map.capacity());
            uefi_map = MemoryMap::get().expect("Failed to get memory map.");
            required_capacity = uefi_map.len() + modifier_capacity;
            continue;
        }

        for uefi_entry in uefi_map.iter() {
            let segment = uefi_entry.physical_segment();
            kernel_map.push(init::MemorySection {
                start: segment.start(),
                len: segment.len(),
                memory_type: uefi_entry.region_type().into(),
            });
        }

        args.memory_map = init::MemoryMap::from_vec(kernel_map);

        unsafe {
            for memory_modifier in memory_modifiers.iter() {
                assert_eq!(
                    args.memory_map.declare_section(*memory_modifier),
                    false,
                    "Declaring modifier sections allocated memory."
                );
            }
        }

        break;
    }

    let key = uefi_map.key();

    mem::forget(uefi_map);
    mem::forget(memory_modifiers);

    key
}
