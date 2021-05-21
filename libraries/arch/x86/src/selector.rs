//**************************************************************************************************
// selector.rs                                                                                     *
// Copyright (c) 2019-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::privilege::ProtectionRing;
use core::convert::TryFrom;
use memory::{GetBit, SetBitAssign};

#[derive(Copy, Clone, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Selector(u16);

impl Selector {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn with_values(index: u16, is_local: bool, privilege_level: ProtectionRing) -> Self {
        let mut value = Self(0);
        value.set_index(index);
        value.set_is_local(is_local);
        value.set_privilege_level(privilege_level);
        value
    }

    pub fn is_local(self) -> bool {
        self.0.get_bit(2)
    }

    pub fn set_is_local(&mut self, value: bool) {
        self.0.set_bit_assign(2, value);
    }

    pub fn index(self) -> u16 {
        self.0 >> 3
    }

    pub fn set_index(&mut self, index: u16) {
        self.0 = (self.0 & 0x7) | (index << 3)
    }

    pub fn privilege_level(self) -> ProtectionRing {
        ProtectionRing::try_from((self.0 & 0x3) as u8).unwrap()
    }

    pub fn set_privilege_level(&mut self, privilege: ProtectionRing) {
        self.0 = (self.0 & !0x3) | (privilege as u16);
    }
}

impl From<Selector> for u16 {
    fn from(value: Selector) -> Self {
        value.0
    }
}

impl From<Selector> for u64 {
    fn from(value: Selector) -> Self {
        value.0 as u64
    }
}

impl From<Selector> for u32 {
    fn from(value: Selector) -> Self {
        value.0 as u32
    }
}

impl From<u16> for Selector {
    fn from(value: u16) -> Self {
        Selector(value)
    }
}
