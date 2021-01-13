//**************************************************************************************************
// xsdt.rs                                                                                         *
// Copyright (c) 2020-2021 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::header::DescriptionHeader;
use crate::RootEntryIter;
use core::slice;
use memory::Address64;

pub type XsdtEntryIter<'a> = RootEntryIter<'a, Address64>;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Xsdt {
    pub header: DescriptionHeader,
}

impl Xsdt {
    pub const SIGNATURE: &'static [u8; 4] = b"XSDT";
    pub const REVISION: u32 = 1;

    pub fn check_signature(&self) -> bool {
        &self.header.signature == Self::SIGNATURE
    }

    pub unsafe fn entry_slice(&self) -> &[Address64] {
        let ptr = memory::Segment::from_ref(self).as_end_ptr();
        slice::from_raw_parts(ptr, self.header.length as usize)
    }

    pub unsafe fn entry_iter(&self) -> XsdtEntryIter {
        XsdtEntryIter::new(self.entry_slice())
    }
}
