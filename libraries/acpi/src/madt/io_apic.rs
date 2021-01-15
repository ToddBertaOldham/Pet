//**************************************************************************************************
// io_apic.rs                                                                                      *
// Copyright (c) 2020 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use memory::{Address32, Address64};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct IoApic {
    pub controller_type: u8,
    pub length: u8,
    pub io_apic_id: u8,
    pub reserved: u8,
    pub io_apic_address: Address32,
    pub global_interrupt_base: u32,
}

impl IoApic {
    pub const CONTROLLER_TYPE: u8 = 1;
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct IoSapic {
    pub controller_type: u8,
    pub length: u8,
    pub io_apic_id: u8,
    pub reserved: u8,
    pub global_interrupt_base: u32,
    pub io_sapic_address: Address64,
}

impl IoSapic {
    pub const CONTROLLER_TYPE: u8 = 6;
}
