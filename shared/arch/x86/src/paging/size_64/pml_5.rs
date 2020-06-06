//**************************************************************************************************
// pml_5.rs                                                                                        *
// Copyright (c) 2020 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::paging::size_64::Pml4Table;
use crate::PhysicalAddress52;
use bits::{ReadBit, WriteBitAssign};
use core::ops::{Index, IndexMut};
use core::slice::{Iter, IterMut};

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
            unimplemented!()
        } else {
            Pml5Value::None
        }
    }

    pub fn set_value(&mut self, value: Pml5Value) {
        match value {
            Pml5Value::None => {
                self.0.write_bit_assign(0, false).unwrap();
            }
            Pml5Value::Pml4Table(pointer) => {
                self.0.write_bit_assign(0, true).unwrap();
            }
        }
    }
}
