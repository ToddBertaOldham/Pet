//**************************************************************************************************
// lib.rs                                                                                          *
// Copyright (c) 2019-2020 Todd Berta-Oldham                                                       *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#![no_std]

mod segment;
mod align;
mod endian;
mod address_macro;

pub use align::*;
pub use endian::*;
pub use segment::*;
pub use address_macro::*;