//**************************************************************************************************
// line_control.rs                                                                                 *
// Copyright (c) 2020 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use bits::{GetBit, SetBitAssign};
use core::convert::TryFrom;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct LineControlValue(u8);

impl LineControlValue {
    pub fn new() -> Self {
        LineControlValue(0)
    }

    pub fn divisor_latch_access_enabled(self) -> bool {
        self.0.get_bit(7)
    }

    pub fn set_divisor_latch_access_enabled(&mut self, value: bool) {
        self.0.set_bit_assign(7, value);
    }

    pub fn word_length(self) -> WordLength {
        WordLength::try_from(self.0 & 0x3).unwrap()
    }

    pub fn set_word_length(&mut self, word_length: WordLength) {
        self.0 = (self.0 & !0x3) | word_length as u8;
    }

    pub fn stop_bits(self) -> StopBits {
        StopBits::try_from(self.0 & 0x4).unwrap()
    }

    pub fn set_stop_bits(&mut self, stop_bits: StopBits) {
        self.0 = (self.0 & !0x4) | stop_bits as u8;
    }

    pub fn parity(&self) -> Parity {
        Parity::try_from(self.0 & 0x38).unwrap()
    }

    pub fn set_parity(&mut self, parity: Parity) {
        self.0 = (self.0 & !0x38) | parity as u8;
    }
}

impl From<u8> for LineControlValue {
    fn from(value: u8) -> Self {
        LineControlValue(value)
    }
}

impl From<LineControlValue> for u8 {
    fn from(value: LineControlValue) -> Self {
        value.0
    }
}

numeric_enum!(
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum WordLength {
        Five = 0,
        Six = 0x1,
        Seven = 0x2,
        Eight = 0x3,
    }

    impl TryFrom<u8>;
);

numeric_enum!(
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Parity {
        None = 0,
        Odd = 0x8,
        Even = 0x18,
        Mark = 0x28,
        Space = 0x38,
    }

    impl TryFrom<u8>;
);

numeric_enum!(
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum StopBits {
        One = 0,
        Two = 0x4,
    }

    impl TryFrom<u8>;
);
