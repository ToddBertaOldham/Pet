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

    program.write_console("Pet UEFI Bootloader\r\n");
    program.write_console("Copyright 2018 Todd Berta-Oldham\r\n\r\n");

    loop { }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}