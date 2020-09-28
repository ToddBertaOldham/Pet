//**************************************************************************************************
// lib.rs                                                                                          *
// Copyright (c) 2019-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#![no_std]

mod debug;
mod memory;

pub use crate::memory::*;
pub use debug::*;

use ::memory::{Address32, Address64};

pub type EntryFunction = unsafe extern "sysv64" fn(args: *const Args);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Args {
    pub version: u32,
    pub rsdt: Address32,
    pub xsdt: Address64,
    pub memory_info: MemoryInfo,
    pub debug_config: DebugConfig,
}

impl Args {
    pub const CURRENT_VERSION: u32 = 1;

    pub fn is_outdated(&self) -> bool {
        self.version != Self::CURRENT_VERSION
    }
}

impl Default for Args {
    fn default() -> Self {
        Args {
            version: Self::CURRENT_VERSION,
            rsdt: Address32::null(),
            xsdt: Address64::null(),
            debug_config: DebugConfig::default(),
            memory_info: MemoryInfo::default(),
        }
    }
}
