//**************************************************************************************************
// mapper.rs                                                                                       *
// Copyright (c) 2020-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::paging::size_64::{
    DirectoryPtrTable, DirectoryPtrValue, DirectoryTable, DirectoryValue, MapType, Pml4Table,
    Pml4Value, Pml5Table, Pml5Value, RootTable, Table, TableValue,
};
use crate::paging::{PAGE_1_GIB_SIZE_IN_BYTES, PAGE_2_MIB_SIZE_IN_BYTES, PAGE_4_KIB_SIZE_IN_BYTES};
use crate::{
    PhysicalAddress52, PhysicalAddressError, VirtualAddress48, VirtualAddress57, VirtualAddress64,
    VirtualAddress64Error,
};
use core::convert::TryInto;
use core::fmt;
use core::ops::IndexMut;
use memory::CheckAlignment;

#[derive(Debug)]
pub struct Mapper<'a, TInterface: MapperInterface> {
    interface: &'a mut TInterface,
}

impl<'a, TAllocator: MapperInterface> Mapper<'a, TAllocator> {
    pub fn new(interface: &'a mut TAllocator) -> Self {
        Self { interface }
    }

    pub fn interface(&self) -> &TAllocator {
        self.interface
    }

    pub unsafe fn map<
        TVirtualAddress: TryInto<VirtualAddress48> + TryInto<VirtualAddress57>,
        TPhysicalAddress: TryInto<PhysicalAddress52>,
    >(
        &mut self,
        root_table: RootTable,
        virtual_address: TVirtualAddress,
        physical_address: TPhysicalAddress,
        map_type: MapType,
        count: u64,
    ) -> Result<(), MapError> {
        let physical_address_52 = physical_address
            .try_into()
            .map_err(|_| MapError::InvalidPhysicalAddress)?;

        match root_table {
            RootTable::Pml5(pml5_table_ptr) => {
                let virtual_address_57: VirtualAddress57 = virtual_address
                    .try_into()
                    .map_err(|_| MapError::InvalidVirtualAddress)?;
                self.map_level_5(
                    pml5_table_ptr,
                    virtual_address_57,
                    physical_address_52,
                    map_type,
                    count,
                )
            }
            RootTable::Pml4(pml4_table_ptr) => {
                let virtual_address_48: VirtualAddress48 = virtual_address
                    .try_into()
                    .map_err(|_| MapError::InvalidVirtualAddress)?;
                self.map_level_4(
                    pml4_table_ptr,
                    virtual_address_48,
                    physical_address_52,
                    map_type,
                    count,
                )
            }
        }
    }

    pub unsafe fn unmap<TVirtualAddress: TryInto<VirtualAddress48> + TryInto<VirtualAddress57>>(
        &mut self,
        root_table: RootTable,
        virtual_address: TVirtualAddress,
    ) -> Result<(), MapError> {
        match root_table {
            RootTable::Pml5(pml5_table_ptr) => {
                let virtual_address_57: VirtualAddress57 = virtual_address
                    .try_into()
                    .map_err(|_| MapError::InvalidVirtualAddress)?;
                self.unmap_level_5(pml5_table_ptr, virtual_address_57)
            }
            RootTable::Pml4(pml4_table_ptr) => {
                let virtual_address_48: VirtualAddress48 = virtual_address
                    .try_into()
                    .map_err(|_| MapError::InvalidVirtualAddress)?;
                self.unmap_level_4(pml4_table_ptr, virtual_address_48)
            }
        }
    }

    pub unsafe fn map_level_4<
        TVirtualAddress: TryInto<VirtualAddress48>,
        TPhysicalAddress: TryInto<PhysicalAddress52>,
    >(
        &mut self,
        pml4_table_ptr: *mut Pml4Table,
        virtual_address: TVirtualAddress,
        physical_address: TPhysicalAddress,
        map_type: MapType,
        count: u64,
    ) -> Result<(), MapError> {
        if pml4_table_ptr.is_null() {
            return Err(MapError::NullTable);
        }

        let virtual_address_48: VirtualAddress48 = virtual_address
            .try_into()
            .map_err(|_| MapError::InvalidVirtualAddress)?;

        let physical_address_52 = physical_address
            .try_into()
            .map_err(|_| MapError::InvalidPhysicalAddress)?;

        match map_type {
            MapType::Page4Kib => {
                Self::check_map_page_4_kib_args(virtual_address_48, physical_address_52)?;
                for i in 0..count {
                    let next_virtual_address = virtual_address_48.add_table_index(i, false)?;
                    let next_physical_address = physical_address_52.add_page_4_kib(i, false)?;
                    self.map_page_4_kib_with_pml_4(
                        pml4_table_ptr,
                        next_virtual_address,
                        next_physical_address,
                    )?;
                }
            }
            MapType::Page2Mib => {
                Self::check_map_page_2_mib_args(virtual_address_48, physical_address_52)?;
                for i in 0..count {
                    let next_virtual_address = virtual_address_48.add_directory_index(i, false)?;
                    let next_physical_address = physical_address_52.add_page_2_mib(i, false)?;
                    self.map_page_2_mib_with_pml_4(
                        pml4_table_ptr,
                        next_virtual_address,
                        next_physical_address,
                    )?;
                }
            }
            MapType::Page1Gib => {
                Self::check_map_page_1_gib_args(virtual_address_48, physical_address_52)?;
                for i in 0..count {
                    let next_virtual_address =
                        virtual_address_48.add_directory_ptr_index(i, false)?;
                    let next_physical_address = physical_address_52.add_page_1_gib(i, false)?;
                    self.map_page_1_gib_with_pml_4(
                        pml4_table_ptr,
                        next_virtual_address,
                        next_physical_address,
                    )?;
                }
            }
        }
        Ok(())
    }

