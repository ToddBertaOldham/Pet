//**************************************************************************************************
// lib.rs                                                                                          *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#![cfg_attr(feature = "no-std", no_std)]
#![feature(specialization)]

#[cfg(feature = "alloc-impl")]
extern crate alloc;

mod read;
mod write;
#[cfg(feature = "no-std")]
pub mod cursor;

pub use read::*;
pub use write::*;
pub use memory::Endian;