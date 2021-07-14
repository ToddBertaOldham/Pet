//**************************************************************************************************
// root_entry.rs                                                                                   *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::hpet::Hpet;
use crate::madt::Madt;
use crate::{DescriptionHeader, Interface};
use core::convert::TryInto;

#[derive(Copy, Clone, Debug)]
pub enum RootEntry {
    Madt(*mut Madt),
    Hpet(*mut Hpet),
    Unknown(*mut DescriptionHeader),
    OutOfRange,
}

impl RootEntry {
    pub unsafe fn from_ptr(ptr: *mut DescriptionHeader) -> Self {
        let header = &*ptr;
        match &header.signature {
            Madt::SIGNATURE => RootEntry::Madt(ptr as *mut Madt),
            Hpet::SIGNATURE => RootEntry::Hpet(ptr as *mut Hpet),
            _ => RootEntry::Unknown(ptr),
        }
    }
}

pub struct RootEntryIter<
    'a,
    'b,
    TInterface: Interface,
    TAddress: Copy + TryInto<*mut DescriptionHeader>,
> {
    interface: &'a TInterface,
    entry_address_slice: &'b [TAddress],
    index: usize,
}

impl<'a, 'b, TInterface: Interface, TAddress: Copy + TryInto<*mut DescriptionHeader>>
    RootEntryIter<'a, 'b, TInterface, TAddress>
{
    pub unsafe fn new(interface: &'a TInterface, entry_address_slice: &'b [TAddress]) -> Self {
        Self {
            interface,
            entry_address_slice,
            index: 0,
        }
    }
}

impl<'a, 'b, TInterface: Interface, TAddress: Copy + TryInto<*mut DescriptionHeader>> Iterator
    for RootEntryIter<'a, 'b, TInterface, TAddress>
{
    type Item = RootEntry;

    fn next(&mut self) -> Option<Self::Item> {
        let raw_entry = *self.entry_address_slice.get(self.index)?;
        self.index += 1;
        match raw_entry.try_into() {
            Ok(ptr) => unsafe {
                let converted_ptr = self.interface.convert_to_virtual_ptr(ptr);
                Some(RootEntry::from_ptr(converted_ptr))
            },
            Err(_) => Some(RootEntry::OutOfRange),
        }
    }
}
