// *************************************************************************
// primitives.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use core::ffi::c_void;
use core::fmt;
use core::mem;
use core::convert;

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Status(usize);

impl Status {
    const ERROR_BIT : usize = 1 << ((mem::size_of::<usize>() * 8) - 1);

    pub const SUCCESS : Status = Self::new(0);

    pub const LOAD_ERROR : Status = Self::new(Self::ERROR_BIT | 1);
    pub const INVALID_PARAMETER : Status = Self::new(Self::ERROR_BIT | 2);
    pub const UNSUPPORTED : Status = Self::new(Self::ERROR_BIT | 3);
    pub const BAD_BUFFER_SIZE : Status = Self::new(Self::ERROR_BIT | 4);
    pub const BUFFER_TOO_SMALL : Status = Self::new(Self::ERROR_BIT | 5);
    pub const NOT_READY : Status = Self::new(Self::ERROR_BIT | 6);
    pub const DEVICE_ERROR : Status = Self::new(Self::ERROR_BIT | 7);
    pub const WRITE_PROTECTED : Status = Self::new(Self::ERROR_BIT | 8);
    pub const OUT_OF_RESOURCES : Status = Self::new(Self::ERROR_BIT | 9);
    pub const VOLUME_CORRUPTED : Status = Self::new(Self::ERROR_BIT | 10);
    pub const VOLUME_FULL : Status = Self::new(Self::ERROR_BIT | 11);
    pub const NO_MEDIA : Status = Self::new(Self::ERROR_BIT | 12);
    pub const MEDIA_CHANGED : Status = Self::new(Self::ERROR_BIT | 13);
    pub const NOT_FOUND : Status = Self::new(Self::ERROR_BIT | 14);
    pub const ACCESS_DENIED : Status = Self::new(Self::ERROR_BIT | 15);
    pub const NO_RESPONSE : Status = Self::new(Self::ERROR_BIT | 16);
    pub const NO_MAPPING : Status = Self::new(Self::ERROR_BIT | 17);
    pub const TIMEOUT : Status = Self::new(Self::ERROR_BIT | 18);
    pub const NOT_STARTED : Status = Self::new(Self::ERROR_BIT | 19);
    pub const ALREADY_STARTED : Status = Self::new(Self::ERROR_BIT | 20);
    pub const ABORTED : Status = Self::new(Self::ERROR_BIT | 21);
    pub const ICMP_ERROR : Status = Self::new(Self::ERROR_BIT | 22);
    pub const TFTP_ERROR : Status = Self::new(Self::ERROR_BIT | 23);
    pub const PROTOCOL_ERROR : Status = Self::new(Self::ERROR_BIT | 24);
    pub const INCOMPATIBLE_VERSION : Status = Self::new(Self::ERROR_BIT | 25);
    pub const SECURITY_VIOLATION : Status = Self::new(Self::ERROR_BIT | 26);
    pub const CRC_ERROR : Status = Self::new(Self::ERROR_BIT | 27);
    pub const END_OF_MEDIA : Status = Self::new(Self::ERROR_BIT | 28);
    pub const END_OF_FILE : Status = Self::new(Self::ERROR_BIT | 31);
    pub const INVALID_LANGUAGE : Status = Self::new(Self::ERROR_BIT | 32);
    pub const COMPROMISED_DATA : Status = Self::new(Self::ERROR_BIT | 33);
    pub const IP_ADDRESS_CONFLICT : Status = Self::new(Self::ERROR_BIT | 34);
    pub const HTTP_ERROR : Status = Self::new(Self::ERROR_BIT | 35);

    pub const WARN_UNKNOWN_GLYPH : Status = Self::new(1);
    pub const WARN_DELETE_FAILURE : Status = Self::new(2);
    pub const WARN_WRITE_FAILURE : Status = Self::new(3);
    pub const WARN_BUFFER_TOO_SMALL : Status = Self::new(4);
    pub const WARN_STALE_DATA : Status = Self::new(5);
    pub const WARN_FILE_SYSTEM : Status = Self::new(6);
    pub const WARN_RESET_REQUIRED : Status = Self::new(7);

