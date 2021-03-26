//**************************************************************************************************
// mod.rs                                                                                          *
// Copyright (c) 2019-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

// This module contains both paging structures and the logic for mapping. size_64 contains both
// level 4 and level 5 paging. size_32 contains both PAE and non-PAE paging. Although PAE
// paging has similar structures to level 4 and level 5 paging, the entries are slightly
// different due to the lack of a protection key.

#[macro_use]
mod macros;
pub mod size_64;

pub const PAGE_4_KIB_SIZE_IN_BYTES: u64 = 4096;

pub const PAGE_2_MIB_SIZE_IN_BYTES: u64 = PAGE_4_KIB_SIZE_IN_BYTES * 512;

pub const PAGE_4_MIB_SIZE_IN_BYTES: u64 = PAGE_4_KIB_SIZE_IN_BYTES * 1024;

pub const PAGE_1_GIB_SIZE_IN_BYTES: u64 = PAGE_2_MIB_SIZE_IN_BYTES * 512;
