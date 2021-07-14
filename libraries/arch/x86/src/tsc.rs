//**************************************************************************************************
// tsc.rs                                                                                          *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use memory::split::Halves;

pub unsafe fn read() -> u64 {
    let lower_half: u32;
    let upper_half: u32;

    llvm_asm!("rdtsc" : "={eax}"(lower_half), "={edx}"(upper_half) ::: "volatile");

    u64::from_halves(lower_half, upper_half)
}
