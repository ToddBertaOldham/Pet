//**************************************************************************************************
// code_data_segment.rs                                                                            *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::privilege::ProtectionRing;
use core::convert::TryFrom;
use encapsulation::BitGetterSetters;

#[derive(Clone, Copy, PartialEq, Eq, BitGetterSetters)]
#[repr(C, packed)]
pub struct Descriptor {
    lower: u32,
    #[bit_access(name = "is_present", index = 15, set = true, borrow_self = false)]
    #[bit_access(name = "avl_enabled", index = 20, set = true, borrow_self = false)]
    #[bit_access(name = "db_enabled", index = 22, set = true, borrow_self = false)]
    #[bit_access(
        name = "granularity_enabled",
        index = 23,
        set = true,
        borrow_self = false
    )]
    upper: u32,
}

impl Descriptor {
    pub const fn base_address(self) -> u32 {
        (self.lower >> 16) | ((self.upper & 0xFF) << 16) | (self.upper & 0xFF00_0000)
    }

    pub fn set_base_address(&mut self, value: u32) {
        self.lower = (self.lower & 0xFFFF) | ((value & 0xFFFF) << 16);
        self.upper = (self.lower & 0xFF_FF00) | ((value & 0xFF_0000) >> 16) | (value & 0xFF00_0000);
    }

    pub const fn limit(self) -> u32 {
        (self.lower & 0xFFFF) | ((self.upper & 0xF0000) << 16)
    }

    pub fn set_limit(&mut self, value: u32) {
        self.lower = (self.lower & !0xFFFF) | (value & 0xFFFF);
        self.upper = (self.upper & !0xF_0000) | (value & 0xF_0000);
    }

    pub fn privilege_level(self) -> ProtectionRing {
        ProtectionRing::try_from(((self.upper & 0x6000) >> 13) as u8).unwrap()
    }

    pub fn set_privilege_level(&mut self, privilege: ProtectionRing) {
        self.upper = (self.upper & !0x6000) | ((privilege as u32) << 13);
    }

    pub fn descriptor_type(self) -> DescriptorType {
        DescriptorType::try_from(self.upper & 0xF00).unwrap()
    }

    pub fn set_descriptor_type(&mut self, descriptor_type: DescriptorType) {
        self.upper = (self.upper & !0xF00) | u32::from(descriptor_type);
    }
}

impl Default for Descriptor {
    fn default() -> Self {
        Descriptor {
            lower: 0,
            upper: 0x1000,
        }
    }
}

impl From<u64> for Descriptor {
    fn from(value: u64) -> Self {
        Self {
            lower: value as u32,
            upper: (value >> 32) as u32,
        }
    }
}

impl From<Descriptor> for u64 {
    fn from(value: Descriptor) -> Self {
        (value.lower as u64) | ((value.upper as u64) << 32)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DescriptorType {
    Data(DataDescriptorType),
    Code(CodeDescriptorType),
    LongCode(CodeDescriptorType),
}

impl From<DescriptorType> for u32 {
    fn from(value: DescriptorType) -> Self {
        match value {
            DescriptorType::Data(inner) => inner as u32,
            DescriptorType::Code(inner) => inner as u32,
            DescriptorType::LongCode(inner) => (inner as u32) | 0x20_0000,
        }
    }
}

impl TryFrom<u32> for DescriptorType {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        //TODO
        unimplemented!()
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DataDescriptorType {
    ReadOnly = 0,
    ReadOnlyAccessed = 0x100,
    ReadWrite = 0x200,
    ReadWriteAccessed = 0x300,
    ReadOnlyExpandDown = 0x400,
    ReadOnlyExpandDownAccessed = 0x500,
    ReadWriteExpandDown = 0x600,
    ReadWriteExpandDownAccessed = 0x700,
}

impl TryFrom<u32> for DataDescriptorType {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        //TODO
        unimplemented!()
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CodeDescriptorType {
    ExecuteOnly = 0x800,
    ExecuteOnlyAccessed = 0x900,
    ExecuteRead = 0xA00,
    ExecuteReadAccessed = 0xB00,
    ExecuteOnlyConforming = 0xC00,
    ExecuteOnlyConformingAccessed = 0xD00,
    ExecuteReadConforming = 0xE00,
    ExecuteReadConformingAccessed = 0xF00,
}

impl TryFrom<u32> for CodeDescriptorType {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        //TODO
        unimplemented!()
    }
}
