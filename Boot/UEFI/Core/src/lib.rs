// *************************************************************************
// lib.rs
// Copyright 2018 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

#![no_std]

mod ffi;

use self::ffi::*;
use core::ffi::c_void;

pub use self::ffi::{ SystemTable, Handle, Status };

pub struct UEFIProgram {
    image_handle : Handle,
    system_table : *mut SystemTable
}

impl UEFIProgram {
    pub fn new(image_handle : Handle, system_table : *mut SystemTable) -> Self {
        UEFIProgram { image_handle : image_handle, system_table : system_table }
    }

    pub fn disable_watch_timer(&self) {
        unsafe {
            ((*(*self.system_table).boot_services).set_watchdog_timer)(0, 0, 0, core::ptr::null_mut::<u16>());
        }
    }

    pub fn exit_boot(&self) {
        
    }

    // Text Output

    fn write_out(&self, protocol : *mut SimpleTextOutputProtocol, string : &str)
    {
        let length = string.encode_utf16().count();

        if length == 0 
        {
            return;
        }

        unsafe {

            let stb : &SystemTable = &*self.system_table;
            let bs : &BootServices = &*stb.boot_services;
            let op : &SimpleTextOutputProtocol = &*protocol;

            let mut buffer : *mut c_void = core::ptr::null_mut();
            let buffer_size : usize = (length + 1) * core::mem::size_of::<u16>();

            (bs.allocate_pool)(MemoryType::LoaderData, buffer_size, &mut buffer as *mut *mut c_void); 

            let characters = buffer as *mut u16;

            let mut next_character = characters;
            for char16 in string.encode_utf16() {
                (*next_character) = char16;
                next_character = next_character.offset(1);
            }

            (*next_character) = 0;

            (op.output_string)(protocol, buffer as *mut u16);

            (bs.free_pool)(buffer as *mut c_void);
        }
    }

    pub fn write_console(&self, string : &str) {
        unsafe {
            self.write_out((*self.system_table).con_out, string);
        }
    }



}