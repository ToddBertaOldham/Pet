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
                $vname:ident = $value:expr;
            )+
        }
    ) => {
        $(#[$attribute])*
        #[derive(PartialEq, Eq, Copy, Clone)]
        #[repr(transparent)]
        $visibility struct $name($type);

        impl $name {
            $(
                pub const $vname : $name = $name::new($value);
            )+

            pub const fn new(value : $type) -> Self {
                $name(value)
            }

            pub fn value(&self) -> $type {
                self.0
            }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match *self {
                    $(
                        $name::$vname => write!(f, "$vname"),
                    )+
                    _ => self.0.fmt(f)
                }
            }
        }
    };
}