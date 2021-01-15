//**************************************************************************************************
// lib.rs                                                                                          *
// Copyright (c) 2019-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#![no_std]

#[macro_use]
extern crate enums;

mod error;
pub mod registers;
mod serial_port;
mod settings;

pub use error::*;
pub use serial_port::*;
pub use settings::*;
