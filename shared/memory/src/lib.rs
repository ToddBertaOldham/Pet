//**************************************************************************************************
// lib.rs                                                                                          *
// Copyright (c) 2019-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#![no_std]

mod address_macro;
mod align;
mod endian;
mod segment;

pub use address_macro::*;
pub use align::*;
pub use endian::*;
pub use segment::*;
