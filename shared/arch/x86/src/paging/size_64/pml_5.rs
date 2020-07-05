//**************************************************************************************************
// pml_5.rs                                                                                        *
// Copyright (c) 2020 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::paging::size_64::Pml4Table;
use crate::PhysicalAddress52;
use bits::{ReadBit, WriteBitAssign};
use core::convert::TryFrom;
use core::ops::{Index, IndexMut};
use core::slice::{Iter, IterMut};
use memory::{AlignmentError, CheckAlignment};

#[repr(align(4096))]
pub struct Pml5Table([Pml5Entry; 512]);

impl Pml5Table {
    pub fn iter(&self) -> Iter<'_, Pml5Entry> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, Pml5Entry> {
        self.0.iter_mut()
    }
}

impl Index<usize> for Pml5Table {
    type Output = Pml5Entry;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl IndexMut<usize> for Pml5Table {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Pml5Value {
    None,
    Pml4Table(PhysicalAddress52),
}

impl Pml5Value {
    pub fn pml4_table(self) -> Option<PhysicalAddress52> {
        match self {
            Pml5Value::Pml4Table(address) => Some(address),
            _ => None,
        }
    }
    pub fn pml4_table_ptr(self) -> Option<*mut Pml4Table> {
        match self {
            Pml5Value::Pml4Table(address) => Some(address.as_mut_ptr()),
            _ => None,
        }
    }
}

level_4_paging_entry!(pub struct Pml5Entry);

impl Pml5Entry {
    pub fn value(self) -> Pml5Value {
        if self.0.read_bit(0).unwrap() {
            let address = self.0.read_bit_segment(12, 12, 40).unwrap();
            Pml5Value::Pml4Table(PhysicalAddress52::try_from(address).unwrap())
        } else {
            Pml5Value::None
        }
    }

    pub fn set_value(&mut self, value: Pml5Value) -> Result<(), AlignmentError> {
        match value {
            Pml5Value::None => {
                self.0.write_bit_assign(0, false).unwrap();
            }
            Pml5Value::Pml4Table(address) => {
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
