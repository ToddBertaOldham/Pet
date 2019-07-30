// *************************************************************************
// ist.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use core::convert::TryFrom;

#[derive(Copy, Clone, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct IstIndex(u8);

impl TryFrom<u8> for IstIndex {
    type Error = ();

    fn try_from(value : u8) -> Result<Self, Self::Error> {
        if value > 7 {
            Err(())
        }
        else {
            Ok(IstIndex(value))
        }
    }
}

impl From<IstIndex> for u8 {
    fn from(value : IstIndex) -> Self {
        value.0
    }
}