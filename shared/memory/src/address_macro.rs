//**************************************************************************************************
// address_macro.rs                                                                                *
// Copyright (c) 2020 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#[macro_export]
macro_rules! address {
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

        impl $crate::Align for $name {
            type Alignment = $type;

            fn align_up(self, alignment: Self::Alignment) -> Result<Self, $crate::AlignmentError> {
                if alignment.is_power_of_two() {
                    Ok(Self(self.0.align_up_unchecked(alignment)))
                } else {
                    Err($crate::AlignmentError)
                }
            }
            fn align_up_unchecked(self, alignment: Self::Alignment) -> Self {
                Self(self.0.align_up_unchecked(alignment))
            }
            fn align_down(self, alignment: Self::Alignment) -> Result<Self, $crate::AlignmentError> {
                if alignment.is_power_of_two() {
                    Ok(Self(self.0.align_down_unchecked(alignment)))
                } else {
                    Err($crate::AlignmentError)
                }
            }
            fn align_down_unchecked(self, alignment: Self::Alignment) -> Self {
                Self(self.0.align_down_unchecked(alignment))
            }
            fn check_alignment(self, alignment: Self::Alignment) -> bool {
                self.0.check_alignment(alignment)
            }
        }

        impl core::convert::From<$name> for $type {
            fn from(value : $name) -> Self {
                value.0
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