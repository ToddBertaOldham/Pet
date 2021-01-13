//**************************************************************************************************
// entry.rs                                                                                        *
// Copyright (c) 2021 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::madt::Madt;
use crate::DescriptionHeader;
use core::convert::TryInto;

#[derive(Copy, Clone, Debug)]
pub enum RootEntry {
    Madt(*mut Madt),
    Unknown(*mut DescriptionHeader),
    OutOfRange,
}

impl RootEntry {
    pub unsafe fn from_header_ptr(ptr: *mut DescriptionHeader) -> Self {
        let header = &*ptr;
        match &header.signature {
            Madt::SIGNATURE => RootEntry::Madt(ptr as *mut Madt),
            _ => RootEntry::Unknown(ptr),
        }
    }
}

#[derive(Debug)]
pub struct RootEntryIter<'a, T: Copy + TryInto<*mut DescriptionHeader>> {
    entry_slice: &'a [T],
    index: usize,
}

impl<'a, T: Copy + TryInto<*mut DescriptionHeader>> RootEntryIter<'a, T> {
    pub fn new(entry_slice: &'a [T]) -> Self {
        Self {
            entry_slice,
            index: 0,
        }
    }
}

impl<'a, T: Copy + TryInto<*mut DescriptionHeader>> Iterator for RootEntryIter<'a, T> {
    type Item = RootEntry;

    fn next(&mut self) -> Option<Self::Item> {
        let raw_entry = *self.entry_slice.get(self.index)?;
        self.index += 1;
        match raw_entry.try_into() {
            Ok(ptr) => unsafe { Some(RootEntry::from_header_ptr(ptr)) },
            Err(_) => Some(RootEntry::OutOfRange),
        }
    }
}
