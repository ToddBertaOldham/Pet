//**************************************************************************************************
// directory_ptr.rs                                                                                *
// Copyright (c) 2019-2020 Todd Berta-Oldham                                                       *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use super::directory::DirectoryTable;
use bits::BitField;

#[repr(align(4096))]
pub struct DirectoryPtrTable([DirectoryPtrEntry; 512]);

#[derive(Copy, Clone, Debug)]
pub enum DirectoryPtrValue {
    None,
    DirectoryTable(PhysicalAddress52),
    Page1Gb(PhysicalAddress52),
}

impl DirectoryPtrValue {
    pub fn directory_table_ptr(&self) -> Option<*mut DirectoryTable> {

    }
    pub fn page_1gb_ptr(&self) -> Option<*mut u8> {

    }
}

level_4_paging_entry!(pub struct DirectoryPtrEntry);

impl DirectoryPtrEntry {
    pub fn value(self) -> DirectoryPtrValue {
        if self.0.is_bit_set(0).unwrap() {
            if self.0.is_bit_set(7).unwrap() {
            } else {
            }
            unimplemented!()
        } else {
            DirectoryPtrValue::None
        }
    }

    pub fn set_value(&mut self, value: DirectoryPtrValue) {
        match value {
            DirectoryPtrValue::None => {
                self.0.set_bit(0, false).unwrap();
                self.0.set_bit(7, false).unwrap();
            }
            DirectoryPtrValue::DirectoryTable(pointer) => {
                self.0.set_bit(0, true).unwrap();
                self.0.set_bit(7, false).unwrap();
            }
            DirectoryPtrValue::Page1Gb(pointer) => {
                self.0.set_bit(0, true).unwrap();
                self.0.set_bit(7, true).unwrap();
            }
        }
    }
}
