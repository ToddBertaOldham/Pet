//**************************************************************************************************
// modem_control.rs                                                                                *
// Copyright (c) 2020 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use bits::{GetBit, SetBitAssign};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct ModemControlValue(u8);

impl ModemControlValue {
    pub const fn new() -> Self {
        Self(0)
    }

    pub fn data_terminal_ready(self) -> bool {
        self.0.get_bit(0)
    }

    pub fn set_data_terminal_ready(&mut self, value: bool) {
        self.0.set_bit_assign(0, value);
    }

    pub fn request_to_send(self) -> bool {
        self.0.get_bit(1)
    }

    pub fn set_request_to_send(&mut self, value: bool) {
        self.0.set_bit_assign(1, value);
    }

    pub fn auxiliary_output_1(self) -> bool {
        self.0.get_bit(2)
    }

    pub fn set_auxiliary_output_1(&mut self, value: bool) {
        self.0.set_bit_assign(2, value);
    }

    pub fn auxiliary_output_2(self) -> bool {
        self.0.get_bit(3)
    }

    pub fn set_auxiliary_output_2(&mut self, value: bool) {
        self.0.set_bit_assign(3, value);
    }

    pub fn loopback_mode(self) -> bool {
        self.0.get_bit(4)
    }

    pub fn set_loopback_mode(&mut self, value: bool) {
        self.0.set_bit_assign(4, value);
    }

    pub fn autoflow_control_enabled(self) -> bool {
        self.0.get_bit(5)
    }

    pub fn set_autoflow_control_enabled(&mut self, value: bool) {
        self.0.set_bit_assign(5, value);
    }
}

impl From<u8> for ModemControlValue {
    fn from(value: u8) -> Self {
        ModemControlValue(value)
    }
}

impl From<ModemControlValue> for u8 {
    fn from(value: ModemControlValue) -> Self {
        value.0
    }
}
