// *************************************************************************
// lib.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

#![no_std]

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