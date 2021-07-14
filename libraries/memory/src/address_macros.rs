//**************************************************************************************************
// address_macro.rs                                                                                *
// Copyright (c) 2020-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

//TODO Quite a bit of this needs to be reworked.

#[macro_export]
macro_rules! address_wrapper {
    (
        $(#[$attribute:meta])*
        $visibility:vis struct $name:ident : $type:ty
    ) => {
        $(#[$attribute])*
        #[derive(PartialEq, Eq, Copy, Clone)]
        #[repr(transparent)]
        $visibility struct $name($type);

        impl $name {
            pub const fn null() -> Self {
                Self(0)
            }

            pub fn is_null(self) -> bool {
                self.0 == 0
            }

            pub fn as_ptr<T>(self) -> *const T {
                self.0 as *const T
            }

            pub fn as_mut_ptr<T>(self) -> *mut T {
                self.0 as *mut T
            }
        }

        impl $crate::CheckAlignment<$type> for $name {
            fn check_alignment(self, alignment: $type) -> bool {
                self.0.check_alignment(alignment)
            }
        }

        impl core::convert::From<$name> for $type {
            fn from(value : $name) -> Self {
                value.0
            }
        }

        impl core::convert::TryFrom<$name> for usize {
            type Error = core::num::TryFromIntError;

            fn try_from(value: $name) -> Result<Self, Self::Error> {
                core::convert::TryFrom::try_from(value.0)
            }
        }

        impl<T> core::convert::TryFrom<$name> for *const T {
            type Error = core::num::TryFromIntError;

            fn try_from(value: $name) -> Result<Self, Self::Error> {
                let converted_value: usize = core::convert::TryFrom::try_from(value)?;
                Ok(converted_value as *const T)
            }
        }

        impl<T> core::convert::TryFrom<$name> for *mut T {
            type Error = core::num::TryFromIntError;

            fn try_from(value: $name) -> Result<Self, Self::Error> {
                let converted_value: usize = core::convert::TryFrom::try_from(value)?;
                Ok(converted_value as *mut T)
            }
        }

        impl core::fmt::Binary for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
                core::fmt::Binary::fmt(&self.0, f)
            }
        }

        impl core::fmt::LowerHex for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
                core::fmt::LowerHex::fmt(&self.0, f)
            }
        }

        impl core::fmt::UpperHex for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
                core::fmt::UpperHex::fmt(&self.0, f)
            }
        }

        impl core::fmt::Octal for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
                core::fmt::Octal::fmt(&self.0, f)
            }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                core::fmt::Debug::fmt(&self.0, f)
            }
        }
    };
}

#[macro_export]
macro_rules! mut_address_wrapper {
    (
        $(#[$attribute:meta])*
        $visibility:vis struct $name:ident : $type:ty
    ) => {
        address_wrapper!(
            $(#[$attribute])*
            $visibility struct $name : $type
        );

        impl $name {
            pub fn new(value: $type) -> Self {
                Self(value)
            }
        }
    };
}
