//**************************************************************************************************
// lib.rs                                                                                          *
// Copyright (c) 2019-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#![no_std]

mod configuration;
mod debug;
mod memory;

pub use crate::memory::*;
pub use configuration::*;
pub use debug::*;

use ::memory::{Address32, Address64};

pub type EntryFunction = unsafe extern "sysv64" fn(args: *const Args);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Args {
    pub version: u32,
    pub configuration: Configuration,
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
            debug_config: DebugConfig::default(),
            memory_info: MemoryInfo::default(),
            configuration: Configuration::default(),
        }
    }
}
