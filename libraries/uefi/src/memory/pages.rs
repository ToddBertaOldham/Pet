//**************************************************************************************************
// pages.rs                                                                                        *
// Copyright (c) 2019-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::ffi::boot::{AllocateType, MemoryType};
use crate::ffi::PhysicalAddress;
use crate::ffi::Status;
use crate::memory::PAGE_SIZE;
use crate::{system, Error};
use core::slice;

#[derive(Debug)]
pub struct MemoryPages {
    address: PhysicalAddress,
    len: usize,
}

impl MemoryPages {
    pub fn with_len<T: Into<MemoryType>>(
        pages: usize,
        memory_type: T,
    ) -> Result<MemoryPages, Error> {
        unsafe {
            let system_table = &*system::table()?;

            if system_table.boot_services.is_null() {
                return Err(Error::BootServicesUnavailable);
            }

            let boot_services = &*system_table.boot_services;

            let mut address: PhysicalAddress = 0;

            let status = (boot_services.allocate_pages)(
                AllocateType::AnyPages,
                memory_type.into(),
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

    pub fn with_byte_len<T: Into<MemoryType>>(
        bytes: usize,
        memory_type: T,
    ) -> Result<MemoryPages, Error> {
        let pages = (bytes + PAGE_SIZE - 1) / PAGE_SIZE;
        Self::with_len(pages, memory_type)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn byte_len(&self) -> usize {
        self.len * PAGE_SIZE
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.address as *const u8, self.byte_len()) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
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
