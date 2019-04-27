// *************************************************************************
// storage.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use crate::protocol::{ Protocol, ProtocolHandleBuffer, ProtocolProvider };
use crate::error::{UefiError, UefiIoError};
use crate::string::C16String;
use crate::ffi::{ Status, SFS_GUID, SimpleFileSystemProtocol, FileProtocol, FILE_INFO_GUID, FileInfo, FILE_DIRECTORY, FILE_MODE_CREATE, FILE_MODE_READ, FILE_MODE_WRITE };
use core::ptr::null_mut;
use core::str::FromStr;
use core::mem;
use core::ffi::c_void;
use alloc::vec::Vec;
use alloc::string::String;

use ::io::{ BinaryReader, BinaryWriter };

pub struct VolumeProvider {
    handle_buffer : ProtocolHandleBuffer
}

impl VolumeProvider {
    pub fn new() -> Result<Self, UefiError> {
        let handle_buffer = ProtocolHandleBuffer::new(SFS_GUID)?;
         Ok(VolumeProvider { handle_buffer })
    }
}

impl ProtocolProvider<Volume> for VolumeProvider {
    fn len(&self) -> usize {
        self.handle_buffer.len()
    }

    fn open(&self, id : usize) -> Result<Volume, UefiError> {
        unsafe {
            let protocol = self.handle_buffer.open(id)?;
            Ok(Volume::new_unchecked(protocol))
        }
    }
}

pub struct Volume {
    protocol : Protocol
}

impl Volume {
    pub fn new(protocol : Protocol) -> Result<Self, UefiError> {
       if protocol.guid() != SFS_GUID {
           return Err(UefiError::InvalidArgument("protocol"));
       }
       Ok(Volume { protocol })
    }

    pub unsafe fn new_unchecked(protocol : Protocol) -> Self {
        Volume { protocol }
    }

    pub fn root_node(&self) -> Result<Node, UefiError> {
        unsafe {
            let interface = self.protocol.interface::<SimpleFileSystemProtocol>();
            let sfs = &*interface;
            let mut file_protocol = null_mut();

            let status = (sfs.open_volume)(interface, &mut file_protocol);

            match status {
                Status::SUCCESS => Node::new(file_protocol),
                Status::UNSUPPORTED => Err(UefiError::IoError(UefiIoError::UnsupportedFileSystem)),
                Status::NO_MEDIA => Err(UefiError::IoError(UefiIoError::NoMedia)),
                Status::DEVICE_ERROR => Err(UefiError::DeviceError),
                Status::VOLUME_CORRUPTED => Err(UefiError::IoError(UefiIoError::VolumeCorrupted)),
                Status::ACCESS_DENIED => Err(UefiError::OperationDenied),
                Status::OUT_OF_RESOURCES => Err(UefiError::OutOfMemory),
                Status::MEDIA_CHANGED => Err(UefiError::IoError(UefiIoError::MediaInvalidated)),
                _ => Err(UefiError::UnexpectedStatus(status))
            }         
        }
    }
}

pub struct Node {
    protocol : *mut FileProtocol
}

impl Node {
    pub unsafe fn new(file_protocol : *mut FileProtocol) -> Result<Node, UefiError> {
        Ok(Node { protocol : file_protocol })
    }

    pub fn open_node(&self, path : &str, read : bool, write : bool) -> Result<Node, UefiError> {
        let mut open_mode = 0;

        if read {
            open_mode |= FILE_MODE_READ;
        }

        if write {
            open_mode |= FILE_MODE_WRITE;
        }

        self.open_node_internal(path, open_mode, 0)
    }
    
    pub fn create_node(&self, path : &str, node_type : NodeType)-> Result<Node, UefiError> {
        let open_mode = FILE_MODE_WRITE | FILE_MODE_CREATE | FILE_MODE_READ;
        let mut attributes = 0;

        if node_type.is_directory() {
            attributes |= FILE_DIRECTORY;
        }

        self.open_node_internal(path, open_mode, attributes)   
    }

