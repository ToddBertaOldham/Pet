//**************************************************************************************************
// file.rs                                                                                         *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use super::primitives::{Event, Guid, Status, Time};
use core::ffi::c_void;

flags!(
    pub struct OpenModes : u64 {
        READ = 0x0000000000000001;
        WRITE = 0x0000000000000002;
        CREATE = 0x8000000000000000; 
    }
);

flags!(
    pub struct Attributes : u64 {
        READ_ONLY = 0x0000000000000001;
        HIDDEN = 0x0000000000000002;
        SYSTEM = 0x0000000000000004;
        RESERVED = 0x0000000000000008;
        DIRECTORY = 0x0000000000000010;
        ARCHIVE = 0x0000000000000020;
        VALID_ATTR = 0x0000000000000037;
    }
);

#[repr(C)]
pub struct Protocol {
    pub revision: u64,
    pub open: extern "win64" fn(
        this: *mut Protocol,
        new_handle: *mut *mut Protocol,
        file_name: *mut u16,
        open_mode: OpenModes,
        attributes: Attributes,
    ) -> Status,
    pub close: extern "win64" fn(this: *mut Protocol) -> Status,
    pub delete: extern "win64" fn(this: *mut Protocol) -> Status,
    pub read: extern "win64" fn(
        this: *mut Protocol,
        buffer_size: *mut usize,
        buffer: *mut c_void,
    ) -> Status,
    pub write: extern "win64" fn(
        this: *mut Protocol,
        buffer_size: *mut usize,
        buffer: *mut c_void,
    ) -> Status,
    pub get_position: extern "win64" fn(this: *mut Protocol, position: *mut u64) -> Status,
    pub set_position: extern "win64" fn(this: *mut Protocol, position: u64) -> Status,
    pub get_info: extern "win64" fn(
        this: *mut Protocol,
        information_type: *mut Guid,
        buffer_size: *mut usize,
        buffer: *mut c_void,
    ) -> Status,
    pub set_info: extern "win64" fn(
        this: *mut Protocol,
        information_type: *mut Guid,
        buffer_size: usize,
        buffer: *mut c_void,
    ) -> Status,
    pub flush: extern "win64" fn(this: *mut Protocol) -> Status,
    pub open_ex: extern "win64" fn(
        this: *mut Protocol,
        new_handle: *mut *mut Protocol,
        file_name: *mut u16,
        open_mode: OpenModes,
        attributes: Attributes,
        token: *mut IOToken,
    ) -> Status,
    pub read_ex: extern "win64" fn(this: *mut Protocol, token: *mut IOToken) -> Status,
    pub write_ex: extern "win64" fn(this: *mut Protocol, token: *mut IOToken) -> Status,
    pub flush_ex: extern "win64" fn(this: *mut Protocol, token: *mut IOToken) -> Status,
}

impl Protocol {
    pub const REVISION: u64 = 0x00010000;
    pub const REVISION_2: u64 = 0x00020000;
    pub const LATEST_REVISION: u64 = Self::REVISION_2;
}

#[repr(C)]
pub struct IOToken {
    pub event: Event,
    pub status: Status,
    pub buffer_size: usize,
    pub buffer: *mut c_void,
}

#[repr(C)]
pub struct SystemInfo {
    pub size: u64,
    pub read_only: bool,
    pub volume_size: u64,
    pub free_space: u64,
    pub block_size: u32,
    pub volume_label: [u16; 1],
}

impl SystemInfo {
    pub const ID: Guid = Guid {
        data_1: 0x9576e93,
        data_2: 0x6d3f,
        data_3: 0x11d2,
        data_4: [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
    };
}

#[repr(C)]
pub struct Info {
    pub size: u64,
    pub file_size: u64,
    pub physical_size: u64,
    pub create_time: Time,
    pub last_access_time: Time,
    pub modification_time: Time,
    pub attribute: Attributes,
    pub volume_label: [u16; 1],
}

impl Info {
    pub const ID: Guid = Guid {
        data_1: 0x9576e92,
        data_2: 0x6d3f,
        data_3: 0x11d2,
        data_4: [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
    };
}
