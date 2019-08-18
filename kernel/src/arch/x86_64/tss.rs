//**************************************************************************************************
// tss.rs                                                                                          *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use x86::tasks::size_64::Tss;

static mut TSS: Tss = Tss::new();

pub fn offset() -> u64 {
    unsafe { (&TSS as *const Tss) as u64 }
}
