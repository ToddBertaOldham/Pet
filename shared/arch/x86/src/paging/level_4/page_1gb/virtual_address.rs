//**************************************************************************************************
// virtual_address.rs                                                                              *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use core::convert::TryFrom;
use bits::BitField;
use super::super::error::NonCanonicalAddressError;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct VirtualAddress(u64);

impl VirtualAddress {
    pub const fn null() -> Self {
        Self(0)
    }

    pub fn is_null(self) -> bool {
        self.0 == 0
    }

    pub fn offset(self) -> u32 {
        (self.0 & 0x3FFF_FFFF) as u32
    }

    pub fn directory_ptr_index(self) -> u16 {
        (self.0 >> 30 & 0x1FF) as u16
    }

    pub fn pml_4_index(self) -> u16 {
        (self.0 >> 39 & 0x1FF) as u16
    }

    pub fn as_ptr<T>(&self) -> *const T {
        self.0 as *const T
    }

    pub fn as_mut_ptr<T>(&mut self) -> *mut T {
        self.0 as *mut T
    }
}

impl TryFrom<u64> for VirtualAddress {
    type Error = NonCanonicalAddressError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let end = value.is_bit_set(47);
        for i in 48..64 {
            if value.is_bit_set(i) != end {
                return Err(NonCanonicalAddressError::new(value));
            }
        }

        Ok(VirtualAddress(value))
    }
}

impl<T> TryFrom<*mut T> for VirtualAddress {
    type Error = NonCanonicalAddressError;

    fn try_from(value: *mut T) -> Result<Self, Self::Error> {
        VirtualAddress::try_from(value as u64)
    }
}

impl<T> TryFrom<*const T> for VirtualAddress {
    type Error = NonCanonicalAddressError;

    fn try_from(value: *const T) -> Result<Self, Self::Error> {
        VirtualAddress::try_from(value as u64)
    }
}

impl From<VirtualAddress> for u64 {
    fn from(value: VirtualAddress) -> u64 {
        value.0
    }
}

impl core::fmt::LowerHex for VirtualAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        core::fmt::LowerHex::fmt(&self.0, f)
    }
}

impl core::fmt::UpperHex for VirtualAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        core::fmt::UpperHex::fmt(&self.0, f)
    }
}

impl core::fmt::Pointer for VirtualAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        core::fmt::LowerHex::fmt(&self.0, f)
    }
}