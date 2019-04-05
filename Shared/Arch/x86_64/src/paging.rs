// *************************************************************************
// paging.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use core::ops::{ Index, IndexMut };
use bit_operations::BitField;

#[repr(align(4096))]
pub struct PageTable([PageTableEntry; 512]);

impl PageTable {
    pub fn new() -> Self {
        PageTable([PageTableEntry::from(0); 512])
    }
}

impl Index<u16> for PageTable {
    type Output = PageTableEntry;

    fn index(&self, index : u16) -> &Self::Output{
        self.0.index(index as usize)
    }
}

impl IndexMut<u16> for PageTable {
    fn index_mut(&mut self, index : u16) -> &mut Self::Output{
        self.0.index_mut(index as usize)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct PageTableEntry(u64);

impl PageTableEntry {
    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn is_present(&self) -> bool {
        self.0.is_bit_set(0)
    }

    pub fn set_is_present(&mut self, value : bool) {
        self.0.set_bit(0, value);
    }

    pub fn write_allowed(&self) -> bool {
        self.0.is_bit_set(1)
    }

    pub fn set_write_allowed(&mut self, value : bool) {
        self.0.set_bit(1, value);
    }

    pub fn user_access_allowed(&self) -> bool {
        self.0.is_bit_set(2)
    }

    pub fn set_user_acess_allowed(&mut self, value : bool) {
        self.0.set_bit(2, value)
    }

    pub fn write_through_enabled(&self) -> bool {
        self.0.is_bit_set(3)
    }

    pub fn set_write_through_enabled(&mut self, value : bool) {
        self.0.set_bit(3, value);
    }

    pub fn cache_disabled(&self) -> bool {
        self.0.is_bit_set(4)
    }

    pub fn set_cache_disabled(&mut self, value : bool) {
        self.0.set_bit(4, value);
    }

    pub fn accessed(&self) -> bool {
        self.0.is_bit_set(5)
    }

    pub fn set_accessed(&mut self, value : bool) {
        self.0.set_bit(5, value);
    }

    pub fn is_dirty(&self) -> bool {
        self.0.is_bit_set(6)
    }

    pub fn set_is_dirty(&mut self, value : bool) {
        self.0.set_bit(6, value);
    }

    pub fn references_page(&self) -> bool {
        self.0.is_bit_set(7)
    }

    pub fn set_references_page(&mut self, value : bool) {
        self.0.set_bit(7, value);
    }

    pub fn is_global(&self) -> bool {
        self.0.is_bit_set(8)
    }

    pub fn set_is_global(&mut self, value : bool) {
        self.0.set_bit(8, value);
    }
}

impl From<u64> for PageTableEntry {
    fn from(value : u64) -> PageTableEntry {
        PageTableEntry(value)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct CR3Value(u64);

impl CR3Value {
    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn write_through_enabled(&self) -> bool {
        self.0.is_bit_set(3)
    }

    pub fn set_write_through_enabled(&mut self, value : bool) {
        self.0.set_bit(3, value);
    }

    pub fn cache_disabled(&self) -> bool {
        self.0.is_bit_set(4)
    }

    pub fn set_cache_disabled(&mut self, value : bool) {
        self.0.set_bit(4, value);
    }

    pub fn physical_address(&self) -> u64 {
        (self.0 & 0xFFFFFFFFFF800) >> 12
    }

    pub fn set_physical_address(&mut self, address : u64) {
        self.0 &= 0xFFF00000000007FF | (address << 12);
    }
}

impl From<u64> for CR3Value {
    fn from(value : u64) -> CR3Value {
        CR3Value(value)
    }
}

pub mod cr3 {
    use super::CR3Value;

    pub fn read() -> CR3Value {
        let value : CR3Value;
        unsafe {
            asm!("mov %cr3, $0" : "=r"(value));
        }
        value
    }

    pub unsafe fn write(value : CR3Value) {
        asm!("mov $0, %cr3" :: "r"(value))
    }
}

pub struct VirtualAddress(u64);

impl VirtualAddress {
    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn offset(&self) -> u16 {
        (self.0 & 0xFFF) as u16
    }

    pub fn index_1(&self) -> u16 {
        (self.0 >> 12 & 0xFFF) as u16
    }

    pub fn index_2(&self) -> u16 {
        (self.0 >> 21 & 0xFFF) as u16
    }

    pub fn index_3(&self) -> u16 {
        (self.0 >> 30 & 0xFFF) as u16
    }

    pub fn index_4(&self) -> u16 {
        (self.0 >> 39 & 0xFFF) as u16
    }

    pub fn as_pointer(&self) -> *const u8 {
        self.0 as *const u8
    }

    pub fn as_pointer_mut(&mut self) -> *mut u8 {
        self.0 as *mut u8
    }
}

impl From<u64> for VirtualAddress {
    fn from(value : u64) -> VirtualAddress {
        VirtualAddress(value)
    }
}