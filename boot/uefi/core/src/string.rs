// *************************************************************************
// string.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use alloc::vec::Vec;

pub fn convert_to_utf16(string : &str) -> Vec<u16> {
    string.encode_utf16().collect()
}