//**************************************************************************************************
// lib.rs                                                                                          *
// Copyright (c) 2018-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#![no_std]
#![feature(alloc_layout_extra)]
#![feature(alloc_error_handler)]
#![feature(abi_efiapi)]

extern crate alloc;
#[macro_use]
extern crate enums;
#[macro_use]
extern crate bits;

mod error;
pub mod ffi;
#[macro_use]
pub mod memory;
pub mod graphics;
#[macro_use]
pub mod io;
pub mod configuration;
pub mod protocol;
pub mod system;

pub use self::error::*;
pub use self::ffi::system::Table as SystemTable;
pub use self::ffi::{Guid, Handle, Status};

use self::io::console;
use core::alloc::Layout;
use core::fmt::Write;
use core::panic::PanicInfo;

#[global_allocator]
static ALLOCATOR: memory::Allocator = memory::Allocator;

#[alloc_error_handler]
fn on_oom(_: Layout) -> ! {
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Try to avoid panicking again.
    if let Ok(mut device) = console::OutputDevice::con_out() {
        let _ = device.write_fmt(format_args!("{}", info));
    }
    loop {}
}
