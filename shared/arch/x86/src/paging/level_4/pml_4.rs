//**************************************************************************************************
// pml_4.rs                                                                                        *
// Copyright (c) 2019-2020 Todd Berta-Oldham                                                       *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use bits::BitField;
use super::directory_ptr::DirectoryPtrTable;

#[repr(align(4096))]
pub struct Pml4Table([Pml4Entry; 512]);

#[derive(Copy, Clone, Debug)]
pub enum Pml4Value {
    None,
    DirectoryPtrTable(PhysicalAddress52),
}

impl Pml4Value {
    pub fn directory_ptr_table(&self) -> Option<*mut DirectoryPtrTable> {

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
