//**************************************************************************************************
// field.rs                                                                                        *
// Copyright (c) 2020 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use core::fmt;
use core::marker;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct BitError {
    start: u32,
    length: u32,
}

impl BitError {
    pub fn new(start: u32, length: u32) -> Self {
        Self { start, length }
    }
}

impl fmt::Display for BitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{} bit(s) starting at bit {} is invalid.",
            self.length, self.start,
        )
    }
}

pub trait ReadBit {
    type Output;

    fn read_bit(self, bit: u32) -> Result<bool, BitError>;
    fn read_bit_segment(
        self,
        self_start: u32,
        output_start: u32,
        length: u32,
    ) -> Result<Self::Output, BitError>;
}

pub trait WriteBit<Source = Self> {
    type Output;

    fn write_bit(self, bit: u32, value: bool) -> Result<Self::Output, BitError>;
    fn write_bit_segment(
        self,
        source: Source,
        self_start: u32,
        source_start: u32,
        length: u32,
    ) -> Result<Self::Output, BitError>;
}

pub trait WriteBitAssign<Source = Self> {
    fn write_bit_assign(&mut self, bit: u32, value: bool) -> Result<(), BitError>;
    fn write_bit_segment_assign(
        &mut self,
        source: Source,
        self_start: u32,
        source_start: u32,
        length: u32,
    ) -> Result<(), BitError>;
}

macro_rules! implement_for_int {
    ($type:ty) => {
        impl ReadBit for $type {
            type Output = Self;

            fn read_bit(self, bit: u32) -> Result<bool, BitError> {
                let modifier: $type = 1;
                let mask = modifier
                    .checked_shl(bit)
                    .ok_or_else(|| BitError::new(bit, 1))?;
                Ok(self & mask != 0)
            }

            fn read_bit_segment(
                self,
                self_start: u32,
                output_start: u32,
                length: u32,
            ) -> Result<Self::Output, BitError>
            where
                Self: marker::Sized,
            {
                let bits = $crate::size_of::<$type>() as u32;
                if self_start + length > bits {
                    Err(BitError::new(self_start, length))
                } else {
                    let mask = Self::MAX.wrapping_shr(bits - length);
                    let self_shifted = self.wrapping_shr(self_start);
                    let self_masked = self_shifted & mask;
                    let output = self_masked.wrapping_shl(output_start);
                    Ok(output)
                }
            }
        }

        impl WriteBit<Self> for $type {
            type Output = Self;

            fn write_bit(self, bit: u32, value: bool) -> Result<Self::Output, BitError> {
                let modifier: $type = 1;
                let shift = modifier
                    .checked_shl(bit)
                    .ok_or_else(|| BitError::new(bit, 1))?;
                if value {
                    Ok(self | shift)
                } else {
                    Ok(self & !shift)
                }
            }

            fn write_bit_segment(
                self,
                source: Self,
                self_start: u32,
                source_start: u32,
                length: u32,
            ) -> Result<Self::Output, BitError>
            where
                Self: marker::Sized,
            {
                let bits = $crate::size_of::<$type>() as u32;

                if self_start + length > bits {
                    Err(BitError::new(self_start, length))
                } else if source_start + length > bits {
                    Err(BitError::new(source_start, length))
                } else {
                    // Move the other value to the to the right by its start and remove the rest
                    // with the mask. Then move that value to the left by self's start.
                    // Finally clear the specified bits from self and apply other's bits.

                    let mask = Self::MAX.wrapping_shr(bits - length);
                    let source_shifted = source.wrapping_shr(source_start);
                    let source_value = source_shifted & mask;
                    let self_mask = !mask.wrapping_shl(self_start);
                    let source_value_shifted = source_value.wrapping_shl(self_start);

                    Ok((self & self_mask) | source_value_shifted)
                }
            }
        }

        impl WriteBitAssign<Self> for $type {
            fn write_bit_assign(&mut self, bit: u32, value: bool) -> Result<(), BitError> {
                let new_value = self.write_bit(bit, value);
                match new_value {
                    Ok(value) => {
                        *self = value;
                        Ok(())
                    }
                    Err(error) => Err(error),
                }
            }

            fn write_bit_segment_assign(
                &mut self,
                source: Self,
                source_start: u32,
                self_start: u32,
                length: u32,
            ) -> Result<(), BitError>
            where
                Self: marker::Sized,
            {
                let new_value = self.write_bit_segment(source, self_start, source_start, length);
                match new_value {
                    Ok(value) => {
                        *self = value;
                        Ok(())
                    }
                    Err(error) => Err(error),
                }
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
