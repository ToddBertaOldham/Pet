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
        PageTable([PageTableEntry::empty(); 512])
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
    pub fn empty() -> Self { 
        PageTableEntry(0)
    }

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

    pub fn physical_address(&self) -> u64 {
        (self.0 & 0xFFFFFFFFFF000) >> 12
    }

    pub fn set_physical_address(&mut self, address : u64) {
        self.0 &= 0xFFF0000000000FFF | (address << 12);
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
        (self.0 & 0xFFFFFFFFFF000) >> 12
    }

    pub fn set_physical_address(&mut self, address : u64) {
        self.0 &= 0xFFF0000000000FFF | (address << 12);
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

    pub fn table_index(&self) -> u16 {
        (self.0 >> 12 & 0x1FF) as u16
    }

    pub fn directory_index(&self) -> u16 {
        (self.0 >> 21 & 0x1FF) as u16
    }

    pub fn directory_ptr_index(&self) -> u16 {
        (self.0 >> 30 & 0x1FF) as u16
    }

    pub fn pml_4_index(&self) -> u16 {
        (self.0 >> 39 & 0x1FF) as u16
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

pub trait PagingManager {
    unsafe fn map(&self, pml_4 : &mut PageTable, physical_address : *const u8, virtual_address : VirtualAddress) -> Result<(), PagingError> {      
        let directory_ptr_table = self.access_sub_table(pml_4, virtual_address.pml_4_index(), true)?;
        let directory_table = self.access_sub_table(directory_ptr_table, virtual_address.directory_ptr_index(), true)?;
        let table = self.access_sub_table(directory_table, virtual_address.directory_index(), true)?;

        let mut table_entry = table[virtual_address.table_index()];
        table_entry.set_is_present(true);
        table_entry.set_write_allowed(true);
        table_entry.set_physical_address(physical_address as u64);

        Ok(())
    }
    unsafe fn unmap(&self, pml_4 : &mut PageTable, virtual_address : VirtualAddress) -> Result<(), PagingError> {
        let directory_ptr_table = self.access_sub_table(pml_4, virtual_address.pml_4_index(), false)?;
        let directory_table = self.access_sub_table(directory_ptr_table, virtual_address.directory_ptr_index(), false)?;
        let table = self.access_sub_table(directory_table, virtual_address.directory_index(), false)?;

        table[virtual_address.table_index()] = PageTableEntry::empty();

        Ok(())
    }
    unsafe fn retrieve_physical_address(&self, pml_4 : &mut PageTable, virtual_address : VirtualAddress) -> Result<*const u8, PagingError> {
        let directory_ptr_table = self.access_sub_table(pml_4, virtual_address.pml_4_index(), false)?;
        let directory_table = self.access_sub_table(directory_ptr_table, virtual_address.directory_ptr_index(), false)?;
        let table = self.access_sub_table(directory_table, virtual_address.directory_index(), false)?;

        let table_entry = table[virtual_address.table_index()];
        if table_entry.is_present() {
            Ok(table_entry.physical_address() as *const u8)
        }
        else {
            Err(PagingError::PageNotFound)
        }
    }
    unsafe fn access_sub_table(&self, base_table : &mut PageTable, index : u16, create : bool) -> Result<&mut PageTable, PagingError> {
        let mut entry = base_table[index];
        if !entry.is_present() {
            if create {
                let new_table = self.allocate_page_table()?;
                entry.set_is_present(true);
                entry.set_write_allowed(true);
                entry.set_physical_address(new_table as u64);
                Ok(&mut *new_table)
            }
            else {
                Err(PagingError::PageNotFound)
            }
        }
        else {
            Ok(&mut*(entry.physical_address() as *mut PageTable))
        }
    }
    fn allocate_page_table(&self) -> Result<*mut PageTable, PagingError>;
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PagingError {
    PageNotFound,
    AllocationFailed
}
