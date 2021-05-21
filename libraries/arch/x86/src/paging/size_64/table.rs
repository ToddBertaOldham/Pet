//**************************************************************************************************
// table.rs                                                                                        *
// Copyright (c) 2019-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::PhysicalAddress52;
use core::convert::TryFrom;
use core::ops::{Index, IndexMut};
use core::slice::{Iter, IterMut};
use memory::{AlignmentError, CheckAlignment, GetBit, SetBitAssign};

#[repr(align(4096))]
pub struct Table([TableEntry; 512]);

impl Table {
    pub fn get(&self, index: usize) -> Option<&TableEntry> {
        self.0.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut TableEntry> {
        self.0.get_mut(index)
    }

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
    //TODO Add protection key.
    Page4Kib(PhysicalAddress52),
}

impl TableValue {
    pub fn page_4_kib(self) -> Option<PhysicalAddress52> {
        match self {
            TableValue::Page4Kib(address) => Some(address),
            _ => None,
        }
    }
    pub fn page_4_kib_ptr(self) -> Option<*mut u8> {
        match self {
            TableValue::Page4Kib(address) => Some(address.as_mut_ptr()),
            _ => None,
        }
    }
}

u64_paging_entry!(pub struct TableEntry);

impl TableEntry {
    pub fn value(self) -> TableValue {
        if self.0.get_bit(0) {
            let address = self.0.get_bits(12, 12, 40);
            TableValue::Page4Kib(PhysicalAddress52::try_from(address).unwrap())
        } else {
            TableValue::None
        }
    }

    pub fn set_value(&mut self, value: TableValue) -> Result<(), AlignmentError> {
        match value {
            TableValue::None => {
                self.0.set_bit_assign(0, false);
            }
            TableValue::Page4Kib(address) => {
                if !address.check_alignment(4096) {
                    return Err(AlignmentError);
                }
                self.0.set_bit_assign(0, true);
                self.0.set_bit_assign(1, true);
                self.0.set_bits_assign(address.into(), 12, 12, 40);
            }
        }
        Ok(())
    }
}
