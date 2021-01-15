//**************************************************************************************************
// interrupt_enable.rs                                                                             *
// Copyright (c) 2020 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use bits::{GetBit, SetBitAssign};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct InterruptEnableValue(u8);

impl InterruptEnableValue {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn received_data_available_interrupt(self) -> bool {
        self.0.get_bit(0)
    }

    pub fn set_data_received_interrupt(&mut self, value: bool) {
        self.0.set_bit_assign(0, value);
    }

    pub fn transmitter_holding_register_empty_interrupt(self) -> bool {
        self.0.get_bit(1)
    }

    pub fn set_transmitter_empty_interrupt(&mut self, value: bool) {
        self.0.set_bit_assign(1, value);
    }

    pub fn line_status_interrupt(self) -> bool {
        self.0.get_bit(2)
    }

    pub fn set_line_status_interrupt(&mut self, value: bool) {
        self.0.set_bit_assign(2, value);
    }

    pub fn modem_status_interrupt(self) -> bool {
        self.0.get_bit(3)
    }

    pub fn set_modem_status_interrupt(&mut self, value: bool) {
        self.0.set_bit_assign(3, value);
    }

    pub fn sleep_mode_enabled(self) -> bool {
        self.0.get_bit(4)
    }

    pub fn set_sleep_mode_enabled(&mut self, value: bool) {
        self.0.set_bit_assign(4, value);
    }

    pub fn low_power_mode_enabled(self) -> bool {
        self.0.get_bit(5)
    }

    pub fn set_low_power_mode_enabled(&mut self, value: bool) {
        self.0.set_bit_assign(5, value);
    }
}

impl From<u8> for InterruptEnableValue {
    fn from(value: u8) -> Self {
        InterruptEnableValue(value)
    }
}

impl From<InterruptEnableValue> for u8 {
    fn from(value: InterruptEnableValue) -> Self {
        value.0
    }
}
