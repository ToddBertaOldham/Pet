//**************************************************************************************************
// table.rs                                                                                        *
// Copyright (c) 2019-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::PhysicalAddress52;
use bits::{ReadBit, WriteBitAssign};
use core::convert::TryFrom;
use core::ops::{Index, IndexMut};
use core::slice::{Iter, IterMut};
use memory::{AlignmentError, CheckAlignment};

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

level_4_paging_entry!(pub struct TableEntry);

impl TableEntry {
    pub fn value(self) -> TableValue {
        if self.0.read_bit(0).unwrap() {
            let address = self.0.read_bit_segment(12, 12, 40).unwrap();
            TableValue::Page4Kib(PhysicalAddress52::try_from(address).unwrap())
        } else {
            TableValue::None
        }
    }

    pub fn set_value(&mut self, value: TableValue) -> Result<(), AlignmentError> {
        match value {
            TableValue::None => {
                self.0.write_bit_assign(0, false).unwrap();
            }
            TableValue::Page4Kib(address) => {
                if !address.check_alignment(4096) {
                    return Err(AlignmentError);
                }
                self.0.write_bit_assign(0, true).unwrap();
                self.0
                    .write_bit_segment_assign(address.into(), 12, 12, 40)
                    .unwrap();
            }
        }
        Ok(())
    }
}
