//**************************************************************************************************
// mapper.rs                                                                                       *
// Copyright (c) 2020 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::paging::level_4::{translate, DirectoryTable, DirectoryValue, Table, TableValue};
use crate::paging::level_4::{DirectoryPtrTable, DirectoryPtrValue, Pml4Table, Pml4Value};
use crate::{Level4VirtualAddress, PhysicalAddress52};
use core::ops::IndexMut;
use crate::control::size_64::register_3 as cr3;

#[derive(Debug)]
pub struct Mapper<'a, TAllocator: MappingAllocator> {
    table: *mut Pml4Table,
    //TODO Make allocator optional.
    allocator: &'a mut TAllocator,
}

impl<'a, TAllocator: MappingAllocator> Mapper<'a, TAllocator> {
    pub unsafe fn with_table(allocator: &'a mut TAllocator, table: *mut Pml4Table) -> Self {
        Self { allocator, table }
    }

    pub unsafe fn alloc_new_table(allocator: &'a mut TAllocator) -> Self {
        let table = allocator.alloc_table().as_mut_ptr();
        Self {
            allocator,
            table,
        }
    }
    pub unsafe fn control_table(allocator: &'a mut TAllocator) -> Self {
        let value = cr3::read().physical_address().as_mut_ptr();
        Self {
            allocator,
            table: value
        }
    }

    pub fn map<TAddress: Level4VirtualAddress>(
        &mut self,
        virtual_address: TAddress,
        map_type: MapType,
    ) {
        unsafe {
            match map_type {
                MapType::None => {}
                MapType::Page4Kb(page_4kb) => {
                    let directory_ptr_table = self
                        .lookup_pml4_value(virtual_address, true)
                        .directory_ptr_table()
                        .unwrap();
                    let directory_table = self
                        .lookup_directory_ptr_value(directory_ptr_table, virtual_address, true)
                        .directory_table_ptr()
                        .unwrap();
                }
                MapType::Page2Mb(page_2_mb) => {}
                MapType::Page1Gb(page_1_gb) => {}
            };
        }
    }

    pub fn map_multiple<TAddress: Level4VirtualAddress>(
        &mut self,
        base_virtual_address: TAddress,
        page_count: usize,
        map_type: MapType,
    ) {
        for i in 0..page_count {
            //self.map()
        }

        unimplemented!()
    }

    pub fn translate<TAddress: Level4VirtualAddress>(
        &mut self,
        virtual_address: TAddress,
    ) -> MapType {
        unsafe {
            match self.lookup_pml4_value(virtual_address, false) {
                Pml4Value::None => MapType::None,
                Pml4Value::DirectoryPtrTable(directory_ptr_table) => {
                    self.translate_directory_ptr(directory_ptr_table.as_mut_ptr(), virtual_address)
                }
            }
        }
    }

    unsafe fn translate_directory_ptr<TAddress: Level4VirtualAddress>(
        &mut self,
        table_ptr: *mut DirectoryPtrTable,
        virtual_address: TAddress,
    ) -> MapType {
        match self.lookup_directory_ptr_value(table_ptr, virtual_address, false) {
            DirectoryPtrValue::None => MapType::None,
            DirectoryPtrValue::DirectoryTable(directory_table) => {
                self.translate_directory(directory_table.as_mut_ptr(), virtual_address)
            }
            DirectoryPtrValue::Page1Gb(page) => MapType::Page1Gb(page),
        }
    }

    unsafe fn translate_directory<TAddress: Level4VirtualAddress>(
        &mut self,
        table_ptr: *mut DirectoryTable,
        virtual_address: TAddress,
    ) -> MapType {
        match self.lookup_directory_value(table_ptr, virtual_address, false) {
            DirectoryValue::None => MapType::None,
            DirectoryValue::Table(table) => {
                self.translate_table(table.as_mut_ptr(), virtual_address)
            }
            DirectoryValue::Page2Mb(page) => MapType::Page2Mb(page),
        }
    }

    unsafe fn translate_table<TAddress: Level4VirtualAddress>(
        &mut self,
        table_ptr: *mut Table,
        virtual_address: TAddress,
    ) -> MapType {
        match self.lookup_table_value(table_ptr, virtual_address) {
            TableValue::None => MapType::None,
            TableValue::Page4Kb(page) => MapType::Page4Kb(page),
        }
    }

    unsafe fn lookup_pml4_value<TAddress: Level4VirtualAddress>(
        &mut self,
        virtual_address: TAddress,
        force_table: bool,
    ) -> Pml4Value {
        let table = &mut *self.table;
        let index = translate::pml4_index(virtual_address);
        let entry = table.index_mut(index);
        let mut value = entry.value();

        if value.directory_ptr_table().is_none() && force_table {
            let new_table = self.allocator.alloc_table();
            value = Pml4Value::DirectoryPtrTable(new_table);
            entry.set_value(value);
        }

        value
    }

    unsafe fn lookup_directory_ptr_value<TAddress: Level4VirtualAddress>(
        &mut self,
        table_ptr: *mut DirectoryPtrTable,
        virtual_address: TAddress,
        force_table: bool,
    ) -> DirectoryPtrValue {
        let table = &mut *table_ptr;
        let index = translate::directory_ptr_index(virtual_address);
        let entry = table.index_mut(index);
        let mut value = entry.value();

        if value.directory_table_ptr().is_none() && force_table {
            let new_table = self.allocator.alloc_table();
            value = DirectoryPtrValue::DirectoryTable(new_table);
            entry.set_value(value);
        }

        value
    }

    unsafe fn lookup_directory_value<TAddress: Level4VirtualAddress>(
        &mut self,
        table_ptr: *mut DirectoryTable,
        virtual_address: TAddress,
        force_table: bool,
    ) -> DirectoryValue {
        let table = &mut *table_ptr;
        let index = translate::directory_index(virtual_address);
        let entry = table.index_mut(index);
        let mut value = entry.value();

        if value.table_ptr().is_none() && force_table {
            let new_table = self.allocator.alloc_table();
            value = DirectoryValue::Table(new_table);
            entry.set_value(value);
        }

        value
    }

    unsafe fn lookup_table_value<TAddress: Level4VirtualAddress>(
        &mut self,
        table_ptr: *mut Table,
        virtual_address: TAddress,
    ) -> TableValue {
        let table = &mut *table_ptr;
        let index = translate::table_index(virtual_address);
        let entry = table.index_mut(index);
        entry.value()
    }
}

pub trait MappingAllocator {
    unsafe fn alloc_table(&mut self) -> PhysicalAddress52;
    unsafe fn dealloc_table(&mut self, address: PhysicalAddress52);
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MapError {}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MapType {
    None,
    Page4Kb(PhysicalAddress52),
    Page2Mb(PhysicalAddress52),
    Page1Gb(PhysicalAddress52),
}

impl MapType {
    pub fn is_mapped(self) -> bool {
        self != MapType::None
    }
}
