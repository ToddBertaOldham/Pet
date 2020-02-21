//**************************************************************************************************
// translate.rs                                                                                    *
// Copyright (c) 2020 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::Level4VirtualAddress;

pub fn pml4_index<T: Level4VirtualAddress>(address: T) -> usize {
    (address.into() >> 39 & 0x1FF) as usize
}

pub fn directory_ptr_index<T: Level4VirtualAddress>(address: T) -> usize {
    (address.into() >> 30 & 0x1FF) as usize
}

pub fn directory_index<T: Level4VirtualAddress>(address: T) -> usize {
    (address.into() >> 21 & 0x1FF) as usize
}

pub fn table_index<T: Level4VirtualAddress>(address: T) -> usize {
    (address.into() >> 12 & 0x1FF) as usize
}