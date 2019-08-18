//**************************************************************************************************
// mod.rs                                                                                          *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

pub mod size_64;

use super::selector::Selector;

pub unsafe fn load_task_register(value: Selector) {
    asm!("ltr $0" :: "r"(value) : "memory");
}
