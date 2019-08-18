//**************************************************************************************************
// mod.rs                                                                                          *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

pub mod size_64;

use super::selector::Selector;

pub unsafe fn load_data_selectors(selector: Selector) {
    load_ss(selector);
    load_ds(selector);
    load_es(selector);
    load_fs(selector);
    load_gs(selector);
}

pub unsafe fn load_ss(selector: Selector) {
    asm!("movw $0, %ss" :: "r"(u16::from(selector)) : "memory");
}

pub unsafe fn load_ds(selector: Selector) {
    asm!("movw $0, %ds" :: "r"(u16::from(selector)) : "memory");
}

pub unsafe fn load_es(selector: Selector) {
    asm!("movw $0, %es" :: "r"(u16::from(selector)) : "memory");
}

pub unsafe fn load_fs(selector: Selector) {
    asm!("movw $0, %fs" :: "r"(u16::from(selector)) : "memory");
}

pub unsafe fn load_gs(selector: Selector) {
    asm!("movw $0, %gs" :: "r"(u16::from(selector)) : "memory");
}
