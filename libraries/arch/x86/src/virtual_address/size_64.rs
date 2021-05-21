//**************************************************************************************************
// size_64.rs                                                                                      *
// Copyright (c) 2020-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use core::convert::{TryFrom, TryInto};
use core::fmt;
use core::ops::Neg;
use memory::{address_wrapper, AlignAssign, GetBit, SetBit};

pub trait VirtualAddress64: Into<u64> + TryFrom<u64> + Clone + Copy {
    fn offset(self, amount: i64) -> Result<Self, VirtualAddress64Error> {
        if amount.is_negative() {
            self.add(amount.neg().try_into().unwrap())
        } else {
            self.sub(amount.try_into().unwrap())
        }
    }

    fn add(self, amount: u64) -> Result<Self, VirtualAddress64Error>;

    fn sub(self, amount: u64) -> Result<Self, VirtualAddress64Error>;

    // Pml 4

    fn pml4_index(self) -> usize;

    fn offset_pml4_index(
        self,
        amount: i64,
        clear_lower: bool,
    ) -> Result<Self, VirtualAddress64Error> {
        if amount.is_negative() {
            self.sub_pml4_index(amount.neg().try_into().unwrap(), clear_lower)
        } else {
            self.add_pml4_index(amount.try_into().unwrap(), clear_lower)
        }
    }

    fn add_pml4_index(self, amount: u64, clear_lower: bool) -> Result<Self, VirtualAddress64Error>;

    fn sub_pml4_index(self, amount: u64, clear_lower: bool) -> Result<Self, VirtualAddress64Error>;

    // Directory ptr

    fn directory_ptr_index(self) -> usize;

    fn offset_directory_ptr_index(
        self,
        amount: i64,
        clear_lower: bool,
    ) -> Result<Self, VirtualAddress64Error> {
        if amount.is_negative() {
            self.sub_directory_ptr_index(amount.neg().try_into().unwrap(), clear_lower)
        } else {
            self.add_directory_ptr_index(amount.try_into().unwrap(), clear_lower)
        }
    }

    fn add_directory_ptr_index(
        self,
        amount: u64,
        clear_lower: bool,
    ) -> Result<Self, VirtualAddress64Error>;

    fn sub_directory_ptr_index(
        self,
        amount: u64,
        clear_lower: bool,
    ) -> Result<Self, VirtualAddress64Error>;

    // Directory

    fn directory_index(self) -> usize;

    fn offset_directory_index(
        self,
        amount: i64,
        clear_lower: bool,
    ) -> Result<Self, VirtualAddress64Error> {
        if amount.is_negative() {
            self.sub_directory_index(amount.neg().try_into().unwrap(), clear_lower)
        } else {
            self.add_directory_index(amount.try_into().unwrap(), clear_lower)
        }
    }

    fn add_directory_index(
        self,
        amount: u64,
        clear_lower: bool,
    ) -> Result<Self, VirtualAddress64Error>;

    fn sub_directory_index(
        self,
        amount: u64,
        clear_lower: bool,
    ) -> Result<Self, VirtualAddress64Error>;

    // Table

    fn table_index(self) -> usize;

    fn offset_table_index(
        self,
        amount: i64,
        clear_lower: bool,
    ) -> Result<Self, VirtualAddress64Error> {
        if amount.is_negative() {
            self.sub_table_index(amount.neg().try_into().unwrap(), clear_lower)
        } else {
            self.add_table_index(amount.try_into().unwrap(), clear_lower)
        }
    }

    fn add_table_index(self, amount: u64, clear_lower: bool)
        -> Result<Self, VirtualAddress64Error>;

    fn sub_table_index(self, amount: u64, clear_lower: bool)
        -> Result<Self, VirtualAddress64Error>;

    // Page offsets

    fn page_offset_4_kib(self) -> u64;

    fn page_offset_2_mib(self) -> u64;

    fn page_offset_1_gib(self) -> u64;
}

#[derive(Copy, Clone, Debug)]
pub struct VirtualAddress64Error;

