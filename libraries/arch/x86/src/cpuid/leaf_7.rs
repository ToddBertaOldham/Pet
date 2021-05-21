//**************************************************************************************************
// leaf_7.rs                                                                                       *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::__cpuid;

#[cfg(target_arch = "x86")]
use core::arch::x86::__cpuid;

use memory::GetBit;

pub struct ExtendedFeatures {
    ebx: u32,
    ecx: u32,
    edx: u32,
}

impl ExtendedFeatures {
    pub fn from_register_values(ebx: u32, ecx: u32, edx: u32) -> Self {
        Self { ebx, ecx, edx }
    }

    pub fn into_ebx(self) -> u32 {
        self.ebx
    }

    pub fn into_ecx(self) -> u32 {
        self.ecx
    }

    pub fn into_edx(self) -> u32 {
        self.edx
    }

    pub fn la57(self) -> bool {
        self.ecx.get_bit(16)
    }
}

pub unsafe fn read() -> ExtendedFeatures {
    let result = __cpuid(1);
    ExtendedFeatures::from_register_values(result.ebx, result.ecx, result.edx)
}
