//**************************************************************************************************
// error.rs                                                                                        *
// Copyright (c) 2019-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use core::fmt;
use io::cursor;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Error {
    SourceTooSmall,
    UnknownClass,
    UnknownData,
    DestinationTooSmall,
    NoLoadProgramSegments,
    InvalidProgramSegmentSize,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::SourceTooSmall => write!(
                f,
                "The source provided is too small and cannot represent a proper ELF file."
            ),
            Error::UnknownClass => write!(
                f,
                "The provided class is unknown. Only 32 and 64 are supported."
            ),
            Error::UnknownData => write!(
                f,
                "The provided data is unknown. Only little endian and big endian are supported."
            ),
            Error::DestinationTooSmall => write!(
                f,
                "The destination provided is not large enough to load the ELF file."
            ),
            Error::NoLoadProgramSegments => write!(
                f,
                "The ELF file does not contain any program segments that can be loaded."
            ),
            Error::InvalidProgramSegmentSize => write!(
                f,
                "A program segment has a file size larger than its memory size."
            ),
        }
    }
}

impl From<cursor::Error> for Error {
    fn from(_: cursor::Error) -> Self {
        Error::SourceTooSmall
    }
}
