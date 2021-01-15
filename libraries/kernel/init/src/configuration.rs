//**************************************************************************************************
// configuration.rs                                                                                *
// Copyright (c) 2021 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use acpi::{RootEntry, Rsdt, Xsdt};
use memory::{Address32, Address64};

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ConfigurationInfo {
    pub rsdt: Address32,
    pub xsdt: Address64,
}

impl ConfigurationInfo {
    pub unsafe fn iter_acpi_entries(&self) -> Option<impl Iterator<Item = RootEntry>> {
        if let Some(xsdt) = self.xsdt.as_mut_ptr::<Xsdt>().as_mut() {
            Some(xsdt.entry_iter())
        } else if let Some(rsdt) = self.rsdt.as_mut_ptr::<Rsdt>().as_mut() {
            Some(rsdt.entry_iter())
        }
        None
    }
}

impl Default for ConfigurationInfo {
    fn default() -> Self {
        ConfigurationInfo {
            rsdt: Address32::null(),
            xsdt: Address64::null(),
        }
    }
}
