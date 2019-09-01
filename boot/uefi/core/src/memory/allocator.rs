//**************************************************************************************************
// allocator.rs                                                                                    *
// Copyright (c) 2018-2019 Todd Berta-Oldham                                                       *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::ffi::boot::MemoryType;
use crate::ffi::Status;
use crate::system;
use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;
use core::ptr;

pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.align() > 8 {
            return ptr::null_mut();
        }

        let system_table =
            &*system::table().expect("system was not initialized before allocating memory.");

        if system_table.boot_services.is_null() {
            return ptr::null_mut();
        }

        let boot_services = &*system_table.boot_services;

        let mut buffer = ptr::null_mut();
        let buffer_size = layout.size();

        match (boot_services.allocate_pool)(MemoryType::LOADER_DATA, buffer_size, &mut buffer) {
            Status::SUCCESS => buffer as *mut u8,
            _ => ptr::null_mut(),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _: Layout) {
        let system_table = &*system::table().unwrap();

        if system_table.boot_services.is_null() {
            return;
        }

        let boot_services = &*system_table.boot_services;

        (boot_services.free_pool)(ptr as *mut c_void);
    }
}
