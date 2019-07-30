// *************************************************************************
// descriptor.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use core::convert::TryFrom;
use encapsulation::BitGetterSetters;
use crate::privilege::ProtectionRing;

#[derive(Clone, Copy, PartialEq, Eq, BitGetterSetters, Default)]
#[repr(C, packed)]
pub struct Descriptor {
    lower : u32,
    #[bit_access(name = "is_present", index = 15, set = true, borrow_self = false)]
    #[bit_access(name = "is_long", index = 21, set = true, borrow_self = false)]
    #[bit_access(name = "db_enabled", index = 22, set = true, borrow_self = false)]
    #[bit_access(name = "granularity_enabled", index = 23, set = true, borrow_self = false)]
    upper : u32
}

impl Descriptor {
    pub const fn new() -> Self {
        Descriptor {
            lower : 0,
            upper : 0
        }
    }

    pub const fn base_address(self) -> u32 {
        (self.lower >> 16) | ((self.upper & 0xFF) << 16) | (self.upper << 24)
    }

    pub fn set_base_address(&mut self, value : u32) {
        self.lower &= 0xFFFF;
        self.lower |= (value & 0xFFFF) << 16;
        self.upper &= 0xFF_FF00;
        self.upper |= (value & 0xFF_0000) >> 16;
        self.upper |= value & 0xFF00_0000;
    }

    pub const fn limit(self) -> u32 {
        (self.lower & 0xFFFF) | ((self.upper & 0xF0000) << 16)
    }

    pub fn set_limit(&mut self, value : u32) {
        self.lower &= 0xFFFF_0000;
        self.lower |= value & 0xFFFF;
    }

    pub fn privilege_level(self) -> ProtectionRing { ProtectionRing::try_from(((self.upper & 0x6000) >> 13) as u8).unwrap() }

    pub fn set_privilege_level(&mut self, privilege : ProtectionRing) { self.upper = (self.upper & !0x6000) | ((privilege as u32) << 13); }

    pub fn descriptor_type(self) -> DescriptorType { DescriptorType::try_from(self.upper & 0x1F00).unwrap() }

    pub fn set_descriptor_type(&mut self, descriptor_type : DescriptorType) { self.upper = (self.upper & !0x1F00) | u32::from(descriptor_type); }
}

impl From<u64> for Descriptor {
    fn from(value : u64) -> Self {
        Self {
            lower : value as u32,
            upper : (value >> 32) as u32
        }
    }
}

impl From<Descriptor> for u64 {
    fn from(value : Descriptor) -> Self {
        (value.lower as u64) | ((value.upper as u64) << 32)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DescriptorType {
    Data(DataDescriptorType),
    Code(CodeDescriptorType),
    System64(SystemDescriptorType64),
    System32(SystemDescriptorType32)
}

impl From<DescriptorType> for u32 {
    fn from(value : DescriptorType) -> Self {
        match value {
            DescriptorType::Data(data) => data as u32,
            DescriptorType::Code(code) => code as u32,
            DescriptorType::System64(system64) => system64 as u32,
            DescriptorType::System32(system32) => system32 as u32
        }
    }
}

impl TryFrom<u32> for DescriptorType {
    type Error = ();

    fn try_from(value : u32) -> Result<Self, Self::Error> {
        //TODO
        unimplemented!()
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DataDescriptorType {
    ReadOnly = 0x1000,
    ReadOnlyAccessed = 0x1100,
    ReadWrite = 0x1200,
    ReadWriteAccessed = 0x1300,
    ReadOnlyExpandDown = 0x1400,
    ReadOnlyExpandDownAccessed = 0x1500,
    ReadWriteExpandDown = 0x1600,
    ReadWriteExpandDownAccessed = 0x1700
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CodeDescriptorType {
    ExecuteOnly = 0x1800,
    ExecuteOnlyAccessed = 0x1900,
    ExecuteRead = 0x1A00,
    ExecuteReadAccessed = 0x1B00,
    ExecuteOnlyConforming = 0x1C00,
    ExecuteOnlyConformingAccessed = 0x1D00,
    ExecuteReadConforming = 0x1E00,
    ExecuteReadConformingAccessed = 0x1F00
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SystemDescriptorType64 {
    LDT = 0x200,
    Tss64BitAvailable = 0x900,
    Tss64BitBusy = 0xB00,
    CallGate64Bit = 0xC00,
    InterruptGate64Bit = 0xE00,
    TrapGate64Bit = 0xF00,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SystemDescriptorType32 {
    Tss16BitAvailable = 0x100,
    LDT = 0x200,
    Tss16BitBusy = 0x300,
    CallGate16Bit = 0x400,
    TaskGate = 0x500,
    InterruptGate16Bit = 0x600,
    TrapGate16Bit = 0x700,
    Tss32BitAvailable = 0x900,
    Tss32BitBusy = 0xB00,
    CallGate32Bit = 0xC00,
    InterruptGate32Bit = 0xE00,
    TrapGate32Bit = 0xF00,
}