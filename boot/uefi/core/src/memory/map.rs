//**************************************************************************************************
// map.rs                                                                                          *
// Copyright (c) 2019-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

pub use crate::ffi::boot::MemoryType;

use crate::ffi::boot::MemoryDescriptor;
use crate::ffi::Status;
use crate::memory::PAGE_SIZE;
use crate::{system, Error};
use alloc::boxed::Box;
use core::convert::TryFrom;
use memory;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct MemoryMapKey(usize);

impl From<usize> for MemoryMapKey {
    fn from(value: usize) -> Self {
        MemoryMapKey(value)
    }
}

impl From<MemoryMapKey> for usize {
    fn from(key: MemoryMapKey) -> Self {
        key.0
    }
}

#[derive(Copy, Clone)]
pub struct MemoryMapEntry<'a>(&'a MemoryDescriptor);

impl<'a> MemoryMapEntry<'a> {
    pub fn physical_segment(&self) -> memory::Segment {
        memory::Segment::new(
            self.0.physical_start as usize,
            usize::try_from(self.byte_len()).expect("Byte len too large."),
        )
    }

    pub fn virtual_segment(&self) -> memory::Segment {
        memory::Segment::new(
            self.0.virtual_start as usize,
            usize::try_from(self.byte_len()).expect("Byte len too large."),
        )
    }

    fn byte_len(&self) -> u64 {
        self.0.number_of_pages * (PAGE_SIZE as u64)
    }

    pub fn page_len(&self) -> u64 {
        self.0.number_of_pages
    }

    pub fn region_type(&self) -> MemoryType {
        self.0.region_type
    }
}

pub struct MemoryMap {
    buffer: Box<[u8]>,
    key: MemoryMapKey,
    map_size: usize,
    descriptor_size: usize,
}

impl MemoryMap {
    pub fn get() -> Result<MemoryMap, Error> {
        unsafe {
            let system_table = &*system::table()?;

            if system_table.boot_services.is_null() {
                return Err(Error::BootServicesUnavailable);
            }

            let boot_services = &*system_table.boot_services;

            let mut buffer = memory_pool!(0);

            let mut map_size = 0;

            let mut key = 0;
            let mut descriptor_size = 0;
            let mut descriptor_version = 0;

            loop {
                let status = (boot_services.get_memory_map)(
                    &mut map_size,
                    buffer.as_mut_ptr() as *mut MemoryDescriptor,
                    &mut key,
                    &mut descriptor_size,
                    &mut descriptor_version,
                );

                match status {
                    Status::SUCCESS => {
                        return Ok(MemoryMap {
                            buffer,
                            map_size,
                            key: MemoryMapKey(key),
                            descriptor_size,
                        })
                    }
                    Status::BUFFER_TOO_SMALL => buffer = memory_pool!(map_size),
                    _ => return Err(Error::UnexpectedStatus(status)),
                }
            }
        }
    }

    pub fn key(&self) -> MemoryMapKey {
        self.key
    }

    pub fn len(&self) -> usize {
        // The buffer allocated may be too large for the memory map so the true size
        // must be stored. Reallocating for the exact size can cause an infinite loop.
        self.map_size / self.descriptor_size
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.len() == 0
    }

    pub fn entry(&self, index: usize) -> Result<MemoryMapEntry, Error> {
        unsafe {
            if index > self.len() {
                return Err(Error::InvalidArgument("index"));
            }
            Ok(self.entry_unchecked(index))
        }
    }

    pub unsafe fn entry_unchecked(&self, index: usize) -> MemoryMapEntry {
        let ptr = self.buffer.as_ptr().add(index * self.descriptor_size) as *const MemoryDescriptor;
        MemoryMapEntry(&*ptr)
    }

    pub fn iter(&self) -> MemoryMapEntryIterator {
        MemoryMapEntryIterator {
            memory_map: self,
            index: 0,
        }
    }
}

impl<'a> IntoIterator for &'a MemoryMap {
    type Item = MemoryMapEntry<'a>;
    type IntoIter = MemoryMapEntryIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct MemoryMapEntryIterator<'a> {
    memory_map: &'a MemoryMap,
    index: usize,
}

impl<'a> Iterator for MemoryMapEntryIterator<'a> {
    type Item = MemoryMapEntry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.memory_map.len() {
            None
        } else {
            unsafe {
                let value = self.memory_map.entry_unchecked(self.index);
                self.index += 1;
                Some(value)
            }
        }
    }
}
