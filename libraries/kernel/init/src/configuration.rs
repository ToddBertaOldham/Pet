//**************************************************************************************************
// configuration.rs                                                                                *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use acpi::{RootEntry, Rsdt, Xsdt};
use memory::{Address32, Address64};

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Configuration {
    pub rsdt: Address32,
    pub xsdt: Address64,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            rsdt: Address32::null(),
            xsdt: Address64::null(),
        }
    }
}
