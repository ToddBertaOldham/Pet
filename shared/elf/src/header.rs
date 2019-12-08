//**************************************************************************************************
// header.rs                                                                                       *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use super::error::Error;
use super::identity::{Class, Data};
use core::convert::TryFrom;
use io::cursor::Cursor;
use io::{Endian, EndianRead};

c_enum!(
    pub enum ObjectType : u16 {
        NONE = 0;
        RELOCATABLE = 1;
        EXECUTABLE = 2;
        DYNAMIC = 3;
        CORE = 4;
    }
);

c_enum!(
    pub enum Machine : u16 {
        NONE = 0;
        M32 = 1;
        SPARC = 2;
        I386 = 3;
        M68K = 4;
        M88K = 5;
        I860 = 7;
        MIPS = 8;
        POWERPC = 0x14;
        S390 = 0x2A;
        ARM = 0x28;
        SUPERH = 0x2A;
        IA_64 = 0x32;
        X86_64 = 0x3E;
        AARCH = 0xB7;
        RISC_V = 0xF3;
    }
);

c_enum!(
    pub enum Version : u32 {
        NONE = 0;
        CURRENT = 1;
    }
);

#[derive(Clone, Debug)]
pub struct Header {
    pub object_type: ObjectType,
    pub machine: Machine,
    pub version: Version,
    pub entry: u64,
    pub program_header_table_offset: u64,
    pub section_header_table_offset: u64,
    pub flags: u32,
    pub header_size: u16,
    pub program_header_entry_size: u16,
    pub program_header_entry_count: u16,
    pub section_header_entry_size: u16,
    pub section_header_entry_count: u16,
    pub section_header_string_table_index: u16,
}

impl Header {
    pub fn read(source: &[u8], class: Class, data: Data) -> Result<Self, Error> {
        let endian = Endian::try_from(data)?;
        let mut cursor = Cursor::new(source);

        match class {
            Class::SIXTY_FOUR => Ok(Header {
                object_type: ObjectType::from(cursor.read_u16(endian)?),
                machine: Machine::from(cursor.read_u16(endian)?),
                version: Version::from(cursor.read_u32(endian)?),
                entry: cursor.read_u64(endian)?,
                program_header_table_offset: cursor.read_u64(endian)?,
                section_header_table_offset: cursor.read_u64(endian)?,
                flags: cursor.read_u32(endian)?,
                header_size: cursor.read_u16(endian)?,
                program_header_entry_size: cursor.read_u16(endian)?,
                program_header_entry_count: cursor.read_u16(endian)?,
                section_header_entry_size: cursor.read_u16(endian)?,
                section_header_entry_count: cursor.read_u16(endian)?,
                section_header_string_table_index: cursor.read_u16(endian)?,
            }),
            Class::THIRTY_TWO => Ok(Header {
                object_type: ObjectType::from(cursor.read_u16(endian)?),
                machine: Machine::from(cursor.read_u16(endian)?),
                version: Version::from(cursor.read_u32(endian)?),
                entry: cursor.read_u32(endian)? as u64,
                program_header_table_offset: cursor.read_u32(endian)? as u64,
                section_header_table_offset: cursor.read_u32(endian)? as u64,
                flags: cursor.read_u32(endian)?,
                header_size: cursor.read_u16(endian)?,
                program_header_entry_size: cursor.read_u16(endian)?,
                program_header_entry_count: cursor.read_u16(endian)?,
                section_header_entry_size: cursor.read_u16(endian)?,
                section_header_entry_count: cursor.read_u16(endian)?,
                section_header_string_table_index: cursor.read_u16(endian)?,
            }),
            _ => Err(Error::UnknownClass),
        }
    }
}
