//**************************************************************************************************
// pml_4.rs                                                                                        *
// Copyright (c) 2019-2020 Todd Berta-Oldham                                                       *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use super::directory_ptr::DirectoryPtrTable;
use crate::PhysicalAddress52;
use bits::BitField;
use core::ops::{Index, IndexMut};
use core::slice::{Iter, IterMut};

#[repr(align(4096))]
pub struct Pml4Table([Pml4Entry; 512]);

impl Pml4Table {
    pub fn iter(&self) -> Iter<'_, Pml4Entry> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, Pml4Entry> {
        self.0.iter_mut()
    }
}

impl Index<usize> for Pml4Table {
    type Output = Pml4Entry;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl IndexMut<usize> for Pml4Table {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Pml4Value {
    None,
    DirectoryPtrTable(PhysicalAddress52),
}

impl Pml4Value {
    pub fn directory_ptr_table(self) -> Option<*mut DirectoryPtrTable> {
        if let Pml4Value::DirectoryPtrTable(address) = self {
            Some(address.as_mut_ptr())
        } else {
            None
        }
    }
}

level_4_paging_entry!(pub struct Pml4Entry);

impl Pml4Entry {
    pub fn value(self) -> Pml4Value {
        if self.0.is_bit_set(0).unwrap() {
            unimplemented!()
        } else {
            Pml4Value::None
        }
    }

    pub fn set_value(&mut self, value: Pml4Value) {
        match value {
            Pml4Value::None => {
                self.0.set_bit(0, false).unwrap();
            }
            Pml4Value::DirectoryPtrTable(pointer) => {
                self.0.set_bit(0, true).unwrap();
            }
        }
    }
}
