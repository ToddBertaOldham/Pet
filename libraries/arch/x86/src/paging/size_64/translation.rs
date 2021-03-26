//**************************************************************************************************
// translation.rs                                                                                  *
// Copyright (c) 2020-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::paging::size_64::{
    DirectoryPtrTable, DirectoryPtrValue, DirectoryTable, DirectoryValue, MapValue, Pml4Table,
    Pml4Value, Pml5Table, Pml5Value, Table, TableValue,
};
use crate::{VirtualAddress57, VirtualAddress64};
use core::ops::IndexMut;

pub unsafe fn walk_pml5(table_ptr: *mut Pml5Table, virtual_address: VirtualAddress57) -> MapValue {
    let table = &mut *table_ptr;
    match table.index_mut(virtual_address.pml_5_index()).value() {
        Pml5Value::None => MapValue::None,
        Pml5Value::Pml4Table(pml4_table) => walk_pml4(pml4_table.as_mut_ptr(), virtual_address),
    }
}

pub unsafe fn walk_pml4<TVirtualAddress: VirtualAddress64>(
    table_ptr: *mut Pml4Table,
    virtual_address: TVirtualAddress,
) -> MapValue {
    let table = &mut *table_ptr;
    match table.index_mut(virtual_address.pml4_index()).value() {
        Pml4Value::None => MapValue::None,
        Pml4Value::DirectoryPtrTable(directory_ptr_table) => {
            walk_directory_ptr(directory_ptr_table.as_mut_ptr(), virtual_address)
        }
    }
}

pub unsafe fn walk_directory_ptr<TVirtualAddress: VirtualAddress64>(
    table_ptr: *mut DirectoryPtrTable,
    virtual_address: TVirtualAddress,
) -> MapValue {
    let table = &mut *table_ptr;
    match table
        .index_mut(virtual_address.directory_ptr_index())
        .value()
    {
        DirectoryPtrValue::None => MapValue::None,
        DirectoryPtrValue::DirectoryTable(directory_table) => {
            walk_directory(directory_table.as_mut_ptr(), virtual_address)
        }
        DirectoryPtrValue::Page1Gib(page) => MapValue::Page1Gib(page),
    }
}

pub unsafe fn walk_directory<TVirtualAddress: VirtualAddress64>(
    table_ptr: *mut DirectoryTable,
    virtual_address: TVirtualAddress,
) -> MapValue {
    let table = &mut *table_ptr;
    match table.index_mut(virtual_address.directory_index()).value() {
        DirectoryValue::None => MapValue::None,
        DirectoryValue::Table(table) => walk_table(table.as_mut_ptr(), virtual_address),
        DirectoryValue::Page2Mib(page) => MapValue::Page2Mib(page),
    }
}

pub unsafe fn walk_table<TVirtualAddress: VirtualAddress64>(
    table_ptr: *mut Table,
    virtual_address: TVirtualAddress,
) -> MapValue {
    let table = &mut *table_ptr;
    match table.index_mut(virtual_address.table_index()).value() {
        TableValue::None => MapValue::None,
        TableValue::Page4Kib(page) => MapValue::Page4Kib(page),
    }
}
