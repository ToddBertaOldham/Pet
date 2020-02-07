//**************************************************************************************************
// field.rs                                                                                        *
// Copyright (c) 2020 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use core::fmt;

#[derive(Copy, Clone, Debug)]
pub struct InvalidBitError(u32);

impl InvalidBitError {
    pub fn new(bit: u32) -> Self {
        Self(bit)
    }
}

impl fmt::Display for InvalidBitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Bit {} is invalid.", self.0)
    }
}

pub trait BitField {
    fn is_bit_set(&self, bit : u32) -> Result<bool, InvalidBitError>;
    fn set_bit(&mut self, bit : u32, value : bool) -> Result<(), InvalidBitError>;
}

macro_rules! implement_for_int {
    ($type:ty) => {
      impl $crate::BitField for $type {
          fn is_bit_set(&self, bit: u32) -> Result<bool, $crate::InvalidBitError> {
            let modifier : $type = 1;
            let shift = modifier.checked_shl(bit).ok_or_else(|| $crate::InvalidBitError::new(bit))?;
            Ok(shift & self != 0)
          }

          fn set_bit(&mut self, bit: u32, value: bool) -> Result<(), $crate::InvalidBitError> {
            let modifier : $type = 1;
            let shift = modifier.checked_shl(bit).ok_or_else(|| $crate::InvalidBitError::new(bit))?;
            if value {
                *self |= shift;
            }
            else {
                *self &= !shift;
            }
            Ok(())
          }
      }
    };
}

implement_for_int!(u8);
implement_for_int!(u16);
implement_for_int!(u32);
implement_for_int!(u64);
implement_for_int!(u128);
implement_for_int!(usize);

implement_for_int!(i8);
implement_for_int!(i16);
implement_for_int!(i32);
implement_for_int!(i64);
implement_for_int!(i128);
implement_for_int!(isize);