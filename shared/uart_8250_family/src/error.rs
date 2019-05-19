// *************************************************************************
// error.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use core::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Error {
    UnsupportedBaudDivisor,
    FifoInvalid,
    InvalidParity,
    InvalidFraming,
    Overrun,
    LineBusy
}

impl From<Error> for fmt::Error {
    fn from(_ : Error) -> Self {
        fmt::Error
    }
}