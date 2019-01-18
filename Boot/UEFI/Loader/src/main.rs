// *************************************************************************
// main.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

#![no_std]
#![no_main]

use uefi_core::*;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "win64" fn efi_main(image_handle : Handle, system_table : *mut SystemTable) -> Status {
    let uefi_system = UEFISystem::new(image_handle, system_table);

    prepare_graphics(&uefi_system);
    
    print_header(&uefi_system);

    let exit_key = prepare_memory_map(&uefi_system);
    
   // uefi_system.exit_boot(exit_key);

    loop { }
}

fn print_header(uefi_system : &UEFISystem) {    
    uefi_system.write_to_console("Pet UEFI Boot Loader\r\n");
    uefi_system.write_to_console("Copyright 2019 Todd Berta-Oldham\r\n");

    if cfg!(debug_assertions) {
        uefi_system.write_to_console("This is a debug build.\r\n");
    }
}

fn prepare_graphics(uefi_system : &UEFISystem) {
    let provider = uefi_system.graphics_output_provider();

}

fn prepare_memory_map(uefi_system : &UEFISystem) -> usize {
    let memory_map = uefi_system.memory_map();

    memory_map.key()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}