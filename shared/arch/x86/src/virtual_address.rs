//**************************************************************************************************
// virtual_address.rs                                                                              *
// Copyright (c) 2020 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use bits::ReadBit;
use core::convert::{TryFrom, TryInto};
use core::fmt;
use memory::address;

#[derive(Copy, Clone, Debug)]
pub struct NonCanonicalAddressError {
    address: u64,
}

impl NonCanonicalAddressError {
    pub fn new(address: u64) -> Self {
        Self { address }
    }
}

impl fmt::Display for NonCanonicalAddressError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "The address \"{:#X}\" is not canonical.", self.address)
    }
}

pub trait VirtualAddress64: Into<u64> + TryFrom<u64> + Clone + Copy {
    fn pml4_index(self) -> usize;

    fn directory_ptr_index(self) -> usize;

    fn directory_index(self) -> usize;

    fn table_index(self) -> usize;

    fn offset_4kb(self) -> u64;

    fn offset_2mb(self) -> u64;

    fn offset_1gb(self) -> u64;
}

macro_rules! virtual_address_64_type {
    ($type:ident, $first:expr, $last:expr) => {
        address!(pub struct $type : u64);

        impl VirtualAddress64 for $type {
            fn pml4_index(self) -> usize {
                self.0
                    .read_bit_segment(39, 0, 9)
                    .unwrap()
                    .try_into()
                    .unwrap()
            }

            fn directory_ptr_index(self) -> usize {
                self.0
                    .read_bit_segment(30, 0, 9)
                    .unwrap()
                    .try_into()
                    .unwrap()
            }

            fn directory_index(self) -> usize {
                self.0
                    .read_bit_segment(21, 0, 9)
                    .unwrap()
                    .try_into()
                    .unwrap()
            }

            fn table_index(self) -> usize {
                self.0
                    .read_bit_segment(12, 0, 9)
                    .unwrap()
                    .try_into()
                    .unwrap()
            }

            fn offset_4kb(self) -> u64 {
                self.0.read_bit_segment(0, 0, 12).unwrap()
            }

            fn offset_2mb(self) -> u64 {
                self.0.read_bit_segment(0, 0, 21).unwrap()
            }

            fn offset_1gb(self) -> u64 {
                self.0.read_bit_segment(0, 0, 30).unwrap()
            }
        }

        impl TryFrom<u64> for $type {
            type Error = NonCanonicalAddressError;

            fn try_from(value: u64) -> Result<Self, Self::Error> {
                let end_value = value.read_bit($first - 1).unwrap();
                for i in $first..$last {
                    if value.read_bit(i).unwrap() != end_value {
                        return Err(NonCanonicalAddressError::new(value));
                    }
                }
                Ok($type(value))
            }
        }
    };
}

virtual_address_64_type!(VirtualAddress48, 48, 63);

virtual_address_64_type!(VirtualAddress57, 57, 63);

impl VirtualAddress57 {
    pub fn pml5_index(self) -> usize {
        self.0
            .read_bit_segment(48, 0, 9)
            .unwrap()
            .try_into()
            .unwrap()
    }
}
