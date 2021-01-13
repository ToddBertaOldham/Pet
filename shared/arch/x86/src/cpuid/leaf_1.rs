//**************************************************************************************************
// leaf_1.rs                                                                                       *
// Copyright (c) 2021 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::__cpuid;

#[cfg(target_arch = "x86")]
use core::arch::x86::__cpuid;

use bits::{GetBit, SetBitAssign};

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct VersionInformation(u32);

impl VersionInformation {}

impl From<u32> for VersionInformation {
    fn from(value: u32) -> Self {
        VersionInformation(value)
    }
}

impl From<VersionInformation> for u32 {
    fn from(value: VersionInformation) -> Self {
        value.0
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct AdditionalInformation(u32);

// The feature flags in leaf 1 are split between two registers. The lower half is in ecx and
// the higher half is in edx.

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Features {
    lower: u32,
    upper: u32,
}

impl Features {
    pub fn x2apic(self) -> bool {
        self.lower.get_bit(21)
    }

    pub fn set_x2apic(&mut self, value: bool) {
        self.lower.set_bit_assign(21, value);
    }

    pub fn apic(self) -> bool {
        self.upper.get_bit(9)
    }

    pub fn set_apic(&mut self, value: bool) {
        self.upper.set_bit_assign(9, value);
    }
}

impl From<(u32, u32)> for Features {
    fn from(value: (u32, u32)) -> Self {
        Self {
            lower: value.0,
            upper: value.1,
        }
    }
}

impl From<Features> for (u32, u32) {
    fn from(value: Features) -> Self {
        (value.lower, value.upper)
    }
}

pub unsafe fn read() -> (VersionInformation, AdditionalInformation, Features) {
    let result = __cpuid(1);
    (
        result.eax.into(),
        result.ebx.into(),
        (result.ecx, result.edx).into(),
    )
}
