// *************************************************************************
// program.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use super::error::ElfError;
use super::identity::{ ElfClass, ElfData };
use io::{ BinaryReader, Cursor, Endian };
use core::convert::TryFrom;

c_enum!(
    pub enum ElfProgramSegmentType : u32 {
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
pub struct ElfProgramHeader {
    segment_type : ElfProgramSegmentType,
    flags : u32,
    offset : u64,
    virtual_address : u64,
    physical_address : u64,
    file_size : u64,
    memory_size : u64,
    alignment : u64
}

impl ElfProgramHeader {
    pub fn read(source : &[u8], class : ElfClass, data : ElfData) -> Result<Self, ElfError> {
        let endian = Endian::try_from(data)?;
        let mut cursor = Cursor::new(source);

        match class {
            ElfClass::SIXTY_FOUR => Ok(ElfProgramHeader {
                segment_type : ElfProgramSegmentType::new(cursor.read_u32(endian)?),
                flags : cursor.read_u32(endian)?,
                offset :  cursor.read_u64(endian)?,
                virtual_address : cursor.read_u64(endian)?,
                physical_address : cursor.read_u64(endian)?,
                file_size : cursor.read_u64(endian)?,
                memory_size : cursor.read_u64(endian)?,
                alignment : cursor.read_u64(endian)?
                }),
            ElfClass::THIRTY_TWO => Ok(ElfProgramHeader {
                segment_type : ElfProgramSegmentType::new(cursor.read_u32(endian)?),
                offset :  cursor.read_u32(endian)? as u64,
                virtual_address : cursor.read_u32(endian)? as u64,
                physical_address : cursor.read_u32(endian)? as u64,
                file_size : cursor.read_u32(endian)? as u64,
                memory_size : cursor.read_u32(endian)? as u64,
                flags : cursor.read_u32(endian)?,
                alignment : cursor.read_u32(endian)? as u64
            }),
            _ => Err(ElfError::UnknownClass)
        }
    }

    pub fn segment_type(&self) -> ElfProgramSegmentType {
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