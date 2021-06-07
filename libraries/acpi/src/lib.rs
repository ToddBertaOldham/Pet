//**************************************************************************************************
// lib.rs                                                                                          *
// Copyright (c) 2020-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#![no_std]
#![feature(associated_type_bounds)]

mod header;
pub mod madt;
mod root_entry;
mod rsdp;
mod rsdt;
mod xsdt;

pub use header::*;
pub use root_entry::*;
pub use rsdp::*;
pub use rsdt::*;
pub use xsdt::*;

use core::convert::TryInto;
use core::fmt::Debug;

pub trait Interface {
    unsafe fn convert_to_virtual_ptr<TPtr, TAddress: TryInto<*mut TPtr, Error: Debug>>(
        &self,
        address: TAddress,
    ) -> *mut TPtr {
        address
            .try_into()
            .expect("Address cannot be converted to pointer.")
    }
}
