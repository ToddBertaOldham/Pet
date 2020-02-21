//**************************************************************************************************
// register_3.rs                                                                                   *
// Copyright (c) 2019-2020 Todd Berta-Oldham                                                       *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::PhysicalAddress52;
use bits::BitField;
use core::convert::TryFrom;
use memory::{Align, AlignmentError};

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct Value(u64);

impl Value {
    pub fn write_through_enabled(self) -> bool {
        self.0.is_bit_set(3).unwrap()
    }

    pub fn set_write_through_enabled(&mut self, value: bool) {
        self.0.set_bit(3, value).unwrap();
    }

    pub fn cache_disabled(self) -> bool {
        self.0.is_bit_set(4).unwrap()
    }

    pub fn set_cache_disabled(&mut self, value: bool) {
        self.0.set_bit(4, value).unwrap();
    }

    pub fn physical_address(self) -> PhysicalAddress52 {
        PhysicalAddress52::try_from(self.0 & 0xFFFFFFFFFF000).unwrap()
    }

    pub fn set_physical_address(
        &mut self,
        address: PhysicalAddress52,
    ) -> Result<(), AlignmentError> {
        if !address.check_alignment(4096) {
            Err(AlignmentError)
        } else {
            self.0 = (self.0 & 0xFFF) | (0xFFFFFFFFFF000 & u64::from(address));
            Ok(())
        }
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Value {
        Value(value)
    }
}

impl From<Value> for u64 {
    fn from(value: Value) -> u64 {
        value.0
    }
}

pub fn read() -> Value {
    let value: Value;
    unsafe {
        asm!("mov %cr3, $0" : "=r"(value) ::: "volatile");
    }
    value
}

pub unsafe fn write(value: Value) {
    asm!("mov $0, %cr3" :: "r"(value) :: "volatile")
}
