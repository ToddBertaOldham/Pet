//**************************************************************************************************
// section.rs                                                                                      *
// Copyright (c) 2019-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use super::error::Error;
use super::identity::{Class, Data};
use core::convert::TryFrom;
use io::cursor::Cursor;
use io::{Endian, EndianRead};

c_enum!(
    pub enum SectionSegmentType : u32 {
        NULL = 0,
        PROGRAM_BITS = 1,
        SYMBOL_TABLE = 2,
        STRING_TABLE = 3,
        RELOCATIONS_WITH_ADDENDS = 4,
        HASH_TABLE = 5,
        DYNAMIC = 6,
        NOTE = 7,
        NO_BITS = 8,
        RELOCATIONS = 9,
        RESERVED = 10,
    }
);

#[derive(Clone, Debug)]
pub struct SectionHeader {
    pub name: u32,
    pub segment_type: SectionSegmentType,
    pub flags: u64,
    pub address: u64,
    pub offset: u64,
    pub size: u64,
    pub link: u32,
    pub info: u32,
    pub address_align: u64,
    pub entry_size: u64,
}

impl SectionHeader {
    pub fn read(source: &[u8], class: Class, data: Data) -> Result<Self, Error> {
        let endian = Endian::try_from(data)?;
        let mut cursor = Cursor::new(source);

        match class {
            Class::SIXTY_FOUR => Ok(SectionHeader {
                name: cursor.read_u32(endian)?,
                segment_type: SectionSegmentType::from(cursor.read_u32(endian)?),
                flags: cursor.read_u64(endian)?,
                address: cursor.read_u64(endian)?,
                offset: cursor.read_u64(endian)?,
                size: cursor.read_u64(endian)?,
                link: cursor.read_u32(endian)?,
                info: cursor.read_u32(endian)?,
                address_align: cursor.read_u64(endian)?,
                entry_size: cursor.read_u64(endian)?,
            }),
            Class::THIRTY_TWO => Ok(SectionHeader {
                name: cursor.read_u32(endian)?,
                segment_type: SectionSegmentType::from(cursor.read_u32(endian)?),
                flags: cursor.read_u32(endian)? as u64,
                address: cursor.read_u32(endian)? as u64,
                offset: cursor.read_u32(endian)? as u64,
                size: cursor.read_u32(endian)? as u64,
                link: cursor.read_u32(endian)?,
                info: cursor.read_u32(endian)?,
                address_align: cursor.read_u32(endian)? as u64,
                entry_size: cursor.read_u32(endian)? as u64,
            }),
            _ => Err(Error::UnknownClass),
        }
    }
}
