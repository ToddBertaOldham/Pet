// *************************************************************************
// main.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

#![no_std]
#![no_main]
#![feature(alloc)]
#![feature(alloc_layout_extra)]
#![feature(alloc_error_handler)]

extern crate alloc;

use uefi_core::{Handle, Status, SystemTable, printrln, uefi_system, ProtocolProvider };
use uefi_core::graphics::GraphicsOutputProvider;
use uefi_core::storage::VolumeProvider;
use core::fmt::Write;
use alloc::vec::Vec;

#[no_mangle]
pub unsafe extern "win64" fn efi_main(image_handle : Handle, system_table : *mut SystemTable) -> Status {
    uefi_system::init(image_handle, system_table).expect("Failed to initialize UEFI system.");
    main();
    Status::Success    
}

fn main() {    
    {
        // Initialize graphics, print header, and then print graphics info.

        let provider = GraphicsOutputProvider::new().expect("Failed to create graphics output provider");
        
        for output in provider.iter() {
            output.maximize(true).unwrap();
        }

        printrln!("Pet UEFI Boot Loader");
        printrln!("Copyright 2019 Todd Berta-Oldham");

        if cfg!(debug_assertions) {
            printrln!("This is a debug build.");
        }

        for id in 0..provider.len() {
            let output = provider.open(id).expect("Failed to open graphics output provider.");
            match output.framebuffer_address() {
                Some(address) => printrln!("Graphics output {} initialized at address {:#X} with {}x{} resolution.", id, address, output.width(), output.height()),
                None => printrln!("Graphics output {} could not be initialized with a linear framebuffer.", id)
            }
        }
    }

    let kernel = read_kernel();
    
    loop { }
}

fn read_kernel() -> Vec<u8> {
    printrln!("Searching for kernel...");

    let provider = VolumeProvider::new().expect("Failed to create volume provider.");

    let mut kernel_buffer = Vec::new();

    for id in 0..provider.len()  {
        printrln!("Checking volume {}...", id);

        match provider.open(id).and_then(|volume| { volume.root_node() }) {
            Ok(root) => {
                match root.open_node("Boot\\Kernel", true, false) {
                    Ok(kernel_node) => { 
                        printrln!("Kernel found. Reading...");
                        kernel_node.read_to_end(&mut kernel_buffer).expect("Failed to read kernel from disk.");
                        printrln!("Read {} bytes from disk.", kernel_buffer.len());
                        return kernel_buffer;
                    }
                    Err(error) => printrln!("Kernel not found.")
                }
            },
            Err(error) => { 
                printrln!("The error \"{}\" occured while trying to open volume {}. Skipping...", error, id) 
            }
        }
    }

    panic!("Failed to find and read kernel. It either doesn't exist or an error occured.");
}