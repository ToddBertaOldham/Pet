//**************************************************************************************************
// align.rs                                                                                        *
// Copyright (c) 2019-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use core::fmt;

#[derive(Debug, Copy, Clone)]
pub struct AlignmentError;

impl fmt::Display for AlignmentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str("Invalid alignment.")
    }
}

pub trait Align<Alignment = Self>
where
    Self: Sized,
{
    type Output;
    type Error;

    fn align_up(self, alignment: Alignment) -> Result<Self::Output, Self::Error>;
    fn align_down(self, alignment: Alignment) -> Result<Self::Output, Self::Error>;
}

pub trait AlignAssign<Alignment = Self>
where
    Self: Sized,
{
    type Error;

    fn align_up_assign(&mut self, alignment: Alignment) -> Result<(), Self::Error>;
    fn align_down_assign(&mut self, alignment: Alignment) -> Result<(), Self::Error>;
}

pub trait CheckAlignment<Alignment = Self>
where
    Self: Sized,
{
    fn check_alignment(self, alignment: Alignment) -> bool;
}

macro_rules! implement_for_int {
    ($type:ty) => {
        impl $crate::Align<Self> for $type {
            type Output = Self;
            type Error = AlignmentError;

            fn align_up(self, alignment: Self) -> Result<Self::Output, $crate::AlignmentError> {
                let address = self + alignment - 1;
                address.align_down(alignment)
            }

            fn align_down(self, alignment: Self) -> Result<Self::Output, $crate::AlignmentError> {
                if Self::is_power_of_two(alignment) {
                    Ok(self & !(alignment - 1))
                } else {
                    Err($crate::AlignmentError)
                }
            }
        }

        impl $crate::AlignAssign<Self> for $type {
            type Error = AlignmentError;

            fn align_up_assign(&mut self, alignment: Self) -> Result<(), $crate::AlignmentError> {
                let new_value = self.align_up(alignment);
                match new_value {
                    Ok(value) => {
                        *self = value;
                        Ok(())
                    }
                    Err(error) => Err(error),
                }
            }

            fn align_down_assign(&mut self, alignment: Self) -> Result<(), $crate::AlignmentError> {
                let new_value = self.align_down(alignment);
                match new_value {
                    Ok(value) => {
                        *self = value;
                        Ok(())
                    }
                    Err(error) => Err(error),
                }
            }
        }

        impl $crate::CheckAlignment<Self> for $type {
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

macro_rules! implement_for_ptr {
    ($type:ty) => {
        impl<T> $crate::Align<usize> for $type {
            type Output = Self;
            type Error = AlignmentError;

            fn align_up(self, alignment: usize) -> Result<Self::Output, $crate::AlignmentError> {
                Ok((self as usize).align_up(alignment)? as *mut T)
            }

            fn align_down(self, alignment: usize) -> Result<Self::Output, $crate::AlignmentError> {
                Ok((self as usize).align_down(alignment)? as *mut T)
            }
        }

        impl<T> $crate::AlignAssign<usize> for $type {
            type Error = AlignmentError;

            fn align_up_assign(&mut self, alignment: usize) -> Result<(), $crate::AlignmentError> {
                let new_value = self.align_up(alignment);
                match new_value {
                    Ok(value) => {
                        *self = value;
                        Ok(())
                    }
                    Err(error) => Err(error),
                }
            }

            fn align_down_assign(
                &mut self,
                alignment: usize,
            ) -> Result<(), $crate::AlignmentError> {
                let new_value = self.align_down(alignment);
                match new_value {
                    Ok(value) => {
                        *self = value;
                        Ok(())
                    }
                    Err(error) => Err(error),
                }
            }
        }

        impl<T> $crate::CheckAlignment<usize> for $type {
            fn check_alignment(self, alignment: usize) -> bool {
                (self as usize).check_alignment(alignment)
            }
        }
    };
}

implement_for_ptr!(*mut T);
implement_for_ptr!(*const T);
