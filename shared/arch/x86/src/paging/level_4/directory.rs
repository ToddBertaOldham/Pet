//**************************************************************************************************
// directory.rs                                                                                    *
// Copyright (c) 2019-2020 Todd Berta-Oldham                                                       *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use super::table::Table;
use crate::PhysicalAddress52;
use bits::BitField;
use core::ops::{Index, IndexMut};
use core::slice::{Iter, IterMut};

#[repr(align(4096))]
pub struct DirectoryTable([DirectoryEntry; 512]);

impl DirectoryTable {
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
    Page2Mb(PhysicalAddress52),
}

impl DirectoryValue {
    pub fn table_ptr(&self) -> Option<*mut Table> {
        unimplemented!()
    }
    pub fn page_2mb_ptr(&self) -> Option<*mut u8> {
        unimplemented!()
    }
}

level_4_paging_entry!(pub struct DirectoryEntry);

impl DirectoryEntry {
    pub fn value(self) -> DirectoryValue {
        if self.0.is_bit_set(0).unwrap() {
            if self.0.is_bit_set(7).unwrap() {
            } else {
            }
            unimplemented!()
        } else {
            DirectoryValue::None
        }
    }

    pub fn set_value(&mut self, value: DirectoryValue) {
        match value {
            DirectoryValue::None => {
                self.0.set_bit(0, false).unwrap();
                self.0.set_bit(7, false).unwrap();
            }
            DirectoryValue::Table(pointer) => {
                self.0.set_bit(0, true).unwrap();
                self.0.set_bit(7, false).unwrap();
            }
            DirectoryValue::Page2Mb(pointer) => {
                self.0.set_bit(0, true).unwrap();
                self.0.set_bit(7, true).unwrap();
            }
        }
    }
}
