//**************************************************************************************************
// tss_ldt.rs                                                                                      *
// Copyright (c) 2020 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::privilege::ProtectionRing;
use bits::{GetBit, SetBitAssign};
use core::convert::TryFrom;
use enums::numeric_enum;

//TODO Finish cleanup.

#[derive(Copy, Clone, PartialEq, Eq, Default)]
#[repr(C)]
pub struct Descriptor {
    lower: u32,
    middle: u32,
    upper: u32,
    reserved: u32,
}

impl Descriptor {
    pub const fn new() -> Descriptor {
        Descriptor {
            lower: 0,
            middle: 0,
            upper: 0,
            reserved: 0,
        }
    }

    pub fn is_present(self) -> bool {
        self.upper.get_bit(15)
    }

    pub fn set_is_present(&mut self, value: bool) {
        self.upper.set_bit_assign(15, value);
    }

    pub fn avl_enabled(self) -> bool {
        self.upper.get_bit(20)
    }

    pub fn set_avl_enabled(&mut self, value: bool) {
        self.upper.set_bit_assign(20, value);
    }

    pub fn granularity_enabled(self) -> bool {
        self.upper.get_bit(23)
    }

    pub fn set_granularity_enabled(&mut self, value: bool) {
        self.upper.set_bit_assign(23, value);
    }

    pub fn base_address(self) -> u64 {
        ((self.lower as u64) & 0xFFFF)
            | ((self.middle as u64) & !0xFFFF)
            | ((self.upper as u64) << 32)
    }

    pub fn set_base_address(&mut self, offset: u64) {
        self.lower = (self.lower & !0xFFFF) | ((offset & 0xFFFF) as u32);
        self.middle = (self.middle & 0xFFFF) | ((offset & !0xFFFF) as u32);
        self.upper = (offset >> 32) as u32;
    }

    pub const fn limit(self) -> u32 {
        (self.lower & 0xFFFF) | ((self.upper & 0xF0000) << 16)
    }

    pub fn set_limit(&mut self, value: u32) {
        self.lower = (self.lower & !0xFFFF) | (value & 0xFFFF);
        self.upper = (self.upper & !0xF_0000) | (value & 0xF_0000);
    }

    pub fn privilege_level(self) -> ProtectionRing {
        ProtectionRing::try_from(((self.middle & 0x6000) >> 13) as u8).unwrap()
    }

    pub fn set_privilege_level(&mut self, privilege: ProtectionRing) {
        self.middle = (self.middle & !0x6000) | ((privilege as u32) << 13);
    }

    pub fn descriptor_type(self) -> DescriptorType {
        DescriptorType::try_from(self.middle & 0xF00).unwrap()
    }

    pub fn set_descriptor_type(&mut self, descriptor_type: DescriptorType) {
        self.middle = (self.middle & !0xF00) | descriptor_type as u32;
    }
}

numeric_enum!(
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub enum DescriptorType {
        TssAvailable = 0x900,
        TssBusy = 0xB00,
        Ldt = 0x200,
    }

    impl TryFrom<u32>;
);
