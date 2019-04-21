// *************************************************************************
// lib.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

#![no_std]
#![feature(alloc)]
#![feature(alloc_layout_extra)]
#![feature(alloc_error_handler)]

extern crate alloc;
#[macro_use]
extern crate generation;

mod ffi;
mod error;
#[macro_use]
pub mod memory;
pub mod graphics;
#[macro_use]
pub mod text_io;
pub mod storage;
pub mod string;
pub mod protocol;
pub mod system;

pub use self::system as uefi_system;
pub use self::ffi::{ SystemTable, Handle, Status };
pub use self::error::*;
pub use self::protocol::ProtocolProvider;

use self::ffi::MemoryType;
use text_io::console_writer;
use core::ffi::c_void;
use core::ptr::null_mut;
use core::alloc::{ GlobalAlloc, Layout };
use core::panic::PanicInfo;
use core::fmt::Write;

struct UefiAllocator;

unsafe impl GlobalAlloc for UefiAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 { 
        if layout.align() > 8 {
            return null_mut();
        }

        let system_table = &*uefi_system::system_table().expect("uefi_system was not initialized before allocating memory.");

        if system_table.boot_services.is_null() {
            return null_mut();
        }

        let boot_services = &*system_table.boot_services;

        let mut buffer = null_mut();
        let buffer_size = layout.size();

        match (boot_services.allocate_pool)(MemoryType::LoaderData, buffer_size, &mut buffer) {
            Status::SUCCESS => buffer as *mut u8,
            _ => null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let system_table = &*uefi_system::system_table().unwrap();

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
    if let Ok(mut writer) = console_writer() {
        let _ = writer.write_fmt(format_args!("{}", info));
    }
    
    loop {}
}