// *************************************************************************
// protocol.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use super::ffi::{ Handle, Guid, Status };
use super::ffi::boot::{ LocateSearchType, OpenProtocolAttributes };
use super::error::Error;
use super::system;
use core::ptr;
use core::ffi::c_void;
use core::iter::Iterator;
use core::marker::Sized;

//TODO This is really just a collection trait. Maybe remove or do properly in a seperate crate.

pub trait ProtocolProvider<T> {
    fn len(&self) -> usize;
    fn open(&self, id : usize) -> Result<T, Error>;
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
    guid : Guid
}

impl ProtocolHandleBuffer {
    pub fn new(protocol_guid : Guid) -> Result<Self, Error> {
        unsafe {
            let system_table = &*system::table()?;

            if system_table.boot_services.is_null() {
                return Err(Error::BootServicesUnavailable);
            }

            let boot_services = &*system_table.boot_services;

            let mut handle_count = 0;
            let mut handle_buffer = ptr::null_mut();

            let mut guid = protocol_guid;

            let status = (boot_services.locate_handle_buffer)(LocateSearchType::ByProtocol, &mut guid, ptr::null_mut(), &mut handle_count, &mut handle_buffer);
            
            match status {
                Status::SUCCESS => Ok(ProtocolHandleBuffer { handle_buffer, handle_count, guid }),
                Status::OUT_OF_RESOURCES => Err(Error::OutOfMemory),
                Status::NOT_FOUND => Err(Error::NotSupported),
                _ => Err(Error::UnexpectedStatus(status))
            }
        }    
    }

    pub fn guid(&self) -> Guid {
        self.guid
    }
}

impl ProtocolProvider<Protocol> for ProtocolHandleBuffer {
    fn len(&self) -> usize {
        self.handle_count
    }

    fn open(&self, id : usize) -> Result<Protocol, Error> {
        if id >= self.handle_count {
            return Err(Error::InvalidArgument("id"));
        }

        unsafe {
            Protocol::new(self.guid,*self.handle_buffer.add(id))
        }
    }
}

impl Drop for ProtocolHandleBuffer {
    fn drop(&mut self) {
        unsafe {
            let system_table = &*system::table().unwrap();

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
    guid : Guid,
    interface : *mut c_void
}

impl Protocol {
    pub fn new(protocol_guid : Guid, handle : Handle) -> Result<Self, Error> {
        unsafe {
            let system_table = &*system::table()?;

            if system_table.boot_services.is_null() {
                return Err(Error::BootServicesUnavailable);
            }

            let boot_services = &*system_table.boot_services;
            let image_handle = system::handle().unwrap();

            let mut guid = protocol_guid;
            let mut interface = ptr::null_mut();

            let status = (boot_services.open_protocol)(handle, &mut guid, &mut interface, image_handle, ptr::null_mut(), OpenProtocolAttributes::BY_HANDLE_PROTOCOL);

            match status {
                Status::SUCCESS => Ok(Protocol { handle, interface, guid }),
                Status::INVALID_PARAMETER => Err(Error::InvalidArgument("handle")),
                _ => Err(Error::UnexpectedStatus(status))
            }
        }
    }

    pub fn interface<T>(&self) -> *mut T {
        self.interface as *mut T
    }

    pub fn guid(&self) -> Guid {
        self.guid
    }

    pub fn handle(&self) -> Handle {
        self.handle
    }
}

impl Drop for Protocol {
    fn drop(&mut self) {
        unsafe {      
            let system_table = &*system::table().unwrap();

            if system_table.boot_services.is_null() { 
                return; 
            }

            let boot_services = &*system_table.boot_services;
            let image_handle = system::handle().unwrap();
            let mut guid = self.guid;

            (boot_services.close_protocol)(self.handle, &mut guid, image_handle, ptr::null_mut());
        }
    }
}