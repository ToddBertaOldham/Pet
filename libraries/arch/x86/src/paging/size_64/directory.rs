//**************************************************************************************************
// directory.rs                                                                                    *
// Copyright (c) 2019-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use super::table::Table;
use crate::paging::PAGE_2_MIB_SIZE_IN_BYTES;
use crate::PhysicalAddress52;
use bits::{GetBit, SetBitAssign};
use core::convert::TryFrom;
use core::ops::{Index, IndexMut};
use core::slice::{Iter, IterMut};
use memory::{AlignmentError, CheckAlignment};

#[repr(align(4096))]
pub struct DirectoryTable([DirectoryEntry; 512]);

impl DirectoryTable {
    pub fn get(&self, index: usize) -> Option<&DirectoryEntry> {
        self.0.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut DirectoryEntry> {
        self.0.get_mut(index)
    }

    pub fn iter(&self) -> Iter<'_, DirectoryEntry> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, DirectoryEntry> {
        self.0.iter_mut()
    }
}

impl Index<usize> for DirectoryTable {
    type Output = DirectoryEntry;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl IndexMut<usize> for DirectoryTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DirectoryValue {
    None,
    Table(PhysicalAddress52),
    //TODO Add protection key.
    Page2Mib(PhysicalAddress52),
}

impl DirectoryValue {
    pub fn table(self) -> Option<PhysicalAddress52> {
        match self {
            DirectoryValue::Table(address) => Some(address),
            _ => None,
        }
    }
    pub fn table_ptr(self) -> Option<*mut Table> {
        match self {
            DirectoryValue::Table(address) => Some(address.as_mut_ptr()),
            _ => None,
        }
    }
    pub fn page_2_mib(self) -> Option<PhysicalAddress52> {
        match self {
            DirectoryValue::Page2Mib(address) => Some(address),
            _ => None,
        }
    }
    pub fn page_2_mib_ptr(self) -> Option<*mut u8> {
        match self {
            DirectoryValue::Page2Mib(address) => Some(address.as_mut_ptr()),
            _ => None,
        }
    }
}

u64_paging_entry!(pub struct DirectoryEntry);

impl DirectoryEntry {
    pub fn value(self) -> DirectoryValue {
        if self.0.get_bit(0) {
            if self.0.get_bit(7) {
                let address = self.0.get_bits(21, 21, 31);
                DirectoryValue::Page2Mib(PhysicalAddress52::try_from(address).unwrap())
            } else {
                let address = self.0.get_bits(12, 12, 40);
                DirectoryValue::Table(PhysicalAddress52::try_from(address).unwrap())
            }
        } else {
            DirectoryValue::None
        }
    }

    pub fn set_value(&mut self, value: DirectoryValue) -> Result<(), AlignmentError> {
        match value {
            DirectoryValue::None => {
                self.0.set_bit_assign(0, false);
                self.0.set_bit_assign(7, false);
            }
            DirectoryValue::Table(address) => {
                if !address.check_alignment(4096) {
                    return Err(AlignmentError);
                }
                self.0.set_bit_assign(0, true);
                self.0.set_bit_assign(7, false);
                self.0.set_bits_assign(address.into(), 12, 12, 40);
            }
            DirectoryValue::Page2Mib(address) => {
                if !address.check_alignment(PAGE_2_MIB_SIZE_IN_BYTES) {
                    return Err(AlignmentError);
                }
                self.0.set_bit_assign(0, true);
                self.0.set_bit_assign(7, true);
                self.0.set_bits_assign(address.into(), 13, 13, 39);
            }
        }
        Ok(())
    }
}
