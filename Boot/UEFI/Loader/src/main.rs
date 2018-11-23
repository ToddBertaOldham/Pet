// *************************************************************************
// main.rs
// Copyright 2018 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

#![no_std]
#![no_main]

extern crate uefi_core;

use uefi_core::*;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "win64" fn efi_main(image_handle : Handle, system_table : *mut SystemTable) -> Status {
    let program = UEFIProgram::new(image_handle, system_table);

    prepare_displays(&program);
    
    program.write_to_console("Pet UEFI Bootloader\r\n");
    program.write_to_console("Copyright 2018 Todd Berta-Oldham\r\n\r\n");

    let exit_key = prepare_memory_map(&program);
    
    program.exit_boot(exit_key);

    loop { }
}

fn prepare_displays(program : &UEFIProgram) {
    let display_manager = program.display_manager();

}

fn prepare_memory_map(program : &UEFIProgram) -> usize {
    let memory_map = program.memory_map();

    memory_map.key()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}