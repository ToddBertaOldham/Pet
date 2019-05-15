// *************************************************************************
// storage.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use crate::protocol::{ Protocol, ProtocolHandleBuffer, ProtocolProvider };
use crate::error::{ Error };
use crate::string::C16String;
use crate::ffi::Status;
use crate::ffi::simple_file_system;
use crate::ffi::file;
use core::ptr;
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
    pub fn new() -> Result<Self, Error> {
        let handle_buffer = ProtocolHandleBuffer::new(simple_file_system::Protocol::GUID)?;
         Ok(VolumeProvider { handle_buffer })
    }
}

impl ProtocolProvider<Volume> for VolumeProvider {
    fn len(&self) -> usize {
        self.handle_buffer.len()
    }

    fn open(&self, id : usize) -> Result<Volume, Error> {
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
    pub fn new(protocol : Protocol) -> Result<Self, Error> {
       if protocol.guid() != simple_file_system::Protocol::GUID {
           return Err(Error::InvalidArgument("protocol"));
       }
       Ok(Volume { protocol })
    }

    pub unsafe fn new_unchecked(protocol : Protocol) -> Self {
        Volume { protocol }
    }

    pub fn root_node(&self) -> Result<Node, Error> {
        unsafe {
            let interface = self.protocol.interface::<simple_file_system::Protocol>();
            let sfs = &*interface;
            let mut file_protocol = ptr::null_mut();

            let status = (sfs.open_volume)(interface, &mut file_protocol);

            match status {
                Status::SUCCESS => Node::new(file_protocol),
                Status::UNSUPPORTED => Err(Error::UnsupportedFileSystem),
                Status::NO_MEDIA => Err(Error::NoMedia),
                Status::DEVICE_ERROR => Err(Error::DeviceError),
                Status::VOLUME_CORRUPTED => Err(Error::VolumeCorrupted),
                Status::ACCESS_DENIED => Err(Error::OperationDenied),
                Status::OUT_OF_RESOURCES => Err(Error::OutOfMemory),
                Status::MEDIA_CHANGED => Err(Error::MediaInvalidated),
                _ => Err(Error::UnexpectedStatus(status))
            }         
        }
    }
}

pub struct Node(*mut file::Protocol);

impl Node {
    pub unsafe fn new(file_protocol : *mut file::Protocol) -> Result<Node, Error> {
        Ok(Node(file_protocol))
    }

    pub fn open_node(&self, path : &str, read : bool, write : bool) -> Result<Node, Error> {
        let mut open_mode = file::OpenModes::empty();

        if read {
            open_mode |= file::OpenModes::READ;
        }

        if write {
            open_mode |= file::OpenModes::WRITE;
        }

        self.open_node_internal(path, open_mode, file::Attributes::empty())
    }
    
    pub fn create_node(&self, path : &str, node_type : NodeType)-> Result<Node, Error> {
        let open_mode = file::OpenModes::WRITE | file::OpenModes::CREATE | file::OpenModes::READ;
        let mut attributes = file::Attributes::empty();

        if node_type.is_directory() {
            attributes |= file::Attributes::DIRECTORY;
        }

        self.open_node_internal(path, open_mode, attributes)   
    }

    fn open_node_internal(&self, path : &str, open_mode : file::OpenModes, attributes : file::Attributes)-> Result<Node, Error> {
        unsafe {
            let converted_path = C16String::from_str(path)?;
            let path_pointer = converted_path.into_raw();

            let protocol = &*self.0;
            let mut new_protocol = ptr::null_mut();

            let status = (protocol.open)(self.0, &mut new_protocol, path_pointer, open_mode, attributes);
            
            C16String::from_raw(path_pointer);

            match status {
                Status::SUCCESS => Node::new(new_protocol),
                Status::NOT_FOUND => Err(Error::PathNonExistent(String::from(path))),
                Status::NO_MEDIA => Err(Error::NoMedia),
                Status::MEDIA_CHANGED => Err(Error::MediaInvalidated),
                Status::DEVICE_ERROR => Err(Error::DeviceError),
                Status::VOLUME_CORRUPTED => Err(Error::VolumeCorrupted),
                Status::WRITE_PROTECTED => Err(Error::ReadOnlyViolation),
                Status::ACCESS_DENIED => Err(Error::OperationDenied),
                Status::OUT_OF_RESOURCES => Err(Error::OutOfMemory),
                Status::VOLUME_FULL => Err(Error::VolumeFull),
                _ => Err(Error::UnexpectedStatus(status))
            }
        }
    }

