//**************************************************************************************************
// lib.rs                                                                                          *
// Copyright (c) 2019-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#![cfg_attr(feature = "no-std", no_std)]
#![feature(min_specialization)]

#[cfg(feature = "alloc-impl")]
extern crate alloc;

#[cfg(feature = "no-std")]
pub mod cursor;
mod read;
mod write;

pub use memory::Endian;
pub use read::*;
pub use write::*;
