// *************************************************************************
// protocol.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use super::ffi::{ Handle, GUID, Status, LocateSearchType, OPEN_PROTOCOL_BY_HANDLE_PROTOCOL };
use super::error::UefiError;
use super::system as uefi_system;
use core::ptr::null_mut;
use core::ffi::c_void;
use core::iter::Iterator;
use core::marker::Sized;

pub trait ProtocolProvider<T> {
    fn len(&self) -> usize;
    fn open(&self, id : usize) -> Result<T, UefiError>;
    fn iter(&self) -> Iter<T> where Self : Sized {
        Iter { provider : self, index : 0 }
    }
}

pub struct Iter<'a, T> {
    provider : &'a ProtocolProvider<T>,
    index : usize
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.provider.len() {
            None
        }
        else {
            let value = self.provider.open(self.index).expect("Failed to open protocol while iterating.");
            self.index += 1;
            Some(value)
        }
    }
}

pub struct ProtocolHandleBuffer {
    handle_buffer : *mut Handle,
    handle_count : usize,
    guid : GUID
}

impl ProtocolHandleBuffer {
    pub fn new(protocol_guid : GUID) -> Result<Self, UefiError> {
        unsafe {
            let system_table = &*uefi_system::system_table()?;

            if system_table.boot_services.is_null() {
                return Err(UefiError::BootServicesUnavailable);
            }

            let boot_services = &*system_table.boot_services;

            let mut handle_count = 0;
            let mut handle_buffer = null_mut();

            let mut guid = protocol_guid;

            let status = (boot_services.locate_handle_buffer)(LocateSearchType::ByProtocol, &mut guid, null_mut(), &mut handle_count, &mut handle_buffer);
            
            match status {
                Status::SUCCESS => Ok(ProtocolHandleBuffer { handle_buffer, handle_count, guid }),
                Status::OUT_OF_RESOURCES => Err(UefiError::OutOfMemory),
                Status::NOT_FOUND => Err(UefiError::NotSupported),
                _ => Err(UefiError::UnexpectedFFIStatus(status))
            }
        }    
    }

    pub fn guid(&self) -> GUID {
        self.guid
    }
}

impl ProtocolProvider<Protocol> for ProtocolHandleBuffer {
    fn len(&self) -> usize {
        self.handle_count
    }

    fn open(&self, id : usize) -> Result<Protocol, UefiError> {
        if id >= self.handle_count {
            return Err(UefiError::InvalidArgument("id"));
        }

        unsafe {
            Protocol::new(self.guid,*self.handle_buffer.add(id))
        }
    }
}

impl Drop for ProtocolHandleBuffer {
    fn drop(&mut self) {
        unsafe {
            let system_table = &*uefi_system::system_table().unwrap();

            if system_table.boot_services.is_null() {
                return;
            }

            let boot_services = &*system_table.boot_services;

            (boot_services.free_pool)(self.handle_buffer as *mut c_void);    
        }
    }
}


pub struct Protocol {
    handle : Handle,
    guid : GUID,
    interface : *mut c_void
}

impl Protocol {
    pub fn new(protocol_guid : GUID, handle : Handle) -> Result<Self, UefiError> {
        unsafe {
            let system_table = &*uefi_system::system_table()?;

            if system_table.boot_services.is_null() {
                return Err(UefiError::BootServicesUnavailable);
            }

            let boot_services = &*system_table.boot_services;
            let image_handle = uefi_system::handle().unwrap();

            let mut guid = protocol_guid;
            let mut interface = null_mut();

            let status = (boot_services.open_protocol)(handle, &mut guid, &mut interface, image_handle, null_mut(), OPEN_PROTOCOL_BY_HANDLE_PROTOCOL);

            match status {
                Status::SUCCESS => Ok(Protocol { handle, interface, guid }),
                Status::INVALID_PARAMETER => Err(UefiError::InvalidArgument("handle")),
                _ => Err(UefiError::UnexpectedFFIStatus(status))
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