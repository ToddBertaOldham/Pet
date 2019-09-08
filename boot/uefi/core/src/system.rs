//**************************************************************************************************
// system.rs                                                                                       *
// Copyright (c) 2018-2019 Todd Berta-Oldham                                                       *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use super::error::Error;
use super::ffi::system;
use super::ffi::{Handle, Status};
use super::memory::MemoryMapKey;

static mut IMAGE_HANDLE: Option<Handle> = None;
static mut SYSTEM_TABLE: Option<*mut system::Table> = None;

pub unsafe fn init(image_handle: Handle, system_table: *mut system::Table) -> Result<(), Error> {
    if IMAGE_HANDLE.is_some() {
        return Err(Error::AlreadyInitialized);
    }

    IMAGE_HANDLE = Some(image_handle);
    SYSTEM_TABLE = Some(system_table);

    Ok(())
}

pub unsafe fn handle() -> Result<Handle, Error> {
    match IMAGE_HANDLE {
        Some(handle) => Ok(handle),
        None => Err(Error::NotInitialized),
    }
}

pub unsafe fn table() -> Result<*mut system::Table, Error> {
    match SYSTEM_TABLE {
        Some(system_table) => Ok(system_table),
        None => Err(Error::NotInitialized),
    }
}

pub fn is_initialized() -> bool {
    unsafe { SYSTEM_TABLE.is_some() }
}

pub fn are_boot_services_available() -> Result<bool, Error> {
    unsafe {
        let system_table = &*table()?;
        Ok(!system_table.boot_services.is_null())
    }
}

pub fn exit(key: MemoryMapKey) -> Result<(), Error> {
    unsafe {
        let system_table = &*table()?;

        if system_table.boot_services.is_null() {
            return Err(Error::BootServicesUnavailable);
        }

        let boot_services = &*system_table.boot_services;
        let image_handle = IMAGE_HANDLE.unwrap();

        let status = (boot_services.exit_boot_services)(image_handle, key.into());

        match status {
            Status::SUCCESS => Ok(()),
            Status::INVALID_PARAMETER => Err(Error::InvalidArgument("key")),
            _ => Err(Error::UnexpectedStatus(status)),
        }
    }
}
