//**************************************************************************************************
// main.rs                                                                                         *
// Copyright (c) 2018-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#![no_std]
#![no_main]

#[macro_use]
extern crate uefi_core;
extern crate alloc;

mod arch;
mod kernel_prep;

use uefi_core::graphics;
use uefi_core::system;
use uefi_core::{Handle, Status, SystemTable};

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
    kernel_prep::run_and_jump();
}

fn initialize_graphics_and_console() {
    let buffer =
        graphics::OutputBuffer::locate().expect("Failed to locate graphics output buffer.");

    let mut output = buffer.open(0).expect("Failed to open graphics output.");
    output
        .set_closest_mode_from_resolution(1280, 720, true)
        .expect("Failed to set graphics output resolution.");

    printrln!("Verdure UEFI Boot Loader");
    printrln!("Copyright (c) 2018-2020 Aurora Berta-Oldham");

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
