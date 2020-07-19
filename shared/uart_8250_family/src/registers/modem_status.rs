//**************************************************************************************************
// modem_status.rs                                                                                 *
// Copyright (c) 2020 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use bits::ReadBit;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct ModemStatusValue(u8);

impl ModemStatusValue {
    pub fn delta_clear_to_send(self) -> bool {
        self.0.read_bit(0).unwrap()
    }

    pub fn delta_data_set_ready(self) -> bool {
        self.0.read_bit(1).unwrap()
    }

    pub fn trailing_edge_ring_indicator(self) -> bool {
        self.0.read_bit(2).unwrap()
    }

    pub fn delta_data_carrier_detect(self) -> bool {
        self.0.read_bit(3).unwrap()
    }

    pub fn clear_to_send(self) -> bool {
        self.0.read_bit(4).unwrap()
    }

    pub fn data_set_ready(self) -> bool {
        self.0.read_bit(5).unwrap()
    }

    pub fn ring_indicator(self) -> bool {
        self.0.read_bit(6).unwrap()
    }

    pub fn carrier_detect(self) -> bool {
        self.0.read_bit(7).unwrap()
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
