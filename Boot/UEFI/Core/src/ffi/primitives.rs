// *************************************************************************
// primitives.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use core::ffi::c_void;

// Reference available at http://wiki.phoenix.com/wiki/index.php/EFI_STATUS.

#[repr(usize)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Status {
    Success = 0,
    LoadError = 1,
    InvalidParameter = 2,
    Unsupported = 3,
    BadBufferSize = 4,
    BufferTooSmall = 5,
    NotReady = 6,
    DeviceError = 7,
    WriteProtected = 8,
    OutOfResources = 9,
    VolumeCorrupted = 10,
    VolumeFull = 11,
    NoMedia = 12,
    MediaChanged = 13,
    NotFound = 14,
    AccessDenied = 15,
    NoResponse = 16,
    NoMapping = 17,
    Timeout = 18,
    NotStarted = 19,
    AlreadyStarted = 20,
    Aborted = 21,
    ICMPError = 22,
    TFTPError = 23,
    ProtocolError = 24,
    IncompatibleVersion = 25,
    SecurityViolation = 26,
    CRCError = 27,
    EndOfMedia = 28, 
    EndOfFile = 31
}

// Reference available at http://wiki.phoenix.com/wiki/index.php/EFI_GUID

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub struct GUID {
    pub data_1 : u32,
    pub data_2 : u16,
    pub data_3 : u16,
    pub data_4 : [u8; 8]
}

pub type Handle = *mut c_void;
pub type Event = *mut c_void;

pub type PhysicalAddress = u64;
pub type VirtualAddress = u64;