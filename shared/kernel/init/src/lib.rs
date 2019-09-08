//**************************************************************************************************
// lib.rs                                                                                          *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#![no_std]

mod debug;
mod memory;

pub use debug::*;
use encapsulation::GetterSetters;
pub use memory::*;

pub type KernelMainFunction = unsafe extern "sysv64" fn(args: &KernelArgs) -> !;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, GetterSetters, Debug)]
pub struct KernelArgs<'a> {
    #[field_access(borrow_self = false)]
    version: u32,
    #[field_access(set = true, borrow_self = false)]
    memory_info: MemoryInfo<'a>,
    #[field_access(set = true, borrow_self = false)]
    debug_config: DebugConfig,
}

impl<'a> KernelArgs<'a> {
    pub const CURRENT_VERSION: u32 = 1;

    pub fn is_outdated(&self) -> bool {
        self.version != Self::CURRENT_VERSION
    }
}

impl<'a> Default for KernelArgs<'a> {
    fn default() -> Self {
        KernelArgs {
            version: Self::CURRENT_VERSION,
            debug_config: DebugConfig::default(),
            memory_info: MemoryInfo::default(),
        }
    }
}
