//**************************************************************************************************
// tss_ldt.rs                                                                                      *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::privilege::ProtectionRing;
use core::convert::TryFrom;
use encapsulation::BitGetterSetters;

#[derive(Copy, Clone, PartialEq, Eq, Default, BitGetterSetters)]
#[repr(C, packed)]
pub struct Descriptor {
    lower: u32,
    #[bit_access(name = "is_present", index = 15, set = true, borrow_self = false)]
    #[bit_access(name = "avl_enabled", index = 20, set = true, borrow_self = false)]
    #[bit_access(
        name = "granularity_enabled",
        index = 23,
        set = true,
        borrow_self = false
    )]
    middle: u32,
    upper: u32,
    reserved: u32,
}

impl Descriptor {
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

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DescriptorType {
    TssAvailable = 0x900,
    TssBusy = 0xB00,
    Ldt = 0x200,
}

impl TryFrom<u32> for DescriptorType {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x900 => Ok(DescriptorType::TssAvailable),
            0xB00 => Ok(DescriptorType::TssBusy),
            0x200 => Ok(DescriptorType::Ldt),
            _ => Err(()),
        }
    }
}
