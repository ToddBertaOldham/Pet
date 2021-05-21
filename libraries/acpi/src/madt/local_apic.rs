//**************************************************************************************************
// local_apic.rs                                                                                   *
// Copyright (c) 2020-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use super::mps::MpsInti;
use memory::{flags, Address64};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct LocalApic {
    pub controller_type: u8,
    pub length: u8,
    pub processor_uid: u8,
    pub apic_id: u8,
    pub flags: LocalApicFlags,
}

impl LocalApic {
    pub const CONTROLLER_TYPE: u8 = 0;
}

flags!(
    pub struct LocalApicFlags : u32 {
        ENABLED = 0b1;
        ONLINE_CAPABLE = 0b10;
    }
);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct LocalApicAddress {
    pub controller_type: u8,
    pub length: u8,
    pub reserved: u16,
    pub address: Address64,
}

impl LocalApicAddress {
    pub const CONTROLLER_TYPE: u8 = 5;
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct LocalApicNmi {
    pub controller_type: u8,
    pub length: u8,
    pub processor_uid: u8,
    pub flags: MpsInti,
    pub lint: u8,
}

impl LocalApicNmi {
    pub const CONTROLLER_TYPE: u8 = 4;
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct LocalSapic {
    pub controller_type: u8,
    pub length: u8,
    pub processor_id: u8,
    pub local_sapic_id: u8,
    pub local_sapic_eid: u8,
    pub reserved: u8,
    pub flags: u32,
    pub processor_uid: u32,
    //TODO Read string
}

impl LocalSapic {
    pub const CONTROLLER_TYPE: u8 = 7;
}
