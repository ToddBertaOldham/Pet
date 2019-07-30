// *************************************************************************
// selector.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use encapsulation::BitGetterSetters;
use crate::privilege::ProtectionRing;
use core::convert::TryFrom;

#[derive(Copy, Clone, PartialEq, Eq, BitGetterSetters, Default)]
#[repr(transparent)]
pub struct Selector(
    #[bit_access(name = "is_local", index = 2, set = true, borrow_self = false)]
    u16);

impl Selector {
    pub fn new(index : u16, is_local : bool, privilege_level : ProtectionRing) -> Self {
        let mut value = Self(0);
        value.set_index(index);
        value.set_is_local(is_local);
        value.set_privilege_level(privilege_level);
        value
    }
    
    pub fn index(self) -> u16 { self.0 >> 3 }

    pub fn set_index(&mut self, index : u16) { self.0 = (self.0 & 0x7) | (index << 3) }

    pub fn privilege_level(self) -> ProtectionRing { ProtectionRing::try_from((self.0 & 0x3) as u8).unwrap() }

    pub fn set_privilege_level(&mut self, privilege : ProtectionRing) { self.0 = (self.0 & !0x3) | (privilege as u16); }
}

impl From<Selector> for u16 {
    fn from(value : Selector) -> Self {
        value.0
    }
}

impl From<Selector> for u64 {
     fn from(value : Selector) -> Self {
        value.0 as u64
    }   
}

impl From<Selector> for u32 {
     fn from(value : Selector) -> Self {
        value.0 as u32
    }   
}

impl From<u16> for Selector {
    fn from(value : u16) -> Self {
        Selector(value)
    }
}

pub unsafe fn load_data_selectors(selector : Selector) {
    load_ss(selector);
    load_ds(selector);
    load_es(selector);
    load_fs(selector);
    load_gs(selector);
}

pub unsafe fn load_ss(selector : Selector) {
    asm!("movw $0, %ss" :: "r"(u16::from(selector)) : "memory");
}

pub unsafe fn load_ds(selector : Selector) {
    asm!("movw $0, %ds" :: "r"(u16::from(selector)) : "memory");
}

pub unsafe fn load_es(selector : Selector) {
    asm!("movw $0, %es" :: "r"(u16::from(selector)) : "memory");
}

pub unsafe fn load_fs(selector : Selector) {
    asm!("movw $0, %fs" :: "r"(u16::from(selector)) : "memory");
}

pub unsafe fn load_gs(selector : Selector) {
    asm!("movw $0, %gs" :: "r"(u16::from(selector)) : "memory");
}