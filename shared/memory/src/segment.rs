//**************************************************************************************************
// segment.rs                                                                                      *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Segment {
    start: usize,
    len: usize,
}

impl Segment {
    pub const fn new(start: usize, len: usize) -> Self {
        Self { start, len }
    }

    pub const fn from_end(start: usize, end: usize) {
        let len = start - end;
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
}
