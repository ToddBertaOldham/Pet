//**************************************************************************************************
// memory_map.rs                                                                                   *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use core::marker::PhantomData;
use core::ptr;
use encapsulation::GetterSetters;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, GetterSetters)]
pub struct MemorySegment {
    #[field_access(set = true)]
    start: usize,
    #[field_access(set = true)]
    length: usize,
}

impl MemorySegment {
    pub fn new(start: usize, length: usize) -> Self {
        Self { start, length }
    }

    pub fn end(&self) -> usize {
        self.start + self.length
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, GetterSetters)]
pub struct MemoryInfo<'a> {
    memory_map: *mut MemoryMapEntry,
    memory_map_count: usize,
    #[field_access(set = true)]
    kernel_segment: MemorySegment,
    phantom: PhantomData<&'a MemoryMapEntry>,
}

impl<'a> MemoryInfo<'a> {
    pub fn memory_map(&self) -> Option<&'a mut [MemoryMapEntry]> {
        if self.memory_map.is_null() || self.memory_map_count == 0 {
            None
        } else {
            unsafe {
                Some(core::slice::from_raw_parts_mut(
                    self.memory_map,
                    self.memory_map_count,
                ))
            }
        }
    }

    pub fn set_memory_map(&mut self, memory_map: Option<&'a mut [MemoryMapEntry]>) {
        if let Some(value) = memory_map {
            self.memory_map = value.as_mut_ptr();
            self.memory_map_count = value.len();
        } else {
            self.memory_map = ptr::null_mut();
            self.memory_map_count = 0;
        }
    }
}

impl<'a> Default for MemoryInfo<'a> {
    fn default() -> Self {
        MemoryInfo {
            memory_map: ptr::null_mut(),
            memory_map_count: 0,
            kernel_segment: MemorySegment::default(),
            phantom: PhantomData,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug, GetterSetters)]
pub struct MemoryMapEntry {
    #[field_access(set = true)]
    start: usize,
    #[field_access(set = true)]
    end: usize,
    #[field_access(set = true)]
    entry_type: MemoryMapEntryType,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum MemoryMapEntryType {
    Conventional = 0,
    Persistent = 1,
    AcpiReclaim = 127,
    Unusable = 128,
    Firmware = 129,
    MemoryMappedIo = 130,
    AcpiNvs = 131,
    ReservedOther = 255,
}

impl MemoryMapEntryType {
    pub fn is_usable(self) -> bool {
        let value = self as u8;
        value >> 7 == 0
    }
}

impl Default for MemoryMapEntryType {
    fn default() -> Self {
        MemoryMapEntryType::ReservedOther
    }
}
