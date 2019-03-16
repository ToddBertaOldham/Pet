// *************************************************************************
// error.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum UEFIError {
    NotInitialized,
    AlreadyInitialized,
    BootServicesUnavailable,
    InvalidArgument(&'static str),
    UnexpectedFFIStatus(super::ffi::Status),
    OutOfMemory,
    NotSupported,
    HardwareFailure
}

impl fmt::Display for UEFIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {        
        match self {
            UEFIError::NotInitialized => write!(f, "UEFI Core has not been initialized."),
            UEFIError::AlreadyInitialized => write!(f, "UEFI Core has already been initialized."),
            UEFIError::BootServicesUnavailable => write!(f, "Boot services are unavailable."),
            UEFIError::InvalidArgument(name) => write!(f, "The argument \"{}\" is invalid.", name),
            UEFIError::UnexpectedFFIStatus(status) => write!(f, "The FFI status \"{:?}\" was unexpected.", status),
            UEFIError::OutOfMemory => write!(f, "Out of usable memory."),
            UEFIError::NotSupported => write!(f, "The requested feature is not available."),
            UEFIError::HardwareFailure => write!(f, "A hardware failure occured.")
        }
    }
}