    pub fn read_to_end(&self, buffer : &mut Vec<u8>) -> Result<(), Error> {
        let info = self.get_info()?;

        if info.node_type() == NodeType::Directory {
            return Err(Error::FileOnlyOperation)
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

    fn read_internal(&self, buffer : &mut [u8]) -> Result<(), Error>  {
        unsafe {
            let data = buffer.as_ptr() as *mut c_void;
            let mut data_size = buffer.len();

            let protocol = &*self.0;
                
            let status = (protocol.read)(self.0, &mut data_size, data);
            
            match status {
                Status::SUCCESS => Ok(()),
                Status::NO_MEDIA => Err(Error::NoMedia),
                Status::DEVICE_ERROR => Err(Error::DeviceError),
                Status::VOLUME_CORRUPTED => Err(Error::VolumeCorrupted),
                _ => Err(Error::UnexpectedStatus(status))
            }
        }
    }

    pub fn set_position(&self, position : u64) -> Result<(), Error> {
        unsafe {
            let protocol = &*self.0;
            
            let status = (protocol.set_position)(self.0, position);

            match status {
                Status::SUCCESS => Ok(()),
                Status::UNSUPPORTED => Err(Error::FileOnlyOperation),
                Status::DEVICE_ERROR => Err(Error::DeviceError),
                _ => Err(Error::UnexpectedStatus(status))
            }
        }
    }

    pub fn get_position(&self) -> Result<u64, Error> {
        unsafe {
            let protocol = &*self.0;
            let mut position = 0;

            let status = (protocol.get_position)(self.0, &mut position);

            match status {
                Status::SUCCESS => Ok(position),
                Status::UNSUPPORTED => Err(Error::FileOnlyOperation),
                Status::DEVICE_ERROR => Err(Error::DeviceError),
                _ => Err(Error::UnexpectedStatus(status))
            }
        }
    }

    pub fn get_info(&self) -> Result<NodeInfo, Error> {
        unsafe {
            let protocol = &*self.0;        
            let mut id = file::Info::ID;
            let mut buffer_size = 0;            

            // Get size first. This should give a buffer too small error.

            let mut status = (protocol.get_info)(self.0, &mut id, &mut buffer_size, ptr::null_mut());

            match status {
                Status::NO_MEDIA => return Err(Error::NoMedia),
                Status::DEVICE_ERROR => return Err(Error::DeviceError),
                Status::VOLUME_CORRUPTED => return Err(Error::VolumeCorrupted),
                Status::BUFFER_TOO_SMALL => {  },
                // SUCCESS and UNSUPPORTED are handled by this.
                _ =>  return Err(Error::UnexpectedStatus(status))

            }

            // Get actual info.

            let mut buffer = memory_pool!(buffer_size);

            status = (protocol.get_info)(self.0, &mut id, &mut buffer_size, buffer.as_mut_ptr() as *mut c_void);

            let info = &*(buffer.as_mut_ptr() as *mut file::Info);

            match status {
                Status::SUCCESS => {
                    if info.attribute.contains(file::Attributes::DIRECTORY) {
                        Ok(NodeInfo::Directory)
                    }
                    else {
                        Ok(NodeInfo::File(info.file_size))
                    }
                },
                _ => Err(Error::UnexpectedStatus(status))
            }
        }
    }

    pub fn flush(&self) -> Result<(), Error> {
        unsafe {
            let protocol = &*self.0;

            let status = (protocol.flush)(self.0);

            match status {
                Status::SUCCESS => Ok(()),
                Status::NO_MEDIA => Err(Error::NoMedia),
                Status::DEVICE_ERROR => Err(Error::DeviceError),
                Status::VOLUME_CORRUPTED => Err(Error::VolumeCorrupted),
                Status::WRITE_PROTECTED => Err(Error::ReadOnlyViolation),
                Status::ACCESS_DENIED => Err(Error::NoWriteAccess),
                Status::VOLUME_FULL => Err(Error::VolumeFull),
                _ => Err(Error::UnexpectedStatus(status))
            }
        }
    }

    pub fn delete(self) -> Result<(), Error> {
        unsafe {
            let protocol = &*self.0;

            let status = (protocol.delete)(self.0);

            mem::forget(self);

            match status  {
                Status::SUCCESS => Ok(()),
                Status::WARN_DELETE_FAILURE => Err(Error::DeleteFailed),
                _ => Err(Error::UnexpectedStatus(status))
            }
        }
    }
}

impl BinaryReader for Node {
    type Error = Error;

	fn read_exact(&mut self, buffer : &mut [u8]) -> Result<(), Self::Error> {
        let info = self.get_info()?;

        if info.node_type() == NodeType::Directory {
            return Err(Error::FileOnlyOperation)
        }

        self.read_internal(buffer)     
    }
}

impl BinaryWriter for Node {
    type Error = Error;

    fn write(&mut self, buffer : &mut [u8]) -> Result<(), Self::Error> {
        unsafe {
            let data = buffer.as_ptr() as *mut c_void;
            let mut data_size = buffer.len();
            let protocol = &*self.0;
            
            let status = (protocol.write)(self.0, &mut data_size, data);

            match status {
                Status::SUCCESS => Ok(()),
                Status::UNSUPPORTED => Err(Error::FileOnlyOperation),
                Status::NO_MEDIA => Err(Error::NoMedia),
                Status::DEVICE_ERROR => Err(Error::DeviceError),
                Status::VOLUME_CORRUPTED => Err(Error::VolumeCorrupted),
                Status::WRITE_PROTECTED => Err(Error::ReadOnlyViolation),
                Status::ACCESS_DENIED => Err(Error::NoWriteAccess),
                Status::VOLUME_FULL => Err(Error::VolumeFull),
                _ => Err(Error::UnexpectedStatus(status))
            }
        }
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        unsafe {
            let protocol = &*self.0;
            (protocol.close)(self.0);
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