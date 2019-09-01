//**************************************************************************************************
// string.rs                                                                                       *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::Error;
use alloc::boxed::Box;
use alloc::vec::Vec;
use ucs2::ToUcs2Buffer;

//TODO Move this into ucs2 crate.

pub fn create_char16_buffer(string: &str) -> Result<Box<[u16]>, Error> {
    let encoding_result: Result<Vec<u16>, ()> = string.encode_usc2().collect();
    let mut converted_string: Vec<u16> =
        encoding_result.map_err(|_| Error::InvalidArgument("string"))?;
    converted_string.push(0);
    Ok(converted_string.into_boxed_slice())
}
