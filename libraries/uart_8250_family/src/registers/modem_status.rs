//**************************************************************************************************
// modem_status.rs                                                                                 *
// Copyright (c) 2020-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use memory::GetBit;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct ModemStatusValue(u8);

impl ModemStatusValue {
    pub fn delta_clear_to_send(self) -> bool {
        self.0.get_bit(0)
    }

    pub fn delta_data_set_ready(self) -> bool {
        self.0.get_bit(1)
    }

    pub fn trailing_edge_ring_indicator(self) -> bool {
        self.0.get_bit(2)
    }

    pub fn delta_data_carrier_detect(self) -> bool {
        self.0.get_bit(3)
    }

    pub fn clear_to_send(self) -> bool {
        self.0.get_bit(4)
    }

    pub fn data_set_ready(self) -> bool {
        self.0.get_bit(5)
    }

    pub fn ring_indicator(self) -> bool {
        self.0.get_bit(6)
    }

    pub fn carrier_detect(self) -> bool {
        self.0.get_bit(7)
    }
}

impl From<u8> for ModemStatusValue {
    fn from(value: u8) -> Self {
        ModemStatusValue(value)
    }
}

impl From<ModemStatusValue> for u8 {
    fn from(value: ModemStatusValue) -> Self {
        value.0
    }
}
