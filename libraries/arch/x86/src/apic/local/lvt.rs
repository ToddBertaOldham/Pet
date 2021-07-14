//**************************************************************************************************
// lvt.rs                                                                                          *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use enums::c_enum;
use memory::{GetBit, SetBitAssign};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct TimerLvt(u32);

impl TimerLvt {
    pub const fn new() -> Self {
        Self(0)
    }

    pub fn vector(self) -> u8 {
        self.0 as u8
    }

    pub fn send_pending(self) -> bool {
        self.0.get_bit(12)
    }

    pub fn is_masked(self) -> bool {
        self.0.get_bit(16)
    }

    pub fn timer_mode(self) -> TimerMode {
        let value = self.0.get_bits(17, 0, 3);
        TimerMode::from(value as u8)
    }

    pub fn set_timer_mode(&mut self, timer_mode: TimerMode) {
        let value = u8::from(timer_mode) as u32;
        self.0.set_bits_assign(value, 17, 0, 3);
    }
}

impl From<u32> for TimerLvt {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<TimerLvt> for u32 {
    fn from(value: TimerLvt) -> Self {
        value.0
    }
}

c_enum!(
    pub enum TimerMode : u8 {
        ONE_SHOT = 0b00,
        PERIODIC = 0b01,
        TSC_DEADLINE = 0b10,
    }
);

c_enum!(
    pub enum LvtDeliveryMode : u8 {
        FIXED = 0b000,
        SMI = 0b010,
        NMI = 0b100,
        EXT_INT = 0b111,
        INIT = 0b101,
    }
);
