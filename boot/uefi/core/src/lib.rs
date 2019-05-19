// *************************************************************************
// lib.rs
// Copyright 2018-2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

#![no_std]
#![feature(alloc)]
#![feature(alloc_layout_extra)]
#![feature(alloc_error_handler)]

extern crate alloc;
#[macro_use]
extern crate generation;

pub mod ffi;
mod error;
#[macro_use]
pub mod memory;
pub mod graphics;
#[macro_use]
pub mod io;
mod string;
pub mod protocol;
pub mod system;

pub use self::ffi::{ Handle, Status };
pub use self::ffi::system::Table as SystemTable;
pub use self::error::*;
pub use self::protocol::ProtocolProvider;

use self::ffi::boot::MemoryType;
use self::io::console;
use core::ffi::c_void;
use core::ptr;
use core::alloc::{ GlobalAlloc, Layout };
use core::panic::PanicInfo;
use core::fmt::Write;

struct UefiAllocator;

unsafe impl GlobalAlloc for UefiAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 { 
        if layout.align() > 8 {
            return ptr::null_mut();
        }

        let system_table = &*system::table().expect("system was not initialized before allocating memory.");

        if system_table.boot_services.is_null() {
            return ptr::null_mut();
        }

        let boot_services = &*system_table.boot_services;

        let mut buffer = ptr::null_mut();
        let buffer_size = layout.size();

        match (boot_services.allocate_pool)(MemoryType::LOADER_DATA, buffer_size, &mut buffer) {
            Status::SUCCESS => buffer as *mut u8,
            _ => ptr::null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let system_table = &*system::table().unwrap();

        if system_table.boot_services.is_null() {
            return;
        }

        let boot_services = &*system_table.boot_services;

        (boot_services.free_pool)(ptr as *mut c_void);    
    }
}

#[global_allocator]
static ALLOCATOR : UefiAllocator = UefiAllocator;

#[alloc_error_handler]
fn on_oom(_layout: Layout) -> ! {
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Ok(mut device) = console::OutputDevice::con_out() {
        let _ = device.write_fmt(format_args!("{}", info));
    }
    
    loop {}
}