impl fmt::Display for VirtualAddress64Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "Cannot create a virtual address from a non-canonical value."
        )
    }
}

macro_rules! create_virtual_address_64 {
    ($type:ident, $start:expr) => {
        address_wrapper!(pub struct $type : u64);

        impl $type {
            pub fn new(address: u64) -> Self {
                Self::new_checked(address).expect("Address is not canonical.")
            }

            pub fn new_checked(address: u64) -> Result<Self, VirtualAddress64Error> {
                let end_value = address.get_bit($start - 1);
                for i in $start..64 {
                    if address.get_bit(i) != end_value {
                        return Err(VirtualAddress64Error);
                    }
                }
                Ok($type(address))
            }

            fn new_apply_extension(value: u64) -> Result<Self, VirtualAddress64Error> {
                // Read the current extension and make sure it is either all 1s or all 0s.
                // If the value provided had grown or shrunk into the invalid area than it is
                // essentially an integer overflow/underflow.
                let extension = value.get_bits($start, 0, 64 - $start);

                if extension != 0 &&
                    extension != u64::MAX.get_bits(0, 0, 64 - $start) {
                    return Err(VirtualAddress64Error);
                }

                let new_value = {
                    if value.get_bit($start - 1) {
                        value.set_bits(u64::MAX, $start, $start, 64 - $start)
                    } else {
                        value.set_bits(0, $start, $start, 64 - $start)
                    }
                };

                Ok(Self(new_value))
            }
        }

        impl VirtualAddress64 for $type {
            fn add(self, amount: u64) -> Result<Self, VirtualAddress64Error> {
                Self::new_apply_extension(self.0 + amount)
            }

            fn sub(self, amount: u64) -> Result<Self, VirtualAddress64Error> {
                Self::new_apply_extension(self.0 - amount)
            }

            fn pml4_index(self) -> usize {
                self.0
                    .get_bits(39, 0, 9)
                    .try_into()
                    .unwrap()
            }

            fn add_pml4_index(self, amount: u64, clear_lower: bool)
                -> Result<Self, VirtualAddress64Error> {
                let mut next = self.0 + (amount * 0x80_0000_0000);
                if clear_lower {
                    next.align_down_assign(0x80_0000_0000).unwrap();
                }
                Self::new_apply_extension(next)
            }

            fn sub_pml4_index(self, amount: u64, clear_lower: bool)
                -> Result<Self, VirtualAddress64Error> {
                let mut next = self.0 - (amount * 0x80_0000_0000);
                if clear_lower {
                    next.align_down_assign(0x80_0000_0000).unwrap();
                }
                Self::new_apply_extension(next)
            }

            fn directory_ptr_index(self) -> usize {
                self.0
                    .get_bits(30, 0, 9)
                    .try_into()
                    .unwrap()
            }

            fn add_directory_ptr_index(self, amount: u64, clear_lower: bool)
                -> Result<Self, VirtualAddress64Error> {
                let mut next = self.0 + (amount * 1073741824);
                if clear_lower {
                    next.align_down_assign(1073741824).unwrap();
                }
                Self::new_apply_extension(next)
            }

            fn sub_directory_ptr_index(self, amount: u64, clear_lower: bool)
                -> Result<Self, VirtualAddress64Error> {
                let mut next = self.0 - (amount * 1073741824);
                if clear_lower {
                    next.align_down_assign(1073741824).unwrap();
                }
                Self::new_apply_extension(next)
            }

            fn directory_index(self) -> usize {
                self.0
                    .get_bits(21, 0, 9)
                    .try_into()
                    .unwrap()
            }

            fn add_directory_index(self, amount: u64, clear_lower: bool)
                -> Result<Self, VirtualAddress64Error> {
                let mut next = self.0 + (amount * 2097152);
                if clear_lower {
                    next.align_down_assign(2097152).unwrap();
                }
                Self::new_apply_extension(next)
            }

            fn sub_directory_index(self, amount: u64, clear_lower: bool)
                -> Result<Self, VirtualAddress64Error> {
                let mut next = self.0 - (amount * 2097152);
                if clear_lower {
                    next.align_down_assign(2097152).unwrap();
                }
                Self::new_apply_extension(next)
            }

            fn table_index(self) -> usize {
                self.0
                    .get_bits(12, 0, 9)
                    .try_into()
                    .unwrap()
            }

            fn add_table_index(self, amount: u64, clear_lower: bool)
                -> Result<Self, VirtualAddress64Error> {
                let mut next = self.0 + (amount * 4096);
                if clear_lower {
                    next.align_down_assign(4096).unwrap();
                }
                Self::new_apply_extension(next)
            }

            fn sub_table_index(self, amount: u64, clear_lower: bool)
                -> Result<Self, VirtualAddress64Error> {
                let mut next = self.0 - (amount * $crate::paging::PAGE_4_KIB_SIZE_IN_BYTES);
                if clear_lower {
                    next.align_down_assign($crate::paging::PAGE_4_KIB_SIZE_IN_BYTES).unwrap();
                }
                Self::new_apply_extension(next)
            }

            fn page_offset_4_kib(self) -> u64 {
                self.0.get_bits(0, 0, 12)
            }

            fn page_offset_2_mib(self) -> u64 {
                self.0.get_bits(0, 0, 21)
            }

            fn page_offset_1_gib(self) -> u64 {
                self.0.get_bits(0, 0, 30)
            }
        }

        impl TryFrom<u64> for $type {
            type Error = VirtualAddress64Error;

            fn try_from(value: u64) -> Result<Self, Self::Error> {
                Self::new_checked(value)
            }
        }

        impl TryFrom<usize> for $type {
            type Error = VirtualAddress64Error;

            fn try_from(value: usize) -> Result<Self, Self::Error> {
                let converted_value = u64::try_from(value).map_err(|_| VirtualAddress64Error)?;
                Self::try_from(converted_value)
            }
        }

        impl<T> TryFrom<*const T> for $type {
            type Error = VirtualAddress64Error;

            fn try_from(value: *const T) -> Result<Self, Self::Error> {
                let converted_value = value as usize;
                Self::try_from(converted_value)
            }
        }

        impl<T> TryFrom<*mut T> for $type {
            type Error = VirtualAddress64Error;

            fn try_from(value: *mut T) -> Result<Self, Self::Error> {
                let converted_value = value as usize;
                Self::try_from(converted_value)
            }
        }
    };
}

