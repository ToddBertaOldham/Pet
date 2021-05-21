//**************************************************************************************************
// pml_4.rs                                                                                        *
// Copyright (c) 2019-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::paging::size_64::DirectoryPtrTable;
use crate::PhysicalAddress52;
use core::convert::TryFrom;
use core::ops::{Index, IndexMut};
use core::slice::{Iter, IterMut};
use memory::{AlignmentError, CheckAlignment, GetBit, SetBitAssign};

#[repr(align(4096))]
pub struct Pml4Table([Pml4Entry; 512]);

impl Pml4Table {
    pub fn get(&self, index: usize) -> Option<&Pml4Entry> {
        self.0.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Pml4Entry> {
        self.0.get_mut(index)
    }

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
    pub fn directory_ptr_table(self) -> Option<PhysicalAddress52> {
        match self {
            Pml4Value::DirectoryPtrTable(address) => Some(address),
            _ => None,
        }
    }
    pub fn directory_ptr_table_ptr(self) -> Option<*mut DirectoryPtrTable> {
        match self {
            Pml4Value::DirectoryPtrTable(address) => Some(address.as_mut_ptr()),
            _ => None,
        }
    }
}

u64_paging_entry!(pub struct Pml4Entry);

impl Pml4Entry {
    pub fn value(self) -> Pml4Value {
        if self.0.get_bit(0) {
            let address = self.0.get_bits(12, 12, 40);
            Pml4Value::DirectoryPtrTable(PhysicalAddress52::try_from(address).unwrap())
        } else {
            Pml4Value::None
        }
    }

    pub fn set_value(&mut self, value: Pml4Value) -> Result<(), AlignmentError> {
        match value {
            Pml4Value::None => {
                self.0.set_bit_assign(0, false);
            }
            Pml4Value::DirectoryPtrTable(address) => {
                if !address.check_alignment(4096) {
                    return Err(AlignmentError);
                }
                self.0.set_bit_assign(0, true);
                self.0.set_bit_assign(1, true);
                self.0.set_bits_assign(u64::from(address), 12, 12, 40);
            }
        }
        Ok(())
    }
}
