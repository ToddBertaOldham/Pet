//**************************************************************************************************
// rsdt.rs                                                                                         *
// Copyright (c) 2020-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::header::DescriptionHeader;
use crate::{Interface, RootEntryIter};
use core::{mem, slice};
use memory::Address32;

pub type RsdtIter<'a, TInterface> = RootEntryIter<'a, 'static, TInterface, Address32>;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Rsdt {
    pub header: DescriptionHeader,
}

impl Rsdt {
    pub const SIGNATURE: &'static [u8; 4] = b"RSDT";
    pub const REVISION: u32 = 1;

    pub fn check_signature(&self) -> bool {
        &self.header.signature == Self::SIGNATURE
    }

    pub unsafe fn entry_address_slice(&self) -> &'static [Address32] {
        // At the end of the RSDT is an array of 32-bit addresses that point to other tables.
        // The length field in the header includes both the size of the actual table and the
        // address array in bytes.

        let self_segment = memory::Segment::from_ref(self);

        let entries_start_ptr = self_segment.as_end_ptr::<Address32>();
        let entries_memory_size = self.header.length - self_segment.len() as u32;
        let entries_len = entries_memory_size as usize / mem::size_of::<Address32>();

        slice::from_raw_parts(entries_start_ptr, entries_len)
    }

    pub unsafe fn iter<'a, T: Interface>(&self, interface: &'a T) -> RsdtIter<'a, T> {
        RsdtIter::new(interface, self.entry_address_slice())
    }
}
