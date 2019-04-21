// *************************************************************************
// memory.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use core::ptr::null_mut;
use core::slice;
use alloc::boxed::Box;
use super::ffi::*;
use super::error::UefiError;
use super::system as uefi_system;

#[macro_export]
macro_rules! memory_pool {
    ($size:expr) => {
        {
            let mut vector = alloc::vec::Vec::<u8>::with_capacity($size);
            for _ in 0..$size {
                vector.push(0);
            }
            vector.into_boxed_slice()
        }
    };
}

pub struct MemoryPages {
    address : PhysicalAddress,
    len : usize
}

impl MemoryPages {
    pub const PAGE_SIZE : usize = 4096;

    pub fn allocate(pages : usize) -> Result<MemoryPages, UefiError> {
        unsafe {
            let system_table = &*uefi_system::system_table()?;

            if system_table.boot_services.is_null() {
                return Err(UefiError::BootServicesUnavailable);
            }

            let boot_services = &*system_table.boot_services;

            let mut address : PhysicalAddress = 0;

            let status = (boot_services.allocate_pages)(AllocateType::AnyPages, MemoryType::LoaderData, pages, &mut address);
            match status {
                Status::SUCCESS => Ok(MemoryPages { address, len : pages }),
                Status::OUT_OF_RESOURCES => Err(UefiError::OutOfMemory),
                _ => Err(UefiError::UnexpectedStatus(status))
            }
        }
    }

    pub fn allocate_for(bytes : usize) -> Result<MemoryPages, UefiError> {
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
        unsafe {
            slice::from_raw_parts(self.address as *const u8, self.byte_len())
        }
    }

    pub fn as_mut_slice(&self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self.address as *mut u8, self.byte_len())
        }
    }
}

impl Drop for MemoryPages {
    fn drop(&mut self) {
        unsafe {
            let system_table = &*uefi_system::system_table().unwrap();

            if system_table.boot_services.is_null() {
                return;
            }

            let boot_services = &*system_table.boot_services;

            (boot_services.free_pages)(self.address, self.len);
        }
    }
}

pub struct MemoryMap {
    buffer : Box<[u8]>,
    key : usize,
    len : usize
}

impl MemoryMap {
    pub fn new() -> Result<MemoryMap, UefiError> {
        unsafe {
            let system_table = &*uefi_system::system_table()?;

            if system_table.boot_services.is_null() { 
                return Err(UefiError::BootServicesUnavailable);
            }
            
            let boot_services = &*system_table.boot_services;

            let mut buffer = memory_pool!(0);

            let mut map_size = 0;

            let mut key = 0;
            let mut descriptor_size = 0;
            let mut descriptor_version = 0;

            loop {
                let status = (boot_services.get_memory_map)(&mut map_size, buffer.as_mut_ptr() as *mut MemoryDescriptor, &mut key, &mut descriptor_size, &mut descriptor_version);
                match status {
                    Status::SUCCESS => return Ok(MemoryMap { buffer, key, len : map_size / descriptor_size }),
                    Status::BUFFER_TOO_SMALL => { buffer = memory_pool!(map_size) },
                    _ => return Err(UefiError::UnexpectedStatus(status))
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
}