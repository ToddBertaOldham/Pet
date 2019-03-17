// *************************************************************************
// error.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use alloc::string::String;
use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum UefiError {
    NotInitialized,
    AlreadyInitialized,
    BootServicesUnavailable,
    InvalidArgument(&'static str),
    UnexpectedFFIStatus(super::ffi::Status),
    OutOfMemory,
    NotSupported,
    DeviceError,
    OperationDenied,
    IoError(UefiIoError)
}

impl fmt::Display for UefiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {        
        match self {
            UefiError::NotInitialized => write!(f, "UEFI Core has not been initialized."),
            UefiError::AlreadyInitialized => write!(f, "UEFI Core has already been initialized."),
            UefiError::BootServicesUnavailable => write!(f, "Boot services are unavailable."),
            UefiError::InvalidArgument(name) => write!(f, "The argument \"{}\" is invalid.", name),
            UefiError::UnexpectedFFIStatus(status) => write!(f, "The FFI status \"{:?}\" was unexpected.", status),
            UefiError::OutOfMemory => write!(f, "Out of usable memory."),
            UefiError::NotSupported => write!(f, "The requested feature is not available."),
            UefiError::DeviceError => write!(f, "A hardware failure occured."),
            UefiError::OperationDenied => write!(f, "The requested operation was denied."),
            UefiError::IoError(io_error) => io_error.fmt(f)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UefiIoError {
    PathNonExistent(String),
    VolumeFull,
    UnsupportedFileSystem,
    VolumeCorrupted,
    ReadOnlyViolation,
    NoWriteAccess,
    NoMedia,
    MediaInvalidated,
    FileOnlyOperation
}

impl fmt::Display for UefiIoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {        
        match self {
            UefiIoError::PathNonExistent(path) => write!(f, "The path \"{}\" does not exist.", path),
            UefiIoError::VolumeFull => write!(f, "The volume is full."),
            UefiIoError::UnsupportedFileSystem => write!(f, "The file system is not supported."),
            UefiIoError::VolumeCorrupted => write!(f, "The volume is corrupted."),
            UefiIoError::ReadOnlyViolation => write!(f, "A write operation was attempted on a read only file or volume."),
            UefiIoError::NoWriteAccess => write!(f, "A write operation was attempted on a file that was opened without write access."),
            UefiIoError::NoMedia => write!(f, "The medium does not exist."),
            UefiIoError::MediaInvalidated => write!(f, "The medium is no longer valid. Possibly due to modification."),
            UefiIoError::FileOnlyOperation => write!(f, "The requested operation can only be done on a file.")
        }
    }
}