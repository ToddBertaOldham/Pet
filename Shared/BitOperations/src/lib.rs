// *************************************************************************
// lib.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

#![no_std]

use core::ops::Range;

pub trait BitField {
    fn is_bit_set(&self, bit : u16) -> bool;
    fn set_bit(&mut self, bit : u16, value : bool);
    fn set_bits(&mut self, range : Range<u16>, value : bool) {
      for i in range {
        self.set_bit(i, value);
      }
    }
}

macro_rules! implement_bit_field {
    ($type:ty) => {
      impl $crate::BitField for $type {
          fn is_bit_set(&self, bit: u16) -> bool {
            1 << bit & self != 0
          }

          fn set_bit(&mut self, bit: u16, value: bool) {
            if value {
                *self |= 1 << bit;
            }
            else {
                *self &= !(1 << bit);
            }
          }
      }
    };
}

implement_bit_field!(u8);
implement_bit_field!(u16);
implement_bit_field!(u32);
implement_bit_field!(u64);
implement_bit_field!(u128);
implement_bit_field!(usize);

implement_bit_field!(i8);
implement_bit_field!(i16);
implement_bit_field!(i32);
implement_bit_field!(i64);
implement_bit_field!(i128);
implement_bit_field!(isize);

