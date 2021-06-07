//**************************************************************************************************
// mod.rs                                                                                          *
// Copyright (c) 2020-2021 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

mod interrupt_source;
mod io_apic;
mod local_apic;
mod mps;

use crate::header::DescriptionHeader;
use memory::{flags, Address32};

pub use interrupt_source::*;
pub use io_apic::*;
pub use local_apic::*;
pub use mps::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Madt {
    pub header: DescriptionHeader,
    pub lic_address: Address32,
    pub flags: MadtFlags,
}

impl Madt {
    pub unsafe fn iter(&self) -> MadtIter {
        let self_segment = memory::Segment::from_ref(self);

        let entries_start_ptr = self_segment.as_mut_end_ptr::<u8>();
        let entries_memory_size = self.header.length - self_segment.len() as u32;

        MadtIter::new(entries_start_ptr, entries_memory_size)
    }
}

impl Madt {
    pub const SIGNATURE: &'static [u8; 4] = b"APIC";
    pub const REVISION: u32 = 5;
}

flags!(
    pub struct MadtFlags : u32 {
        PCAT_COMPAT = 0b1;
    }
);

pub struct MadtIter {
    start_ptr: *mut u8,
    length: u32,
}

impl MadtIter {
    pub unsafe fn new(start_ptr: *mut u8, length: u32) -> Self {
        Self { start_ptr, length }
    }
}

impl Iterator for MadtIter {
    type Item = MadtEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.length == 0 {
            return None;
        }
        unsafe {
            let item = self.start_ptr;
            let length = *item.add(1);

            self.start_ptr = item.add(length as usize);
            self.length = self.length.saturating_sub(length as u32);

            Some(MadtEntry::from_ptr(item))
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum MadtEntry {
    InterruptSourceOverride(*mut InterruptSourceOverride),
    NmiSource(*mut NmiSource),
    IoApic(*mut IoApic),
    IoSapic(*mut IoSapic),
    LocalApic(*mut LocalApic),
    LocalApicNmi(*mut LocalApicNmi),
    LocalApicAddressOverride(*mut LocalApicAddressOverride),
    LocalSapic(*mut LocalSapic),
    Unknown(*mut u8),
}

impl MadtEntry {
    pub unsafe fn from_ptr(ptr: *mut u8) -> Self {
        // ptr points towards the start of the MADT entry structure which is also the
        // the location of the structure's 8-bit controller type.

        match *ptr {
            LocalApic::CONTROLLER_TYPE => MadtEntry::LocalApic(ptr.cast()),
            IoApic::CONTROLLER_TYPE => MadtEntry::IoApic(ptr.cast()),
            InterruptSourceOverride::CONTROLLER_TYPE => {
                MadtEntry::InterruptSourceOverride(ptr.cast())
            }
            NmiSource::CONTROLLER_TYPE => MadtEntry::NmiSource(ptr.cast()),
            LocalApicNmi::CONTROLLER_TYPE => MadtEntry::LocalApicNmi(ptr.cast()),
            LocalApicAddressOverride::CONTROLLER_TYPE => {
                MadtEntry::LocalApicAddressOverride(ptr.cast())
            }
            IoSapic::CONTROLLER_TYPE => MadtEntry::IoSapic(ptr.cast()),
            LocalSapic::CONTROLLER_TYPE => MadtEntry::LocalSapic(ptr.cast()),
            _ => MadtEntry::Unknown(ptr),
        }
    }
}
