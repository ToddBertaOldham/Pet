//**************************************************************************************************
// line_status.rs                                                                                  *
// Copyright (c) 2020 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::Error;
use bits::GetBit;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct LineStatusValue(u8);

impl LineStatusValue {
    pub fn data_ready(self) -> bool {
        self.0.get_bit(0)
    }

    pub fn overrun_error(self) -> bool {
        self.0.get_bit(1)
    }

    pub fn parity_error(self) -> bool {
        self.0.get_bit(2)
    }

    pub fn framing_error(self) -> bool {
        self.0.get_bit(3)
    }

    pub fn break_interrupt(self) -> bool {
        self.0.get_bit(4)
    }

    pub fn transmitter_empty(self) -> bool {
        self.0.get_bit(5)
    }

    pub fn receiver_empty(self) -> bool {
        self.0.get_bit(6)
    }

    pub fn fifo_error(self) -> bool {
        self.0.get_bit(7)
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
