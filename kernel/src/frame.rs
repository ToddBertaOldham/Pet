//**************************************************************************************************
// frame.rs                                                                                        *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::arch;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Frame(usize);

impl Frame {
    pub const BYTE_WIDTH: usize = arch::PAGE_SIZE;

    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    pub fn from_address(address: usize) -> Self {
        let index = address / Self::BYTE_WIDTH;
        Self(index)
    }

    pub fn index(self) -> usize {
        self.0
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
