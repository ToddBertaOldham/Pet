//**************************************************************************************************
// line_status.rs                                                                                  *
// Copyright (c) 2020 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::Error;
use bits::ReadBit;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct LineStatusValue(u8);

impl LineStatusValue {
    pub fn data_ready(self) -> bool {
        self.0.read_bit(0).unwrap()
    }

    pub fn overrun_error(self) -> bool {
        self.0.read_bit(1).unwrap()
    }

    pub fn parity_error(self) -> bool {
        self.0.read_bit(2).unwrap()
    }

    pub fn framing_error(self) -> bool {
        self.0.read_bit(3).unwrap()
    }

    pub fn break_interrupt(self) -> bool {
        self.0.read_bit(4).unwrap()
    }

    pub fn transmitter_empty(self) -> bool {
        self.0.read_bit(5).unwrap()
    }

    pub fn receiver_empty(self) -> bool {
        self.0.read_bit(6).unwrap()
    }

    pub fn fifo_error(self) -> bool {
        self.0.read_bit(7).unwrap()
    }
}

impl From<u8> for LineStatusValue {
    fn from(value: u8) -> Self {
        LineStatusValue(value)
    }
}

impl From<LineStatusValue> for u8 {
    fn from(value: LineStatusValue) -> Self {
        value.0
    }
}

impl From<LineStatusValue> for Result<(), Error> {
    fn from(value: LineStatusValue) -> Self {
        if value.framing_error() {
            return Err(Error::InvalidFraming);
        }

        if value.parity_error() {
            return Err(Error::InvalidParity);
        }

        if value.overrun_error() {
            return Err(Error::Overrun);
        }

        if value.fifo_error() {
            return Err(Error::FifoInvalid);
        }

        Ok(())
    }
}
