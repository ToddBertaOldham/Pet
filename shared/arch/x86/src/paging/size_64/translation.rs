//**************************************************************************************************
// translation.rs                                                                                  *
// Copyright (c) 2020 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::paging::size_64::{
    DirectoryPtrTable, DirectoryPtrValue, DirectoryTable, DirectoryValue, MapType, Pml4Table,
    Pml4Value, Pml5Table, Pml5Value, Table, TableValue,
};
use crate::{VirtualAddress57, VirtualAddress64};
use core::ops::IndexMut;

pub unsafe fn walk_pml5(table_ptr: *mut Pml5Table, virtual_address: VirtualAddress57) -> MapType {
    let table = &mut *table_ptr;
    match table.index_mut(virtual_address.pml5_index()).value() {
        Pml5Value::None => MapType::None,
        Pml5Value::Pml4Table(pml4_table) => walk_pml4(pml4_table.as_mut_ptr(), virtual_address),
    }
}

pub unsafe fn walk_pml4<TVirtualAddress: VirtualAddress64>(
    table_ptr: *mut Pml4Table,
    virtual_address: TVirtualAddress,
) -> MapType {
    let table = &mut *table_ptr;
    match table.index_mut(virtual_address.pml4_index()).value() {
        Pml4Value::None => MapType::None,
        Pml4Value::DirectoryPtrTable(directory_ptr_table) => {
            walk_directory_ptr(directory_ptr_table.as_mut_ptr(), virtual_address)
        }
    }
}

pub unsafe fn walk_directory_ptr<TVirtualAddress: VirtualAddress64>(
    table_ptr: *mut DirectoryPtrTable,
    virtual_address: TVirtualAddress,
) -> MapType {
    let table = &mut *table_ptr;
    match table
        .index_mut(virtual_address.directory_ptr_index())
        .value()
    {
        DirectoryPtrValue::None => MapType::None,
        DirectoryPtrValue::DirectoryTable(directory_table) => {
            walk_directory(directory_table.as_mut_ptr(), virtual_address)
        }
        DirectoryPtrValue::Page1Gb(page) => MapType::Page1Gb(page),
    }
}

pub unsafe fn walk_directory<TVirtualAddress: VirtualAddress64>(
    table_ptr: *mut DirectoryTable,
    virtual_address: TVirtualAddress,
) -> MapType {
    let table = &mut *table_ptr;
    match table.index_mut(virtual_address.directory_index()).value() {
        DirectoryValue::None => MapType::None,
        DirectoryValue::Table(table) => walk_table(table.as_mut_ptr(), virtual_address),
        DirectoryValue::Page2Mb(page) => MapType::Page2Mb(page),
    }
}

pub unsafe fn walk_table<TVirtualAddress: VirtualAddress64>(
    table_ptr: *mut Table,
    virtual_address: TVirtualAddress,
) -> MapType {
    let table = &mut *table_ptr;
    match table.index_mut(virtual_address.table_index()).value() {
        TableValue::None => MapType::None,
        TableValue::Page4Kb(page) => MapType::Page4Kb(page),
    }
}
