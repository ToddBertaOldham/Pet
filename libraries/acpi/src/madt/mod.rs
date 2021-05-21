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
use core::mem;
use memory::flags;

pub use interrupt_source::*;
pub use io_apic::*;
pub use local_apic::*;
pub use mps::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Madt {
    header: DescriptionHeader,
    lic_address: u32,
    flags: MadtFlags,
}

impl Madt {
    pub unsafe fn iter_interrupt_controllers(&mut self) -> MadtEntryIter {
        let base_size = mem::size_of::<Self>();
        let base_ptr = self as *mut Madt as *mut u8;

        let structures_ptr = base_ptr.add(base_size);
        let structures_size = self.header.length - base_size as u32;

        MadtEntryIter::new(structures_ptr, structures_size)
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

pub struct MadtEntryIter {
    start_ptr: *mut u8,
    length: u32,
}

impl MadtEntryIter {
    pub unsafe fn new(start_ptr: *mut u8, length: u32) -> Self {
        Self { start_ptr, length }
    }
}

impl Iterator for MadtEntryIter {
    type Item = MadtEntry;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.length == 0 {
                None
            } else {
                let item = self.start_ptr;

                let controller_type = *item;
                let length = *item.add(1);

                self.start_ptr = item.add(length as usize);
                self.length = self.length.saturating_sub(length as u32);

                Some(match controller_type {
                    LocalApic::CONTROLLER_TYPE => MadtEntry::LocalApic(item.cast()),
                    IoApic::CONTROLLER_TYPE => MadtEntry::IoApic(item.cast()),
                    InterruptSourceOverride::CONTROLLER_TYPE => {
                        MadtEntry::InterruptSourceOverride(item.cast())
                    }
                    NmiSource::CONTROLLER_TYPE => MadtEntry::NmiSource(item.cast()),
                    LocalApicNmi::CONTROLLER_TYPE => MadtEntry::LocalApicNmi(item.cast()),
                    LocalApicAddress::CONTROLLER_TYPE => MadtEntry::LocalApicAddress(item.cast()),
                    IoSapic::CONTROLLER_TYPE => MadtEntry::IoSapic(item.cast()),
                    LocalSapic::CONTROLLER_TYPE => MadtEntry::LocalSapic(item.cast()),
                    _ => MadtEntry::Unknown(item),
                })
            }
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
    LocalApicAddress(*mut LocalApicAddress),
    LocalSapic(*mut LocalSapic),
    Unknown(*mut u8),
}
