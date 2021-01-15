//**************************************************************************************************
// physical_address.rs                                                                             *
// Copyright (c) 2020 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::paging::{PAGE_1_GIB_SIZE_IN_BYTES, PAGE_2_MIB_SIZE_IN_BYTES, PAGE_4_KIB_SIZE_IN_BYTES};
use bits::GetBit;
use core::convert::{TryFrom, TryInto};
use core::fmt;
use core::ops::Neg;
use memory::{address_wrapper, AlignAssign};

#[derive(Copy, Clone, Debug)]
pub struct PhysicalAddressError;

impl fmt::Display for PhysicalAddressError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "Cannot create a physical address from an oversized value."
        )
    }
}

address_wrapper!(pub struct PhysicalAddress52 : u64);

impl PhysicalAddress52 {
    pub fn offset(self, amount: i64) -> Result<Self, PhysicalAddressError> {
        if amount.is_negative() {
            self.sub(amount.neg().try_into().unwrap())
        } else {
            self.add(amount.try_into().unwrap())
        }
    }

    pub fn add(self, amount: u64) -> Result<Self, PhysicalAddressError> {
        Self::try_from(self.0 + amount)
    }

    pub fn sub(self, amount: u64) -> Result<Self, PhysicalAddressError> {
        Self::try_from(self.0 - amount)
    }

    // Page 4 KIB

    pub fn offset_page_4_kib(self, amount: i64, align: bool) -> Result<Self, PhysicalAddressError> {
        if amount.is_negative() {
            self.sub_page_4_kib(amount.neg().try_into().unwrap(), align)
        } else {
            self.add_page_4_kib(amount.try_into().unwrap(), align)
        }
    }

    pub fn add_page_4_kib(self, amount: u64, align: bool) -> Result<Self, PhysicalAddressError> {
        let mut next = self.0 + (amount * PAGE_4_KIB_SIZE_IN_BYTES);
        if align {
            next.align_down_assign(PAGE_4_KIB_SIZE_IN_BYTES).unwrap();
        }
        Self::try_from(next)
    }

    pub fn sub_page_4_kib(self, amount: u64, align: bool) -> Result<Self, PhysicalAddressError> {
        let mut next = self.0 - (amount * PAGE_4_KIB_SIZE_IN_BYTES);
        if align {
            next.align_down_assign(PAGE_4_KIB_SIZE_IN_BYTES).unwrap();
        }
        Self::try_from(next)
    }

    // Page 2 MIB

    pub fn offset_page_2_mib(self, amount: i64, align: bool) -> Result<Self, PhysicalAddressError> {
        if amount.is_negative() {
            self.sub_page_2_mib(amount.neg().try_into().unwrap(), align)
        } else {
            self.add_page_2_mib(amount.try_into().unwrap(), align)
        }
    }

    pub fn add_page_2_mib(self, amount: u64, align: bool) -> Result<Self, PhysicalAddressError> {
        let mut next = self.0 + (amount * PAGE_2_MIB_SIZE_IN_BYTES);
        if align {
            next.align_down_assign(PAGE_2_MIB_SIZE_IN_BYTES).unwrap();
        }
        Self::try_from(next)
    }

    pub fn sub_page_2_mib(self, amount: u64, align: bool) -> Result<Self, PhysicalAddressError> {
        let mut next = self.0 - (amount * PAGE_2_MIB_SIZE_IN_BYTES);
        if align {
            next.align_down_assign(PAGE_2_MIB_SIZE_IN_BYTES).unwrap();
        }
        Self::try_from(next)
    }

    // Page 1 GIB

    pub fn offset_page_1_gib(self, amount: i64, align: bool) -> Result<Self, PhysicalAddressError> {
        if amount.is_negative() {
            self.sub_page_1_gib(amount.neg().try_into().unwrap(), align)
        } else {
            self.add_page_1_gib(amount.try_into().unwrap(), align)
        }
    }

    pub fn add_page_1_gib(self, amount: u64, align: bool) -> Result<Self, PhysicalAddressError> {
        let mut next = self.0 + (amount * PAGE_1_GIB_SIZE_IN_BYTES);
        if align {
            next.align_down_assign(PAGE_1_GIB_SIZE_IN_BYTES).unwrap();
        }
        Self::try_from(next)
    }

    pub fn sub_page_1_gib(self, amount: u64, align: bool) -> Result<Self, PhysicalAddressError> {
        let mut next = self.0 - (amount * PAGE_1_GIB_SIZE_IN_BYTES);
        if align {
            next.align_down_assign(PAGE_1_GIB_SIZE_IN_BYTES).unwrap();
        }
        Self::try_from(next)
    }
}

impl TryFrom<u64> for PhysicalAddress52 {
    type Error = PhysicalAddressError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value.get_bits(52, 0, 12) != 0 {
            Err(PhysicalAddressError)
        } else {
            Ok(PhysicalAddress52(value))
        }
    }
}

impl TryFrom<usize> for PhysicalAddress52 {
    type Error = PhysicalAddressError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let converted_value = u64::try_from(value).map_err(|_| PhysicalAddressError)?;
        Self::try_from(converted_value)
    }
}

impl<T> TryFrom<*mut T> for PhysicalAddress52 {
    type Error = PhysicalAddressError;

    fn try_from(value: *mut T) -> Result<Self, Self::Error> {
        Self::try_from(value as usize)
    }
}

impl<T> TryFrom<*const T> for PhysicalAddress52 {
    type Error = PhysicalAddressError;

    fn try_from(value: *const T) -> Result<Self, Self::Error> {
        Self::try_from(value as usize)
    }
}
