//**************************************************************************************************
// mod.rs                                                                                          *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

pub mod size_64;

pub unsafe fn enable() {
    asm!("sti" :::: "volatile");
}

pub unsafe fn disable() {
    asm!("cli" :::: "volatile");
}