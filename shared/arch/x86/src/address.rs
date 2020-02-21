//**************************************************************************************************
// address.rs                                                                                      *
// Copyright (c) 2020 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use memory::address;
use core::convert::TryFrom;
use core::fmt;

#[derive(Copy, Clone, Debug)]
pub struct NonCanonicalAddressError(u64);

impl NonCanonicalAddressError {
    pub fn new(address: u64) -> Self {
        Self(address)
    }
}

impl fmt::Display for NonCanonicalAddressError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "The address \"{:#X}\" is not canonical.", self.0)
    }
}

pub trait Level4VirtualAddress : TryFrom<u64> + Into<u64> + Copy + Clone { }

address!(pub struct VirtualAddress48 : u64);

impl Level4VirtualAddress for VirtualAddress48 { }

impl TryFrom<u64> for VirtualAddress48 {
    type Error = NonCanonicalAddressError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

address!(pub struct VirtualAddress57 : u64);

impl Level4VirtualAddress for VirtualAddress57 { }

impl TryFrom<u64> for VirtualAddress57 {
    type Error = NonCanonicalAddressError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

address!(pub struct PhysicalAddress52 : u64);

impl TryFrom<u64> for PhysicalAddress52 {
    type Error = NonCanonicalAddressError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}