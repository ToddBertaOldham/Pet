//**************************************************************************************************
// mod.rs                                                                                          *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

pub mod size_64;

pub unsafe fn enable() {
    llvm_asm!("sti" :::: "volatile");
}

pub unsafe fn disable() {
    llvm_asm!("cli" :::: "volatile");
}