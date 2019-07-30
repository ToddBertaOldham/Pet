// *************************************************************************
// mod.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

mod descriptor;

pub use descriptor::*;

use core::convert::TryFrom;
use core::mem;
use encapsulation::GetterSetters;

#[repr(C, packed)]
#[derive(Clone, Copy, PartialEq, Eq, GetterSetters)]
pub struct IdtPointer {
    #[field_access]
    limit : u16,

    #[field_access]
    entries : u64
}

impl IdtPointer {
    pub const fn new(limit : u16, entries : u64) -> Self {
        IdtPointer { limit, entries }
    }
}

impl TryFrom<&'static [Descriptor]> for IdtPointer {
    type Error = ();

    fn try_from(value : &'static [Descriptor]) -> Result<Self, Self::Error> {
        if value.len() > 256 {
            return Err(());
        }
        // Subtract 1 to get end address of last entry.
        let limit = u16::try_from(value.len() * mem::size_of::<Descriptor>() - 1).map_err(|_| ())?;
        let entries = value.as_ptr() as u64;
        Ok(IdtPointer { limit, entries })
    }
}

pub unsafe fn load_idt(pointer : &IdtPointer) {
    asm!("lidt ($0)" :: "r"(pointer) : "memory");
}