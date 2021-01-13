//**************************************************************************************************
// msr.rs                                                                                          *
// Copyright (c) 2020-2021 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use split::Halves;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Msr(u32);

impl Msr {
    pub const fn new(address: u32) -> Self {
        Self(address)
    }

    pub const fn address(self) -> u32 {
        self.0
    }

    pub unsafe fn write(self, value: u64) {
        llvm_asm!("wrmsr" :: "{ecx}"(self.0), "{eax}"(value.lower_half()), 
            "{edx}"(value.upper_half()) :: "volatile");
    }

    pub unsafe fn read(self) -> u64 {
        let lower_half: u32;
        let upper_half: u32;
        llvm_asm!("rdmsr" : "={eax}"(lower_half), "={edx}"(upper_half) : 
            "{ecx}"(self.0) :: "volatile");
        u64::from_halves(lower_half, upper_half)
    }
}