    pub unsafe fn unmap_level_4<TVirtualAddress: VirtualAddress64>(
        &mut self,
        pml4_table_ptr: *mut Pml4Table,
        virtual_address: TVirtualAddress,
    ) -> Result<(), MapError> {
        self.map_none_with_pml_4(pml4_table_ptr, virtual_address)
    }

    pub unsafe fn map_level_5<
        TVirtualAddress: TryInto<VirtualAddress57>,
        TPhysicalAddress: TryInto<PhysicalAddress52>,
    >(
        &mut self,
        pml5_table_ptr: *mut Pml5Table,
        virtual_address: TVirtualAddress,
        physical_address: TPhysicalAddress,
        map_type: MapType,
        count: u64,
    ) -> Result<(), MapError> {
        if pml5_table_ptr.is_null() {
            return Err(MapError::NullTable);
        }
        let virtual_address_57: VirtualAddress57 = virtual_address
            .try_into()
            .map_err(|_| MapError::InvalidVirtualAddress)?;

        let physical_address_52 = physical_address
            .try_into()
            .map_err(|_| MapError::InvalidPhysicalAddress)?;

        match map_type {
            MapType::Page4Kib => {
                Self::check_map_page_4_kib_args(virtual_address_57, physical_address_52)?;
                for i in 0..count {
                    let next_virtual_address = virtual_address_57.add_table_index(i, false)?;
                    let next_physical_address = physical_address_52.add_page_4_kib(i, false)?;
                    let pml_4_table_ptr =
                        self.create_pml_4_table(pml5_table_ptr, next_virtual_address)?;
                    self.map_page_4_kib_with_pml_4(
                        pml_4_table_ptr,
                        next_virtual_address,
                        next_physical_address,
                    )?;
                }
            }
            MapType::Page2Mib => {
                Self::check_map_page_2_mib_args(virtual_address_57, physical_address_52)?;
                for i in 0..count {
                    let next_virtual_address = virtual_address_57.add_directory_index(i, false)?;
                    let next_physical_address = physical_address_52.add_page_2_mib(i, false)?;
                    let pml_4_table_ptr =
                        self.create_pml_4_table(pml5_table_ptr, next_virtual_address)?;
                    self.map_page_2_mib_with_pml_4(
                        pml_4_table_ptr,
                        next_virtual_address,
                        next_physical_address,
                    )?;
                }
            }
            MapType::Page1Gib => {
                Self::check_map_page_1_gib_args(virtual_address_57, physical_address_52)?;
                for i in 0..count {
                    let next_virtual_address =
                        virtual_address_57.add_directory_ptr_index(i, false)?;
                    let next_physical_address = physical_address_52.add_page_1_gib(i, false)?;
                    let pml_4_table_ptr =
                        self.create_pml_4_table(pml5_table_ptr, next_virtual_address)?;
                    self.map_page_1_gib_with_pml_4(
                        pml_4_table_ptr,
                        next_virtual_address,
                        next_physical_address,
                    )?;
                }
            }
        }
        Ok(())
    }

    pub unsafe fn unmap_level_5(
        &mut self,
        pml5_table_ptr: *mut Pml5Table,
        virtual_address: VirtualAddress57,
    ) -> Result<(), MapError> {
        let pml_5_table = &mut *pml5_table_ptr;

        let pml4_table_address: PhysicalAddress52;
        match pml_5_table.index_mut(virtual_address.pml_5_index()).value() {
            Pml5Value::None => return Ok(()),
            Pml5Value::Pml4Table(address) => pml4_table_address = address,
        }

        let pml4_table = self.interface.convert_to_virtual_ptr(pml4_table_address);

        self.map_none_with_pml_4(pml4_table, virtual_address)
    }

