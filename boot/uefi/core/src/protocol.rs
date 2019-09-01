//**************************************************************************************************
// protocol.rs                                                                                     *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use super::error::Error;
use super::ffi::boot::{LocateSearchType, OpenProtocolAttributes};
use super::ffi::{Guid, Handle, Status};
use super::system;
use core::ffi::c_void;
use core::ptr;

#[derive(Debug)]
pub struct HandleBuffer {
    buffer: *mut Handle,
    len: usize,
    guid: Guid,
}

impl HandleBuffer {
    pub fn new(protocol_guid: Guid) -> Result<Self, Error> {
        unsafe {
            let system_table = &*system::table()?;

            if system_table.boot_services.is_null() {
                return Err(Error::BootServicesUnavailable);
            }

            let boot_services = &*system_table.boot_services;

            let mut len = 0;
            let mut buffer = ptr::null_mut();

            let mut guid = protocol_guid;

            let status = (boot_services.locate_handle_buffer)(
                LocateSearchType::ByProtocol,
                &mut guid,
                ptr::null_mut(),
                &mut len,
                &mut buffer,
            );

            match status {
                Status::SUCCESS => Ok(HandleBuffer { buffer, len, guid }),
                Status::OUT_OF_RESOURCES => Err(Error::OutOfMemory),
                Status::NOT_FOUND => Err(Error::NotSupported),
                _ => Err(Error::UnexpectedStatus(status)),
            }
        }
    }

    pub fn guid(&self) -> Guid {
        self.guid
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn open(&self, index: usize) -> Result<Interface, Error> {
        if index >= self.len {
            return Err(Error::InvalidArgument("index"));
        }

        unsafe { Interface::new(self.guid, *self.buffer.add(index)) }
    }
}

impl Drop for HandleBuffer {
    fn drop(&mut self) {
        unsafe {
            let system_table = &*system::table().unwrap();

            if system_table.boot_services.is_null() {
                return;
            }

            let boot_services = &*system_table.boot_services;

            (boot_services.free_pool)(self.buffer as *mut c_void);
        }
    }
}

#[derive(Debug)]
pub struct Interface {
    handle: Handle,
    protocol_guid: Guid,
    value: *mut c_void,
}

impl Interface {
    pub fn new(protocol_guid: Guid, handle: Handle) -> Result<Self, Error> {
        unsafe {
            let system_table = &*system::table()?;

            if system_table.boot_services.is_null() {
                return Err(Error::BootServicesUnavailable);
            }

            let boot_services = &*system_table.boot_services;
            let image_handle = system::handle().unwrap();

            let mut guid = protocol_guid;
            let mut value = ptr::null_mut();

            let status = (boot_services.open_protocol)(
                handle,
                &mut guid,
                &mut value,
                image_handle,
                ptr::null_mut(),
                OpenProtocolAttributes::BY_HANDLE_PROTOCOL,
            );

            match status {
                Status::SUCCESS => Ok(Interface {
                    handle,
                    protocol_guid,
                    value,
                }),
                Status::INVALID_PARAMETER => Err(Error::InvalidArgument("handle")),
                _ => Err(Error::UnexpectedStatus(status)),
            }
        }
    }

    pub fn get<T>(&self) -> *mut T {
        self.value as *mut T
    }

    pub fn protocol_guid(&self) -> Guid {
        self.protocol_guid
    }

    pub fn handle(&self) -> Handle {
        self.handle
    }
}

impl Drop for Interface {
    fn drop(&mut self) {
        unsafe {
            let system_table = &*system::table().unwrap();

            if system_table.boot_services.is_null() {
                return;
            }

            let boot_services = &*system_table.boot_services;
            let image_handle = system::handle().unwrap();
            let mut guid = self.protocol_guid;

            (boot_services.close_protocol)(self.handle, &mut guid, image_handle, ptr::null_mut());
        }
    }
}
