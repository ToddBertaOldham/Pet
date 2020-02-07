//**************************************************************************************************
// error.rs                                                                                        *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use core::fmt;

#[derive(Copy, Clone, Debug)]
pub struct NonCanonicalAddressError(u64);

impl NonCanonicalAddressError {
    pub fn new(address: u64) -> Self {
        Self(address)
    }
}

impl fmt::Display for NonCanonicalAddressError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "The address \"{:#X}\" is not canonical.", self.0)
    }
}