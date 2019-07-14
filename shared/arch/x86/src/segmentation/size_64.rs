// *************************************************************************
// size_64.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use super::Descriptor;
use core::mem;
use encapsulation::GetterSetters;
use core::convert::TryFrom;

#[repr(C, packed)]
#[derive(Clone, Copy, PartialEq, Eq, GetterSetters)]
pub struct GdtPointer {
    #[field_access]
    limit : u16,

    #[field_access]
    entries : u64
}

impl GdtPointer {
    pub const fn new(limit : u16, entries : u64) -> Self {
        GdtPointer { limit, entries }
    }
}

impl TryFrom<&'static [Descriptor]> for GdtPointer {
    //TODO How should errors be handled?
    type Error = core::num::TryFromIntError;

    fn try_from(value : &'static [Descriptor]) -> Result<Self, Self::Error> {
        // Subtract 1 to get end address of last entry.
        let limit = u16::try_from(value.len() * mem::size_of::<Descriptor>() - 1)?;
        let entries = value.as_ptr() as u64;
        Ok(GdtPointer { limit, entries })
    }
}

pub unsafe fn load_gdt(pointer : &GdtPointer) {
    asm!("lgdt ($0)" :: "r"(pointer) : "memory");
}