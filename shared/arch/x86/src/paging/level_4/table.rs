//**************************************************************************************************
// table.rs                                                                                        *
// Copyright (c) 2019-2020 Todd Berta-Oldham                                                       *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::PhysicalAddress52;
use bits::BitField;
use core::ops::{Index, IndexMut};
use core::slice::{Iter, IterMut};

#[repr(align(4096))]
pub struct Table([TableEntry; 512]);

impl Table {
    pub fn iter(&self) -> Iter<'_, TableEntry> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, TableEntry> {
        self.0.iter_mut()
    }
}

impl Index<usize> for Table {
    type Output = TableEntry;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl IndexMut<usize> for Table {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TableValue {
    None,
    Page4Kb(PhysicalAddress52),
}

impl TableValue {
    pub fn page_4kb_ptr(&self) -> Option<*mut u8> {
        unimplemented!()
    }
}

level_4_paging_entry!(pub struct TableEntry);

impl TableEntry {
    pub fn value(self) -> TableValue {
        if self.0.is_bit_set(0).unwrap() {
            unimplemented!()
        } else {
            TableValue::None
        }
    }

    pub fn set_value(&mut self, value: TableValue) {
        match value {
            TableValue::None => {
                self.0.set_bit(0, false).unwrap();
            }
            TableValue::Page4Kb(address) => {
                self.0.set_bit(0, true).unwrap();
            }
        }
    }
}
