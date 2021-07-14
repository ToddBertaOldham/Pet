//**************************************************************************************************
// error.rs                                                                                        *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use core::fmt;

#[derive(Copy, Clone, Debug)]
pub struct ConvertError;

impl fmt::Display for ConvertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Unit conversion is not impossible due to the limited \
             width of the value type or the base constants type."
        )
    }
}
