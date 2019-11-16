//**************************************************************************************************
// write.rs                                                                                        *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use memory::Endian;
#[cfg(not(feature = "no-std"))]
use std::io::{Error, Write};

//TODO Consider switching to const generics for endian once available.
// https://github.com/rust-lang/rust/issues/44580

pub trait Write {
    type Error;

    fn write(&mut self, buffer : &mut [u8]) -> Result<(), Self::Error>;
}

pub trait EndianWrite {
    type Error;

    fn write_u8(&mut self, value : u8) -> Result<(), Self::Error>;
    fn write_u16(&mut self, value : u16, endian : Endian) -> Result<(), Self::Error>;
    fn write_u32(&mut self, value : u32, endian : Endian) -> Result<(), Self::Error>;
    fn write_u64(&mut self, value : u64, endian : Endian) -> Result<(), Self::Error>;
    fn write_u128(&mut self, value : u128, endian : Endian) -> Result<(), Self::Error>;

    fn write_i8(&mut self, value : i8) -> Result<(), Self::Error>;
    fn write_i16(&mut self, value : i16, endian : Endian) -> Result<(), Self::Error>;
    fn write_i32(&mut self, value : i32, endian : Endian) -> Result<(), Self::Error>;
    fn write_i64(&mut self, value : i64, endian : Endian) -> Result<(), Self::Error>;
    fn write_i128(&mut self, value : i128, endian : Endian) -> Result<(), Self::Error>;

    fn write_f32(&mut self, value : f32, endian : Endian) -> Result<(), Self::Error>;
    fn write_f64(&mut self, value : f64, endian : Endian) -> Result<(), Self::Error>;
}

impl<T: Write> EndianWrite for T {
    #[cfg(feature = "no-std")]
    type Error = T::Error;

    #[cfg(not(feature = "no-std"))]
    type Error = Error;

    fn write_u8(&mut self, value : u8) -> Result<(), Self::Error> {
        let mut buffer = [ value ];
        self.write(&mut buffer)
    }
    fn write_u16(&mut self, value : u16, endian : Endian) -> Result<(), Self::Error> {
        let mut buffer = endian.bytes_from_u16(value);
        self.write(&mut buffer)
    }
    fn write_u32(&mut self, value : u32, endian : Endian) -> Result<(), Self::Error> {
        let mut buffer = endian.bytes_from_u32(value);
        self.write(&mut buffer)
    }
    fn write_u64(&mut self, value : u64, endian : Endian) -> Result<(), Self::Error> {
        let mut buffer = endian.bytes_from_u64(value);
        self.write(&mut buffer)
    }
    fn write_u128(&mut self, value : u128, endian : Endian) -> Result<(), Self::Error> {
        let mut buffer = endian.bytes_from_u128(value);
        self.write(&mut buffer)
    }

    fn write_i8(&mut self, value : i8) -> Result<(), Self::Error> {
        self.write_u8(value as u8)
    }
    fn write_i16(&mut self, value : i16, endian : Endian) -> Result<(), Self::Error> {
        let mut buffer = endian.bytes_from_i16(value);
        self.write(&mut buffer)
    }
    fn write_i32(&mut self, value : i32, endian : Endian) -> Result<(), Self::Error> {
        let mut buffer = endian.bytes_from_i32(value);
        self.write(&mut buffer)
    }
    fn write_i64(&mut self, value : i64, endian : Endian) -> Result<(), Self::Error> {
        let mut buffer = endian.bytes_from_i64(value);
        self.write(&mut buffer)
    }
    fn write_i128(&mut self, value : i128, endian : Endian) -> Result<(), Self::Error> {
        let mut buffer = endian.bytes_from_i128(value);
        self.write(&mut buffer)
    }

    fn write_f32(&mut self, value : f32, endian : Endian) -> Result<(), Self::Error> {
        let mut buffer = endian.bytes_from_f32(value);
        self.write(&mut buffer)
    }
    fn write_f64(&mut self, value : f64, endian : Endian) -> Result<(), Self::Error> {
        let mut buffer = endian.bytes_from_f64(value);
        self.write(&mut buffer)
    }
}