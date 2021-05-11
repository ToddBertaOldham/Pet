//**************************************************************************************************
// main.rs                                                                                         *
// Copyright (c) 2018-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(llvm_asm)]

#[macro_use]
extern crate uefi;
extern crate alloc;

mod arch;
mod kernel_prep;

use core::alloc::Layout;
use core::fmt::Write;
use core::panic::PanicInfo;
use uefi::graphics;
use uefi::io::console;
use uefi::memory;
use uefi::system;
use uefi::{Handle, Status, SystemTable};

#[global_allocator]
static ALLOCATOR: memory::Allocator = memory::Allocator;

#[no_mangle]
pub unsafe extern "C" fn efi_main(image_handle: Handle, system_table: *mut SystemTable) -> Status {
    system::init(image_handle, system_table).expect("Init failed.");
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
        .set_closest_resolution(1280, 720, true)
        .expect("Failed to set graphics output resolution.");

    con_out_println!("Verdure OS UEFI Boot Loader");
    con_out_println!("Copyright (c) 2018-2021 The Verdure Project");

    if cfg!(debug_assertions) {
        con_out_println!("This is a debug build.");
    }

    match output.framebuffer_address() {
        Some(address) => {
            con_out_println!(
                "Graphics output initialized at address {:#X} with {}x{} resolution.",
                address,
                output.width(),
                output.height()
            );
        }
        None => panic!("Graphics output could not be initialized with a linear framebuffer."),
    }
}

#[alloc_error_handler]
fn on_oom(_: Layout) -> ! {
    unsafe {
        arch::stall();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Try to avoid panicking again.
    if let Ok(mut device) = console::OutputDevice::con_out() {
        let _ = device.write_fmt(format_args!("{}", info));
    }
    unsafe {
        arch::stall();
    }
}
