//**************************************************************************************************
// mod.rs                                                                                          *
// Copyright (c) 2019-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

// This module contains both paging structures and the logic for mapping. size_64 contains both
// level 4 and level 5 paging. size_32 contains both PAE and non-PAE paging. Although PAE
// paging has similar structures to level 4 and level 5 paging, the entries are slightly
// different due to the lack of a protection key.

#[macro_use]
mod macros;
pub mod size_64;