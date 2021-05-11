//**************************************************************************************************
// configuration_table.rs                                                                          *
// Copyright (c) 2020-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::ffi::configuration::Table as FfiTable;
use crate::{system, Error};
use acpi::{Rsdp, RsdpOriginal};
use core::ffi::c_void;

pub enum Table {
    Acpi1(*mut RsdpOriginal),
    Acpi2(*mut Rsdp),
    Sal(*mut c_void),
    Mps(*mut c_void),
    Smbios(*mut c_void),
    Smbios3(*mut c_void),
    Unknown(FfiTable),
}

pub struct TablesIter {
    ptr: *mut FfiTable,
    length: usize,
    index: usize,
}

impl Iterator for TablesIter {
    type Item = Table;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.length {
            unsafe {
                let value = &*self.ptr.add(self.index);
                self.index += 1;
                Some(match value.vendor_guid {
                    FfiTable::ACPI_10_GUID => Table::Acpi1(value.vendor_table as *mut RsdpOriginal),
                    FfiTable::ACPI_20_GUID => Table::Acpi2(value.vendor_table as *mut Rsdp),
                    FfiTable::SAL_GUID => Table::Sal(value.vendor_table),
                    FfiTable::MPS_GUID => Table::Mps(value.vendor_table),
                    FfiTable::SMBIOS_GUID => Table::Smbios(value.vendor_table),
                    FfiTable::SMBIOS_3_GUID => Table::Smbios3(value.vendor_table),
                    _ => Table::Unknown(*value),
                })
            }
        } else {
            None
        }
    }
}

pub fn iter_tables() -> Result<TablesIter, Error> {
    unsafe {
        let table = &*system::table()?;
        Ok(TablesIter {
            ptr: table.configuration_table,
            length: table.number_of_table_entries,
            index: 0,
        })
    }
}
