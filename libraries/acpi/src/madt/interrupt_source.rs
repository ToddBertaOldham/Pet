//**************************************************************************************************
// interrupt_source.rs                                                                             *
// Copyright (c) 2020 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use super::mps::MpsInti;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct InterruptSourceOverride {
    pub controller_type: u8,
    pub length: u8,
    pub bus: u8,
    pub source: u8,
    pub global_interrupt: u32,
    pub flags: MpsInti,
}

impl InterruptSourceOverride {
    pub const CONTROLLER_TYPE: u8 = 2;
}

pub struct NmiSource {
    pub controller_type: u8,
    pub length: u8,
    pub flags: MpsInti,
    pub global_interrupt: u32,
}

impl NmiSource {
    pub const CONTROLLER_TYPE: u8 = 3;
}
