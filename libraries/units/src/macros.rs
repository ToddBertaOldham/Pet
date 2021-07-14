//**************************************************************************************************
// macros.rs                                                                                       *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

// The macro generates a trait that both serves as a marker type and provides functionality.
// The trait serving as a marker type is important to make sure Length isn't converted
// into Information. This only works for units that can be converted with a ratio.

macro_rules! unit_type {
    ($name:ident) => {
        pub trait $name<T>: Sized + core::fmt::Display + core::fmt::Debug {
            const BASE_NUMERATOR: u128;
            const BASE_DENOMINATOR: u128;
            const BASE_OFFSET: i128;

            fn new(amount: T) -> Self;

            fn into_inner(self) -> T;

            fn convert<O: $name<T>>(&self) -> O {
                self.checked_convert().expect("Conversion failed.")
            }

            fn checked_convert<O: $name<T>>(&self) -> Result<O, $crate::ConvertError>;
        }
    };
}

// This macro creates an unit struct with a given name and implements the unit type trait with
// the specified ratio relative to the base unit.

//TODO F64 and F32 don't support TryFrom<i128>. See https://github.com/rust-lang/rfcs/pull/2484.

macro_rules! unit {
    (
        $name:ident ($symbol:literal) : $unit_type:ident = $num:expr , $den:expr, $offset:expr
    ) => {
        #[repr(transparent)]
        #[derive(Copy, Clone, PartialEq)]
        pub struct $name<T>(T)
        where
            T: Copy + core::convert::TryFrom<i128> + core::convert::TryFrom<u128> + math::Number;

        //TODO Const impl for unit type. Requires quite a few things first.

        impl<T> $unit_type<T> for $name<T>
        where
            T: Copy + core::convert::TryFrom<i128> + core::convert::TryFrom<u128> + math::Number,
        {
            const BASE_NUMERATOR: u128 = $num;
            const BASE_DENOMINATOR: u128 = $den;
            const BASE_OFFSET: i128 = $offset;

            fn new(amount: T) -> Self {
                Self(amount)
            }

            fn into_inner(self) -> T {
                self.0
            }

            fn checked_convert<O: $unit_type<T>>(&self) -> Result<O, $crate::ConvertError> {
                let mut big_num = Self::BASE_NUMERATOR * O::BASE_DENOMINATOR;
                let mut big_den = Self::BASE_DENOMINATOR * O::BASE_NUMERATOR;

                let gcf = math::Gcf::gcf(big_num, big_den);

                big_num /= gcf;
                big_den /= gcf;

                let self_offset =
                    T::try_from(Self::BASE_OFFSET).map_err(|_| $crate::ConvertError)?;
                let other_offset = T::try_from(O::BASE_OFFSET).map_err(|_| $crate::ConvertError)?;
                let num = T::try_from(big_num).map_err(|_| $crate::ConvertError)?;
                let den = T::try_from(big_den).map_err(|_| $crate::ConvertError)?;

                let value = (((self.0 - self_offset) * num) / den) + other_offset;

                Ok(O::new(value))
            }
        }

        //TODO Cannot have two units of same unit types and a unit and T be added together
        // (assuming this should happen at all). This may require something like negative trait
        // bounds which would also allow for a TryFrom implementation for other unit types.

        // Add imp for unit_type.

        impl<T, U> core::ops::Add<U> for $name<T>
        where
            T: Copy + core::convert::TryFrom<i128> + core::convert::TryFrom<u128> + math::Number,
            U: $unit_type<T>,
        {
            type Output = $name<T>;

            fn add(self, rhs: U) -> Self::Output {
                let value: $name<T> = rhs.convert();
                Self(self.0 + value.into_inner())
            }
        }

        impl<T, U> core::ops::AddAssign<U> for $name<T>
        where
            T: Copy + core::convert::TryFrom<i128> + core::convert::TryFrom<u128> + math::Number,
            U: $unit_type<T>,
        {
            fn add_assign(&mut self, rhs: U) {
                let value: $name<T> = rhs.convert();
                self.0 += value.into_inner();
            }
        }

        // Sub imp for unit_type.

        impl<T, U> core::ops::Sub<U> for $name<T>
        where
            T: Copy + core::convert::TryFrom<i128> + core::convert::TryFrom<u128> + math::Number,
            U: $unit_type<T>,
        {
            type Output = $name<T>;

            fn sub(self, rhs: U) -> Self::Output {
                let value: $name<T> = rhs.convert();
                Self(self.0 - value.into_inner())
            }
        }

        impl<T, U> core::ops::SubAssign<U> for $name<T>
        where
            T: Copy + core::convert::TryFrom<i128> + core::convert::TryFrom<u128> + math::Number,
            U: $unit_type<T>,
        {
            fn sub_assign(&mut self, rhs: U) {
                let value: $name<T> = rhs.convert();
                self.0 -= value.into_inner();
            }
        }

        // Add imp for T.

        // impl<T> core::ops::Add<T> for $name<T>
        // where
        //     T: Copy + core::convert::TryFrom<i128> + core::convert::TryFrom<u128> + math::Number,
        // {
        //     type Output = $name<T>;
        //
        //     fn add(self, rhs: T) -> Self::Output {
        //         Self(self.0 + rhs)
        //     }
        // }
        //
        // impl<T> core::ops::AddAssign<T> for $name<T>
        // where
        //     T: Copy + core::convert::TryFrom<i128> + core::convert::TryFrom<u128> + math::Number,
        // {
        //     fn add_assign(&mut self, rhs: T) {
        //         self.0 += rhs;
        //     }
        // }

        // Sub imp for T.

        // impl<T> core::ops::Sub<T> for $name<T>
        // where
        //     T: Copy + core::convert::TryFrom<i128> + core::convert::TryFrom<u128> + math::Number,
        // {
        //     type Output = $name<T>;
        //
        //     fn sub(self, rhs: T) -> Self::Output {
        //         Self(self.0 - rhs)
        //     }
        // }
        //
        // impl<T> core::ops::SubAssign<T> for $name<T>
        // where
        //     T: Copy + core::convert::TryFrom<i128> + core::convert::TryFrom<u128> + math::Number,
        // {
        //     fn sub_assign(&mut self, rhs: T) {
        //         self.0 -= rhs;
        //     }
        // }

        // Mul imp for T.

        impl<T> core::ops::Mul<T> for $name<T>
        where
            T: Copy + core::convert::TryFrom<i128> + core::convert::TryFrom<u128> + math::Number,
        {
            type Output = $name<T>;

            fn mul(self, rhs: T) -> Self::Output {
                Self(self.0 * rhs)
            }
        }

        impl<T> core::ops::MulAssign<T> for $name<T>
        where
            T: Copy + core::convert::TryFrom<i128> + core::convert::TryFrom<u128> + math::Number,
        {
            fn mul_assign(&mut self, rhs: T) {
                self.0 *= rhs;
            }
        }

        // Div imp for T.

        impl<T> core::ops::Div<T> for $name<T>
        where
            T: Copy + core::convert::TryFrom<i128> + core::convert::TryFrom<u128> + math::Number,
        {
            type Output = $name<T>;

            fn div(self, rhs: T) -> Self::Output {
                Self(self.0 / rhs)
            }
        }

        impl<T> core::ops::DivAssign<T> for $name<T>
        where
            T: Copy + core::convert::TryFrom<i128> + core::convert::TryFrom<u128> + math::Number,
        {
            fn div_assign(&mut self, rhs: T) {
                self.0 /= rhs;
            }
        }

        // Debug and Display impl

        impl<T> core::fmt::Display for $name<T>
        where
            T: Copy + core::convert::TryFrom<i128> + core::convert::TryFrom<u128> + math::Number,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, concat!("{} ", $symbol), self.0)
            }
        }

        impl<T> core::fmt::Debug for $name<T>
        where
            T: Copy + core::convert::TryFrom<i128> + core::convert::TryFrom<u128> + math::Number,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{:?}", self.0)
            }
        }
    };
}
