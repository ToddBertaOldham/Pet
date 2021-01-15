//**************************************************************************************************
// lib.rs                                                                                          *
// Copyright (c) 2019-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#![no_std]

use core::fmt;

#[macro_export]
macro_rules! c_enum {
    (
        $(#[$attribute:meta])*
        $visibility:vis enum $name:ident : $type:ty {
            $(
                $value_name:ident = $value:expr;
            )+
        }
    ) => {
        $(#[$attribute])*
        #[derive(PartialEq, Eq, Copy, Clone)]
        #[repr(transparent)]
        $visibility struct $name($type);

        impl $name {
            $(
                pub const $value_name : $name = $name($value);
            )+
        }

        impl core::convert::From<$type> for $name {
            fn from(value : $type) -> Self {
                Self(value)
            }
        }

        impl core::convert::From<$name> for $type {
            fn from(value : $name) -> Self {
                value.0
            }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match *self {
                    $(
                        $name::$value_name => write!(f, stringify!($vname)),
                    )+
                    _ => self.0.fmt(f)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! numeric_enum {
    (
        $(#[$attribute:meta])*
        $visibility:vis enum $name:ident {
            $(
                $value_name:ident = $value:expr,
            )+
        }

        impl TryFrom<$type:ty>;
    ) => {
        $(#[$attribute])*
        $visibility enum $name {
            $(
                $value_name = $value,
            )+
        }

        impl core::convert::TryFrom<$type> for $name {
            type Error = $crate::EnumIntegerConvertError;

            fn try_from(value: $type) -> Result<Self, Self::Error> {
                match value {
                    $(
                        $value => Ok($name::$value_name),
                    )+
                    _ => Err($crate::EnumIntegerConvertError),
                }
            }
        }
    };
}

#[derive(Copy, Clone, Debug)]
pub struct EnumIntegerConvertError;

impl fmt::Display for EnumIntegerConvertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Integer is out of range of acceptable values for the enum.")
    }
}