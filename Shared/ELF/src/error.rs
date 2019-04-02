// *************************************************************************
// error.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use core::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ElfError {
    SourceTooSmall,
    UnknownClass,
    DestinationTooSmall,
    NoLoadProgramSegments,
    InvalidProgramSegmentSize
}

impl fmt::Display for ElfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {        
        match self {
            ElfError::SourceTooSmall => write!(f, "The source provided is too small and cannot represent a proper ELF file."),
            ElfError::UnknownClass => write!(f, "The provided class is unkown. Only 32 and 64 are supported."),
            ElfError::DestinationTooSmall => write!(f, "The destination provided is not large enough to load the ELF file."),
            ElfError::NoLoadProgramSegments => write!(f, "The ELF file does not contain any program segments that can be loaded."),
            ElfError::InvalidProgramSegmentSize => write!(f, "A program segment has a file size larger than its memory size.")
        }
    }
}
