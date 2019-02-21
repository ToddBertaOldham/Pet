// *************************************************************************
// system.rs
// Copyright 2018 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use super::ffi::{Handle, SystemTable, Status};
use super::error::UEFIError;
use super::text_io::TextOuputWriter;
use super::graphics::GraphicsOutputProvider;
use super::memory::MemoryMap;
use core::ptr::null_mut;

static mut IMAGE_HANDLE : Option<Handle> = None;
static mut SYSTEM_TABLE : Option<*mut SystemTable> = None;

pub unsafe fn init(image_handle : Handle, system_table : *mut SystemTable) {
    IMAGE_HANDLE = Some(image_handle);
    SYSTEM_TABLE = Some(system_table);
}

pub unsafe fn handle() -> Result<Handle, UEFIError> {
    match IMAGE_HANDLE {
        Some(handle) => Ok(handle),
        None => Err(UEFIError::NotInitialized)
    }
}

pub unsafe fn system_table() -> Result<*mut SystemTable, UEFIError> {
    match SYSTEM_TABLE {
        Some(system_table) => Ok(system_table),
        None => Err(UEFIError::NotInitialized)
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

pub fn console_writer() -> Result<TextOuputWriter, UEFIError> {
    unsafe {
        let system_table = &*system_table()?;

        if system_table.con_out == null_mut() {
            return Err(UEFIError::BootServicesUnavailable);
        }

        Ok(TextOuputWriter::new(system_table.con_out))
    }
}

pub fn std_error_writer()-> Result<TextOuputWriter, UEFIError> {
    unsafe {
        let system_table = &*system_table()?;

        if system_table.std_error == null_mut() {
            return Err(UEFIError::BootServicesUnavailable);
        }

        Ok(TextOuputWriter::new(system_table.std_error))
    }
}

// Graphics

pub fn graphics_output_provider() -> Result<GraphicsOutputProvider, UEFIError> {
    GraphicsOutputProvider::new()
}

// Memory

pub fn memory_map() -> Result<MemoryMap, UEFIError> {
    MemoryMap::new()  
}