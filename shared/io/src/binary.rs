// *************************************************************************
// binary.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use super::endian::Endian;

//TODO Consider switching to const generics for endian once available. https://github.com/rust-lang/rust/issues/44580

pub trait BinaryReader {
	type Error;

	fn read_exact(&mut self, buffer : &mut [u8]) -> Result<(), Self::Error>;

    fn read_u8(&mut self) -> Result<u8, Self::Error> {
        let mut bytes = [0; 1];
		self.read_exact(&mut bytes)?;
		Ok(bytes[0])
    }
	fn read_u16(&mut self, endian : Endian) -> Result<u16, Self::Error> {
		let mut bytes = [0; 2];
		self.read_exact(&mut bytes)?;
        Ok(endian.u16_from_bytes(bytes))
    }
	fn read_u32(&mut self, endian : Endian) -> Result<u32, Self::Error> {
		let mut bytes = [0; 4];
		self.read_exact(&mut bytes)?;
        Ok(endian.u32_from_bytes(bytes))
	}
	fn read_u64(&mut self, endian : Endian) -> Result<u64, Self::Error> {
        let mut bytes = [0; 8];
		self.read_exact(&mut bytes)?;
        Ok(endian.u64_from_bytes(bytes))
    }
	fn read_u128(&mut self, endian : Endian) -> Result<u128, Self::Error> {
        let mut bytes = [0; 16];
		self.read_exact(&mut bytes)?;
        Ok(endian.u128_from_bytes(bytes))
    }
	
	fn read_i8(&mut self) -> Result<i8, Self::Error> {
        Ok(self.read_u8()? as i8)
    }
	fn read_i16(&mut self, endian : Endian) -> Result<i16, Self::Error> {
		let mut bytes = [0; 2];
		self.read_exact(&mut bytes)?;
        Ok(endian.i16_from_bytes(bytes))
    }
	fn read_i32(&mut self, endian : Endian) -> Result<i32, Self::Error> {
		let mut bytes = [0; 4];
		self.read_exact(&mut bytes)?;
        Ok(endian.i32_from_bytes(bytes))
    }
	fn read_i64(&mut self, endian : Endian) -> Result<i64, Self::Error> {
        let mut bytes = [0; 8];
		self.read_exact(&mut bytes)?;
        Ok(endian.i64_from_bytes(bytes))
    }
	fn read_i128(&mut self, endian : Endian) -> Result<i128, Self::Error> {
        let mut bytes = [0; 16];
		self.read_exact(&mut bytes)?;
        Ok(endian.i128_from_bytes(bytes))
    }

    fn read_f32(&mut self, endian : Endian) -> Result<f32, Self::Error> {
		let mut bytes = [0; 4];
		self.read_exact(&mut bytes)?;
        Ok(endian.f32_from_bytes(bytes))        
    }
    fn read_f64(&mut self, endian : Endian) -> Result<f64, Self::Error> {
		let mut bytes = [0; 8];
		self.read_exact(&mut bytes)?;
        Ok(endian.f64_from_bytes(bytes))        
    }
}

pub trait BinaryWriter {
    type Error;

    fn write(&mut self, buffer : &mut [u8]) -> Result<(), Self::Error>;

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