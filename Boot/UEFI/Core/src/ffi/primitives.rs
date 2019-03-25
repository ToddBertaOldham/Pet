// *************************************************************************
// primitives.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use core::ffi::c_void;
use core::mem;

const ERROR_BIT : usize = 1 << ((mem::size_of::<usize>() * 8) - 1);

c_enum!(
    pub enum Status : usize {
        SUCCESS = 0;

        LOAD_ERROR = ERROR_BIT | 1;
        INVALID_PARAMETER = ERROR_BIT | 2;
        UNSUPPORTED = ERROR_BIT | 3;
        BAD_BUFFER_SIZE = ERROR_BIT | 4;
        BUFFER_TOO_SMALL = ERROR_BIT | 5;
        NOT_READY = ERROR_BIT | 6;
        DEVICE_ERROR = ERROR_BIT | 7;
        WRITE_PROTECTED = ERROR_BIT | 8;
        OUT_OF_RESOURCES = ERROR_BIT | 9;
        VOLUME_CORRUPTED = ERROR_BIT | 10;
        VOLUME_FULL = ERROR_BIT | 11;
        NO_MEDIA = ERROR_BIT | 12;
        MEDIA_CHANGED = ERROR_BIT | 13;
        NOT_FOUND = ERROR_BIT | 14;
        ACCESS_DENIED = ERROR_BIT | 15;
        NO_RESPONSE = ERROR_BIT | 16;
        NO_MAPPING = ERROR_BIT | 17;
        TIMEOUT = ERROR_BIT | 18;
        NOT_STARTED = ERROR_BIT | 19;
        ALREADY_STARTED = ERROR_BIT | 20;
        ABORTED = ERROR_BIT | 21;
        ICMP_ERROR = ERROR_BIT | 22;
        TFTP_ERROR = ERROR_BIT | 23;
        PROTOCOL_ERROR = ERROR_BIT | 24;
        INCOMPATIBLE_VERSION = ERROR_BIT | 25;
        SECURITY_VIOLATION = ERROR_BIT | 26;
        CRC_ERROR = ERROR_BIT | 27;
        END_OF_MEDIA = ERROR_BIT | 28;
        END_OF_FILE = ERROR_BIT | 31;
        INVALID_LANGUAGE = ERROR_BIT | 32;
        COMPROMISED_DATA = ERROR_BIT | 33;
        IP_ADDRESS_CONFLICT = ERROR_BIT | 34;
        HTTP_ERROR = ERROR_BIT | 35;

        WARN_UNKNOWN_GLYPH = 1;
        WARN_DELETE_FAILURE = 2;
        WARN_WRITE_FAILURE = 3;
        WARN_BUFFER_TOO_SMALL = 4;
        WARN_STALE_DATA = 5;
        WARN_FILE_SYSTEM = 6;
        WARN_RESET_REQUIRED = 7;
    }
);

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