    fn check_map_page_1_gib_args<TVirtualAddress: VirtualAddress64>(
        virtual_address: TVirtualAddress,
        physical_address: PhysicalAddress52,
    ) -> Result<(), MapError> {
        if !physical_address.check_alignment(PAGE_1_GIB_SIZE_IN_BYTES) {
            return Err(MapError::InvalidPhysicalAddress);
        }
        if virtual_address.page_offset_1_gib() != 0 {
            return Err(MapError::InvalidVirtualAddress);
        }
        Ok(())
    }

    fn check_map_page_2_mib_args<TVirtualAddress: VirtualAddress64>(
        virtual_address: TVirtualAddress,
        physical_address: PhysicalAddress52,
    ) -> Result<(), MapError> {
        if !physical_address.check_alignment(PAGE_2_MIB_SIZE_IN_BYTES) {
            return Err(MapError::InvalidPhysicalAddress);
        }
        if virtual_address.page_offset_2_mib() != 0 {
            return Err(MapError::InvalidVirtualAddress);
        }
        Ok(())
    }

    fn check_map_page_4_kib_args<TVirtualAddress: VirtualAddress64>(
        virtual_address: TVirtualAddress,
        physical_address: PhysicalAddress52,
    ) -> Result<(), MapError> {
        if !physical_address.check_alignment(PAGE_4_KIB_SIZE_IN_BYTES) {
            return Err(MapError::InvalidPhysicalAddress);
        }
        if virtual_address.page_offset_4_kib() != 0 {
            return Err(MapError::InvalidVirtualAddress);
        }
        Ok(())
    }

    unsafe fn map_none_with_pml_4<TVirtualAddress: VirtualAddress64>(
        &mut self,
        pml4_table_ptr: *mut Pml4Table,
        virtual_address: TVirtualAddress,
    ) -> Result<(), MapError> {
        let pml_4_table = &mut *pml4_table_ptr;

        let directory_ptr_table_address: PhysicalAddress52;
        match pml_4_table.index_mut(virtual_address.pml4_index()).value() {
            Pml4Value::None => return Ok(()),
            Pml4Value::DirectoryPtrTable(address) => directory_ptr_table_address = address,
        }

        let directory_ptr_table =
            &mut *directory_ptr_table_address.as_mut_ptr::<DirectoryPtrTable>();
        let directory_ptr_table_entry =
            directory_ptr_table.index_mut(virtual_address.directory_ptr_index());

        let directory_table_address: PhysicalAddress52;
        match directory_ptr_table_entry.value() {
            DirectoryPtrValue::None => return Ok(()),
            DirectoryPtrValue::Page1Gib(_) => {
                directory_ptr_table_entry
                    .set_value(DirectoryPtrValue::None)
                    .unwrap();
                return Ok(());
            }
            DirectoryPtrValue::DirectoryTable(address) => directory_table_address = address,
        }

        let directory_table = &mut *directory_table_address.as_mut_ptr::<DirectoryTable>();
        let directory_table_entry = directory_table.index_mut(virtual_address.directory_index());

        let table_address: PhysicalAddress52;
        match directory_table_entry.value() {
            DirectoryValue::None => return Ok(()),
            DirectoryValue::Page2Mib(_) => {
                directory_table_entry
                    .set_value(DirectoryValue::None)
                    .unwrap();
                return Ok(());
            }
            DirectoryValue::Table(address) => table_address = address,
        }

        let table = &mut *table_address.as_mut_ptr::<Table>();
        let table_entry = table.index_mut(virtual_address.table_index());

        return match table_entry.value() {
            TableValue::None => Ok(()),
            TableValue::Page4Kib(_) => {
                table_entry.set_value(TableValue::None).unwrap();
                Ok(())
            }
        };
    }

    unsafe fn map_page_4_kib_with_pml_4<TVirtualAddress: VirtualAddress64>(
        &mut self,
        pml4_table_ptr: *mut Pml4Table,
        virtual_address: TVirtualAddress,
        physical_address: PhysicalAddress52,
    ) -> Result<(), MapError> {
        let directory_ptr_table_ptr =
            self.create_directory_ptr_table(pml4_table_ptr, virtual_address)?;
        let directory_table_ptr =
            self.create_directory_table(directory_ptr_table_ptr, virtual_address)?;
        let table_ptr = self.create_table(directory_table_ptr, virtual_address)?;

        let table = &mut *table_ptr;
        let entry = table.index_mut(virtual_address.table_index());

        entry
            .set_value(TableValue::Page4Kib(physical_address))
            .unwrap();

        Ok(())
    }

