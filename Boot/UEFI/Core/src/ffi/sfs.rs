// *************************************************************************
// sfs.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use super::primitives::{ GUID, Status };
use core::ffi::c_void;

pub const SFS_GUID : GUID = GUID { data_1 : 0x0964e5b22, data_2 : 0x6459, data_3 : 0x11d2, data_4 : [ 0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b ] };

pub const SFS_REVISION : u64 = 0x00010000;

#[repr(C)]
pub struct SimpleFileSystemProtocol {
    pub revision : u64,
    pub open_volume : extern "win64" fn(this : *mut SimpleFileSystemProtocol, root : *mut *mut FileProtocol) -> Status
}

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
}