    pub const fn new(code : usize) -> Self {
        Status(code)
    }
}

impl fmt::Debug for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Status::SUCCESS => write!(f, "SUCCESS")?,

            Status::LOAD_ERROR => write!(f, "LOAD_ERROR")?,
            Status::INVALID_PARAMETER => write!(f, "INVALID_PARAMETER")?,
            Status::UNSUPPORTED => write!(f, "UNSUPPORTED")?,
            Status::BAD_BUFFER_SIZE => write!(f, "BAD_BUFFER_SIZE")?,
            Status::BUFFER_TOO_SMALL => write!(f, "BUFFER_TOO_SMALL")?,
            Status::NOT_READY => write!(f, "NOT_READY")?,
            Status::DEVICE_ERROR => write!(f, "DEVICE_ERROR")?,
            Status::WRITE_PROTECTED => write!(f, "WRITE_PROTECTED")?,
            Status::OUT_OF_RESOURCES => write!(f, "OUT_OF_RESOURCES")?,
            Status::VOLUME_CORRUPTED => write!(f, "VOLUME_CORRUPTED")?,
            Status::VOLUME_FULL => write!(f, "VOLUME_FULL")?,
            Status::NO_MEDIA => write!(f, "NO_MEDIA")?,
            Status::MEDIA_CHANGED => write!(f, "MEDIA_CHANGED")?,
            Status::NOT_FOUND => write!(f, "NOT_FOUND")?,
            Status::ACCESS_DENIED => write!(f, "ACCESS_DENIED")?,
            Status::NO_RESPONSE => write!(f, "NO_RESPONSE")?,
            Status::NO_MAPPING => write!(f, "NO_MAPPING")?,
            Status::TIMEOUT => write!(f, "TIMEOUT")?,
            Status::NOT_STARTED => write!(f, "NOT_STARTED")?,
            Status::ALREADY_STARTED => write!(f, "ALREADY_STARTED")?,
            Status::ABORTED => write!(f, "ABORTED")?,
            Status::ICMP_ERROR => write!(f, "ICMP_ERROR")?,
            Status::TFTP_ERROR => write!(f, "TFTP_ERROR")?,
            Status::PROTOCOL_ERROR => write!(f, "PROTOCOL_ERROR")?,
            Status::INCOMPATIBLE_VERSION => write!(f, "INCOMPATIBLE_VERSION")?,
            Status::SECURITY_VIOLATION => write!(f, "SECURITY_VIOLATION")?,
            Status::CRC_ERROR => write!(f, "CRC_ERROR")?,
            Status::END_OF_MEDIA => write!(f, "END_OF_MEDIA")?,
            Status::END_OF_FILE => write!(f, "END_OF_FILE")?,
            Status::INVALID_LANGUAGE => write!(f, "INVALID_LANGUAGE")?,
            Status::COMPROMISED_DATA => write!(f, "COMPROMISED_DATA")?,
            Status::IP_ADDRESS_CONFLICT => write!(f, "IP_ADDRESS_CONFLICT")?,
            Status::HTTP_ERROR => write!(f, "HTTP_ERROR")?,

            Status::WARN_UNKNOWN_GLYPH => write!(f, "WARN_UNKNOWN_GLYPH")?,
            Status::WARN_DELETE_FAILURE => write!(f, "WARN_DELETE_FAILURE")?,
            Status::WARN_WRITE_FAILURE => write!(f, "WARN_WRITE_FAILURE")?,
            Status::WARN_BUFFER_TOO_SMALL => write!(f, "WARN_BUFFER_TOO_SMALL")?,
            Status::WARN_STALE_DATA => write!(f, "WARN_STALE_DATA")?,
            Status::WARN_FILE_SYSTEM => write!(f, "WARN_FILE_SYSTEM")?,
            Status::WARN_RESET_REQUIRED => write!(f, "WARN_RESET_REQUIRED")?,

            _ => self.0.fmt(f)?
        }

        Ok(())
    }
}

impl convert::From<usize> for Status {
    fn from(item: usize) -> Self {
        Status::new(item)
    }
}

impl convert::Into<usize> for Status {
    fn into(self) -> usize {
        self.0
    }
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
