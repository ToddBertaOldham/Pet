//**************************************************************************************************
// cr3.rs                                                                                          *
// Copyright (c) 2019-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::PhysicalAddress52;
use bits::{GetBit, SetBitAssign};
use core::convert::TryFrom;
use memory::{AlignmentError, CheckAlignment};

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct FlagsValue(u64);

impl FlagsValue {
    pub fn new(
        physical_address: PhysicalAddress52,
        write_through_enabled: bool,
        cache_disabled: bool,
    ) -> Result<Self, AlignmentError> {
        let mut instance = Self::null();
        instance.set_physical_address(physical_address)?;
        instance.set_write_through_enabled(write_through_enabled);
        instance.set_cache_disabled(cache_disabled);
        Ok(instance)
    }

    pub const fn null() -> Self {
        FlagsValue(0)
    }

    pub fn write_through_enabled(self) -> bool {
        self.0.get_bit(3)
    }

    pub fn set_write_through_enabled(&mut self, value: bool) {
        self.0.set_bit_assign(3, value);
    }

    pub fn cache_disabled(self) -> bool {
        self.0.get_bit(4)
    }

    pub fn set_cache_disabled(&mut self, value: bool) {
        self.0.set_bit_assign(4, value);
    }

    pub fn physical_address(self) -> PhysicalAddress52 {
        let address = self.0.get_bits(12, 12, 40);
        PhysicalAddress52::try_from(address).unwrap()
    }

    pub fn set_physical_address(
        &mut self,
        address: PhysicalAddress52,
    ) -> Result<(), AlignmentError> {
        if !address.check_alignment(4096) {
            Err(AlignmentError)
        } else {
            self.0.set_bits_assign(address.into(), 12, 12, 40);
            Ok(())
        }
    }
}

impl From<u64> for FlagsValue {
    fn from(value: u64) -> FlagsValue {
        FlagsValue(value)
    }
}

impl From<FlagsValue> for u64 {
    fn from(value: FlagsValue) -> u64 {
        value.0
    }
}

pub fn read<T: From<u64>>() -> T {
    let inner_value: u64;
    unsafe {
        llvm_asm!("mov %cr3, $0" : "=r"(inner_value) ::: "volatile");
    }
    T::from(inner_value)
}

pub unsafe fn write<T: Into<u64>>(value: T) {
    let inner_value = value.into();
    llvm_asm!("mov $0, %cr3" :: "r"(inner_value) :: "volatile");
}
