//**************************************************************************************************
// timing.rs                                                                                       *
// Copyright (c) 2021 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use x86::apic::{LocalX2apic, LocalXapic};

pub fn init() {}

pub enum ScheduleTimer {
    None,
    X2apic(LocalX2apic),
    Xapic(LocalXapic),
}
