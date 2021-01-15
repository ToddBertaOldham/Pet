//**************************************************************************************************
// endian.rs                                                                                       *
// Copyright (c) 2019-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Endian {
    Little,
    Big,
}

impl Endian {
    #[cfg(target_endian = "little")]
    pub const CURRENT: Endian = Endian::Little;

    #[cfg(target_endian = "big")]
    pub const CURRENT: Endian = Endian::Big;

    pub fn u16_from_bytes(self, bytes: [u8; 2]) -> u16 {
        match self {
            Endian::Big => u16::from_be_bytes(bytes),
            Endian::Little => u16::from_le_bytes(bytes),
        }
    }
    pub fn u32_from_bytes(self, bytes: [u8; 4]) -> u32 {
        match self {
            Endian::Big => u32::from_be_bytes(bytes),
            Endian::Little => u32::from_le_bytes(bytes),
        }
    }
    pub fn u64_from_bytes(self, bytes: [u8; 8]) -> u64 {
        match self {
            Endian::Big => u64::from_be_bytes(bytes),
            Endian::Little => u64::from_le_bytes(bytes),
        }
    }
    pub fn u128_from_bytes(self, bytes: [u8; 16]) -> u128 {
        match self {
            Endian::Big => u128::from_be_bytes(bytes),
            Endian::Little => u128::from_le_bytes(bytes),
        }
    }

    pub fn i16_from_bytes(self, bytes: [u8; 2]) -> i16 {
        match self {
            Endian::Big => i16::from_be_bytes(bytes),
            Endian::Little => i16::from_le_bytes(bytes),
        }
    }
    pub fn i32_from_bytes(self, bytes: [u8; 4]) -> i32 {
        match self {
            Endian::Big => i32::from_be_bytes(bytes),
            Endian::Little => i32::from_le_bytes(bytes),
        }
    }
    pub fn i64_from_bytes(self, bytes: [u8; 8]) -> i64 {
        match self {
            Endian::Big => i64::from_be_bytes(bytes),
            Endian::Little => i64::from_le_bytes(bytes),
        }
    }
    pub fn i128_from_bytes(self, bytes: [u8; 16]) -> i128 {
        match self {
            Endian::Big => i128::from_be_bytes(bytes),
            Endian::Little => i128::from_le_bytes(bytes),
        }
    }

    pub fn f32_from_bytes(self, bytes: [u8; 4]) -> f32 {
        let bits = self.u32_from_bytes(bytes);
        f32::from_bits(bits)
    }
    pub fn f64_from_bytes(self, bytes: [u8; 8]) -> f64 {
        let bits = self.u64_from_bytes(bytes);
        f64::from_bits(bits)
    }

    pub fn bytes_from_u16(self, value: u16) -> [u8; 2] {
        match self {
            Endian::Big => value.to_be_bytes(),
            Endian::Little => value.to_le_bytes(),
        }
    }
    pub fn bytes_from_u32(self, value: u32) -> [u8; 4] {
        match self {
            Endian::Big => value.to_be_bytes(),
            Endian::Little => value.to_le_bytes(),
        }
    }
    pub fn bytes_from_u64(self, value: u64) -> [u8; 8] {
        match self {
            Endian::Big => value.to_be_bytes(),
            Endian::Little => value.to_le_bytes(),
        }
    }
    pub fn bytes_from_u128(self, value: u128) -> [u8; 16] {
        match self {
            Endian::Big => value.to_be_bytes(),
            Endian::Little => value.to_le_bytes(),
        }
    }

    pub fn bytes_from_i16(self, value: i16) -> [u8; 2] {
        match self {
            Endian::Big => value.to_be_bytes(),
            Endian::Little => value.to_le_bytes(),
        }
    }
    pub fn bytes_from_i32(self, value: i32) -> [u8; 4] {
        match self {
            Endian::Big => value.to_be_bytes(),
            Endian::Little => value.to_le_bytes(),
        }
    }
    pub fn bytes_from_i64(self, value: i64) -> [u8; 8] {
        match self {
            Endian::Big => value.to_be_bytes(),
            Endian::Little => value.to_le_bytes(),
        }
    }
    pub fn bytes_from_i128(self, value: i128) -> [u8; 16] {
        match self {
            Endian::Big => value.to_be_bytes(),
            Endian::Little => value.to_le_bytes(),
        }
    }

    pub fn bytes_from_f32(self, value: f32) -> [u8; 4] {
        let bits = value.to_bits();
        self.bytes_from_u32(bits)
    }
    pub fn bytes_from_f64(self, value: f64) -> [u8; 8] {
        let bits = value.to_bits();
        self.bytes_from_u64(bits)
    }
}
