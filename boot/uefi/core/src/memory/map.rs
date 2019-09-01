//**************************************************************************************************
// map.rs                                                                                          *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::ffi::boot::{MemoryDescriptor, MemoryType};
use crate::ffi::Status;
use crate::memory::PAGE_SIZE;
use crate::{system, Error};
use alloc::boxed::Box;
use core::ops::Range;

#[derive(Copy, Clone)]
pub struct MemoryMapEntry(*const MemoryDescriptor);

impl MemoryMapEntry {
    pub fn physical_range(&self) -> Range<u64> {
        unsafe {
            let descriptor = &*self.0;
            (descriptor.physical_start..(descriptor.physical_start + self.byte_len()))
        }
    }
    pub fn virtual_range(&self) -> Range<u64> {
        unsafe {
            let descriptor = &*self.0;
            (descriptor.virtual_start..(descriptor.virtual_start + self.byte_len()))
        }
    }
    pub fn byte_len(&self) -> u64 {
        unsafe {
            let descriptor = &*self.0;
            descriptor.number_of_pages * (PAGE_SIZE as u64)
        }
    }
    pub fn len(&self) -> u64 {
        unsafe {
            let descriptor = &*self.0;
            descriptor.number_of_pages
        }
    }
    pub fn region_type(&self) -> MemoryType {
        unsafe {
            let descriptor = &*self.0;
            descriptor.region_type
        }
    }
}

pub struct MemoryMap {
    buffer: Box<[u8]>,
    key: usize,
    len: usize,
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
                            key,
                            len: map_size / descriptor_size,
                        })
                    }
                    Status::BUFFER_TOO_SMALL => buffer = memory_pool!(map_size),
                    _ => return Err(Error::UnexpectedStatus(status)),
                }
            }
        }
    }

    pub fn key(&self) -> usize {
        self.key
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn entry(&self, index: usize) -> Result<MemoryMapEntry, Error> {
        unsafe {
            if index > self.len {
                return Err(Error::InvalidArgument("index"));
            }
            let ptr = (self.buffer.as_ptr() as *const MemoryDescriptor).add(index);
            Ok(MemoryMapEntry(ptr))
        }
    }
}
