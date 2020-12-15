//**************************************************************************************************
// lib.rs                                                                                          *
// Copyright (c) 2019-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#![no_std]
#![feature(asm)]
#![feature(llvm_asm)]

pub mod apic;
pub mod control;
pub mod interrupts;
mod io_port;
mod model_specific_register;
pub mod paging;
mod physical_address;
mod privilege;
pub mod segmentation;
mod selector;
pub mod tasks;
mod virtual_address;

pub use io_port::*;
pub use model_specific_register::*;
pub use physical_address::*;
pub use privilege::ProtectionRing;
pub use selector::Selector;
pub use virtual_address::*;

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
