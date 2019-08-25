//**************************************************************************************************
// memory.rs                                                                                       *
// Copyright (c) 2018-2019 Todd Berta-Oldham                                                       *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use super::error::Error;
pub use super::ffi::boot::MemoryType;
use super::ffi::boot::{AllocateType, MemoryDescriptor};
use super::ffi::{PhysicalAddress, Status};
use super::system;
use alloc::boxed::Box;
use core::ops::{Index, Range};
use core::slice;

#[macro_export]
macro_rules! memory_pool {
    ($size:expr) => {{
        let mut vector = alloc::vec::Vec::<u8>::with_capacity($size);
        vector.resize($size, 0);
        vector.into_boxed_slice()
    }};
}

pub struct MemoryPages {
    address: PhysicalAddress,
    len: usize,
}

impl MemoryPages {
    pub const PAGE_SIZE: usize = 4096;

    pub fn allocate(pages: usize) -> Result<MemoryPages, Error> {
        unsafe {
            let system_table = &*system::table()?;

            if system_table.boot_services.is_null() {
                return Err(Error::BootServicesUnavailable);
            }

            let boot_services = &*system_table.boot_services;

            let mut address: PhysicalAddress = 0;

            let status = (boot_services.allocate_pages)(
                AllocateType::AnyPages,
                MemoryType::LOADER_DATA,
                pages,
                &mut address,
            );

            match status {
                Status::SUCCESS => Ok(MemoryPages {
                    address,
                    len: pages,
                }),
                Status::OUT_OF_RESOURCES => Err(Error::OutOfMemory),
                _ => Err(Error::UnexpectedStatus(status)),
            }
        }
    }

    pub fn allocate_for(bytes: usize) -> Result<MemoryPages, Error> {
        let pages = (bytes + Self::PAGE_SIZE - 1) / Self::PAGE_SIZE;
        Self::allocate(pages)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn byte_len(&self) -> usize {
        self.len * Self::PAGE_SIZE
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.address as *const u8, self.byte_len()) }
    }

    pub fn as_mut_slice(&self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.address as *mut u8, self.byte_len()) }
    }
}

impl Drop for MemoryPages {
    fn drop(&mut self) {
        unsafe {
            let system_table = &*system::table().unwrap();

            if system_table.boot_services.is_null() {
                return;
            }

            let boot_services = &*system_table.boot_services;

            (boot_services.free_pages)(self.address, self.len);
        }
    }
}

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
            descriptor.number_of_pages * (MemoryPages::PAGE_SIZE as u64)
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
