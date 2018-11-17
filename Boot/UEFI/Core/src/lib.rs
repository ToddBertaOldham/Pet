// *************************************************************************
// lib.rs
// Copyright 2018 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

#![no_std]

mod ffi;

pub use self::ffi::SystemTable;

pub struct UEFIProgram {
    system_table : *mut SystemTable
}

impl UEFIProgram {
    pub fn new(system_table : *mut SystemTable) -> Self {
        UEFIProgram { system_table : system_table }
    }

    pub fn exit(&self) {

    }
}