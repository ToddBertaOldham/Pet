//**************************************************************************************************
// bits.rs                                                                                         *
// Copyright (c) 2020-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use core::fmt;
use core::marker;
use core::mem;

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
            "{} bit(s) starting at bit {} is out of range.",
            self.length, self.start,
        )
    }
}

pub trait GetBit: marker::Sized {
    type Output;
    type Error;

    fn get_bit(self, bit: u32) -> bool;

    fn get_bits(self, self_start: u32, output_start: u32, length: u32) -> Self::Output;

    fn checked_get_bit(self, bit: u32) -> Result<bool, Self::Error>;

    fn checked_get_bits(
        self,
        self_start: u32,
        output_start: u32,
        length: u32,
    ) -> Result<Self::Output, Self::Error>;
}

pub trait SetBit<Source = Self>: marker::Sized {
    type Output;
    type Error;

    fn set_bit(self, bit: u32, value: bool) -> Self::Output;

    fn set_bits(
        self,
        source: Source,
        self_start: u32,
        source_start: u32,
        length: u32,
    ) -> Self::Output;

    fn checked_set_bit(self, bit: u32, value: bool) -> Result<Self::Output, Self::Error>;

    fn checked_set_bits(
        self,
        source: Source,
        self_start: u32,
        source_start: u32,
        length: u32,
    ) -> Result<Self::Output, Self::Error>;
}

pub trait SetBitAssign<Source = Self> {
    type Error;

    fn set_bit_assign(&mut self, bit: u32, value: bool);

    fn set_bits_assign(&mut self, source: Source, self_start: u32, source_start: u32, length: u32);

    fn checked_set_bit_assign(&mut self, bit: u32, value: bool) -> Result<(), Self::Error>;

    fn checked_set_bits_assign(
        &mut self,
        source: Source,
        self_start: u32,
        source_start: u32,
        length: u32,
    ) -> Result<(), Self::Error>;
}

macro_rules! implement_for_int {
    ($type:ty) => {
        impl GetBit for $type {
            type Output = Self;
            type Error = BitError;

            fn get_bit(self, bit: u32) -> bool {
                self.checked_get_bit(bit)
                    .expect("Specified bit is out of range.")
            }

            fn get_bits(self, self_start: u32, output_start: u32, length: u32) -> Self::Output {
                self.checked_get_bits(self_start, output_start, length)
                    .expect("Specified bit range is out of range.")
            }

            fn checked_get_bit(self, bit: u32) -> Result<bool, Self::Error> {
                let mask = (1 as $type)
                    .checked_shl(bit)
                    .ok_or_else(|| BitError::new(bit, 1))?;
                Ok(self & mask != 0)
            }

            fn checked_get_bits(
                self,
                self_start: u32,
                output_start: u32,
                length: u32,
            ) -> Result<Self::Output, Self::Error> {
                let bits = (mem::size_of::<$type>() * 8) as u32;
                if self_start + length > bits {
                    Err(BitError::new(self_start, length))
                } else if output_start + length > bits {
                    Err(BitError::new(output_start, length))
                } else {
                    let mask = Self::MAX.wrapping_shr(bits - length);
                    let self_shifted = self.wrapping_shr(self_start);
                    let self_masked = self_shifted & mask;
                    let output = self_masked.wrapping_shl(output_start);
                    Ok(output)
                }
            }
        }

        impl SetBit<Self> for $type {
            type Output = Self;
            type Error = BitError;

            fn set_bit(self, bit: u32, value: bool) -> Self::Output {
                self.checked_set_bit(bit, value)
                    .expect("Specified bit is out of range.")
            }

            fn set_bits(
                self,
                source: Self,
                self_start: u32,
                source_start: u32,
                length: u32,
            ) -> Self::Output {
                self.checked_set_bits(source, self_start, source_start, length)
                    .expect("Specified bit range is out of range.")
            }

            fn checked_set_bit(self, bit: u32, value: bool) -> Result<Self::Output, Self::Error> {
                let shift = (1 as $type)
                    .checked_shl(bit)
                    .ok_or_else(|| BitError::new(bit, 1))?;
                if value {
                    Ok(self | shift)
                } else {
                    Ok(self & !shift)
                }
            }

            fn checked_set_bits(
                self,
                source: Self,
                self_start: u32,
                source_start: u32,
                length: u32,
            ) -> Result<Self::Output, Self::Error> {
                let bits = (mem::size_of::<$type>() * 8) as u32;

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

        impl SetBitAssign<Self> for $type {
            type Error = BitError;

            fn set_bit_assign(&mut self, bit: u32, value: bool) {
                self.checked_set_bit_assign(bit, value)
                    .expect("Specified bit is out of range.");
            }

            fn set_bits_assign(
                &mut self,
                source: Self,
                self_start: u32,
                source_start: u32,
                length: u32,
            ) {
                self.checked_set_bits_assign(source, self_start, source_start, length)
                    .expect("Specified bit range is out of range.");
            }

            fn checked_set_bit_assign(&mut self, bit: u32, value: bool) -> Result<(), Self::Error> {
                *self = self.checked_set_bit(bit, value)?;
                Ok(())
            }

            fn checked_set_bits_assign(
                &mut self,
                source: Self,
                self_start: u32,
                source_start: u32,
                length: u32,
            ) -> Result<(), Self::Error> {
                *self = self.checked_set_bits(source, self_start, source_start, length)?;
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
