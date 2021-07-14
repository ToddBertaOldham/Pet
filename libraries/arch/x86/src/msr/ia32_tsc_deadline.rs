//**************************************************************************************************
// ia32_tsc_deadline.rs                                                                            *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::msr::Msr;

const MSR: Msr = Msr::new(0x6E0);

pub unsafe fn read() -> u64 {
    MSR.read()
}

pub unsafe fn write(value: u64) {
    MSR.write(value);
}