    fn open_node_internal(&self, path : &str, open_mode : u64, attributes : u64)-> Result<Node, UefiError> {
        unsafe {
            let converted_path = C16String::from_str(path)?;
            let path_pointer = converted_path.into_raw();

            let protocol = &*self.protocol;
            let mut new_protocol = null_mut();

            let status = (protocol.open)(self.protocol, &mut new_protocol, path_pointer, open_mode, attributes);
            
            C16String::from_raw(path_pointer);

            match status {
                Status::SUCCESS => Node::new(new_protocol),
                Status::NOT_FOUND => Err(UefiError::IoError(UefiIoError::PathNonExistent(String::from(path)))),
                Status::NO_MEDIA => Err(UefiError::IoError(UefiIoError::NoMedia)),
                Status::MEDIA_CHANGED => Err(UefiError::IoError(UefiIoError::MediaInvalidated)),
                Status::DEVICE_ERROR => Err(UefiError::DeviceError),
                Status::VOLUME_CORRUPTED => Err(UefiError::IoError(UefiIoError::VolumeCorrupted)),
                Status::WRITE_PROTECTED => Err(UefiError::IoError(UefiIoError::ReadOnlyViolation)),
                Status::ACCESS_DENIED => Err(UefiError::OperationDenied),
                Status::OUT_OF_RESOURCES => Err(UefiError::OutOfMemory),
                Status::VOLUME_FULL => Err(UefiError::IoError(UefiIoError::VolumeFull)),
                _ => Err(UefiError::UnexpectedStatus(status))
            }
        }
    }

    pub fn read_to_end(&self, buffer : &mut Vec<u8>) -> Result<(), UefiError> {
        let info = self.get_info()?;

        if info.node_type() == NodeType::Directory {
            return Err(UefiError::IoError(UefiIoError::FileOnlyOperation))
        }

        let position = self.get_position()? as usize;
        let length = buffer.len();
        let size = info.size().unwrap() as usize;
        let additional = (size + length) - (buffer.capacity() + position);

        if additional > 0 {
            buffer.reserve_exact(additional);
        }

        for _ in 0..additional {
            buffer.push(0);
        }

        self.read_internal(&mut buffer[length..])
    }

    fn read_internal(&self, buffer : &mut [u8]) -> Result<(), UefiError>  {
        unsafe {
            let data = buffer.as_ptr() as *mut c_void;
            let mut data_size = buffer.len();

            let protocol = &*self.protocol;
                
            let status = (protocol.read)(self.protocol, &mut data_size, data);
            
            match status {
                Status::SUCCESS => Ok(()),
                Status::NO_MEDIA => Err(UefiError::IoError(UefiIoError::NoMedia)),
                Status::DEVICE_ERROR => Err(UefiError::DeviceError),
                Status::VOLUME_CORRUPTED => Err(UefiError::IoError(UefiIoError::VolumeCorrupted)),
                _ => Err(UefiError::UnexpectedStatus(status))
            }
        }
    }

    pub fn set_position(&self, position : u64) -> Result<(), UefiError> {
        unsafe {
            let protocol = &*self.protocol;
            
            let status = (protocol.set_position)(self.protocol, position);

            match status {
                Status::SUCCESS => Ok(()),
                Status::UNSUPPORTED => Err(UefiError::IoError(UefiIoError::FileOnlyOperation)),
                Status::DEVICE_ERROR => Err(UefiError::DeviceError),
                _ => Err(UefiError::UnexpectedStatus(status))
            }
        }
    }

    pub fn get_position(&self) -> Result<u64, UefiError> {
        unsafe {
            let protocol = &*self.protocol;
            let mut position = 0;

            let status = (protocol.get_position)(self.protocol, &mut position);

            match status {
                Status::SUCCESS => Ok(position),
                Status::UNSUPPORTED => Err(UefiError::IoError(UefiIoError::FileOnlyOperation)),
                Status::DEVICE_ERROR => Err(UefiError::DeviceError),
                _ => Err(UefiError::UnexpectedStatus(status))
            }
        }
    }

    pub fn get_info(&self) -> Result<NodeInfo, UefiError> {
        unsafe {
            let protocol = &*self.protocol;        
            let mut guid = FILE_INFO_GUID;
            let mut buffer_size = 0;            

            // Get size first. This should give a buffer too small error.

            let mut status = (protocol.get_info)(self.protocol, &mut guid, &mut buffer_size, null_mut());

            match status {
                Status::NO_MEDIA => return Err(UefiError::IoError(UefiIoError::NoMedia)),
                Status::DEVICE_ERROR => return Err(UefiError::DeviceError),
                Status::VOLUME_CORRUPTED => return Err(UefiError::IoError(UefiIoError::VolumeCorrupted)),
                Status::BUFFER_TOO_SMALL => {  },
                // SUCCESS and UNSUPPORTED are handled by this.
                _ =>  return Err(UefiError::UnexpectedStatus(status))

            }

            // Get actual info.

            let mut buffer = memory_pool!(buffer_size);

            status = (protocol.get_info)(self.protocol, &mut guid, &mut buffer_size, buffer.as_mut_ptr() as *mut c_void);

            let info = &*(buffer.as_mut_ptr() as *mut FileInfo);

            match status {
                Status::SUCCESS => {
                    if (info.attribute & FILE_DIRECTORY) != 0 {
                        Ok(NodeInfo::Directory)
                    }
                    else {
                        Ok(NodeInfo::File(info.file_size))
                    }
                },
                _ => Err(UefiError::UnexpectedStatus(status))
            }
        }
    }

