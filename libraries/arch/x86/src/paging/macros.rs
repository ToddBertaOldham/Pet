//**************************************************************************************************
// macros.rs                                                                                       *
// Copyright (c) 2019-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

macro_rules! u64_paging_entry {
    (
        $(#[$attribute:meta])*
        $visibility:vis struct $name:ident
    ) => {
        $(#[$attribute])*
        #[derive(PartialEq, Eq, Copy, Clone, Debug)]
        #[repr(transparent)]
        $visibility struct $name(u64);

        impl $name {
            pub fn execute_disabled(self) -> bool {
                self.0.get_bit(63)
            }

            pub fn set_execute_disabled(&mut self, value: bool) {
                self.0.set_bit_assign(63, value);
            }
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
