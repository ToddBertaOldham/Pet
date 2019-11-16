//**************************************************************************************************
// lib.rs                                                                                          *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#![no_std]

mod debug;
mod memory;

pub use crate::memory::*;
pub use debug::*;

pub type KernelEntryFunction = unsafe extern "sysv64" fn(args: *const KernelArgs);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct KernelArgs {
    pub version: u32,
    pub memory_info: MemoryInfo,
    pub debug_config: DebugConfig,
}

impl KernelArgs {
    pub const CURRENT_VERSION: u32 = 1;

    pub fn is_outdated(&self) -> bool {
        self.version != Self::CURRENT_VERSION
    }
}

impl Default for KernelArgs {
    fn default() -> Self {
        KernelArgs {
            version: Self::CURRENT_VERSION,
            debug_config: DebugConfig::default(),
            memory_info: MemoryInfo::default(),
        }
    }
}