    pub fn flush(&self) -> Result<(), UefiError> {
        unsafe {
            let protocol = &*self.protocol;

            let status = (protocol.flush)(self.protocol);

            match status {
                Status::SUCCESS => Ok(()),
                Status::NO_MEDIA => Err(UefiError::IoError(UefiIoError::NoMedia)),
                Status::DEVICE_ERROR => Err(UefiError::DeviceError),
                Status::VOLUME_CORRUPTED => Err(UefiError::IoError(UefiIoError::VolumeCorrupted)),
                Status::WRITE_PROTECTED => Err(UefiError::IoError(UefiIoError::ReadOnlyViolation)),
                Status::ACCESS_DENIED => Err(UefiError::IoError(UefiIoError::NoWriteAccess)),
                Status::VOLUME_FULL => Err(UefiError::IoError(UefiIoError::VolumeFull)),
                _ => Err(UefiError::UnexpectedStatus(status))
            }
        }
    }

    pub fn delete(self) -> Result<(), UefiError> {
        unsafe {
            let protocol = &*self.protocol;

            let status = (protocol.delete)(self.protocol);

            mem::forget(self);

            match status  {
                Status::SUCCESS => Ok(()),
                Status::WARN_DELETE_FAILURE => Err(UefiError::IoError(UefiIoError::DeleteFailed)),
                _ => Err(UefiError::UnexpectedStatus(status))
            }
        }
    }
}

impl BinaryReader for Node {
    type Error = UefiError;

	fn read_exact(&mut self, buffer : &mut [u8]) -> Result<(), Self::Error> {
        let info = self.get_info()?;

        if info.node_type() == NodeType::Directory {
            return Err(UefiError::IoError(UefiIoError::FileOnlyOperation))
        }

        self.read_internal(buffer)     
    }
}

impl BinaryWriter for Node {
    type Error = UefiError;

    fn write(&mut self, buffer : &mut [u8]) -> Result<(), Self::Error> {
        unsafe {
            let data = buffer.as_ptr() as *mut c_void;
            let mut data_size = buffer.len();
            let protocol = &*self.protocol;
            
            let status = (protocol.write)(self.protocol, &mut data_size, data);

            match status {
                Status::SUCCESS => Ok(()),
                Status::UNSUPPORTED => Err(UefiError::IoError(UefiIoError::FileOnlyOperation)),
                Status::NO_MEDIA => Err(UefiError::IoError(UefiIoError::NoMedia)),
                Status::DEVICE_ERROR => Err(UefiError::DeviceError),
                Status::VOLUME_CORRUPTED => Err(UefiError::IoError(UefiIoError::VolumeCorrupted)),
                Status::WRITE_PROTECTED => Err(UefiError::IoError(UefiIoError::ReadOnlyViolation)),
                Status::ACCESS_DENIED => Err(UefiError::IoError(UefiIoError::NoWriteAccess)),
                Status::VOLUME_FULL => Err(UefiError::IoError(UefiIoError::VolumeFull)),
                _ => Err(UefiError::UnexpectedStatus(status))
            }
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

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum NodeType {
    File,
    Directory
}

impl NodeType {
    pub fn is_file(self) -> bool {
        match self {
            NodeType::File => true,
            _ => false
        }
    }
    pub fn is_directory(self) -> bool {
        match self {
            NodeType::Directory => true,
            _ => false
        }
    }
}

#[derive(Debug)]
pub enum NodeInfo {
    File(u64),
    Directory
}

impl NodeInfo {
    pub fn node_type(&self) -> NodeType {
        match self {
            NodeInfo::File(_) => NodeType::File,
            NodeInfo::Directory => NodeType::Directory
        }
    }

    pub fn size(&self) -> Option<u64> {
        match self {
            NodeInfo::File(size) => Some(*size),
            _ => None
        }
    }
}