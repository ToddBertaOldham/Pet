//**************************************************************************************************
// manager.rs                                                                                      *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::memory::{Frame, FrameAllocator};
use crate::spinlock::Spinlock;
use kernel_init::MemoryInfo;

static ALLOCATOR: Spinlock<Option<FrameAllocator>> = Spinlock::new(None);

pub fn init(info: &MemoryInfo) {
    let allocator = ALLOCATOR.lock();
    assert!(allocator.is_none(), "Physical memory manager has already been initialized.");

    println!("Initializing physical memory manager...");

    let memory_map = info.memory_map().expect("Memory map unavailable.");
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

    *allocator = Some(FrameAllocator::new_unchecked(*info));
    println!("Physical memory manager initialized.");
}

pub fn allocate_frame() -> Frame {
    ALLOCATOR
        .lock()
        .expect("Physical memory manager was not initialized before allocating.")
        .allocate()
}

pub fn free_frame(frame: Frame) {
    ALLOCATOR
        .lock()
        .expect("Physical memory manager was not initialized before freeing.")
        .free(frame);
}
