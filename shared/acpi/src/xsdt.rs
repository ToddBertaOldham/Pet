//**************************************************************************************************
// xsdt.rs                                                                                         *
// Copyright (c) 2020 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::header::DescriptionHeader;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Xsdt {
    pub header: DescriptionHeader,
}

impl Xsdt {
    pub const SIGNATURE: &'static [u8; 4] = b"XSDT";
    pub const REVISION: u32 = 1;
}
