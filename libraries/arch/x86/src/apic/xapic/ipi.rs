//**************************************************************************************************
// ipi.rs                                                                                          *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::apic::xapic::Id;
use memory::split::Halves;
use memory::SetBitAssign;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(transparent)]
pub struct Ipi(u64);

impl Ipi {
    pub const fn new() -> Self {
        Ipi(0)
    }

    pub fn set_destination_id(&mut self, id: Id) {
        let inner_id: u32 = id.into();
        self.0.set_bits_assign(inner_id as u64, 32, 0, 32);
    }

    pub fn destination_id(self) -> Id {
        Id::from(self.0.upper_half())
    }
}

impl From<Ipi> for u64 {
    fn from(value: Ipi) -> Self {
        value.0
    }
}

impl From<u64> for Ipi {
    fn from(value: u64) -> Self {
        Ipi(value)
    }
}
