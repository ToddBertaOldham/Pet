// *************************************************************************
// main.rs
// Copyright 2018 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

#![no_std]

extern crate uefi_core;

use uefi_core::*;
use core::ffi::c_void;

#[no_mangle]
pub extern "win64" fn efi_main(image_handle : *mut c_void, system_table : *mut SystemTable) {
    let program = UEFIProgram::new(system_table);
}