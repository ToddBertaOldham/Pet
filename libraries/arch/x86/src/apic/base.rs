//**************************************************************************************************
// base.rs                                                                                         *
// Copyright (c) 2021 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::Msr;
use memory::{Address32, GetBit, SetBitAssign};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Value(u64);

impl Value {
    pub fn is_bsp(&self) -> bool {
        self.0.get_bit(8)
    }

    pub fn set_is_bsp(&mut self, value: bool) {
        self.0.set_bit_assign(8, value);
    }

    pub fn global_enabled(&self) -> bool {
        self.0.get_bit(11)
    }

    pub fn set_global_enabled(&mut self, value: bool) {
        self.0.set_bit_assign(11, value);
    }

    pub fn x2apic_enabled(&self) -> bool {
        self.0.get_bit(10)
    }

    pub fn set_x2apic_enabled(&mut self, value: bool) {
        self.0.set_bit_assign(10, value);
    }

    pub fn address(self) -> Address32 {
        todo!()
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Value(value)
    }
}

impl From<Value> for u64 {
    fn from(value: Value) -> Self {
        value.0
    }
}

pub unsafe fn read() -> Value {
    Msr::new(0x01B).read().into()
}

pub unsafe fn write(value: Value) {
    Msr::new(0x01B).write(value.into());
}
