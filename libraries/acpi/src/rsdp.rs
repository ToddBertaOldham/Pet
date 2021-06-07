//**************************************************************************************************
// rsdp.rs                                                                                         *
// Copyright (c) 2020-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::{Interface, RootEntry, Rsdt, RsdtIter, Xsdt, XsdtIter};
use memory::{Address32, Address64};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Rsdp2 {
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

impl Rsdp2 {
    pub const SIGNATURE: &'static [u8; 8] = Rsdp1::SIGNATURE;
    pub const REVISION: u8 = 2;

    pub fn check_signature(&self) -> bool {
        &self.signature == Self::SIGNATURE
    }

    pub unsafe fn iter<'a, T: Interface>(&self, interface: &'a T) -> RsdpIter<'a, T> {
        let rsdt_ptr: *mut Rsdt = interface.convert_to_virtual_ptr(self.rsdt_address);
        let rsdt = &*rsdt_ptr;

        let xsdt_ptr: *mut Xsdt = interface.convert_to_virtual_ptr(self.xsdt_address);
        let xsdt = &*xsdt_ptr;

        RsdpIter::new(Some(rsdt.iter(interface)), Some(xsdt.iter(interface)))
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Rsdp1 {
    pub signature: [u8; 8],
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub revision: u8,
    pub rsdt_address: Address32,
}

impl Rsdp1 {
    pub const SIGNATURE: &'static [u8; 8] = b"RSD PTR ";
    pub const REVISION: u8 = 1;

    pub fn check_signature(&self) -> bool {
        &self.signature == Self::SIGNATURE
    }

    pub unsafe fn iter<'a, T: Interface>(&self, interface: &'a T) -> RsdpIter<'a, T> {
        let rsdt_ptr: *mut Rsdt = interface.convert_to_virtual_ptr(self.rsdt_address);
        let rsdt = &*rsdt_ptr;

        RsdpIter::new(Some(rsdt.iter(interface)), None)
    }
}

pub enum RsdpLayout {
    Two(*mut Rsdp2),
    One(*mut Rsdp1),
    Invalid(*mut u8),
}

pub unsafe fn get_rsdp_layout(ptr: *mut u8) -> RsdpLayout {
    let rsdp = &*(ptr as *mut Rsdp2);
    if rsdp.check_signature() {
        if rsdp.revision < Rsdp2::REVISION {
            RsdpLayout::One(ptr as *mut Rsdp1)
        } else {
            RsdpLayout::Two(ptr as *mut Rsdp2)
        }
    } else {
        RsdpLayout::Invalid(ptr)
    }
}

pub struct RsdpIter<'a, TInterface: Interface> {
    rsdt_iter: Option<RsdtIter<'a, TInterface>>,
    xsdt_iter: Option<XsdtIter<'a, TInterface>>,
}

impl<'a, TInterface: Interface> RsdpIter<'a, TInterface> {
    pub fn new(
        rsdt_iter: Option<RsdtIter<'a, TInterface>>,
        xsdt_iter: Option<XsdtIter<'a, TInterface>>,
    ) -> Self {
        Self {
            rsdt_iter,
            xsdt_iter,
        }
    }
}

impl<'a, TInterface: Interface> Iterator for RsdpIter<'a, TInterface> {
    type Item = RootEntry;

    fn next(&mut self) -> Option<Self::Item> {
        let xsdt_iter = self.xsdt_iter.as_mut()?;
        let xsdt_item = xsdt_iter.next();

        if xsdt_item.is_some() {
            return xsdt_item;
        }

        let rsdt_iter = self.rsdt_iter.as_mut()?;
        rsdt_iter.next()
    }
}
