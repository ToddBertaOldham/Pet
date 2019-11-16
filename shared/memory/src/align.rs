//**************************************************************************************************
// align.rs                                                                                        *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use core::fmt;

pub struct Error;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str("Alignment is not power of 2.")
    }
}

pub fn check(address: usize, alignment: usize) -> bool {
    address % alignment == 0
}

pub fn up(address: usize, alignment: usize) -> Result<usize, Error> {
    if is_power_of_2(alignment) {
        Ok(up_unchecked(address, alignment))
    } else {
        Err(Error)
    }
}

pub fn up_unchecked(address: usize, alignment: usize) -> usize {
    down_unchecked(address + alignment - 1, alignment)
}

pub fn down(address: usize, alignment: usize) -> Result<usize, Error> {
    if is_power_of_2(alignment) {
        Ok(down_unchecked(address, alignment))
    } else {
        Err(Error)
    }
}

pub fn down_unchecked(address: usize, alignment: usize) -> usize {
    address & !(alignment - 1)
}

pub fn is_power_of_2(alignment: usize) -> bool {
    (alignment & (alignment - 1)) == 0
}

