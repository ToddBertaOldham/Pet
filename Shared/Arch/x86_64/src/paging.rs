// *************************************************************************
// paging.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use core::ops::Index;

#[repr(align(4096))]
pub struct PageTable([PageTableEntry; 512]);

impl PageTable {
    pub fn new() -> Self {
        PageTable([PageTableEntry::from_u64(0); 512])
    }
}

impl Index<usize> for PageTable {
    type Output = PageTableEntry;

    fn index(&self, index : usize) -> &Self::Output{
        self.0.index(index)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct PageTableEntry(u64);

impl PageTableEntry {
    pub fn from_u64(value : u64) -> Self {
        PageTableEntry(value)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn is_present(&self) -> bool {
        self.0 & 1 != 0
    }

    pub fn set_is_present(&mut self, value : bool) {
        if value {
            self.0 |= 1;
        }
        else {
            self.0 &= !1;
        }
    }

    pub fn write_allowed(&self) -> bool {
        1 << 1 & self.0 != 0
    }

    pub fn set_write_allowed(&mut self, value : bool) {
        if value {
            self.0 |= 1 << 1;
        }
        else {
            self.0 &= !(1 << 1);
        }
    }
}