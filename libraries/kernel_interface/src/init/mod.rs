//**************************************************************************************************
// mod.rs                                                                                          *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

mod debug;
mod memory_map;
mod system;

pub use debug::*;
pub use memory_map::*;
pub use system::*;

pub type EntryFunction = unsafe extern "sysv64" fn(args: *const Args);

pub const KERNEL_VIRTUAL_START: u64 = 0xffffffff80000000;

pub const STACK_PAGES: u64 = 5;

pub const STACK_SIZE: u64 = STACK_PAGES * 4096;

pub const BP_STACK_VIRTUAL_TOP: u64 = 0xffffffff80000000;

pub const BP_STACK_VIRTUAL_BOTTOM: u64 = BP_STACK_VIRTUAL_TOP - STACK_SIZE;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Args {
    pub version: u32,
    pub system_info: SystemInfo,
    pub memory_map: MemoryMap,
    pub debug_config: DebugConfig,
}

impl Args {
    pub const CURRENT_VERSION: u32 = 1;

    pub const fn new() -> Self {
        Args {
            version: Self::CURRENT_VERSION,
            system_info: SystemInfo::new(),
            memory_map: MemoryMap::new(),
            debug_config: DebugConfig::new(),
        }
    }

    pub fn is_outdated(&self) -> bool {
        self.version != Self::CURRENT_VERSION
    }
}

impl Default for Args {
    fn default() -> Self {
        Self::new()
    }
}
