//**************************************************************************************************
// system.rs                                                                                       *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

pub use acpi::{Interface as AcpiInterface, RsdpIter as AcpiRootEntryIter};
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

    pub unsafe fn iter_acpi<'a, T: AcpiInterface>(
        &self,
        interface: &'a T,
    ) -> AcpiRootEntryIter<'a, T> {
        let rsdt_ptr: *mut Rsdt = interface.convert_to_virtual_ptr(self.rsdt);
        let rsdt = &*rsdt_ptr;

        let xsdt_ptr: *mut Xsdt = interface.convert_to_virtual_ptr(self.xsdt);
        let xsdt = &*xsdt_ptr;

        AcpiRootEntryIter::new(Some(rsdt.iter(interface)), Some(xsdt.iter(interface)))
    }
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self::new()
    }
}
