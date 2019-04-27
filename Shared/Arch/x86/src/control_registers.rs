// *************************************************************************
// control_registers.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use bits::BitField;

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct CR3Value(u64);

impl CR3Value {
    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn write_through_enabled(&self) -> bool {
        self.0.is_bit_set(3)
    }

    pub fn set_write_through_enabled(&mut self, value : bool) {
        self.0.set_bit(3, value);
    }

    pub fn cache_disabled(&self) -> bool {
        self.0.is_bit_set(4)
    }

    pub fn set_cache_disabled(&mut self, value : bool) {
        self.0.set_bit(4, value);
    }

    pub fn physical_address(&self) -> u64 {
        self.0 & 0xFFFFFFFFFF000
    }

    pub fn set_physical_address(&mut self, address : u64) {
        self.0 |= 0xFFFFFFFFFF000 & address;
    }
}

impl From<u64> for CR3Value {
    fn from(value : u64) -> CR3Value {
        CR3Value(value)
    }
}

pub mod cr3 {
    use super::CR3Value;

    pub fn read() -> CR3Value {
        let value : CR3Value;
        unsafe {
            asm!("mov %cr3, $0" : "=r"(value));
        }
        value
    }

    pub unsafe fn write(value : CR3Value) {
        asm!("mov $0, %cr3" :: "r"(value))
    }
}
