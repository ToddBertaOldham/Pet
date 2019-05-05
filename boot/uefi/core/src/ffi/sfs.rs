// *************************************************************************
// sfs.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use super::primitives::{ GUID, Status, Event, Time };
use core::ffi::c_void;

pub const SFS_GUID : GUID = GUID { data_1 : 0x0964e5b22, data_2 : 0x6459, data_3 : 0x11d2, data_4 : [ 0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b ] };

pub const SFS_REVISION : u64 = 0x00010000;
pub const SFS_REVISION2 : u64 = 0x00020000;
pub const SFS_LATEST_REVISION2 : u64 = SFS_REVISION2;

#[repr(C)]
pub struct SimpleFileSystemProtocol {
    pub revision : u64,
    pub open_volume : extern "win64" fn(this : *mut SimpleFileSystemProtocol, root : *mut *mut FileProtocol) -> Status
}

pub const FILE_MODE_READ : u64 = 0x0000000000000001;
pub const FILE_MODE_WRITE : u64 = 0x0000000000000002;
pub const FILE_MODE_CREATE : u64 = 0x8000000000000000;

pub const FILE_READ_ONLY : u64 = 0x0000000000000001;
pub const FILE_HIDDEN : u64 = 0x0000000000000002;
pub const FILE_SYSTEM : u64 = 0x0000000000000004;
pub const FILE_RESERVED : u64 = 0x0000000000000008;
pub const FILE_DIRECTORY : u64 = 0x0000000000000010;
pub const FILE_ARCHIVE : u64 = 0x0000000000000020;
pub const FILE_VALID_ATTR : u64 = 0x0000000000000037;

#[repr(C)]
pub struct FileProtocol {
    pub revision : u64,
    pub open : extern "win64" fn(this : *mut FileProtocol, new_handle : *mut *mut FileProtocol, file_name : *mut u16, open_mode : u64, attributes : u64) -> Status,
    pub close : extern "win64" fn(this : *mut FileProtocol) -> Status,
    pub delete : extern "win64" fn(this : *mut FileProtocol) -> Status,
    pub read : extern "win64" fn(this : *mut FileProtocol, buffer_size : *mut usize, buffer : *mut c_void) -> Status,
    pub write : extern "win64" fn(this : *mut FileProtocol, buffer_size : *mut usize, buffer : *mut c_void) -> Status,
    pub get_position : extern "win64" fn(this : *mut FileProtocol, position : *mut u64) -> Status,
    pub set_position : extern "win64" fn(this : *mut FileProtocol, position : u64) -> Status,
    pub get_info : extern "win64" fn(this : *mut FileProtocol, information_type : *mut GUID, buffer_size : *mut usize, buffer : *mut c_void) -> Status,
    pub set_info : extern "win64" fn(this : *mut FileProtocol, information_type : *mut GUID, buffer_size : usize, buffer : *mut c_void) -> Status,
    pub flush : extern "win64" fn(this : *mut FileProtocol) -> Status,
    pub open_ex : extern "win64" fn(this : *mut FileProtocol, new_handle : *mut *mut FileProtocol, file_name : *mut u16, open_mode : u64, attributes : u64, token : *mut FileIOToken) -> Status,
    pub read_ex : extern "win64" fn(this : *mut FileProtocol, token : *mut FileIOToken) -> Status,
    pub write_ex : extern "win64" fn(this : *mut FileProtocol, token : *mut FileIOToken) -> Status,
    pub flush_ex : extern "win64" fn(this : *mut FileProtocol, token : *mut FileIOToken) -> Status
}

#[repr(C)]
pub struct FileIOToken {
    pub event : Event,
    pub status : Status,
    pub buffer_size : usize,
    pub buffer : *mut c_void
}

pub const FILE_SYSTEM_INFO_GUID : GUID = GUID { data_1 : 0x9576e93, data_2 : 0x6d3f, data_3 : 0x11d2, data_4 : [ 0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b ] };

#[repr(C)]
pub struct FileSystemInfo {
    pub size : u64,
    pub read_only : bool,
    pub volume_size : u64,
    pub free_space : u64,
    pub block_size : u32,
    pub volume_label : [u16; 1]
}

pub const FILE_INFO_GUID : GUID = GUID { data_1 : 0x9576e92, data_2 : 0x6d3f, data_3 : 0x11d2, data_4 : [ 0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b ] };

#[repr(C)]
pub struct FileInfo {
    pub size : u64,
    pub file_size : u64,
    pub physical_size : u64,
    pub create_time : Time,
    pub last_access_time : Time,
    pub modification_time : Time,
    pub attribute : u64,
    pub volume_label : [u16; 1]
}