// *************************************************************************
// system.rs
// Copyright 2018-2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use super::ffi::{Handle, Status};
use super::ffi::system;
use super::error::UefiError;

static mut IMAGE_HANDLE : Option<Handle> = None;
static mut SYSTEM_TABLE : Option<*mut system::Table> = None;

pub unsafe fn init(image_handle : Handle, system_table : *mut system::Table) -> Result<(), UefiError> {
    if IMAGE_HANDLE.is_some() {
        return Err(UefiError::AlreadyInitialized);
    }

    IMAGE_HANDLE = Some(image_handle);
    SYSTEM_TABLE = Some(system_table);

    Ok(())
}

pub unsafe fn handle() -> Result<Handle, UefiError> {
    match IMAGE_HANDLE {
        Some(handle) => Ok(handle),
        None => Err(UefiError::NotInitialized)
    }
}

pub unsafe fn system_table() -> Result<*mut system::Table, UefiError> {
    match SYSTEM_TABLE {
        Some(system_table) => Ok(system_table),
        None => Err(UefiError::NotInitialized)
    }
}

pub fn is_initialized() -> bool {
    unsafe {
        SYSTEM_TABLE.is_some()
    }
}

pub fn are_boot_services_available() -> Result<bool, UefiError> {
    unsafe {
        let system_table = &*system_table()?;
        Ok(!system_table.boot_services.is_null())
    }
}

pub fn exit_boot_services(key : usize) -> Result<(), UefiError> {
    unsafe {
        let system_table = &*system_table()?;

        if system_table.boot_services.is_null() {
            return Err(UefiError::BootServicesUnavailable);
        }

        let boot_services = &*system_table.boot_services;
        let image_handle = IMAGE_HANDLE.unwrap();

        let status = (boot_services.exit_boot_services)(image_handle, key);

        match status {
            Status::SUCCESS => Ok(()),
            Status::INVALID_PARAMETER => Err(UefiError::InvalidArgument("key")),
            _ => Err(UefiError::UnexpectedStatus(status))
        }
    }
}
