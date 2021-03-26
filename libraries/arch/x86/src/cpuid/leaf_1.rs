//**************************************************************************************************
// leaf_1.rs                                                                                       *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::__cpuid;

#[cfg(target_arch = "x86")]
use core::arch::x86::__cpuid;

use bits::GetBit;

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

impl From<u32> for AdditionalInformation {
    fn from(value: u32) -> Self {
        AdditionalInformation(value)
    }
}

impl From<AdditionalInformation> for u32 {
    fn from(value: AdditionalInformation) -> Self {
        value.0
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Features {
    ecx: u32,
    edx: u32,
}

impl Features {
    pub fn from_register_values(ecx: u32, edx: u32) -> Self {
        Self { ecx, edx }
    }

    pub fn into_ecx(self) -> u32 {
        self.ecx
    }

    pub fn into_edx(self) -> u32 {
        self.edx
    }

    // ECX

    pub fn x2apic(self) -> bool {
        self.ecx.get_bit(21)
    }

    // EDX

    pub fn apic(self) -> bool {
        self.edx.get_bit(9)
    }
}

pub unsafe fn read() -> (VersionInformation, AdditionalInformation, Features) {
    let result = __cpuid(1);
    (
        result.eax.into(),
        result.ebx.into(),
        Features::from_register_values(result.ecx, result.edx),
    )
}
