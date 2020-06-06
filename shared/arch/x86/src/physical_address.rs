//**************************************************************************************************
// physical_address.rs                                                                             *
// Copyright (c) 2020 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use bits::ReadBit;
use core::convert::TryFrom;
use memory::address;
use core::fmt;

#[derive(Copy, Clone, Debug)]
pub struct PhysicalAddressConvertError;

impl fmt::Display for PhysicalAddressConvertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "The value is too large to be converted into the desired physical address.")
    }
}

address!(pub struct PhysicalAddress52 : u64);

impl TryFrom<u64> for PhysicalAddress52 {
    type Error = PhysicalAddressConvertError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value.read_bit_segment(52, 0, 12).unwrap() != 0 {
            Err(PhysicalAddressConvertError)
        } else {
            Ok(PhysicalAddress52(value))
        }
    }
}

impl TryFrom<usize> for PhysicalAddress52 {
    type Error = PhysicalAddressConvertError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let converted_value = u64::try_from(value).map_err(|_| PhysicalAddressConvertError)?;
        Self::try_from(converted_value)
    }
}

impl<T> TryFrom<*mut T> for PhysicalAddress52 {
    type Error = PhysicalAddressConvertError;

    fn try_from(value: *mut T) -> Result<Self, Self::Error> {
        Self::try_from(value as usize)
    }
}

impl<T> TryFrom<*const T> for PhysicalAddress52 {
    type Error = PhysicalAddressConvertError;

    fn try_from(value: *const T) -> Result<Self, Self::Error> {
        Self::try_from(value as usize)
    }
}
