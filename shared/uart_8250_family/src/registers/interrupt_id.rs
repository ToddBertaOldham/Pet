//**************************************************************************************************
// interrupt_id                                                                                    *
// Copyright (c) 2020 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use bits::GetBit;
use core::convert::TryFrom;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct InterruptIdValue(u8);

impl InterruptIdValue {
    pub fn expanded_fifo_enabled(self) -> bool {
        self.0.get_bit(5)
    }

    pub fn interrupt_event(&self) -> InterruptEvent {
        InterruptEvent::try_from(self.0 & 0xF).unwrap()
    }

    pub fn fifo_state(&self) -> FifoState {
        FifoState::try_from(self.0 & 0xC0).unwrap()
    }
}

impl From<u8> for InterruptIdValue {
    fn from(value: u8) -> Self {
        InterruptIdValue(value)
    }
}

impl From<InterruptIdValue> for u8 {
    fn from(value: InterruptIdValue) -> Self {
        value.0
    }
}

numeric_enum!(
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum InterruptEvent {
        None = 0x0,
        ModemStatus = 0x1,
        TransmitterEmpty = 0x3,
        DataRecieved = 0x5,
        LineStatus = 0x7,
        TimeOut = 0xD,
    }

    impl TryFrom<u8>;
);

numeric_enum!(
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FifoState {
        NotEnabled = 0,
        NotFunctioning = 0x80,
        Functioning = 0xC0,
    }

    impl TryFrom<u8>;
);
