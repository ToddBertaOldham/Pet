//**************************************************************************************************
// lib.rs                                                                                          *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#![no_std]

#[macro_use]
mod macros;
mod error;
mod information;
mod time;

pub use error::*;
pub use information::*;
pub use time::*;

//TODO Merge into math crate as module?
