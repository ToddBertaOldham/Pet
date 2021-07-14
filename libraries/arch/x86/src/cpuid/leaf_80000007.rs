//**************************************************************************************************
// leaf_80000007.rs                                                                                *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::__cpuid;

#[cfg(target_arch = "x86")]
use core::arch::x86::__cpuid;

use memory::GetBit;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Features {
    edx: u32,
}

impl Features {
    pub fn from_register_values(edx: u32) -> Self {
        Self { edx }
    }

    pub fn into_edx(self) -> u32 {
        self.edx
    }

    pub fn invariant_tsc(self) -> bool {
        self.edx.get_bit(8)
    }
}

pub unsafe fn read() -> Features {
    let result = __cpuid(1);
    Features::from_register_values(result.edx)
}
