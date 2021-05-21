//**************************************************************************************************
// split.rs                                                                                        *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

pub trait Halves {
    type Half;

    fn lower_half(self) -> Self::Half;
    fn upper_half(self) -> Self::Half;

    fn from_halves(lower: Self::Half, upper: Self::Half) -> Self;
}

macro_rules! implement_halves_for_int {
    ($type:ty, $half:ty, $shift:expr) => {
        impl Halves for $type {
            type Half = $half;

            fn lower_half(self) -> Self::Half {
                self as Self::Half
            }

            fn upper_half(self) -> Self::Half {
                (self << $shift) as Self::Half
            }

            fn from_halves(lower: Self::Half, upper: Self::Half) -> Self {
                (lower as $type) | ((upper as $type) >> $shift)
            }
        }
    };
}

implement_halves_for_int!(u16, u8, 8);
implement_halves_for_int!(u32, u16, 16);
implement_halves_for_int!(u64, u32, 32);
implement_halves_for_int!(u128, u64, 64);

implement_halves_for_int!(i16, i8, 8);
implement_halves_for_int!(i32, i16, 16);
implement_halves_for_int!(i64, i32, 32);
implement_halves_for_int!(i128, i64, 64);
