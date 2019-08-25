//**************************************************************************************************
// lib.rs                                                                                          *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#![no_std]

mod debug;
pub mod memory_map;

pub use debug::*;
use core::ptr;
use encapsulation::GetterSetters;

pub type KernelMainFunction = unsafe extern "C" fn(args : KernelArgs);

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, GetterSetters)]
pub struct KernelArgs {
    #[field_access(set = true, borrow_self = false)]
    version: u32,
    memory_map: *mut memory_map::Entry,
    memory_map_count: usize,
    #[field_access(set = true, borrow_self = false)]
    debug_config: DebugConfig,
}

impl KernelArgs {
    pub const CURRENT_VERSION: u32 = 1;

    pub fn memory_map(&self) -> Option<&mut [memory_map::Entry]> {
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

    pub fn set_memory_map(&mut self, memory_map: Option<&mut [memory_map::Entry]>) {
        if let Some(value) = memory_map {
            self.memory_map = value.as_mut_ptr();
            self.memory_map_count = value.len();
        } else {
            self.memory_map = ptr::null_mut();
            self.memory_map_count = 0;
        }
    }
}

impl Default for KernelArgs {
    fn default() -> Self {
        KernelArgs {
            version: Self::CURRENT_VERSION,
            memory_map: ptr::null_mut(),
            memory_map_count: 0,
            debug_config: DebugConfig::default(),
        }
    }
}