//**************************************************************************************************
// table.rs                                                                                        *
// Copyright (c) 2019-2020 Todd Berta-Oldham                                                       *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use bits::BitField;

#[repr(align(4096))]
pub struct Table([TableEntry; 512]);

#[derive(Copy, Clone, Debug)]
pub enum TableValue {
    None,
    Page4Kb(*mut u8)
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
