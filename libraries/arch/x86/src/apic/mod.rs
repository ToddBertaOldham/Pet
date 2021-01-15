//**************************************************************************************************
// mod.rs                                                                                          *
// Copyright (c) 2020 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

pub mod base;
mod local;
mod x2apic;
mod xapic;

pub use local::*;
pub use x2apic::*;
pub use xapic::*;
