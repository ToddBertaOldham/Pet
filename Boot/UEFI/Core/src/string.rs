// *************************************************************************
// string.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use super::error::UEFIError;
use core::str::FromStr;
use alloc::boxed::Box;
use alloc::vec::Vec;
use alloc::slice;

pub struct C16String {
    buffer : Box<[u16]>
}

impl C16String {
    pub unsafe fn from_raw(ptr : *mut u16) -> Self {
        let mut length = 0;
        loop {
            if *(ptr.add(length)) == 0 {
                break;
            }
            length += 1;
        }

        let slice = slice::from_raw_parts_mut(ptr, length);
        let buffer = Box::from_raw(slice);
        C16String { buffer }
    }

    pub fn into_raw(self) -> *mut u16 {
        Box::into_raw(self.buffer) as *mut u16
    }
}

impl FromStr for C16String {
    type Err = UEFIError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let length = s.encode_utf16().count() + 1;
        let mut buffer = Vec::with_capacity(length);
        
        for char16 in s.encode_utf16() {
            buffer.push(char16);
        }

        buffer.push(0);

        Ok(C16String { buffer : buffer.into_boxed_slice() })       
    }
}