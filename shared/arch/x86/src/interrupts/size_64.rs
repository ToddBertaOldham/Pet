//**************************************************************************************************
// size_64.rs                                                                                      *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::descriptors::size_64::interrupt_trap_gate;
use core::convert::TryFrom;
use core::mem;
use encapsulation::GetterSetters;

#[repr(C, packed)]
#[derive(Clone, Copy, PartialEq, Eq, GetterSetters)]
pub struct IdtRegisterValue {
    #[field_access(set = true, borrow_self = false)]
    limit: u16,
    #[field_access(set = true, borrow_self = false)]
    entries: u64,
}

impl IdtRegisterValue {
    pub const fn new(limit: u16, entries: u64) -> Self {
        IdtRegisterValue { limit, entries }
    }
    pub fn from_entry_count(entry_count: usize, entries: u64) -> Result<Self, ()> {
        if entry_count > 256 {
            return Err(());
        }
        let limit = u16::try_from(entry_count * mem::size_of::<u64>() - 1).map_err(|_| ())?;
        Ok(Self { limit, entries })
    }
}

impl TryFrom<&'static [u128]> for IdtRegisterValue {
    type Error = ();

    fn try_from(value: &'static [u128]) -> Result<Self, Self::Error> {
        Self::from_entry_count(value.len(), value.as_ptr() as u64)
    }
}

impl TryFrom<&'static [interrupt_trap_gate::Descriptor]> for IdtRegisterValue {
    type Error = ();

    fn try_from(value: &'static [interrupt_trap_gate::Descriptor]) -> Result<Self, Self::Error> {
        Self::from_entry_count(value.len(), value.as_ptr() as u64)
    }
}

pub unsafe fn load_idt(value: &IdtRegisterValue) {
    asm!("lidt ($0)" :: "r"(value) : "memory");
}
