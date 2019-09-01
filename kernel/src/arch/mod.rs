// *************************************************************************
// mod.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

#[cfg(target_arch = "x86_64")]
#[macro_use]
mod x86_64;

#[cfg(target_arch = "x86_64")]
pub use self::x86_64::*;
