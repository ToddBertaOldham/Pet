// *************************************************************************
// error.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use core::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum UEFIError {
    BootServicesUnavailable,
    InvalidMemoryMapKey,
    UnexpectedFFIStatus(super::ffi::Status)
}

impl fmt::Display for UEFIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {        
        match self {
            UEFIError::BootServicesUnavailable => write!(f, "Boot services are unavailable. Did you exit early?"),
            UEFIError::InvalidMemoryMapKey => write!(f, "The memory map key specified is invalid. Memory must not be allocated or freed after the key is retrieved."),
            UEFIError::UnexpectedFFIStatus(status) => write!(f, "The FFI status \"{:?}\" was unexpected.", status)
        }
    }
}
