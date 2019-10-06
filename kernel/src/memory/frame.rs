//**************************************************************************************************
// frame.rs                                                                                        *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use alloc::vec::Vec;
use core::slice;
use kernel_init::MemoryInfo;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Frame(usize);

impl Frame {
    pub const BYTE_WIDTH: usize = 4096;

    pub const fn byte_start(&self) -> usize {
        Self::BYTE_WIDTH * self.0
    }

    pub const fn byte_end(&self) -> usize {
        Self::BYTE_WIDTH * (self.0 + 1)
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.byte_start() as *const u8
    }

    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.byte_start() as *mut u8
    }

    pub unsafe fn as_slice(&self) -> &[u8] {
        slice::from_raw_parts(self.as_ptr(), Self::BYTE_WIDTH)
    }

    pub unsafe fn as_mut_slice(&mut self) -> &mut [u8] {
        slice::from_raw_parts_mut(self.as_mut_ptr(), Self::BYTE_WIDTH)
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

#[derive(Debug)]
pub struct FrameAllocator {
    memory_info: MemoryInfo<'static>,
    next: usize,
    free: Vec<Frame>,
}

impl FrameAllocator {
    pub fn new(memory_info: MemoryInfo<'static>) -> Result<Self, ()> {
        if memory_info.memory_map().is_none() {
            return Err(());
        }

        Ok(Self::new_unchecked(memory_info))
    }

    pub fn new_unchecked(memory_info: MemoryInfo<'static>) -> Self {
        Self {
            memory_info,
            next: 0,
            free: Vec::new(),
        }
    }

    pub fn allocate(&mut self) -> Frame {
        if let Some(freed_frame) = self.free.pop() {
            freed_frame
        } else {
            loop {
                let frame = Frame::from(self.next);
                if self.is_free(frame) {
                    return frame;
                }
                self.next += 1;
            }
        }
    }

    fn is_free(&self, frame: Frame) -> bool {
        if self.memory_info.kernel_segment().start() <= frame.byte_start()
            && self.memory_info.kernel_segment().end() >= frame.byte_end()
        {
            return false;
        }

        self.check_memory_map(frame)
    }

    fn check_memory_map(&self, frame: Frame) -> bool {
        for entry in self.memory_info.memory_map().unwrap() {
            if entry.start() <= frame.byte_start() && entry.end() >= frame.byte_end() {
                if entry.entry_type().is_usable() {
                    return true;
                } else {
                    return false;
                }
            }
        }

        panic!("Out of physical memory.");
    }

    pub fn free(&mut self, frame: Frame) {
        self.free.push(frame);
    }
}
