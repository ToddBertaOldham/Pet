//**************************************************************************************************
// information.rs                                                                                  *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(transparent)]
pub struct Information(u128);

impl Information {
    pub const BITS_IN_BYTE: u128 = 8;

    pub const BITS_IN_KIBIBYTE: u128 = Self::BITS_IN_BYTE * 1024;
    pub const BITS_IN_MEBIBYTE: u128 = Self::BITS_IN_KIBIBYTE * 1024;
    pub const BITS_IN_GIBIBYTE: u128 = Self::BITS_IN_MEBIBYTE * 1024;
    pub const BITS_IN_TEBIBYTE: u128 = Self::BITS_IN_GIBIBYTE * 1024;
    pub const BITS_IN_PEBIBYTE: u128 = Self::BITS_IN_TEBIBYTE * 1024;
    pub const BITS_IN_EXBIBYTE: u128 = Self::BITS_IN_PEBIBYTE * 1024;
    pub const BITS_IN_ZEBIBYTE: u128 = Self::BITS_IN_EXBIBYTE * 1024;
    pub const BITS_IN_YOBIBYTE: u128 = Self::BITS_IN_ZEBIBYTE * 1024;

    pub const BITS_IN_KILOBYTE: u128 = Self::BITS_IN_BYTE * 1000;
    pub const BITS_IN_MEGABYTE: u128 = Self::BITS_IN_KILOBYTE * 1000;
    pub const BITS_IN_GIGABYTE: u128 = Self::BITS_IN_MEGABYTE * 1000;
    pub const BITS_IN_TERABYTE: u128 = Self::BITS_IN_GIGABYTE * 1000;
    pub const BITS_IN_PETABYTE: u128 = Self::BITS_IN_TERABYTE * 1000;
    pub const BITS_IN_EXABYTE: u128 = Self::BITS_IN_PETABYTE * 1000;
    pub const BITS_IN_ZETTABYTE: u128 = Self::BITS_IN_EXABYTE * 1000;
    pub const BITS_IN_YOTTABYTE: u128 = Self::BITS_IN_ZETTABYTE * 1000;

    pub const fn new_binary_bytes(
        yobibyte: u128,
        zebibyte: u128,
        exbibyte: u128,
        pebibyte: u128,
        tebibyte: u128,
        gibibyte: u128,
        mebibyte: u128,
        kibibyte: u128,
        bytes: u128,
        bits: u128,
    ) -> Self {
        Self(
            bits + (bytes * Self::BITS_IN_BYTE)
                + (kibibyte * Self::BITS_IN_KIBIBYTE)
                + (mebibyte * Self::BITS_IN_MEBIBYTE)
                + (gibibyte * Self::BITS_IN_GIBIBYTE)
                + (tebibyte * Self::BITS_IN_TEBIBYTE)
                + (pebibyte * Self::BITS_IN_PEBIBYTE)
                + (exbibyte * Self::BITS_IN_EXBIBYTE)
                + (zebibyte * Self::BITS_IN_ZEBIBYTE)
                + (yobibyte * Self::BITS_IN_YOBIBYTE),
        )
    }

    pub const fn from_bits(value: u128) -> Self {
        Self(value)
    }

    pub const fn from_bytes(value: u128) -> Self {
        Self(value * Self::BITS_IN_BYTE)
    }

    pub const fn from_tebibyte(value: u128) -> Self {
        Self(value * Self::BITS_IN_TEBIBYTE)
    }

    pub const fn from_pebibyte(value: u128) -> Self {
        Self(value * Self::BITS_IN_PEBIBYTE)
    }

    pub const fn bits(self) -> u128 {
        self.0
    }

    pub const fn bytes(self) -> u128 {
        self.0 / Self::BITS_IN_BYTE
    }
}
