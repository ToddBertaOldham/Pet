//**************************************************************************************************
// manager.rs                                                                                      *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::memory::{Frame, FrameAllocator};
use crate::spinlock::Spinlock;
use kernel_init::MemoryInfo;

static ALLOCATOR: Spinlock<Option<FrameAllocator>> = Spinlock::new(None);

pub unsafe fn init(info: &MemoryInfo) {
    assert!(info.memory_map.is_null(), "Memory map is null.");

    let mut allocator = ALLOCATOR.lock();
    assert!(
        allocator.is_none(),
        "Physical memory manager has already been initialized."
    );

    println!("Initializing physical memory manager...");

    println!("Provided memory map:");
    for index in 0..info.memory_map_count {
        let entry = &*info.memory_map.add(index);
        let segment = entry.segment();
        println!(
            "{}: Start: {:#X} End: {:#X} Type: {:?}",
            index,
            segment.start(),
            segment.end(),
            entry.entry_type()
        );
    }

    *allocator = Some(FrameAllocator::new_unchecked(*info));
    println!("Physical memory manager initialized.");
}

pub unsafe fn allocate_frame() -> Frame {
    ALLOCATOR
        .lock()
        .as_mut()
        .expect("Physical memory manager was not initialized before allocating.")
        .allocate()
}

pub unsafe fn free_frame(frame: Frame) {
    ALLOCATOR
        .lock()
        .as_mut()
        .expect("Physical memory manager was not initialized before freeing.")
        .free(frame);
}
