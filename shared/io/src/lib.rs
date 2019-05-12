// *************************************************************************
// lib.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

#![no_std]
#![cfg_attr(feature = "alloc-impl", feature(alloc))]
#![feature(specialization)]

#[cfg(feature = "alloc-impl")]
extern crate alloc;

mod endian;
mod binary;
pub mod cursor;

pub use endian::*;
pub use binary::*;