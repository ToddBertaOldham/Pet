//**************************************************************************************************
// flags_macro.rs                                                                                  *
// Copyright (c) 2020 Aurora Berta-Oldham                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#[macro_export]
macro_rules! flags {
    (
        $(#[$attribute:meta])*
        $visibility:vis struct $name:ident : $type:ty {
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
                pub const $vname : $name = $name($value);
            )+

            pub const fn empty() -> Self {
                Self(0)
            }

            pub fn is_empty(&self) -> bool {
                self.0 == 0
            }

            pub fn contains(&self, value : $name) -> bool {
                self.0 & value.0 != 0
            }

            pub fn add(&mut self, value : $name) {
                self.0 |= value.0;
            }

            pub fn remove(&mut self, value : $name) {
                self.0 &= !value.0
            }
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

        impl core::ops::BitAnd for $name {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self::Output {
                Self(self.0 & rhs.0)
            }
        }

        impl core::ops::BitAndAssign for $name {
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0;
            }
        }

        impl core::ops::BitOr for $name {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }

        impl core::ops::BitOrAssign for $name {
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0;
            }
        }

        impl core::ops::Not for $name {
            type Output = Self;

            fn not(self) -> Self::Output {
                $name(!self.0)
            }
        }

        impl core::ops::BitXor for $name {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self::Output {
                Self(self.0 ^ rhs.0)
            }
        }

        impl core::ops::BitXorAssign for $name {
            fn bitxor_assign(&mut self, rhs: Self) {
                self.0 ^= rhs.0;
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
                if self.is_empty() {
                    write!(f, "empty")
                }
                else {
                    let mut add_divider = false;
                    $(
                        if self.contains($name::$vname) {
                            if add_divider {
                                write!(f, " | ")?;
                            }

                            write!(f, stringify!($vname))?;
                            add_divider = true;
                        }
                    )+
                    Ok(())
                }
            }
        }
    };
}
