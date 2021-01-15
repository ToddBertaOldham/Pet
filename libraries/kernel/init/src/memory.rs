//**************************************************************************************************
// memory_map.rs                                                                                   *
// Copyright (c) 2019-2021 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use core::mem;
use core::ptr;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MemoryInfo {
    pub memory_map: *mut MemoryMapEntry,
    pub memory_map_count: usize,
    pub kernel_physical_start: usize,
    pub kernel_length: usize,
}

impl MemoryInfo {
    pub fn kernel_physical_segment(&self) -> memory::Segment {
        memory::Segment::with_len(self.kernel_physical_start, self.kernel_length)
    }

    pub fn set_kernel_physical_segment(&mut self, value: memory::Segment) {
        self.kernel_physical_start = value.start();
        self.kernel_length = value.len();
    }

    pub fn memory_map_segment(&self) -> memory::Segment {
        memory::Segment::with_len(
            self.memory_map as usize,
            self.memory_map_count * mem::size_of::<MemoryMapEntry>(),
        )
    }
}

unsafe impl Send for MemoryInfo {}

impl Default for MemoryInfo {
    fn default() -> Self {
        MemoryInfo {
            memory_map: ptr::null_mut(),
            memory_map_count: 0,
            kernel_physical_start: 0,
            kernel_length: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct MemoryMapEntry {
    pub start: usize,
    pub end: usize,
    pub entry_type: MemoryMapEntryType,
}

impl MemoryMapEntry {
    pub fn new(segment: memory::Segment, entry_type: MemoryMapEntryType) -> Self {
        Self {
            start: segment.start(),
            end: segment.end(),
            entry_type,
        }
    }

    pub fn segment(&self) -> memory::Segment {
        memory::Segment::with_len(self.start, self.end)
    }

    pub fn set_segment(&mut self, value: memory::Segment) {
        self.start = value.start();
        self.end = value.end();
    }

    pub fn entry_type(&self) -> MemoryMapEntryType {
        self.entry_type
    }

    pub fn set_entry_type(&mut self, value: MemoryMapEntryType) {
        self.entry_type = value;
    }
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
