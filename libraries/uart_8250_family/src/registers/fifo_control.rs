//**************************************************************************************************
// fifo_control.rs                                                                                 *
// Copyright (c) 2020 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use bits::{GetBit, SetBitAssign};
use core::convert::TryFrom;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct FifoControlValue(u8);

impl FifoControlValue {
    pub const fn new() -> Self {
        Self(0)
    }

    pub fn clear_receive(self) -> bool {
        self.0.get_bit(1)
    }

    pub fn set_clear_receive(&mut self, value: bool) {
        self.0.set_bit_assign(1, value);
    }

    pub fn clear_transmit(self) -> bool {
        self.0.get_bit(2)
    }

    pub fn set_clear_transmit(&mut self, value: bool) {
        self.0.set_bit_assign(2, value);
    }

    pub fn dma_enabled(self) -> bool {
        self.0.get_bit(3)
    }

    pub fn set_dma_enabled(&mut self, value: bool) {
        self.0.set_bit_assign(3, value);
    }

    pub fn fifo_mode(&self) -> FifoMode {
        FifoMode::try_from(self.0 & 0xE1).unwrap()
    }

    pub fn set_fifo_mode(&mut self, fifo_mode: FifoMode) {
        self.0 = (self.0 & !0x1E) | fifo_mode as u8;
    }
}

impl From<u8> for FifoControlValue {
    fn from(value: u8) -> Self {
        FifoControlValue(value)
    }
}

impl From<FifoControlValue> for u8 {
    fn from(value: FifoControlValue) -> Self {
        value.0
    }
}

numeric_enum!(
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FifoMode {
        Disabled = 0x0,
        OneByte = 0x1,
        FourBytes = 0x41,
        EightBytes = 0x81,
        FourteenBytes = 0xC1,
        SixteenBytes = 0x61,
        ThirtyTwoBytes = 0xA1,
        FiftySixBytes = 0xE1,
    }

    impl TryFrom<u8>;
);
