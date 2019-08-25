//**************************************************************************************************
// memory_map.rs                                                                                   *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use encapsulation::GetterSetters;

#[repr(C)]
#[derive(Copy, Clone, Default, Debug, GetterSetters)]
pub struct Entry {
    #[field_access(set = true)]
    start: usize,
    #[field_access(set = true)]
    end: usize,
    #[field_access(set = true)]
    entry_type: EntryType,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum EntryType {
    Conventional = 0,
    Persistent = 1,
    AcpiReclaim = 127,
    Unusable = 128,
    Firmware = 129,
    MemoryMappedIo = 130,
    AcpiNvs = 131,
    ReservedOther = 255,
}

impl EntryType {
    pub fn is_usable(self) -> bool {
        let value = self as u8;
        value >> 7 == 0
    }
}

impl Default for EntryType {
    fn default() -> Self {
        EntryType::ReservedOther
    }
}
