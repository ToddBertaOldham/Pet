//**************************************************************************************************
// lib.rs                                                                                          *
// Copyright (c) 2018-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#![no_std]
#![feature(alloc_layout_extra)]
#![feature(abi_efiapi)]

extern crate alloc;
#[macro_use]
extern crate enums;
#[macro_use]
extern crate bits;

mod error;
pub mod ffi;
#[macro_use]
pub mod memory;
pub mod graphics;
#[macro_use]
pub mod io;
pub mod configuration;
pub mod protocol;
pub mod system;

pub use self::error::*;
pub use self::ffi::system::Table as SystemTable;
pub use self::ffi::{Guid, Handle, Status};
