//**************************************************************************************************
// macros.rs                                                                                       *
// Copyright (c) 2019-2020 Todd Berta-Oldham                                                       *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#[macro_export]
macro_rules! basic_u64_paging_entry {
    (
        $(#[$attribute:meta])*
        $visibility:vis struct $name:ident
    ) => {
        $(#[$attribute])*
        #[derive(PartialEq, Eq, Copy, Clone)]
        #[repr(transparent)]
        $visibility struct $name(u64);

        impl $name {

        }

        impl core::convert::From<u64> for $name {
            fn from(value : u64) -> Self {
                Self(value)
            }
        }

        impl core::convert::From<$name> for u64 {
            fn from(value : $name) -> Self {
                value.0
            }
        }
    };
}