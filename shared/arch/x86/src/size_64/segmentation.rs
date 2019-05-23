// *************************************************************************
// segmentation.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use core::mem;
use core::convert::TryFrom;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C, packed)]
pub struct Descriptor {
    limit : u16,
    base_a : u16,
    base_b : u8,
    flags_a : u8,
    flags_b : u8,
    base_c : u8
}

impl Descriptor {
    pub const fn new() -> Self {
        Descriptor {
            limit : 0,
            base_a : 0,
            base_b : 0,
            flags_a : 0,
            flags_b : 0,
            base_c : 0
        }
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GdtPointer {
    limit : u16,
    entries : u64
}

impl GdtPointer {
    pub fn new(limit : u16, entries : u64) -> Self {
        GdtPointer { limit, entries }
    }

    pub fn limit(&self) -> u16 {
        self.limit
    }

    pub fn entries(&self) -> u64 {
        self.entries
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