// *************************************************************************
// program.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use super::error::Error;
use super::identity::{ Class, Data };
use io::{EndianRead, Endian };
use io::cursor::Cursor;
use encapsulation::GetterSetters;
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

#[derive(Clone, Debug, GetterSetters)]
pub struct ProgramHeader {
    #[field_access]
    segment_type : ProgramSegmentType,
    
    #[field_access]
    flags : u32,

    #[field_access]
    offset : u64,

    #[field_access]
    virtual_address : u64,
    
    #[field_access]
    physical_address : u64,

    #[field_access]
    file_size : u64,

    #[field_access]
    memory_size : u64,
    
    #[field_access]
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
}