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

mod ffi;
mod graphics;
mod drawing;
mod memory;
mod error;
mod text_io;

pub use self::ffi::{ SystemTable, Handle, Status };
pub use self::graphics::*;
pub use self::drawing::*;
pub use self::memory::*;
pub use self::error::*;
pub use self::text_io::*;

use self::ffi::*;
use core::ffi::c_void;
use core::ptr::null_mut;
use core::result::Result;
use core::alloc::{ GlobalAlloc, Layout };

static mut IMAGE_HANDLE : Option<Handle> = None;
static mut SYSTEM_TABLE : Option<*mut SystemTable> = None;

pub fn init(image_handle : Handle, system_table : *mut SystemTable) {
    unsafe {
        IMAGE_HANDLE = Some(image_handle);
        SYSTEM_TABLE = Some(system_table);
    }
}

pub fn exit_boot(key : usize) -> Result<(), UEFIError> {
    unsafe {
        let system_table = &*SYSTEM_TABLE.expect("UEFI system has not been initialized!");
        let boot_services = &*system_table.boot_services;
        let image_handle = IMAGE_HANDLE.unwrap();

        let status = (boot_services.exit_boot_services)(image_handle, key);

        match status {
            Status::Success => Ok(()),
            _ => Err(UEFIError::UnexpectedFFIStatus(status))
        }
    }
}

// Text Output

pub fn console_writer() -> TextOuputWriter {
    unsafe {
        let system_table = &*SYSTEM_TABLE.expect("UEFI system has not been initialized!");
        TextOuputWriter::new(system_table.con_out)
    }
}

pub fn std_error_writer()-> TextOuputWriter {
    unsafe {
        let system_table = &*SYSTEM_TABLE.expect("UEFI system has not been initialized!");
        TextOuputWriter::new(system_table.std_error)
    }
}


#[macro_export]
macro_rules! writerln {
    ($dst:expr) => (
        write!($dst, "\r\n")
    );
    ($dst:expr,) => (
        writerln!($dst)
    );
    ($dst:expr, $($arg:tt)*) => (
        $dst.write_fmt(format_args!("{}\r\n", format_args!($($arg)*)))
    );
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console_writer().write_fmt(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! printrln {
    () => (print!("\r\n"));
    ($($arg:tt)*) => ($crate::console_writer().write_fmt(format_args!("{}\r\n", format_args!($($arg)*))))
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => ($crate::std_error_writer().write_fmt(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! eprintrln {
    () => (eprint!("\r\n"));
    ($($arg:tt)*) => ($crate::std_error_writer().write_fmt(format_args!("{}\r\n", format_args!($($arg)*))))
}

// Graphics

pub fn graphics_output_provider() -> GraphicsOutputProvider {
    unsafe {
        let system_table = SYSTEM_TABLE.expect("UEFI system has not been initialized!");
        let image_handle = IMAGE_HANDLE.unwrap();
        GraphicsOutputProvider::new(image_handle, system_table)     
    } 
}

// Memory

pub fn memory_map() -> MemoryMap {
    unsafe {
        let system_table = SYSTEM_TABLE.expect("UEFI system has not been initialized!");
        MemoryMap::new(system_table)
    }
}

// Allocator Definition

struct UEFIAllocator;

unsafe impl GlobalAlloc for UEFIAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 { 
        let system_table = &*SYSTEM_TABLE.expect("UEFI system has not been initialized!");
        let boot_services = &*system_table.boot_services;

        let mut buffer = null_mut::<c_void>();
        let buffer_size = layout.size();

        ((boot_services.allocate_pool)(MemoryType::LoaderData, buffer_size, &mut buffer as *mut *mut c_void));

        return buffer as *mut u8;
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let system_table = &*SYSTEM_TABLE.expect("UEFI system has not been initialized!");
        let boot_services = &*system_table.boot_services;

        (boot_services.free_pool)(ptr as *mut c_void);    
    }
}

#[global_allocator]
static ALLOCATOR : UEFIAllocator = UEFIAllocator;

#[alloc_error_handler]
fn on_oom(_layout: Layout) -> ! {
    loop {}
}