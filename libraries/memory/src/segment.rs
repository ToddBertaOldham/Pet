//**************************************************************************************************
// segment.rs                                                                                      *
// Copyright (c) 2019-2021 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use core::slice;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Segment {
    start: usize,
    len: usize,
}

impl Segment {
    pub const fn with_len(start: usize, len: usize) -> Self {
        Self { start, len }
    }

    pub fn with_end(start: usize, end: usize) -> Self {
        let len = end.saturating_sub(start);
        Self { start, len }
    }

    pub fn from_ref<T>(value: &T) -> Self {
        unsafe {
            let beginning_ptr = value as *const T;
            let end_ptr = beginning_ptr.add(1);
            Self::with_end(beginning_ptr as usize, end_ptr as usize)
        }
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

    pub fn as_ptr<T>(self) -> *const T {
        self.start as *const T
    }

    pub fn as_mut_ptr<T>(self) -> *mut T {
        self.start as *mut T
    }

    pub fn as_end_ptr<T>(self) -> *const T {
        self.end() as *const T
    }

    pub fn as_mut_end_ptr<T>(self) -> *mut T {
        self.end() as *mut T
    }

    pub unsafe fn as_slice<T>(self) -> &'static [T] {
        slice::from_raw_parts(self.as_ptr(), self.len())
    }

    pub unsafe fn as_mut_slice<T>(self) -> &'static mut [T] {
        slice::from_raw_parts_mut(self.as_mut_ptr(), self.len())
    }
}
