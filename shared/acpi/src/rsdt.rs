//**************************************************************************************************
// rsd.rs                                                                                          *
// Copyright (c) 2020 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::header::DescriptionHeader;
use crate::madt::Madt;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Rsdt {
    pub header: DescriptionHeader,
}

impl Rsdt {
    pub fn check_signature(&self) -> bool {
        &self.header.signature == Self::SIGNATURE
    }
}

impl Rsdt {
    pub const SIGNATURE: &'static [u8; 4] = b"RSDT";
    pub const REVISION: u32 = 1;
}

pub struct RsdtEntryIter {
    start_ptr: *const u8,
    length: u32,
}

#[derive(Copy, Clone, Debug)]
pub enum RsdtEntry {
    Madt(*const Madt),
    Unknown(*const u8),
}
