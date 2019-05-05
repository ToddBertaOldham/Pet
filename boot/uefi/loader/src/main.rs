// *************************************************************************
// main.rs
// Copyright 2018-2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

#![no_std]
#![no_main]
#![feature(alloc)]
#![feature(alloc_layout_extra)]
#![feature(alloc_error_handler)]

extern crate uefi_core;
extern crate alloc;

mod paging;

use uefi_core::{Handle, Status, SystemTable, printrln, uefi_system, ProtocolProvider, UefiError, UefiIoError };
use uefi_core::graphics::GraphicsOutputProvider;
use uefi_core::io::storage::VolumeProvider;
use uefi_core::memory::{ MemoryPages, MemoryMap };
use x86::sixty_four::paging::{ PageTable, VirtualAddress, operations as paging_operations };
use x86::control_registers::cr3;
use core::fmt::Write;
use core::mem;
use core::convert::TryFrom;
use alloc::vec::Vec;
use elf::ElfFile;
use self::paging::UefiPagingAllocator;

type KernelMainFunction = unsafe extern fn();

#[no_mangle]
pub unsafe extern "C" fn efi_main(image_handle : Handle, system_table : *mut SystemTable) -> Status {
    uefi_system::init(image_handle, system_table).expect("Failed to initialize UEFI system.");
    main();
    Status::SUCCESS    
}

fn main() {    
    initialize_graphics_and_console();

    let entry_address = load_kernel();

    printrln!("Kernel main at {:#X}.", entry_address);
    printrln!("Preparing to create memory map and then jump...");

    let map = MemoryMap::new().expect("Failed to get memory map.");

    uefi_system::exit_boot_services(map.key()).expect("Failed to exit boot.");

    unsafe {
        let entry : KernelMainFunction = mem::transmute(entry_address);
        (entry)();
    }

    loop { }
}

fn initialize_graphics_and_console() {
    let provider = GraphicsOutputProvider::new().expect("Failed to create graphics output provider.");
    
    let output = provider.open(0).expect("Failed to open graphics output.");
    output.maximize(true).expect("Failed to maximize graphics output.");

    printrln!("Pet UEFI Boot Loader");
    printrln!("Copyright 2018-2019 Todd Berta-Oldham");

    if cfg!(debug_assertions) {
        printrln!("This is a debug build.");
    }

    match output.framebuffer_address() {
        Some(address) => { 
            printrln!("Graphics output initialized at address {:#X} with {}x{} resolution.", address, output.width(),  output.height());
        },
        None => panic!("Graphics output could not be initialized with a linear framebuffer.")
    }
}

fn load_kernel() -> u64 {
    let kernel_buffer = read_kernel_from_disk().into_boxed_slice();   
    let kernel_file = ElfFile::new(kernel_buffer.as_ref());

    let id_header = kernel_file.read_identity_header().expect("Failed to read kernel identification header.");
    
    if !id_header.is_valid() {
        panic!("Kernel is not a valid ELF file. Header shows {:#X} {:#X} {:#X} {:#X}.", id_header.magic_0, id_header.magic_1, id_header.magic_2, id_header.magic_3);
    }

    if !id_header.is_64bit() {
        panic!("Kernel is not 64 bit.");
    }

    if !id_header.is_little_endian() {
        panic!("Kernel is not little endian.");
    }

    printrln!("Kernel is valid.");

    let header = kernel_file.read_header().expect("Failed to read kernel header.");

    let memory_range = kernel_file.memory_range().expect("Failed to read kernel memory range.");

    printrln!("Kernel requires {} byte(s) of memory.", memory_range.len());

    let pages = MemoryPages::allocate_for(memory_range.len()).expect("Failed to allocate pages for kernel.");

    printrln!("Allocated {} page(s) for kernel.", pages.len());

    let pages_slice = pages.as_mut_slice();
    kernel_file.load_to(pages_slice).expect("Failed to load kernel to paged memory.");

    printrln!("Loaded kernel at {:#X}.", pages_slice.as_ptr() as usize);

    unsafe {
        let page_allocator = UefiPagingAllocator;
    
        let cr3_value = cr3::read();
        let page_table = &mut *(cr3_value.physical_address() as *mut PageTable);    

        for i in 0..pages.len() {
            let offset = i * 4096;
            
            let physical_address = pages_slice.as_ptr().add(offset);
            let adjust_memory_range = memory_range.start_address() + offset;
            let virtual_address = VirtualAddress::try_from(adjust_memory_range as u64).expect("Invalid virtual address.");

            paging_operations::map(page_table, physical_address, virtual_address, Some(&page_allocator)).expect("Mapping operation failed.");
        }
    }

    printrln!("Successfully mapped kernel to {:#X}.", memory_range.start_address());  

    mem::forget(pages);

    header.entry()
}

fn read_kernel_from_disk() -> Vec<u8> {
    printrln!("Searching for kernel...");

    let provider = VolumeProvider::new().expect("Failed to create volume provider.");

    let mut kernel_buffer = Vec::new();

    for id in 0..provider.len()  {
        printrln!("Checking volume {}...", id);

        match provider.open(id).and_then(|volume| { volume.root_node() }) {
            Ok(root) => {
                match root.open_node("boot\\kernel", true, false) {
                    Ok(kernel_node) => { 
                        printrln!("Kernel found. Reading...");
                        kernel_node.read_to_end(&mut kernel_buffer).expect("Failed to read kernel from disk.");
                        printrln!("Read {} bytes from disk.", kernel_buffer.len());
                        return kernel_buffer;
                    }
                    Err(error) => {
                        if let UefiError::IoError(io_error) = &error {
                            if let UefiIoError::PathNonExistent(_) = io_error {
                                printrln!("Kernel not found.");
                                continue;
                            }
                        }
                        panic!("The error \"{}\" occured while trying to find and open kernel.", &error);
                    }
                }
            },
            Err(error) => { 
                printrln!("The error \"{}\" occured while trying to open volume {}. Skipping...", error, id) 
            }
        }
    }

    panic!("Failed to find and read kernel.");
}