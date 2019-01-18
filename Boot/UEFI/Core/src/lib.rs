// *************************************************************************
// lib.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

#![no_std]

mod ffi;
mod graphics;
mod drawing;
mod memory;
mod error;

pub use self::ffi::{ SystemTable, Handle, Status };
pub use self::graphics::*;
pub use self::drawing::*;
pub use self::memory::*;
pub use self::error::*;

use self::ffi::*;
use core::ffi::c_void;
use core::ptr::null_mut;
use core::mem::size_of;
use core::result::Result;

pub struct UEFISystem {
    image_handle : Handle,
    system_table : *mut SystemTable
}

impl UEFISystem {
    pub fn new(image_handle : Handle, system_table : *mut SystemTable) -> Self {
        UEFISystem { image_handle : image_handle, system_table : system_table }      
    }

    fn check_boot_services(&self) -> Result<(), UEFIError> {
        unsafe {
            if (*self.system_table).boot_services == null_mut() {
                return Err(UEFIError::BootServicesUnavailable);
            }
            Ok(())
        }
    }

    pub fn disable_watch_timer(&self)-> Result<(), UEFIError> {
        unsafe {
            self.check_boot_services()?;

            let status : Status = ((*(*self.system_table).boot_services).set_watchdog_timer)(0, 0, 0, null_mut::<u16>());

            match status {
                Status::Success => Ok(()),
                _ => Err(UEFIError::UnexpectedFFIStatus(status))
            }
        }
    }

    pub fn exit_boot(&self, key : usize) -> Result<(), UEFIError> {        
        unsafe {
            self.check_boot_services()?;

            let status : Status = ((*(*self.system_table).boot_services).exit_boot_services)(self.image_handle, key);

            match status {
                Status::Success => Ok(()),
                Status::InvalidParameter => Err(UEFIError::InvalidMemoryMapKey),
                _ => Err(UEFIError::UnexpectedFFIStatus(status))
            }
        }
    }

    // Text Output

    fn write_out(&self, output : *mut SimpleTextOutputProtocol, string : &str) {
        let length = string.encode_utf16().count();

        if length == 0 {
            return;
        }

        unsafe {

            let boot_services : &BootServices = &*(*self.system_table).boot_services;
            let protocol : &SimpleTextOutputProtocol = &*output;

            let mut buffer : *mut c_void = null_mut();
            let buffer_size : usize = (length + 1) * size_of::<u16>();

            (boot_services.allocate_pool)(MemoryType::LoaderData, buffer_size, &mut buffer as *mut *mut c_void); 

            let characters = buffer as *mut u16;

            let mut next_character = characters;
            for char16 in string.encode_utf16() {
                (*next_character) = char16;
                next_character = next_character.offset(1);
            }

            (*next_character) = 0;

            (protocol.output_string)(output, characters);

            (boot_services.free_pool)(buffer);
        }
    }

    pub fn write_to_console(&self, string : &str) {
        unsafe {
            self.write_out((*self.system_table).con_out, string);   
        }
    }

    pub fn write_to_std_error(&self, string : &str) {
        unsafe {
            self.write_out((*self.system_table).std_error, string);       
        }
    }

    // Video 

    pub fn graphics_output_provider(&self) -> GraphicsOutputProvider {
        GraphicsOutputProvider::new(self.image_handle, self.system_table)      
    }

    // Memory

    pub fn memory_map(&self) -> MemoryMap {
        MemoryMap::new(self.system_table)
    }
}