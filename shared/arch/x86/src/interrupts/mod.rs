// *************************************************************************
// mod.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

mod ist;
pub mod size_64;

pub use ist::*;

pub unsafe fn enable() {
    asm!("sti" :::: "volatile");
}

pub unsafe fn disable() {
    asm!("cli" :::: "volatile");
}