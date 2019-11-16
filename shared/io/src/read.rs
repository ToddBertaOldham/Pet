//**************************************************************************************************
// read.rs                                                                                         *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use memory::Endian;
#[cfg(not(feature = "no-std"))]
use std::io::{Error, Read};

//TODO Consider switching to const generics for endian once available.
// https://github.com/rust-lang/rust/issues/44580

#[cfg(feature = "no-std")]
pub trait Read {
    type Error;

    fn read_exact(&mut self, buffer: &mut [u8]) -> Result<(), Self::Error>;
}

pub trait EndianRead {
    type Error;

    fn read_u8(&mut self) -> Result<u8, Self::Error>;
    fn read_u16(&mut self, endian: Endian) -> Result<u16, Self::Error>;
    fn read_u32(&mut self, endian: Endian) -> Result<u32, Self::Error>;
    fn read_u64(&mut self, endian: Endian) -> Result<u64, Self::Error>;
    fn read_u128(&mut self, endian: Endian) -> Result<u128, Self::Error>;

    fn read_i8(&mut self) -> Result<i8, Self::Error>;
    fn read_i16(&mut self, endian: Endian) -> Result<i16, Self::Error>;
    fn read_i32(&mut self, endian: Endian) -> Result<i32, Self::Error>;
    fn read_i64(&mut self, endian: Endian) -> Result<i64, Self::Error>;
    fn read_i128(&mut self, endian: Endian) -> Result<i128, Self::Error>;

    fn read_f32(&mut self, endian: Endian) -> Result<f32, Self::Error>;
    fn read_f64(&mut self, endian: Endian) -> Result<f64, Self::Error>;
}

impl<T: Read> EndianRead for T {
    #[cfg(feature = "no-std")]
    type Error = T::Error;

    #[cfg(not(feature = "no-std"))]
    type Error = Error;

    fn read_u8(&mut self) -> Result<u8, Self::Error> {
        let mut bytes = [0; 1];
        self.read_exact(&mut bytes)?;
        Ok(bytes[0])
    }
    fn read_u16(&mut self, endian: Endian) -> Result<u16, Self::Error> {
        let mut bytes = [0; 2];
        self.read_exact(&mut bytes)?;
        Ok(endian.u16_from_bytes(bytes))
    }
    fn read_u32(&mut self, endian: Endian) -> Result<u32, Self::Error> {
        let mut bytes = [0; 4];
        self.read_exact(&mut bytes)?;
        Ok(endian.u32_from_bytes(bytes))
    }
    fn read_u64(&mut self, endian: Endian) -> Result<u64, Self::Error> {
        let mut bytes = [0; 8];
        self.read_exact(&mut bytes)?;
        Ok(endian.u64_from_bytes(bytes))
    }
    fn read_u128(&mut self, endian: Endian) -> Result<u128, Self::Error> {
        let mut bytes = [0; 16];
        self.read_exact(&mut bytes)?;
        Ok(endian.u128_from_bytes(bytes))
    }

    fn read_i8(&mut self) -> Result<i8, Self::Error> {
        Ok(self.read_u8()? as i8)
    }
    fn read_i16(&mut self, endian: Endian) -> Result<i16, Self::Error> {
        let mut bytes = [0; 2];
        self.read_exact(&mut bytes)?;
        Ok(endian.i16_from_bytes(bytes))
    }
    fn read_i32(&mut self, endian: Endian) -> Result<i32, Self::Error> {
        let mut bytes = [0; 4];
        self.read_exact(&mut bytes)?;
        Ok(endian.i32_from_bytes(bytes))
    }
    fn read_i64(&mut self, endian: Endian) -> Result<i64, Self::Error> {
        let mut bytes = [0; 8];
        self.read_exact(&mut bytes)?;
        Ok(endian.i64_from_bytes(bytes))
    }
    fn read_i128(&mut self, endian: Endian) -> Result<i128, Self::Error> {
        let mut bytes = [0; 16];
        self.read_exact(&mut bytes)?;
        Ok(endian.i128_from_bytes(bytes))
    }

    fn read_f32(&mut self, endian: Endian) -> Result<f32, Self::Error> {
        let mut bytes = [0; 4];
        self.read_exact(&mut bytes)?;
        Ok(endian.f32_from_bytes(bytes))
    }
    fn read_f64(&mut self, endian: Endian) -> Result<f64, Self::Error> {
        let mut bytes = [0; 8];
        self.read_exact(&mut bytes)?;
        Ok(endian.f64_from_bytes(bytes))
    }
}
