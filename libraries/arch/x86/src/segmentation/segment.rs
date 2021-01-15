//**************************************************************************************************
// segment.rs                                                                                      *
// Copyright (c) 2019-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::privilege::ProtectionRing;
use bits::{GetBit, SetBit, SetBitAssign};
use core::convert::TryFrom;
use enums::{numeric_enum, EnumIntegerConvertError};

//TODO Finish cleanup.

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Descriptor {
    lower: u32,
    upper: u32,
}

impl Descriptor {
    pub const fn new() -> Self {
        Descriptor {
            lower: 0,
            upper: 0x1000,
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

    pub fn db_enabled(self) -> bool {
        self.upper.get_bit(22)
    }

    pub fn set_db_enabled(&mut self, value: bool) {
        self.upper.set_bit_assign(22, value);
    }

    pub fn granularity_enabled(self) -> bool {
        self.upper.get_bit(23)
    }

    pub fn set_granularity_enabled(&mut self, value: bool) {
        self.upper.set_bit_assign(23, value);
    }

    pub fn base_address(self) -> u32 {
        self.lower.get_bits(16, 0, 16)
            | self.upper.get_bits(0, 16, 8)
            | self.upper.get_bits(24, 24, 8)
    }

    pub fn set_base_address(&mut self, value: u32) {
        self.lower.set_bits_assign(value, 16, 0, 16);
        self.upper.set_bits_assign(value, 0, 16, 8);
        self.upper.set_bits_assign(value, 24, 24, 8);
    }

    pub fn limit(self) -> u32 {
        self.lower.get_bits(0, 0, 16) | self.upper.get_bits(16, 16, 4)
    }

    pub fn set_limit(&mut self, value: u32) {
        self.lower.set_bits_assign(value, 0, 0, 16);
        self.upper.set_bits_assign(value, 16, 16, 4);
    }

    pub fn privilege_level(self) -> ProtectionRing {
        ProtectionRing::try_from(self.upper.get_bits(13, 0, 2) as u8).unwrap()
    }

    pub fn set_privilege_level(&mut self, privilege: ProtectionRing) {
        self.upper.set_bits_assign(privilege as u32, 13, 0, 2);
    }

    pub fn descriptor_type(self) -> DescriptorType {
        DescriptorType::try_from(self.upper & 0x20_0F00).unwrap()
    }

    pub fn set_descriptor_type(&mut self, descriptor_type: DescriptorType) {
        self.upper = (self.upper & !0x20_0F00) | u32::from(descriptor_type);
    }
}

impl Default for Descriptor {
    fn default() -> Self {
        Self::new()
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
            DescriptorType::LongCode(inner) => (inner as u32).set_bit(21, true),
        }
    }
}

impl TryFrom<u32> for DescriptorType {
    type Error = EnumIntegerConvertError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value.get_bit(21) {
            Ok(DescriptorType::LongCode(CodeDescriptorType::try_from(
                value.set_bit(21, false),
            )?))
        } else {
            if value > 0x700 {
                Ok(DescriptorType::Code(CodeDescriptorType::try_from(value)?))
            } else {
                Ok(DescriptorType::Data(DataDescriptorType::try_from(value)?))
            }
        }
    }
}

numeric_enum!(
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

    impl TryFrom<u32>;
);

numeric_enum!(
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

    impl TryFrom<u32>;
);
