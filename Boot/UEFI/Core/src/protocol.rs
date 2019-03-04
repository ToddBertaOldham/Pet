// *************************************************************************
// protocol.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use super::ffi::{ Handle, GUID, Status, LocateSearchType, OPEN_PROTOCOL_BY_HANDLE_PROTOCOL };
use super::error::UEFIError;
use super::system as uefi_system;
use alloc::boxed::Box;
use core::slice;
use core::ptr::null_mut;
use core::ffi::c_void;

pub struct ProtocolHandleBuffer {
    handles : Box<[Handle]>,
    guid : GUID
}

impl ProtocolHandleBuffer {
    pub fn new(protocol_guid : GUID) -> Result<Self, UEFIError> {
        unsafe {
            let system_table = &*uefi_system::system_table()?;

            if system_table.boot_services.is_null() {
                return Err(UEFIError::BootServicesUnavailable);
            }

            let boot_services = &*system_table.boot_services;

            let mut handle_count = 0;
            let mut handle_buffer = null_mut();

            let mut guid = protocol_guid;

            let status = (boot_services.locate_handle_buffer)(LocateSearchType::ByProtocol, &mut guid, null_mut(), &mut handle_count, &mut handle_buffer);
            
            match status {
                Status::Success => { 
                    let slice = slice::from_raw_parts_mut(handle_buffer, handle_count);
                    let handles = Box::from_raw(slice);
                    return Ok(ProtocolHandleBuffer { handles : handles, guid });
                },
                Status::OutOfResources => Err(UEFIError::OutOfMemory),
                Status::NotFound => Err(UEFIError::NotSupported),
                _ => Err(UEFIError::UnexpectedFFIStatus(status))
            }
        }    
    }

    pub fn guid(&self) -> GUID {
        self.guid
    }

    pub fn len(&self) -> usize {
        self.handles.len()
    }

    pub fn get(&self, id : usize) -> Result<Protocol, UEFIError> {
        match self.handles.get(id) {
            Some(handle) => Protocol::new(self.guid, *handle),
            None => Err(UEFIError::InvalidArgument("id"))
        }
    }
}


pub struct Protocol {
    handle : Handle,
    guid : GUID,
    interface : *mut c_void
}

impl Protocol {
    pub fn new(protocol_guid : GUID, handle : Handle) -> Result<Self, UEFIError> {
        unsafe {
            let system_table = &*uefi_system::system_table()?;

            if system_table.boot_services.is_null() {
                return Err(UEFIError::BootServicesUnavailable);
            }

            let boot_services = &*system_table.boot_services;
            let image_handle = uefi_system::handle().unwrap();

            let mut guid = protocol_guid;
            let mut interface = null_mut();

            let status = (boot_services.open_protocol)(handle, &mut guid, &mut interface, image_handle, null_mut(), OPEN_PROTOCOL_BY_HANDLE_PROTOCOL);

            match status {
                Status::Success => Ok(Protocol { handle, interface, guid }),
                Status::InvalidParameter => Err(UEFIError::InvalidArgument("handle")),
                _ => Err(UEFIError::UnexpectedFFIStatus(status))
            }
        }
    }

    pub fn interface<T>(&self) -> *mut T {
        self.interface as *mut T
    }

    pub fn guid(&self) -> GUID {
        self.guid
    }

    pub fn handle(&self) -> Handle {
        self.handle
    }
}

impl Drop for Protocol {
    fn drop(&mut self) {
        unsafe {      
            let system_table = &*uefi_system::system_table().unwrap();

            if system_table.boot_services.is_null() { 
                return; 
            }

            let boot_services = &*system_table.boot_services;
            let image_handle = uefi_system::handle().unwrap();
            let mut guid = self.guid;

            (boot_services.close_protocol)(self.handle, &mut guid, image_handle, null_mut());
        }
    }
}