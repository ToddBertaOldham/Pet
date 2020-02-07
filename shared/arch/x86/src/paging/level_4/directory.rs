//**************************************************************************************************
// directory.rs                                                                                    *
// Copyright (c) 2019-2020 Todd Berta-Oldham                                                       *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use bits::BitField;
use super::table::Table;

#[repr(align(4096))]
pub struct DirectoryTable([DirectoryEntry; 512]);

#[derive(Copy, Clone, Debug)]
pub enum DirectoryValue {
    None,
    Table(*mut Table),
    Page2Mb(*mut u8)
}

impl DirectoryValue {

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
            DirectoryValue::DirectoryTable(pointer) => {
                self.0.set_bit(0, true).unwrap();
                self.0.set_bit(7, false).unwrap();
            },
            DirectoryValue::Page2Mb(pointer) => {
                self.0.set_bit(0, true).unwrap();
                self.0.set_bit(7, true).unwrap();
            }
        }
    }
}
