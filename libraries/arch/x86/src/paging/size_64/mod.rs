//**************************************************************************************************
// mod.rs                                                                                          *
// Copyright (c) 2020-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

mod directory;
mod directory_ptr;
mod mapper;
mod pml_4;
mod pml_5;
mod table;
pub mod translation;

pub use directory::*;
pub use directory_ptr::*;
pub use mapper::*;
pub use pml_4::*;
pub use pml_5::*;
pub use table::*;

use crate::paging::{PAGE_1_GIB_SIZE_IN_BYTES, PAGE_2_MIB_SIZE_IN_BYTES, PAGE_4_KIB_SIZE_IN_BYTES};
use crate::{PhysicalAddress52, PhysicalAddressError};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RootTable {
    Pml5(*mut Pml5Table),
    Pml4(*mut Pml4Table),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MapType {
    Page4Kib,
    Page2Mib,
    Page1Gib,
}

impl MapType {
    pub fn size_in_bytes(self) -> u64 {
        match self {
            MapType::Page4Kib => PAGE_4_KIB_SIZE_IN_BYTES,
            MapType::Page2Mib => PAGE_2_MIB_SIZE_IN_BYTES,
            MapType::Page1Gib => PAGE_1_GIB_SIZE_IN_BYTES,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MapValue {
    None,
    Page4Kib(PhysicalAddress52),
    Page2Mib(PhysicalAddress52),
    Page1Gib(PhysicalAddress52),
}

impl MapValue {
    pub fn is_mapped(self) -> bool {
        self != MapValue::None
    }
}
