//**************************************************************************************************
// directory_ptr.rs                                                                                *
// Copyright (c) 2019-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use super::directory::DirectoryTable;
use crate::paging::PAGE_1_GIB_SIZE_IN_BYTES;
use crate::PhysicalAddress52;
use bits::{ReadBit, WriteBitAssign};
use core::convert::TryFrom;
use core::ops::{Index, IndexMut};
use core::slice::{Iter, IterMut};
use memory::{AlignmentError, CheckAlignment};

#[repr(align(4096))]
pub struct DirectoryPtrTable([DirectoryPtrEntry; 512]);

impl DirectoryPtrTable {
    pub fn get(&self, index: usize) -> Option<&DirectoryPtrEntry> {
        self.0.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut DirectoryPtrEntry> {
        self.0.get_mut(index)
    }

    pub fn iter(&self) -> Iter<'_, DirectoryPtrEntry> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, DirectoryPtrEntry> {
        self.0.iter_mut()
    }
}

impl Index<usize> for DirectoryPtrTable {
    type Output = DirectoryPtrEntry;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl IndexMut<usize> for DirectoryPtrTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DirectoryPtrValue {
    None,
    DirectoryTable(PhysicalAddress52),
    Page1Gib(PhysicalAddress52),
}

impl DirectoryPtrValue {
    pub fn directory_table(self) -> Option<PhysicalAddress52> {
        match self {
            DirectoryPtrValue::DirectoryTable(address) => Some(address),
            _ => None,
        }
    }
    pub fn directory_table_ptr(self) -> Option<*mut DirectoryTable> {
        match self {
            DirectoryPtrValue::DirectoryTable(address) => Some(address.as_mut_ptr()),
            _ => None,
        }
    }
    pub fn page_1_gib(self) -> Option<PhysicalAddress52> {
        match self {
            DirectoryPtrValue::Page1Gib(address) => Some(address),
            _ => None,
        }
    }
    pub fn page_1_gib_ptr(self) -> Option<*mut u8> {
        match self {
            DirectoryPtrValue::Page1Gib(address) => Some(address.as_mut_ptr()),
            _ => None,
        }
    }
}

level_4_paging_entry!(pub struct DirectoryPtrEntry);

impl DirectoryPtrEntry {
    pub fn value(self) -> DirectoryPtrValue {
        if self.0.read_bit(0).unwrap() {
            if self.0.read_bit(7).unwrap() {
                let address = self.0.read_bit_segment(30, 30, 22).unwrap();
                DirectoryPtrValue::Page1Gib(PhysicalAddress52::try_from(address).unwrap())
            } else {
                let address = self.0.read_bit_segment(12, 12, 40).unwrap();
                DirectoryPtrValue::DirectoryTable(PhysicalAddress52::try_from(address).unwrap())
            }
        } else {
            DirectoryPtrValue::None
        }
    }

    pub fn set_value(&mut self, value: DirectoryPtrValue) -> Result<&mut Self, AlignmentError> {
        match value {
            DirectoryPtrValue::None => {
                self.0.write_bit_assign(0, false).unwrap();
                self.0.write_bit_assign(7, false).unwrap();
            }
            DirectoryPtrValue::DirectoryTable(address) => {
                if !address.check_alignment(4096) {
                    return Err(AlignmentError);
                }
                self.0.write_bit_assign(0, true).unwrap();
                self.0.write_bit_assign(7, false).unwrap();
                self.0
                    .write_bit_segment_assign(address.into(), 12, 12, 40)
                    .unwrap();
            }
            DirectoryPtrValue::Page1Gib(address) => {
                if !address.check_alignment(PAGE_1_GIB_SIZE_IN_BYTES) {
                    return Err(AlignmentError);
                }
                self.0.write_bit_assign(0, true).unwrap();
                self.0.write_bit_assign(7, true).unwrap();
                self.0.write_bit_segment_assign(0, 13, 13, 17).unwrap();
                self.0
                    .write_bit_segment_assign(address.into(), 30, 30, 22)
                    .unwrap();
            }
        }
        Ok(self)
    }
}
