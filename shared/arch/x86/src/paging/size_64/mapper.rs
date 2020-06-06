//**************************************************************************************************
// mapper.rs                                                                                       *
// Copyright (c) 2020 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::paging::size_64::{
    DirectoryPtrTable, DirectoryPtrValue, DirectoryTable, DirectoryValue, MapType, Pml4Table,
    Pml4Value, Pml5Table, Table, TableValue,
};
use crate::{VirtualAddress57, VirtualAddress64, PhysicalAddress52};
use core::ops::IndexMut;

#[derive(Debug)]
pub struct Mapper<'a, TAllocator: MapperAllocator> {
    allocator: &'a mut TAllocator,
}

impl<'a, TAllocator: MapperAllocator> Mapper<'a, TAllocator> {
    pub fn new(allocator: &'a mut TAllocator) -> Self {
        Self { allocator }
    }

    pub unsafe fn map_level_4<TVirtualAddress: VirtualAddress64>(
        &mut self,
        pml4_table: *mut Pml4Table,
        virtual_address: TVirtualAddress,
        map_type: MapType,
    ) -> Result<(), MapError> {
        if pml4_table.is_null() {
            return Err(MapError::InvalidTable);
        }
        match map_type {
            MapType::None => {}
            MapType::Page4Kb(page_4kb) => {
                let directory_ptr_table_ptr =
                    self.create_directory_ptr_table(pml4_table, virtual_address)?;
                let directory_table_ptr =
                    self.create_directory_table(directory_ptr_table_ptr, virtual_address)?;
                let table_ptr = self.create_table(directory_table_ptr, virtual_address)?;

                let table = &mut *table_ptr;
                let entry = table.index_mut(virtual_address.table_index());
                //TODO What about offset?
                entry.set_value(TableValue::Page4Kb(page_4kb));
            }
            MapType::Page2Mb(page_2_mb) => {
                let directory_ptr_table_ptr =
                    self.create_directory_ptr_table(pml4_table, virtual_address)?;
                let directory_table_ptr =
                    self.create_directory_table(directory_ptr_table_ptr, virtual_address)?;

                let directory_table = &mut *directory_table_ptr;
                let entry = directory_table.index_mut(virtual_address.directory_index());

                if let DirectoryValue::Table(current_table) = entry.value() {
                    self.allocator.dealloc_table(current_table);
                }

                //TODO What about offset?
                entry.set_value(DirectoryValue::Page2Mb(page_2_mb));
            }
            MapType::Page1Gb(page_1_gb) => {
                let directory_ptr_table_ptr =
                    self.create_directory_ptr_table(pml4_table, virtual_address)?;

                let directory_ptr_table = &mut *directory_ptr_table_ptr;
                let entry = directory_ptr_table.index_mut(virtual_address.directory_ptr_index());

                if let DirectoryPtrValue::DirectoryTable(current_table) = entry.value() {
                    self.allocator.dealloc_table(current_table);
                }

                //TODO What about offset?
                entry.set_value(DirectoryPtrValue::Page1Gb(page_1_gb));
            }
        };
        unimplemented!()
    }

    pub unsafe fn map_level_5(
        &mut self,
        pml5_table: *mut Pml5Table,
        virtual_address: VirtualAddress57,
        map_type: MapType,
    ) -> Result<(), MapError> {
        if pml5_table.is_null() {
            return Err(MapError::InvalidTable);
        }

        unimplemented!()
    }

    unsafe fn create_directory_ptr_table<TAddress: VirtualAddress64>(
        &mut self,
        table_ptr: *mut Pml4Table,
        virtual_address: TAddress,
    ) -> Result<*mut DirectoryPtrTable, MapError> {
        let table = &mut *table_ptr;
        let entry = table.index_mut(virtual_address.pml4_index());

        let mut value = entry
            .value()
            .directory_ptr_table()
            .unwrap_or(PhysicalAddress52::null());

        if value.is_null() {
            value = self.allocator.alloc_table();
            if value.is_null() {
                return Err(MapError::AllocationFailed);
            }
            entry.set_value(Pml4Value::DirectoryPtrTable(value));
        }

        Ok(value.as_mut_ptr())
    }

    unsafe fn create_directory_table<TAddress: VirtualAddress64>(
        &mut self,
        table_ptr: *mut DirectoryPtrTable,
        virtual_address: TAddress,
    ) -> Result<*mut DirectoryTable, MapError> {
        let table = &mut *table_ptr;
        let entry = table.index_mut(virtual_address.directory_ptr_index());

        let mut value = entry
            .value()
            .directory_table()
            .unwrap_or(PhysicalAddress52::null());

        if value.is_null() {
            value = self.allocator.alloc_table();
            if value.is_null() {
                return Err(MapError::AllocationFailed);
            }
            entry.set_value(DirectoryPtrValue::DirectoryTable(value));
        }

        Ok(value.as_mut_ptr())
    }

    unsafe fn create_table<TAddress: VirtualAddress64>(
        &mut self,
        table_ptr: *mut DirectoryTable,
        virtual_address: TAddress,
    ) -> Result<*mut Table, MapError> {
        let table = &mut *table_ptr;
        let entry = table.index_mut(virtual_address.directory_index());

        let mut value = entry.value().table().unwrap_or(PhysicalAddress52::null());

        if value.is_null() {
            value = self.allocator.alloc_table();
            if value.is_null() {
                return Err(MapError::AllocationFailed);
            }
            entry.set_value(DirectoryValue::Table(value));
        }

        Ok(value.as_mut_ptr())
    }
}

pub trait MapperAllocator {
    unsafe fn alloc_table(&mut self) -> PhysicalAddress52;
    unsafe fn dealloc_table(&mut self, address: PhysicalAddress52);
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MapError {
    AllocationFailed,
    InvalidTable,
}
