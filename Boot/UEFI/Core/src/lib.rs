// *************************************************************************
// lib.rs
// Copyright 2018 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

#![no_std]

mod ffi;

pub use self::ffi::{ SystemTable, Handle };
use self::ffi::*;
use core::ffi::c_void;

pub struct UEFIProgram {
    image_handle : Handle,
    system_table : *mut SystemTable
}

impl UEFIProgram {
    pub fn new(image_handle : Handle, system_table : *mut SystemTable) -> Self {
        UEFIProgram { image_handle : image_handle, system_table : system_table }
    }

    pub fn disable_watch_timer(&self) {
        
    }

    pub fn exit_boot(&self) {
        
    }

    // Text Output

    fn write_out(&self, protocol : *mut SimpleTextOutputProtocol, string : &str) 
    {
        let length = string.len();

        if length == 0 
        {
            return; 
        }

        unsafe {

            let stb : &SystemTable = &*self.system_table;
            let bs : &BootServices = &*stb.boot_services;
            let op : &SimpleTextOutputProtocol = &*protocol;

            let char_16_buffer : *mut u16 = core::ptr::null_mut();
            let buffer_size : usize = (length + 1) * core::mem::size_of::<u16>();

            (bs.allocate_pool)(MemoryType::LoaderData, buffer_size, char_16_buffer as *mut *mut c_void);
        
            (*char_16_buffer.offset(length as isize)) = 0;

            (op.output_string)(protocol, char_16_buffer);

            (bs.free_pool)(char_16_buffer as *mut c_void);
        }
    }

    pub fn write_console(&self, string : &str) {
        unsafe {
            self.write_out((*self.system_table).con_out, string);
        }
    }



}