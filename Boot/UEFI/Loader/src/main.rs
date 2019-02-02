// *************************************************************************
// main.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

#![no_std]
#![no_main]

use uefi_core::{Handle, Status, SystemTable, printrln };
use core::fmt::Write;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "win64" fn efi_main(image_handle : Handle, system_table : *mut SystemTable) -> Status {

    uefi_core::init(image_handle, system_table);

    print_header();

    prepare_graphics();
    
    // let exit_key = prepare_memory_map(&uefi_system);
    
   // uefi_system.exit_boot(exit_key);

    loop { }
}

fn print_header() {    
    printrln!("Pet UEFI Boot Loader").unwrap();
    printrln!("Copyright 2019 Todd Berta-Oldham").unwrap();

    if cfg!(debug_assertions) {
        printrln!("This is a debug build.").unwrap();
    }
}

fn prepare_graphics() {
    let provider = uefi_core::graphics_output_provider();

    for id in 0..provider.count() {
        let output = provider.get(id);        
        printrln!("Graphics output {} is at {:#X}.", id, output.linear_framebuffer()).unwrap();
    }
}

/* fn prepare_memory_map(uefi_system : &UEFISystem) -> usize {
    let memory_map = uefi_system.memory_map();

    memory_map.key()
} */

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}