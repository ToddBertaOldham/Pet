//**************************************************************************************************
// error.rs                                                                                        *
// Copyright (c) 2019-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use core::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Error {
    InvalidBaudDivisor,
    FifoInvalid,
    InvalidParity,
    InvalidFraming,
    Overrun,
    LineBusy,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidBaudDivisor => write!(f, "The requested baud divisor is invalid."),
            Error::FifoInvalid => write!(f, "An error has occurred in the serial port's FIFO."),
            Error::InvalidParity => write!(
                f,
                "The serial port has likely been configured with an incorrect parity."
            ),
            Error::InvalidFraming => {
                write!(f, "The serial port has not been configured correctly.")
            }
            Error::Overrun => write!(
                f,
                "The serial port is trying to receive data but its buffer is full."
            ),
            Error::LineBusy => write!(
                f,
                "The serial port is not ready to send or receive more data."
            ),
        }
    }
}

impl From<Error> for fmt::Error {
    fn from(_: Error) -> Self {
        fmt::Error
    }
}
