//**************************************************************************************************
// lib.rs                                                                                          *
// Copyright (c) 2019-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::convert::TryFrom;
use core::str::Chars;

pub fn encode_string_with_null(string: &str) -> Result<Box<[u16]>, ()> {
    let mut encoded_string: Vec<u16> = string.encode_usc2().collect::<Result<Vec<u16>, ()>>()?;
    encoded_string.push(0);
    Ok(encoded_string.into_boxed_slice())
}

pub trait FromUcs2Buffer {
    type Error;

    fn from_usc2(buffer: &[u16]) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

pub trait ToUcs2Buffer {
    type Return;

    fn encode_usc2(&self) -> Self::Return;
}

impl FromUcs2Buffer for String {
    type Error = ();

    fn from_usc2(buffer: &[u16]) -> Result<Self, Self::Error> {
        let mut string = String::with_capacity(buffer.len());
        for character in buffer {
            let new_character = char::from_usc2(*character)?;
            string.push(new_character);
        }
        Ok(string)
    }
}

impl<'a> ToUcs2Buffer for &'a str {
    type Return = EncodeUcs2<'a>;

    fn encode_usc2(&self) -> Self::Return {
        EncodeUcs2(self.chars())
    }
}

pub struct EncodeUcs2<'a>(Chars<'a>);

impl<'a> Iterator for EncodeUcs2<'a> {
    type Item = Result<u16, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .and_then(|character| Some(character.encode_usc2()))
    }
}

pub trait FromUcs2Char {
    type Error;

    fn from_usc2(character: u16) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

pub trait ToUcs2Char {
    type Error;

    fn encode_usc2(&self) -> Result<u16, Self::Error>;
}

impl FromUcs2Char for char {
    type Error = ();

    fn from_usc2(character: u16) -> Result<Self, Self::Error> {
        char::try_from(character as u32).map_err(|_| ())
    }
}

impl ToUcs2Char for char {
    type Error = ();

    fn encode_usc2(&self) -> Result<u16, Self::Error> {
        let mut buffer = [0; 2];
        let result = self.encode_utf16(&mut buffer);
        if result.len() > 1 {
            Err(())
        } else {
            Ok(result[0])
        }
    }
}
