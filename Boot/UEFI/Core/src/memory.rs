// *************************************************************************
// memory.rs
// Copyright 2018 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use core::ptr::null_mut;
use core::ffi::c_void;
use super::ffi::*;
use super::error::UEFIError;
use super::system as uefi_system;

pub struct MemoryMap {
    map : *mut MemoryDescriptor,
    key : usize,
    count : usize
}

impl MemoryMap {
    pub fn new() -> Result<MemoryMap, UEFIError> {
        unsafe {
            let system_table = &*uefi_system::system_table()?;

            let mut buffer : *mut c_void = null_mut();
            let mut map_size : usize = 0;

            let mut key : usize = 0;
            let mut descriptor_size : usize = 0;
            let mut descriptor_version : u32 = 0;

            // First get size of memory map. This call should compain about the buffer being too small.

            ((*(*system_table).boot_services).get_memory_map)(&mut map_size as *mut usize, null_mut(), &mut key as *mut usize, &mut descriptor_size as *mut usize, &mut descriptor_version as *mut u32);

            // Second create the buffer and retrieve the memory map.

            ((*(*system_table).boot_services).allocate_pool)(MemoryType::LoaderData, map_size, &mut buffer as *mut *mut c_void); 

            let map : *mut MemoryDescriptor = buffer as *mut MemoryDescriptor;

            ((*(*system_table).boot_services).get_memory_map)(&mut map_size as *mut usize, map, &mut key as *mut usize, &mut descriptor_size as *mut usize, &mut descriptor_version as *mut u32);

            Ok(MemoryMap { map : map, key : key, count : map_size / descriptor_size })
        }
    }

    pub fn key(&self) -> usize {
        self.key
    }
}

impl Drop for MemoryMap {
    fn drop(&mut self) {
        unsafe {
            let system_table = &*uefi_system::system_table().unwrap();
            let boot_services = &*system_table.boot_services;

            if system_table.boot_services == null_mut() { 
                return; 
            }
            
            (boot_services.free_pool)(self.map as *mut c_void);
        }
    }
}