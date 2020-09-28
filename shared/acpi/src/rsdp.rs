//**************************************************************************************************
// rsdp.rs                                                                                         *
// Copyright (c) 2020 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use memory::{Address32, Address64};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Rsdp {
    pub signature: [u8; 8],
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub revision: u8,
    pub rsdt_address: Address32,
    pub length: u32,
    pub xsdt_address: Address64,
    pub extended_checksum: u8,
    pub reserved: [u8; 3],
}

impl Rsdp {
    pub const SIGNATURE: &'static [u8; 8] = RsdpOriginal::SIGNATURE;
    pub const REVISION: u8 = 2;

    pub fn check_signature(&self) -> bool {
        &self.signature == Self::SIGNATURE
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RsdpOriginal {
    pub signature: [u8; 8],
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub revision: u8,
    pub rsdt_address: Address32,
}

impl RsdpOriginal {
    pub const SIGNATURE: &'static [u8; 8] = b"RSD PTR ";
    pub const REVISION: u8 = 1;

    pub fn check_signature(&self) -> bool {
        &self.signature == Self::SIGNATURE
    }
}

pub enum RsdpLayout {
    Current(*mut Rsdp),
    Original(*mut RsdpOriginal),
    Invalid(*mut u8),
}

pub unsafe fn get_rsdp_layout(ptr: *mut u8) -> RsdpLayout {
    let rsdp = &*(ptr as *mut Rsdp);
    if rsdp.check_signature() {
        if rsdp.revision < Rsdp::REVISION {
            RsdpLayout::Original(ptr as *mut RsdpOriginal)
        } else {
            RsdpLayout::Current(ptr as *mut Rsdp)
        }
    } else {
        RsdpLayout::Invalid(ptr)
    }
}
