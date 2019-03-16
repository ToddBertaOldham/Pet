// *************************************************************************
// storage.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use super::protocol::{ Protocol, ProtocolHandleBuffer, ProtocolProvider };
use super::error::UEFIError;
use super::string::C16String;
use super::ffi::{ Status, SFS_GUID, SimpleFileSystemProtocol, FileProtocol, FILE_INFO_GUID, FileInfo, FILE_DIRECTORY, FILE_MODE_CREATE, FILE_MODE_READ, FILE_MODE_WRITE };
use core::ptr::null_mut;
use core::str::FromStr;
use core::mem;
use core::ffi::c_void;
use alloc::alloc::{ alloc, dealloc, Layout };
use alloc::vec::Vec;

pub struct VolumeProvider {
    handle_buffer : ProtocolHandleBuffer
}

impl VolumeProvider {
    pub fn new() -> Result<Self, UEFIError> {
        let handle_buffer = ProtocolHandleBuffer::new(SFS_GUID)?;
         Ok(VolumeProvider { handle_buffer })
    }
}

impl ProtocolProvider<Volume> for VolumeProvider {
    fn len(&self) -> usize {
        self.handle_buffer.len()
    }

    fn open(&self, id : usize) -> Result<Volume, UEFIError> {
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
            let interface = self.protocol.interface::<SimpleFileSystemProtocol>();
            let sfs = &*interface;
            let mut file_protocol = null_mut();

            let status = (sfs.open_volume)(interface, &mut file_protocol);

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

    pub fn open_node(&self, path : &str, read : bool, write : bool) -> Result<Node, UEFIError> {
        let mut open_mode = 0;

        if read {
            open_mode = open_mode | FILE_MODE_READ;
        }

        if write {
            open_mode = open_mode | FILE_MODE_WRITE;
        }

        self.open_node_internal(path, open_mode, 0)
    }
    
    pub fn create_node(&self, path : &str, node_type : NodeType)-> Result<Node, UEFIError> {
        let open_mode = FILE_MODE_WRITE | FILE_MODE_CREATE | FILE_MODE_READ;
        let mut attributes = 0;

        if node_type.is_directory() {
            attributes = attributes | FILE_DIRECTORY;
        }

        self.open_node_internal(path, open_mode, attributes)   
    }

    fn open_node_internal(&self, path : &str, open_mode : u64, attributes : u64)-> Result<Node, UEFIError> {
        unsafe {
            let converted_path = C16String::from_str(path)?;
            let path_pointer = converted_path.into_raw();

            let protocol = &*self.protocol;
            let mut new_protocol = null_mut();

            let status = (protocol.open)(self.protocol, &mut new_protocol, path_pointer, open_mode, attributes);
            
            C16String::from_raw(path_pointer);

            match status {
                Status::Success => Node::new(new_protocol),
                //TODO Errors.
                _ => Err(UEFIError::UnexpectedFFIStatus(status))
            }
        }
    }

    pub fn read_to_end(&self, buffer : &mut Vec<u8>) -> Result<(), UEFIError> {
        let info = self.get_info()?;

        //TODO Directories can be read and this is kind of weird. Probably worth changing.
        if info.node_type() == NodeType::Directory {
            return Ok(());
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

        self.read_exact(&mut buffer[length..])
    }

    pub fn read_exact(&self, buffer : &mut [u8]) -> Result<(), UEFIError>  {
        unsafe {
            let data = buffer.as_ptr() as *mut c_void;
            let mut data_size = buffer.len();

            let protocol = &*self.protocol;
                
            //TODO Error handling and what about directories?
            let status = (protocol.read)(self.protocol, &mut data_size, data);
            match status {
                Status::Success => Ok(()),
                _ => Err(UEFIError::UnexpectedFFIStatus(status))
            }
        }
    }

    pub fn write(&self, buffer : &[u8]) -> Result<(), UEFIError> {
        unsafe {
            let data = buffer.as_ptr() as *mut c_void;
            let mut data_size = buffer.len();
            let protocol = &*self.protocol;
            
            //TODO Error handling.
            let status = (protocol.write)(self.protocol, &mut data_size, data);
            match status {
                Status::Success => Ok(()),
                _ => Err(UEFIError::UnexpectedFFIStatus(status))
            }
        }
    }

    pub fn set_position(&self, position : u64) -> Result<(), UEFIError> {
        unsafe {
            let protocol = &*self.protocol;
            let status = (protocol.set_position)(self.protocol, position);
            //TODO Error handling.
            match status {
                Status::Success => Ok(()),
                _ => Err(UEFIError::UnexpectedFFIStatus(status))
            }
        }
    }

    pub fn get_position(&self) -> Result<u64, UEFIError> {
        unsafe {
            let protocol = &*self.protocol;
            let mut position = 0;
            let status = (protocol.get_position)(self.protocol, &mut position);
            //TODO Error handling.
            match status {
                Status::Success => Ok(position),
                _ => Err(UEFIError::UnexpectedFFIStatus(status))
            }
        }
    }

    pub fn get_info(&self) -> Result<NodeInfo, UEFIError> {
        unsafe {
            let protocol = &*self.protocol;        
            let mut guid = FILE_INFO_GUID;
            let mut buffer_size = 0;            

            //TODO Error handling and cleanup.
            let mut status = (protocol.get_info)(self.protocol, &mut guid, &mut buffer_size, null_mut());
            if status != Status::BufferTooSmall {
                return Err(UEFIError::UnexpectedFFIStatus(status));
            }

            let layout = Layout::from_size_align(buffer_size, 8).unwrap();
            let buffer = alloc(layout) as *mut c_void;

            status = (protocol.get_info)(self.protocol, &mut guid, &mut buffer_size, buffer);

            let info = &*(buffer as *mut FileInfo);

            let is_directory = (info.attribute & FILE_DIRECTORY) != 0;
            let size = info.file_size;

            dealloc(buffer as *mut u8, layout);

            match status {
                Status::Success => {
                    if is_directory {
                        Ok(NodeInfo::Directory)
                    }
                    else {
                        Ok(NodeInfo::File(size))
                    }
                },
                _ => Err(UEFIError::UnexpectedFFIStatus(status))
            }
        }
    }

    pub fn flush(&self) {
        unsafe {
            let protocol = &*self.protocol;
            //TODO Errors.
            (protocol.flush)(self.protocol);
        }
    }

    pub fn delete(self) -> bool {
        unsafe {
            let protocol = &*self.protocol;
            let result = (protocol.delete)(self.protocol) == Status::Success;
            // Prevent drop from being called. Delete closes the protocol.
            mem::forget(self);
            result
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
    pub fn is_file(&self) -> bool {
        match self {
            NodeType::File => true,
            _ => false
        }
    }
    pub fn is_directory(&self) -> bool {
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