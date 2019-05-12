// *************************************************************************
// program.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use super::error::Error;
use super::identity::{ Class, Data };
use io::{ BinaryReader, Endian };
use io::cursor::Cursor;
use core::convert::TryFrom;

c_enum!(
    pub enum ProgramSegmentType : u32 {
        NULL = 0;
        LOAD = 1;
        DYNAMIC = 2;
        INTERPRETER = 3;
        NOTE = 4;
        SHLIB = 5;
        PROGRAM_HEADER = 6;
    }
);

#[derive(Clone, Debug)]
pub struct ProgramHeader {
    segment_type : ProgramSegmentType,
    flags : u32,
    offset : u64,
    virtual_address : u64,
    physical_address : u64,
    file_size : u64,
    memory_size : u64,
    alignment : u64
}

impl ProgramHeader {
    pub fn read(source : &[u8], class : Class, data : Data) -> Result<Self, Error> {
        let endian = Endian::try_from(data)?;
        let mut cursor = Cursor::new(source);

        match class {
            Class::SIXTY_FOUR => Ok(ProgramHeader {
                segment_type : ProgramSegmentType::from(cursor.read_u32(endian)?),
                flags : cursor.read_u32(endian)?,
                offset :  cursor.read_u64(endian)?,
                virtual_address : cursor.read_u64(endian)?,
                physical_address : cursor.read_u64(endian)?,
                file_size : cursor.read_u64(endian)?,
                memory_size : cursor.read_u64(endian)?,
                alignment : cursor.read_u64(endian)?
                }),
            Class::THIRTY_TWO => Ok(ProgramHeader {
                segment_type : ProgramSegmentType::from(cursor.read_u32(endian)?),
                offset :  cursor.read_u32(endian)? as u64,
                virtual_address : cursor.read_u32(endian)? as u64,
                physical_address : cursor.read_u32(endian)? as u64,
                file_size : cursor.read_u32(endian)? as u64,
                memory_size : cursor.read_u32(endian)? as u64,
                flags : cursor.read_u32(endian)?,
                alignment : cursor.read_u32(endian)? as u64
            }),
            _ => Err(Error::UnknownClass)
        }
    }

    pub fn segment_type(&self) -> ProgramSegmentType {
        self.segment_type
    }

    pub fn flags(&self) -> u32 {
        self.flags
    }

    pub fn offset(&self) -> u64 {
        self.offset
    }

    pub fn virtual_address(&self) -> u64 {
        self.virtual_address
    }

    pub fn physical_address(&self) -> u64 {
        self.physical_address
    }

    pub fn file_size(&self) -> u64 {
        self.file_size
    }

    pub fn memory_size(&self) -> u64 {
        self.memory_size
    }

    pub fn alignment(&self) -> u64 {
        self.alignment
    }
}