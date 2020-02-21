//**************************************************************************************************
// align.rs                                                                                        *
// Copyright (c) 2019-2020 Todd Berta-Oldham                                                       *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use core::fmt;

pub struct AlignmentError;

impl fmt::Display for AlignmentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str("Alignment is invalid.")
    }
}

pub trait Align where Self : Sized {
    type Alignment;

    fn align_up(self, alignment: Self::Alignment) -> Result<Self, AlignmentError>;
    fn align_up_unchecked(self, alignment: Self::Alignment) -> Self;
    fn align_down(self, alignment: Self::Alignment) -> Result<Self, AlignmentError>;
    fn align_down_unchecked(self, alignment: Self::Alignment) -> Self;
    fn check_alignment(self, alignment: Self::Alignment) -> bool;
}

macro_rules! implement_for_int {
    ($type:ty) => {
      impl $crate::Align for $type {
        type Alignment = Self;

        fn align_up(self, alignment: Self) -> Result<Self, $crate::AlignmentError> {
            if alignment.is_power_of_two() {
                Ok(self.align_up_unchecked(alignment))
            } else {
                Err($crate::AlignmentError)
            }
        }
        fn align_up_unchecked(self, alignment: Self) -> Self {
            let address = self + alignment -1;
            address.align_down_unchecked(alignment)
        }
        fn align_down(self, alignment: Self) -> Result<Self, $crate::AlignmentError> {
            if Self::is_power_of_two(alignment) {
                Ok(self.align_down_unchecked(alignment))
            } else {
                Err($crate::AlignmentError)
            }
        }
        fn align_down_unchecked(self, alignment: Self) -> Self {
            self & !(alignment - 1)
        }
        fn check_alignment(self, alignment: Self) -> bool {
            self % alignment == 0
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

impl<T> Align for *mut T {
    type Alignment = usize;

    fn align_up(self, alignment: Self::Alignment) -> Result<Self, AlignmentError> {
        Ok((self as usize).align_up(alignment)? as *mut T)
    }

    fn align_up_unchecked(self, alignment: Self::Alignment) -> Self {
        (self as usize).align_up_unchecked(alignment) as *mut T
    }

    fn align_down(self, alignment: Self::Alignment) -> Result<Self, AlignmentError> {
        Ok((self as usize).align_down(alignment)? as *mut T)
    }

    fn align_down_unchecked(self, alignment: Self::Alignment) -> Self {
        (self as usize).align_down_unchecked(alignment) as *mut T
    }

    fn check_alignment(self, alignment: Self::Alignment) -> bool {
        (self as usize).check_alignment(alignment)
    }
}

impl<T> Align for *const T {
    type Alignment = usize;

    fn align_up(self, alignment: Self::Alignment) -> Result<Self, AlignmentError> {
        Ok((self as usize).align_up(alignment)? as *const T)
    }

    fn align_up_unchecked(self, alignment: Self::Alignment) -> Self {
        (self as usize).align_up_unchecked(alignment) as *const T
    }

    fn align_down(self, alignment: Self::Alignment) -> Result<Self, AlignmentError> {
        Ok((self as usize).align_down(alignment)? as *const T)
    }

    fn align_down_unchecked(self, alignment: Self::Alignment) -> Self {
        (self as usize).align_down_unchecked(alignment) as *const T
    }

    fn check_alignment(self, alignment: Self::Alignment) -> bool {
        (self as usize).check_alignment(alignment)
    }
}