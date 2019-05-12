// *************************************************************************
// section.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use super::error::Error;
use super::identity::{ Class, Data };
use io::{ BinaryReader, Endian };
use io::cursor::Cursor;
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

#[derive(Clone, Debug)]
pub struct SectionHeader {
    name : u32,
    segment_type : SectionSegmentType,
    flags : u64,
    address : u64,
    offset : u64,
    size : u64,
    link : u32,
    info : u32,
    address_align : u64,
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

    pub fn name(&self) -> u32 {
        self.name
    }

    pub fn segment_type(&self) -> SectionSegmentType {
        self.segment_type
    }

    pub fn flags(&self) -> u64 {
        self.flags
    }

    pub fn address(&self) -> u64 {
        self.address
    }

    pub fn offset(&self) -> u64 {
        self.offset
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn link(&self) -> u32 {
        self.link
    }

    pub fn info(&self) -> u32 {
        self.info
    }

    pub fn address_align(&self) -> u64 {
        self.address_align
    }

    pub fn entry_size(&self) -> u64 {
        self.entry_size
    }
}