    unsafe fn map_page_2_mib_with_pml_4<TVirtualAddress: VirtualAddress64>(
        &mut self,
        pml4_table_ptr: *mut Pml4Table,
        virtual_address: TVirtualAddress,
        physical_address: PhysicalAddress52,
    ) -> Result<(), MapError> {
        let directory_ptr_table_ptr =
            self.create_directory_ptr_table(pml4_table_ptr, virtual_address)?;
        let directory_table_ptr =
            self.create_directory_table(directory_ptr_table_ptr, virtual_address)?;

        let directory_table = &mut *directory_table_ptr;
        let entry = directory_table.index_mut(virtual_address.directory_index());

        if let DirectoryValue::Table(current_table) = entry.value() {
            self.interface.dealloc_table(current_table);
        }

        entry
            .set_value(DirectoryValue::Page2Mib(physical_address))
            .unwrap();

        Ok(())
    }

    unsafe fn map_page_1_gib_with_pml_4<TVirtualAddress: VirtualAddress64>(
        &mut self,
        pml4_table_ptr: *mut Pml4Table,
        virtual_address: TVirtualAddress,
        physical_address: PhysicalAddress52,
    ) -> Result<(), MapError> {
        let directory_ptr_table_ptr =
            self.create_directory_ptr_table(pml4_table_ptr, virtual_address)?;

        let directory_ptr_table = &mut *directory_ptr_table_ptr;
        let entry = directory_ptr_table.index_mut(virtual_address.directory_ptr_index());

        if let DirectoryPtrValue::DirectoryTable(current_table) = entry.value() {
            self.interface.dealloc_table(current_table);
        }

        entry
            .set_value(DirectoryPtrValue::Page1Gib(physical_address))
            .unwrap();

        Ok(())
    }

    unsafe fn create_pml_4_table(
        &mut self,
        table_ptr: *mut Pml5Table,
        virtual_address: VirtualAddress57,
    ) -> Result<*mut Pml4Table, MapError> {
        let table = &mut *table_ptr;
        let entry = table.index_mut(virtual_address.pml_5_index());

        let mut value = entry
            .value()
            .pml4_table()
            .unwrap_or(PhysicalAddress52::null());

        if value.is_null() {
            value = self.interface.alloc_table();
            if value.is_null() {
                return Err(MapError::AllocationFailed);
            }
            entry
                .set_value(Pml5Value::Pml4Table(value))
                .map_err(|_| MapError::BadAllocation)?;
        }

        let value_ptr = self.interface.convert_to_virtual_ptr(value);

        Ok(value_ptr)
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
            value = self.interface.alloc_table();
            if value.is_null() {
                return Err(MapError::AllocationFailed);
            }
            entry
                .set_value(Pml4Value::DirectoryPtrTable(value))
                .map_err(|_| MapError::BadAllocation)?;
        }

        let value_ptr = self.interface.convert_to_virtual_ptr(value);

        Ok(value_ptr)
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
            value = self.interface.alloc_table();
            if value.is_null() {
                return Err(MapError::AllocationFailed);
            }
            entry
                .set_value(DirectoryPtrValue::DirectoryTable(value))
                .map_err(|_| MapError::BadAllocation)?;
        }

        let value_ptr = self.interface.convert_to_virtual_ptr(value);

        Ok(value_ptr)
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
            value = self.interface.alloc_table();
            if value.is_null() {
                return Err(MapError::AllocationFailed);
            }
            entry
                .set_value(DirectoryValue::Table(value))
                .map_err(|_| MapError::BadAllocation)?;
        }

        let value_ptr = self.interface.convert_to_virtual_ptr(value);

        Ok(value_ptr)
    }
}

pub trait MapperInterface {
    unsafe fn alloc_table(&mut self) -> PhysicalAddress52;
    unsafe fn dealloc_table(&mut self, address: PhysicalAddress52);

    unsafe fn convert_to_virtual_ptr<T>(&mut self, physical_address: PhysicalAddress52) -> *mut T {
        physical_address.as_mut_ptr()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MapError {
    AllocationFailed,
    BadAllocation,
    NullTable,
    InvalidVirtualAddress,
    InvalidPhysicalAddress,
}

impl From<VirtualAddress64Error> for MapError {
    fn from(_: VirtualAddress64Error) -> Self {
        MapError::InvalidVirtualAddress
    }
}

impl From<PhysicalAddressError> for MapError {
    fn from(_: PhysicalAddressError) -> Self {
        MapError::InvalidPhysicalAddress
    }
}

impl fmt::Display for MapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MapError::AllocationFailed => {
                write!(f, "The mapper allocator returned a null address.")
            }
            MapError::BadAllocation => write!(
                f,
                "The mapper allocator returned an bad address. It most likely wasn't \
                aligned."
            ),
            MapError::NullTable => write!(f, "The root table passed was null."),
            MapError::InvalidVirtualAddress => {
                write!(f, "The virtual address specified is invalid.")
            }
            MapError::InvalidPhysicalAddress => {
                write!(f, "The physical address specified is invalid.")
            }
        }
    }
}
