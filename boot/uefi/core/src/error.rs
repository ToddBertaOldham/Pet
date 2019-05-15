// *************************************************************************
// error.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use core::fmt;
use alloc::string::String;
use ::io::cursor;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    NotInitialized,
    AlreadyInitialized,
    BootServicesUnavailable,
    InvalidArgument(&'static str),
    UnexpectedStatus(super::ffi::Status),
    OutOfMemory,
    NotSupported,
    DeviceError,
    OperationDenied,
    PathNonExistent(String),
    VolumeFull,
    UnsupportedFileSystem,
    VolumeCorrupted,
    ReadOnlyViolation,
    NoWriteAccess,
    NoMedia,
    MediaInvalidated,
    FileOnlyOperation,
    DeleteFailed,
    UnexpectedEnd
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {        
        match self {
            Error::NotInitialized => write!(f, "UEFI Core has not been initialized."),
            Error::AlreadyInitialized => write!(f, "UEFI Core has already been initialized."),
            Error::BootServicesUnavailable => write!(f, "Boot services are unavailable."),
            Error::InvalidArgument(name) => write!(f, "The argument \"{}\" is invalid.", name),
            Error::UnexpectedStatus(status) => write!(f, "The FFI status \"{:?}\" was unexpected.", status),
            Error::OutOfMemory => write!(f, "Out of usable memory."),
            Error::NotSupported => write!(f, "The requested feature is not available."),
            Error::DeviceError => write!(f, "A hardware failure occured."),
            Error::OperationDenied => write!(f, "The requested operation was denied."),
            Error::PathNonExistent(path) => write!(f, "The path \"{}\" does not exist.", path),
            Error::VolumeFull => write!(f, "The volume is full."),
            Error::UnsupportedFileSystem => write!(f, "The file system is not supported."),
            Error::VolumeCorrupted => write!(f, "The volume is corrupted."),
            Error::ReadOnlyViolation => write!(f, "A write operation was attempted on a read only file or volume."),
            Error::NoWriteAccess => write!(f, "A write operation was attempted on a file that was opened without write access."),
            Error::NoMedia => write!(f, "The medium does not exist."),
            Error::MediaInvalidated => write!(f, "The medium is no longer valid. Possibly due to modification."),
            Error::FileOnlyOperation => write!(f, "The requested operation can only be done on a file."),
            Error::DeleteFailed => write!(f, "Failed to delete the file or directory."),
            Error::UnexpectedEnd => write!(f, "The end of the source was reached unexpectedly.")
        }
    }
}

impl From<cursor::Error> for Error {
    fn from(_ : cursor::Error) -> Self {
        Error::UnexpectedEnd
    }
}