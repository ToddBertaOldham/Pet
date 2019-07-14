// *************************************************************************
// paging.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use core::ops::{ Index, IndexMut };
use core::convert::TryFrom;
use bits::BitField;
use encapsulation::BitGetterSetters;

#[repr(align(4096))]
pub struct PageTable([PageTableEntry; 512]);

impl PageTable {
    pub fn new() -> Self {
        PageTable([Default::default(); 512])
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
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, BitGetterSetters)]
pub struct PageTableEntry(
    #[bit_access(name = "is_present", set = true, index = 0, borrow_self = false)]
    #[bit_access(name = "write_allowed", set = true, index = 1, borrow_self = false)]
    #[bit_access(name = "user_access_allowed", set = true, index = 2, borrow_self = false)]
    #[bit_access(name = "write_through_enabled", set = true, index = 3, borrow_self = false)]
    #[bit_access(name = "cache_disabled", set = true, index = 4, borrow_self = false)]
    #[bit_access(name = "accessed", set = true, index = 5, borrow_self = false)]
    #[bit_access(name = "is_dirty", set = true, index = 6, borrow_self = false)]
    #[bit_access(name = "references_page", set = true, index = 7, borrow_self = false)]
    #[bit_access(name = "is_global", set = true, index = 8, borrow_self = false)]
    u64);

impl PageTableEntry {
    pub fn physical_address(&self) -> u64 {
        self.0 & 0xFFFFFFFFFF000
    }

    pub fn set_physical_address(&mut self, address : u64) {
        self.0 &= 0xFFF;
        self.0 |= 0xFFFFFFFFFF000 & address;
    }
}

impl From<u64> for PageTableEntry {
    fn from(value : u64) -> PageTableEntry {
        PageTableEntry(value)
    }
}

impl From<PageTableEntry> for u64 {
    fn from(value : PageTableEntry) -> u64 {
        value.0
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct VirtualAddress(u64);

impl VirtualAddress {
    pub fn offset(self) -> u16 { (self.0 & 0xFFF) as u16 }

    pub fn table_index(self) -> u16 { (self.0 >> 12 & 0x1FF) as u16 }

    pub fn directory_index(self) -> u16 { (self.0 >> 21 & 0x1FF) as u16 }

    pub fn directory_ptr_index(self) -> u16 { (self.0 >> 30 & 0x1FF) as u16  }

    pub fn pml_4_index(self) -> u16 { (self.0 >> 39 & 0x1FF) as u16 }

    pub fn as_ptr<T>(&self) -> *const T { self.0 as *const T }

    pub fn as_mut_ptr<T>(&mut self) -> *mut T { self.0 as *mut T }
}

impl TryFrom<u64> for VirtualAddress {
    type Error = ();

    fn try_from(value : u64) -> Result<Self, Self::Error> {
        let end = value.is_bit_set(47);
        for i in 48..64 {
            if value.is_bit_set(i) != end {
                return Err(());
            }
        }

        Ok(VirtualAddress(value))
    }
}

impl<T> TryFrom<*mut T> for VirtualAddress {
    type Error = ();

    fn try_from(value : *mut T) -> Result<Self, Self::Error> {
        VirtualAddress::try_from(value as u64)
    }
}

impl<T> TryFrom<*const T> for VirtualAddress {
    type Error = ();

    fn try_from(value : *const T) -> Result<Self, Self::Error> {
        VirtualAddress::try_from(value as u64)
    }
}

impl From<VirtualAddress> for u64 {
    fn from(value : VirtualAddress) -> u64 {
        value.0
    }  
}

impl core::fmt::LowerHex for VirtualAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        core::fmt::LowerHex::fmt(&self.0, f)
    }
}

impl core::fmt::UpperHex for VirtualAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        core::fmt::UpperHex::fmt(&self.0, f)
    }
}

impl core::fmt::Pointer for VirtualAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        core::fmt::LowerHex::fmt(&self.0, f)
    }
}

pub trait PagingAllocator {
    fn allocate_page_table(&self) -> Result<*mut PageTable, PagingError>;
}

pub mod operations {
    use super::*;

    pub unsafe fn map(pml_4 : &mut PageTable, physical_address : *const u8, virtual_address : VirtualAddress, allocator : Option<&PagingAllocator>) -> Result<(), PagingError> {      
        let directory_ptr_table = &mut*access_sub_table(pml_4, virtual_address.pml_4_index(), allocator)?;
        let directory_table = &mut*access_sub_table(directory_ptr_table, virtual_address.directory_ptr_index(), allocator)?;
        let table = &mut*access_sub_table(directory_table, virtual_address.directory_index(), allocator)?;

        let table_entry = table.index_mut(virtual_address.table_index());
        table_entry.set_is_present(true);
        table_entry.set_write_allowed(true);
        table_entry.set_physical_address(physical_address as u64);

        Ok(())
    }
    pub unsafe fn unmap(pml_4 : &mut PageTable, virtual_address : VirtualAddress) -> Result<(), PagingError> {
        let directory_ptr_table = &mut*access_sub_table(pml_4, virtual_address.pml_4_index(), None)?;
        let directory_table = &mut*access_sub_table(directory_ptr_table, virtual_address.directory_ptr_index(), None)?;
        let table = &mut*access_sub_table(directory_table, virtual_address.directory_index(), None)?;

        table[virtual_address.table_index()] = Default::default();

        Ok(())
    }
    pub unsafe fn retrieve_physical_address(pml_4 : &mut PageTable, virtual_address : VirtualAddress) -> Result<*const u8, PagingError> {
        let directory_ptr_table = &mut*access_sub_table(pml_4, virtual_address.pml_4_index(), None)?;
        let directory_table = &mut*access_sub_table(directory_ptr_table, virtual_address.directory_ptr_index(), None)?;
        let table = &mut*access_sub_table(directory_table, virtual_address.directory_index(), None)?;

        let table_entry = table.index_mut(virtual_address.table_index());
        if table_entry.is_present() {
            Ok(table_entry.physical_address() as *const u8)
        }
        else {
            Err(PagingError::PageNotFound)
        }
    }

    unsafe fn access_sub_table(base_table : &mut PageTable, index : u16, page_allocator : Option<&PagingAllocator>) -> Result<*mut PageTable, PagingError> {
        let entry = base_table.index_mut(index);
        if !entry.is_present() {
            if let Some(allocator) = page_allocator {
                let new_table = allocator.allocate_page_table()?;
                entry.set_is_present(true);
                entry.set_write_allowed(true);
                entry.set_physical_address(new_table as u64);
                Ok(new_table)
            }
            else {
                Err(PagingError::PageNotFound)
            }
        }
        else {
            Ok(entry.physical_address() as *mut PageTable)
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PagingError {
    PageNotFound,
    AllocationFailed
}
