//**************************************************************************************************
// frame.rs                                                                                        *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use alloc::vec::Vec;
use core::fmt;
use kernel_init::MemoryInfo;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Frame(usize);

impl Frame {
    pub const BYTE_WIDTH: usize = 4096;

    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    pub fn segment(&self) -> memory::Segment {
        memory::Segment::new(Self::BYTE_WIDTH * self.0, Self::BYTE_WIDTH)
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

#[derive(Debug)]
pub struct FrameAllocator {
    memory_info: MemoryInfo,
    next: usize,
    free: Vec<Frame>,
}

impl FrameAllocator {
    pub fn new(memory_info: MemoryInfo) -> Result<Self, InvalidMemoryMapError> {
        if memory_info.memory_map.is_null() || memory_info.memory_map_count == 0 {
            return Err(InvalidMemoryMapError);
        }

        Ok(Self::new_unchecked(memory_info))
    }

    pub fn new_unchecked(memory_info: MemoryInfo) -> Self {
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
            .kernel_segment()
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
                if entry.entry_type().is_usable() {
                    return true;
                } else {
                    return false;
                }
            }
        }

        panic!("Out of physical memory.");
    }

    pub unsafe fn free(&mut self, frame: Frame) {
        self.free.push(frame);
    }
}
