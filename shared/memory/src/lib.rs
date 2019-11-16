//**************************************************************************************************
// lib.rs                                                                                          *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#![no_std]

mod segment;
pub mod align;
mod endian;

pub use endian::*;
pub use segment::*;