//**************************************************************************************************
// number.rs                                                                                       *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use core::fmt::{Debug, Display};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub trait Number:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + Debug
    + Display
    + Sized
{
}

impl Number for u8 {}
impl Number for u16 {}
impl Number for u32 {}
impl Number for u64 {}
impl Number for u128 {}

impl Number for i8 {}
impl Number for i16 {}
impl Number for i32 {}
impl Number for i64 {}
impl Number for i128 {}
