//**************************************************************************************************
// lib.rs                                                                                          *
// Copyright (c) 2019-2020 Todd Berta-Oldham                                                       *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#![no_std]
#![feature(asm)]

pub mod descriptors;
pub mod interrupts;
pub mod paging;
pub mod port_io;
mod privilege;
pub mod segmentation;
mod selector;
pub mod tasks;
pub mod control;
mod address;

pub use privilege::ProtectionRing;
pub use selector::Selector;
pub use address::*;

pub unsafe fn halt() {
    asm!("hlt" :::: "volatile");
}

pub unsafe fn stall() -> ! {
    loop {
        asm!(
        "cli
        hlt"
        :::: "volatile");
    }
}
