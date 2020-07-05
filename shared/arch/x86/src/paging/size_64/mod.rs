//**************************************************************************************************
// mod.rs                                                                                          *
// Copyright (c) 2020 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#[macro_use]
mod macros;
mod directory;
mod directory_ptr;
mod mapper;
mod pml_4;
mod pml_5;
mod table;
pub mod translation;

pub use directory::*;
pub use directory_ptr::*;
pub use pml_4::*;
pub use pml_5::*;
pub use table::*;
pub use mapper::*;

use crate::PhysicalAddress52;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MapType {
    None,
    Page4Kib(PhysicalAddress52),
    Page2Mib(PhysicalAddress52),
    Page1Gib(PhysicalAddress52),
}

impl MapType {
    pub fn is_mapped(self) -> bool {
        self != MapType::None
    }
}