create_virtual_address_64!(VirtualAddress48, 48);

create_virtual_address_64!(VirtualAddress57, 57);

impl VirtualAddress57 {
    const PML_5_START: u64 = 0x1_0000_0000_0000;

    pub fn pml_5_index(self) -> usize {
        self.0.get_bits(48, 0, 9).try_into().unwrap()
    }

    pub fn offset_pml_5_index(
        self,
        amount: i64,
        clear_lower: bool,
    ) -> Result<Self, VirtualAddress64Error> {
        if amount.is_negative() {
            self.sub_pml_5_index(amount.neg().try_into().unwrap(), clear_lower)
        } else {
            self.add_pml_5_index(amount.try_into().unwrap(), clear_lower)
        }
    }

    pub fn add_pml_5_index(
        self,
        amount: u64,
        clear_lower: bool,
    ) -> Result<Self, VirtualAddress64Error> {
        let mut next = self.0 + (amount * Self::PML_5_START);
        if clear_lower {
            next.align_down_assign(Self::PML_5_START).unwrap();
        }
        Self::new_apply_extension(next)
    }

    pub fn sub_pml_5_index(
        self,
        amount: u64,
        clear_lower: bool,
    ) -> Result<Self, VirtualAddress64Error> {
        let mut next = self.0 - (amount * Self::PML_5_START);
        if clear_lower {
            next.align_down_assign(Self::PML_5_START).unwrap();
        }
        Self::new_apply_extension(next)
    }
}
