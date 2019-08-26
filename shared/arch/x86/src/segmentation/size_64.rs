//**************************************************************************************************
// size_64.rs                                                                                      *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use super::Selector;
use core::convert::TryFrom;
use core::mem;
use encapsulation::GetterSetters;

#[repr(C, packed)]
#[derive(Clone, Copy, PartialEq, Eq, GetterSetters)]
pub struct GdtRegisterValue {
    #[field_access(set = true, borrow_self = false)]
    limit: u16,
    #[field_access(set = true, borrow_self = false)]
    entries: u64,
}

impl GdtRegisterValue {
    pub const fn new(limit: u16, entries: u64) -> Self {
        GdtRegisterValue { limit, entries }
    }
    pub fn from_entry_count(entry_count: usize, entries: u64) -> Result<Self, ()> {
        let limit = u16::try_from(entry_count * mem::size_of::<u64>() - 1).map_err(|_| ())?;
        Ok(GdtRegisterValue { limit, entries })
    }
}

impl TryFrom<&'static [u64]> for GdtRegisterValue {
    type Error = ();

    fn try_from(value: &'static [u64]) -> Result<Self, Self::Error> {
        Self::from_entry_count(value.len(), value.as_ptr() as u64)
    }
}

pub unsafe fn load_gdt(value: &GdtRegisterValue) {
    asm!("lgdt ($0)" :: "r"(value) : "memory");
}

pub unsafe fn load_cs(selector: Selector) {
    asm!(
    "pushq $0
    leaq 1f, %rax
    pushq %rax
    lretq
    1:
    " :: "ri"(u64::from(selector)) : "rax" "memory");
}
