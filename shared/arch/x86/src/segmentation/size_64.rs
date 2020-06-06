//**************************************************************************************************
// size_64.rs                                                                                      *
// Copyright (c) 2019-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use super::Selector;
use core::convert::TryFrom;
use core::mem;

#[repr(C, packed)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GdtRegisterValue {
    limit: u16,
    entries: u64,
}

impl GdtRegisterValue {
    //TODO Error
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
    llvm_asm!("lgdt ($0)" :: "r"(value) : "memory");
}

pub unsafe fn load_cs(selector: Selector) {
    llvm_asm!(
    "pushq $0
    leaq 1f, %rax
    pushq %rax
    lretq
    1:
    " :: "ri"(u64::from(selector)) : "rax" "memory");
}
