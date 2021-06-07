//**************************************************************************************************
// id.rs                                                                                           *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Id(u32);

impl From<Id> for u8 {
    fn from(value: Id) -> Self {
        (value.0 >> 24) as u8
    }
}

impl From<u8> for Id {
    fn from(value: u8) -> Self {
        Id((value as u32) << 24)
    }
}

impl From<Id> for u32 {
    fn from(value: Id) -> Self {
        value.0
    }
}

impl From<u32> for Id {
    fn from(value: u32) -> Self {
        Id(value)
    }
}
