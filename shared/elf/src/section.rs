// *************************************************************************
// section.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use super::error::Error;
use super::identity::{ Class, Data };
use io::{ BinaryReader, Endian };
use io::cursor::Cursor;
use encapsulation::GetterSetters;
use core::convert::TryFrom;

c_enum!(
    pub enum SectionSegmentType : u32 {
        NULL = 0;
        PROGRAM_BITS = 1;
        SYMBOL_TABLE = 2;
        STRING_TABLE = 3;
        RELOCATIONS_WITH_ADDENDS = 4;
        HASH_TABLE = 5;
        DYNAMIC = 6;
        NOTE = 7;
        NO_BITS = 8;
        RELOCATIONS = 9;
        RESERVED = 10;
    }
);

#[derive(Clone, Debug, GetterSetters)]
pub struct SectionHeader {
    #[field_access]
    name : u32,

    #[field_access]
    segment_type : SectionSegmentType,

    #[field_access]
    flags : u64,

    #[field_access]
    address : u64,

    #[field_access]
    offset : u64,

    #[field_access]
    size : u64,

    #[field_access]
    link : u32,

    #[field_access]
    info : u32,

    #[field_access]
    address_align : u64,

    #[field_access]
    entry_size : u64
}

impl SectionHeader {
    pub fn read(source : &[u8], class : Class, data : Data) -> Result<Self, Error> {
        let endian = Endian::try_from(data)?;
        let mut cursor = Cursor::new(source);

        match class {
            Class::SIXTY_FOUR => Ok(SectionHeader {
                name : cursor.read_u32(endian)?,
                segment_type : SectionSegmentType::from(cursor.read_u32(endian)?),
                flags : cursor.read_u64(endian)?,
                address :  cursor.read_u64(endian)?,
                offset : cursor.read_u64(endian)?,
                size : cursor.read_u64(endian)?,
                link : cursor.read_u32(endian)?,
                info : cursor.read_u32(endian)?,
                address_align : cursor.read_u64(endian)?,
                entry_size : cursor.read_u64(endian)?
                }),
            Class::THIRTY_TWO => Ok(SectionHeader {
                name : cursor.read_u32(endian)?,
                segment_type : SectionSegmentType::from(cursor.read_u32(endian)?),
                flags : cursor.read_u32(endian)? as u64,
                address :  cursor.read_u32(endian)? as u64,
                offset : cursor.read_u32(endian)? as u64,
                size : cursor.read_u32(endian)? as u64,
                link : cursor.read_u32(endian)?,
                info : cursor.read_u32(endian)?,
                address_align : cursor.read_u32(endian)? as u64,
                entry_size : cursor.read_u32(endian)? as u64
            }),
            _ => Err(Error::UnknownClass)
        }
    }
}