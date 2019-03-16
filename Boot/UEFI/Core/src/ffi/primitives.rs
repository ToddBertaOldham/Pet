// *************************************************************************
// primitives.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use core::ffi::c_void;

const ERROR_BIT : usize = 0x8000000000000000;

//TODO Add warnings.
//TODO Would this be better as a struct? This works for now but the UEFI spec allows for OEM values.
#[repr(usize)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Status {
    Success = 0,
    LoadError = ERROR_BIT | 1,
    InvalidParameter = ERROR_BIT | 2,
    Unsupported = ERROR_BIT | 3,
    BadBufferSize = ERROR_BIT | 4,
    BufferTooSmall = ERROR_BIT| 5,
    NotReady = ERROR_BIT | 6,
    DeviceError = ERROR_BIT | 7,
    WriteProtected = ERROR_BIT | 8,
    OutOfResources = ERROR_BIT | 9,
    VolumeCorrupted = ERROR_BIT | 10,
    VolumeFull = ERROR_BIT | 11,
    NoMedia = ERROR_BIT | 12,
    MediaChanged = ERROR_BIT | 13,
    NotFound = ERROR_BIT | 14,
    AccessDenied = ERROR_BIT | 15,
    NoResponse = ERROR_BIT | 16,
    NoMapping =ERROR_BIT |  17,
    Timeout = ERROR_BIT | 18,
    NotStarted = ERROR_BIT | 19,
    AlreadyStarted = ERROR_BIT | 20,
    Aborted = ERROR_BIT | 21,
    ICMPError = ERROR_BIT | 22,
    TFTPError = ERROR_BIT | 23,
    ProtocolError = ERROR_BIT | 24,
    IncompatibleVersion = ERROR_BIT | 25,
    SecurityViolation = ERROR_BIT | 26,
    CRCError = ERROR_BIT | 27,
    EndOfMedia = ERROR_BIT | 28, 
    EndOfFile = ERROR_BIT | 31
}

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

#[repr(C)]
pub struct Time {
    pub year : u16,
    pub month : u8,
    pub day : u8,
    pub hour : u8,
    pub minute : u8,
    pub second : u8,
    pad1 : u8,
    pub nanosecond : u32,
    pub time_zone : i16,
    pub daylight : u8,
    pad2 : u8
}
