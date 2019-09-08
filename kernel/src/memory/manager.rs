//**************************************************************************************************
// manager.rs                                                                                      *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use x86::stall;
use kernel_init::MemoryInfo;

pub fn init(info: &MemoryInfo) {
    println!("Initializing memory manager...");
    if let Some(memory_map) = info.memory_map() {
        println!("Provided memory map:");
        for (index, entry) in memory_map.iter().enumerate() {
            println!(
                "{}: Start: {:#X} End: {:#X} Type: {:?}",
                index,
                entry.start(),
                entry.end(),
                entry.entry_type()
            );
        }
        println!("Memory manager initialized.");
    } else {
        println!("Memory map unavailable. Kernel cannot be started.");
        unsafe {
            stall();
        }
    }
}
