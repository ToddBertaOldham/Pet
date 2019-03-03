// *************************************************************************
// storage.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use super::error::UEFIError;
use super::system as uefi_system;
use super::string::C16String;
use super::ffi::{ Status, Handle, LocateSearchType, SFS_GUID, SimpleFileSystemProtocol, OPEN_PROTOCOL_BY_HANDLE_PROTOCOL, FileProtocol };
use core::ptr::null_mut;
use core::ffi::c_void;
use core::str::FromStr;

pub struct VolumeProvider {
    file_volume_handles : *mut Handle,
    file_volume_handle_count : usize
}

impl VolumeProvider {
    pub fn new() -> Result<Self, UEFIError> {
        unsafe {
            let system_table = &*uefi_system::system_table()?;

            if system_table.boot_services.is_null() {
                return Err(UEFIError::BootServicesUnavailable);
            }

            let boot_services = &*system_table.boot_services;

            let mut guid = SFS_GUID;
            let mut handle_count = 0;
            let mut handle_buffer = null_mut();

            let status = (boot_services.locate_handle_buffer)(LocateSearchType::ByProtocol, &mut guid, null_mut(), &mut handle_count, &mut handle_buffer);
            
            match status {
                Status::Success => Ok(VolumeProvider { file_volume_handles : handle_buffer, file_volume_handle_count : handle_count }),
                Status::OutOfResources => Err(UEFIError::OutOfMemory),
                Status::NotFound => Err(UEFIError::NotSupported),
                _ => Err(UEFIError::UnexpectedFFIStatus(status))
            }
        }    
    }

    pub fn count(&self) -> usize {
        self.file_volume_handle_count
    }

    pub fn get(&self, id : usize) -> Result<Volume, UEFIError> {
        unsafe {
            if id >= self.file_volume_handle_count {
                return Err(UEFIError::InvalidArgument("id"));
            }

            let handle = *(self.file_volume_handles.add(id));

            Volume::new(handle)
        }
    }
}

impl Drop for VolumeProvider {
    fn drop(&mut self) {
        unsafe {
            let system_table = &*uefi_system::system_table().unwrap();

            if system_table.boot_services.is_null() { 
                return; 
            }

            let boot_services = &*system_table.boot_services;
            
            (boot_services.free_pool)(self.file_volume_handles as *mut c_void);
        }
    }
}


pub struct Volume {
    handle : Handle,
    sfs : *mut SimpleFileSystemProtocol
}

impl Volume {
    pub unsafe fn new(handle : Handle) -> Result<Self, UEFIError> {
        let system_table = &*uefi_system::system_table()?;
        let boot_services = &*system_table.boot_services;
        let image_handle = uefi_system::handle().unwrap();

        let mut guid = SFS_GUID;
        let mut interface = null_mut();

        let status = (boot_services.open_protocol)(handle, &mut guid, &mut interface, image_handle, null_mut(), OPEN_PROTOCOL_BY_HANDLE_PROTOCOL);

        match status {
            Status::Success => Ok(Volume { handle, sfs : interface as *mut SimpleFileSystemProtocol }),
            Status::InvalidParameter => Err(UEFIError::InvalidArgument("handle")),
            _ => Err(UEFIError::UnexpectedFFIStatus(status))
        }
    }

    pub fn root_node(&self) -> Result<Node, UEFIError> {
        unsafe {
            let sfs = &*self.sfs;
            let mut file_protocol = null_mut();

            let status = (sfs.open_volume)(self.sfs, &mut file_protocol);

            match status {
                Status::Success => Node::new(file_protocol),
                //TODO Errors.           
                _ => Err(UEFIError::UnexpectedFFIStatus(status))
            }         
        }
    }
}

impl Drop for Volume {
    fn drop(&mut self) {
        unsafe {
            let system_table = &*uefi_system::system_table().unwrap();

            if system_table.boot_services.is_null() { 
                return; 
            }

            let boot_services = &*system_table.boot_services;
            let image_handle = uefi_system::handle().unwrap();
            let mut guid = SFS_GUID;

            (boot_services.close_protocol)(self.handle, &mut guid, image_handle, null_mut());
        }
    }
}

pub struct Node {
    protocol : *mut FileProtocol
}

impl Node {
    pub unsafe fn new(file_protocol : *mut FileProtocol) -> Result<Node, UEFIError> {
        Ok(Node { protocol : file_protocol })
    }

    pub fn open_node(&self, path : &str) -> Result<Node, UEFIError> {
        unsafe {
            let converted_path = C16String::from_str(path)?;
            let path_pointer = C16String::into_raw(converted_path);

            let protocol = &*self.protocol;
            let mut new_protocol = null_mut();

            let status = (protocol.open)(self.protocol, &mut new_protocol, path_pointer, 0, 0);
            
            C16String::from_raw(path_pointer);

            match status {
                Status::Success => Node::new(new_protocol),
                //TODO Errors.
                _ => Err(UEFIError::UnexpectedFFIStatus(status))
            }
        }
    }
}