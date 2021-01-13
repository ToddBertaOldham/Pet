//**************************************************************************************************
// physical                                                                                        *
// Copyright (c) 2019-2021 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::arch;
use crate::spinlock::Spinlock;
use alloc::vec::Vec;
use core::fmt;
use kernel_init;

static ALLOCATOR: Spinlock<Option<Allocator>> = Spinlock::new(None);

pub unsafe fn init(info: &kernel_init::MemoryInfo) {
    assert!(!info.memory_map.is_null(), "Memory map is null.");

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

    *allocator = Some(Allocator::new_unchecked(*info));
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

#[derive(Debug)]
pub struct Allocator {
    memory_info: kernel_init::MemoryInfo,
    next: usize,
    free: Vec<Frame>,
}

impl Allocator {
    pub fn new(memory_info: kernel_init::MemoryInfo) -> Result<Self, InvalidMemoryMapError> {
        if memory_info.memory_map.is_null() || memory_info.memory_map_count == 0 {
            return Err(InvalidMemoryMapError);
        }

        Ok(Self::new_unchecked(memory_info))
    }

    pub fn new_unchecked(memory_info: kernel_init::MemoryInfo) -> Self {
        Self {
            memory_info,
            next: 0,
            free: Vec::new(),
        }
    }

    pub unsafe fn allocate(&mut self) -> Frame {
        if let Some(freed_frame) = self.free.pop() {
            freed_frame
        } else {
            loop {
                let frame = Frame::new(self.next);
                if self.is_free(frame) {
                    return frame;
                }
                self.next += 1;
            }
        }
    }

    unsafe fn is_free(&self, frame: Frame) -> bool {
        if self
            .memory_info
            .kernel_physical_segment()
            .intersects(frame.segment())
        {
            return false;
        }

        if self
            .memory_info
            .memory_map_segment()
            .intersects(frame.segment())
        {
            return false;
        }

        self.check_memory_map(frame)
    }

    unsafe fn check_memory_map(&self, frame: Frame) -> bool {
        for index in 0..self.memory_info.memory_map_count {
            let entry = &*self.memory_info.memory_map.add(index);
            if entry.segment().intersects(frame.segment()) {
                return entry.entry_type().is_usable();
            }
        }

        panic!("Out of physical memory.");
    }

    pub unsafe fn free(&mut self, frame: Frame) {
        self.free.push(frame);
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Frame(usize);

impl Frame {
    pub const BYTE_WIDTH: usize = arch::PAGE_SIZE;

    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    pub fn segment(self) -> memory::Segment {
        memory::Segment::with_len(Self::BYTE_WIDTH * self.0, Self::BYTE_WIDTH)
    }
}

impl From<usize> for Frame {
    fn from(value: usize) -> Self {
        Frame(value)
    }
}

impl From<Frame> for usize {
    fn from(value: Frame) -> Self {
        value.0
    }
}

pub struct InvalidMemoryMapError;

impl fmt::Display for InvalidMemoryMapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str("Memory map is null or memory map count is 0.")
    }
}
