//**************************************************************************************************
// lib.rs                                                                                          *
// Copyright (c) 2019-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#![no_std]
#![feature(const_fn_trait_bound)]

pub use address::*;
pub use address_macros::*;
pub use align::*;
pub use bits::*;
pub use endian::*;
pub use segment::*;

#[macro_use]
mod address_macros;
mod address;
mod align;
pub mod allocators;
mod bits;
mod endian;
pub mod flags_macro;
mod segment;
pub mod split;
pub mod structures;
