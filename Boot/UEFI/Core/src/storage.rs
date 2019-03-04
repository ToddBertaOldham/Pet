// *************************************************************************
// storage.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use super::protocol::{ Protocol, ProtocolHandleBuffer };
use super::error::UEFIError;
use super::string::C16String;
use super::ffi::{ Status, SFS_GUID, SimpleFileSystemProtocol, FileProtocol };
use core::ptr::null_mut;
use core::str::FromStr;

pub struct VolumeProvider {
    handle_buffer : ProtocolHandleBuffer
}

impl VolumeProvider {
    pub fn new() -> Result<Self, UEFIError> {
        let handle_buffer = ProtocolHandleBuffer::new(SFS_GUID)?;
         Ok(VolumeProvider { handle_buffer })
    }

    pub fn len(&self) -> usize {
        self.handle_buffer.len()
    }

    pub fn get(&self, id : usize) -> Result<Volume, UEFIError> {
        unsafe {
            let protocol = self.handle_buffer.get(id)?;
            Ok(Volume::new_unchecked(protocol))
        }
    }
}

pub struct Volume {
    protocol : Protocol
}

impl Volume {
    pub fn new(protocol : Protocol) -> Result<Self, UEFIError> {
       if protocol.guid() != SFS_GUID {
           return Err(UEFIError::InvalidArgument("protocol"));
       }
       Ok(Volume { protocol })
    }

    pub unsafe fn new_unchecked(protocol : Protocol) -> Self {
        Volume { protocol }
    }

    pub fn root_node(&self) -> Result<Node, UEFIError> {
        unsafe {
            let sfs = &*self.protocol.interface::<SimpleFileSystemProtocol>();
            let mut file_protocol = null_mut();

            let status = (sfs.open_volume)(self.protocol.interface(), &mut file_protocol);

            match status {
                Status::Success => Node::new(file_protocol),
                //TODO Errors.           
                _ => Err(UEFIError::UnexpectedFFIStatus(status))
            }         
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
            let path_pointer = converted_path.into_raw();

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

    pub fn close(self) {
        drop(self);
    }

    pub fn delete(self) -> bool {
        unsafe {
            let protocol = &*self.protocol;
            (protocol.delete)(self.protocol) == Status::Success
        }
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        unsafe {
            let protocol = &*self.protocol;
            (protocol.close)(self.protocol);
        }
    }
}