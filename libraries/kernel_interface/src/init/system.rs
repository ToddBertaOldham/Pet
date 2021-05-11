//**************************************************************************************************
// system.rs                                                                                       *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use acpi::{RootEntry, Rsdt, Xsdt};
use memory::{Address32, Address64};
use uefi::ffi::runtime;

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct SystemInfo {
    pub rsdt: Address32,
    pub xsdt: Address64,
    pub uefi_runtime: Address64,
}

impl SystemInfo {
    pub const fn new() -> Self {
        Self {
            rsdt: Address32::null(),
            xsdt: Address64::null(),
            uefi_runtime: Address64::null(),
        }
    }

    pub fn uefi_runtime_ptr(&self) -> *const runtime::Services {
        self.uefi_runtime.as_ptr()
    }
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self::new()
    }
}
