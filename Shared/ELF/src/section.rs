// *************************************************************************
// section.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use super::error::ElfError;
use super::identity::{ ElfClass, ElfData };
use io::{ BinaryReader, Cursor, Endian };
use core::convert::TryFrom;

c_enum!(
    pub enum ElfSectionSegmentType : u32 {
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
pub struct ElfSectionHeader {
    name : u32,
    segment_type : ElfSectionSegmentType,
    flags : u64,
    address : u64,
    offset : u64,
    size : u64,
    link : u32,
    info : u32,
    address_align : u64,
    entry_size : u64
}

impl ElfSectionHeader {
    pub fn read(source : &[u8], class : ElfClass, data : ElfData) -> Result<Self, ElfError> {
        let endian = Endian::try_from(data)?;
        let mut cursor = Cursor::new(source);

        match class {
            ElfClass::SIXTY_FOUR => Ok(ElfSectionHeader {
                name : cursor.read_u32(endian)?,
                segment_type : ElfSectionSegmentType::new(cursor.read_u32(endian)?),
                flags : cursor.read_u64(endian)?,
                address :  cursor.read_u64(endian)?,
                offset : cursor.read_u64(endian)?,
                size : cursor.read_u64(endian)?,
                link : cursor.read_u32(endian)?,
                info : cursor.read_u32(endian)?,
                address_align : cursor.read_u64(endian)?,
                entry_size : cursor.read_u64(endian)?
                }),
            ElfClass::THIRTY_TWO => Ok(ElfSectionHeader {
                name : cursor.read_u32(endian)?,
                segment_type : ElfSectionSegmentType::new(cursor.read_u32(endian)?),
                flags : cursor.read_u32(endian)? as u64,
                address :  cursor.read_u32(endian)? as u64,
                offset : cursor.read_u32(endian)? as u64,
                size : cursor.read_u32(endian)? as u64,
                link : cursor.read_u32(endian)?,
                info : cursor.read_u32(endian)?,
                address_align : cursor.read_u32(endian)? as u64,
                entry_size : cursor.read_u32(endian)? as u64
            }),
            _ => Err(ElfError::UnknownClass)
        }
    }

    pub fn name(&self) -> u32 {
        self.name
    }

    pub fn segment_type(&self) -> ElfSectionSegmentType {
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