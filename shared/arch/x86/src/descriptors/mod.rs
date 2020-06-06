//**************************************************************************************************
// mod.rs                                                                                          *
// Copyright (c) 2019-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

// This module contains all of the descriptors used by GDTs, LDTs, and IDTs. They are kept here
// due to some descriptors such as the Task Gate Descriptor being included in both GDTs/LDTs and
// IDTs.

pub mod size_64;