//**************************************************************************************************
// segment.rs                                                                                      *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use core::slice;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Segment {
    start: usize,
    len: usize,
}

impl Segment {
    pub const fn new(start: usize, len: usize) -> Self {
        Self { start, len }
    }

    pub fn from_end(start: usize, end: usize) -> Self {
        let len = end.saturating_sub(start);
        Self { start, len }
    }

    pub const fn start(self) -> usize {
        self.start
    }

    pub const fn len(self) -> usize {
        self.len
    }

    pub const fn is_null(self) -> bool {
        self.start == 0
    }

    pub const fn is_zero_sized(self) -> bool {
        self.len == 0
    }

    pub const fn end(self) -> usize {
        self.start + self.len
    }

    pub fn intersects(self, segment: Segment) -> bool {
        self.start < segment.end() && self.end() > segment.start
    }

    pub fn contains(self, segment: Segment) -> bool {
        self.start < segment.start && self.end() > segment.end()
    }

    pub fn contains_address(self, address: usize) -> bool {
        address >= self.start && address < self.end()
    }

    pub fn as_ptr(self) -> *const u8 {
        self.start as *const u8
    }

    pub fn as_mut_ptr(self) -> *mut u8 {
        self.start as *mut u8
    }

    pub unsafe fn as_slice(self) -> &'static [u8] {
        slice::from_raw_parts(self.as_ptr(), self.len())
    }

    pub unsafe fn as_mut_slice(self) -> &'static mut [u8] {
        slice::from_raw_parts_mut(self.as_mut_ptr(), self.len())
    }
}
