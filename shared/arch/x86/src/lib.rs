//**************************************************************************************************
// lib.rs                                                                                          *
// Copyright (c) 2019-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#![no_std]
#![feature(asm)]
#![feature(llvm_asm)]

pub mod descriptors;
pub mod interrupts;
pub mod paging;
pub mod port_io;
mod privilege;
pub mod segmentation;
mod selector;
pub mod tasks;
pub mod control;
mod virtual_address;
mod physical_address;

pub use privilege::ProtectionRing;
pub use selector::Selector;
pub use virtual_address::*;
pub use physical_address::*;

pub unsafe fn halt() {
    llvm_asm!("hlt" :::: "volatile");
}

pub unsafe fn stall() -> ! {
    loop {
        llvm_asm!(
        "cli
        hlt"
        :::: "volatile");
    }
}
