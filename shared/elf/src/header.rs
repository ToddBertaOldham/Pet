// *************************************************************************
// header.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use super::error::ElfError;
use super::identity::{ ElfClass, ElfData };
use io::{ BinaryReader, Cursor, Endian };
use core::convert::TryFrom;

c_enum!(
    pub enum ElfType : u16 {
        NONE = 0;
        RELOCATABLE = 1;
        EXECUTABLE = 2;
        DYNAMIC = 3;
        CORE = 4;
    }
);

c_enum!(
    pub enum ElfMachine : u16 {
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
    pub enum ElfVersion : u32 {
        NONE = 0;
        CURRENT = 1;
    }
);

#[derive(Clone, Debug)]
pub struct ElfHeader {
    object_type : ElfType,
    machine : ElfMachine,
    version : ElfVersion, 
    entry : u64,
    program_header_table_offset : u64,
    section_header_table_offset : u64,
    flags : u32,
    elf_header_size : u16,
    program_header_entry_size : u16,
    program_header_entry_count : u16,
    section_header_entry_size : u16,
    section_header_entry_count : u16,
    section_header_string_table_index : u16
}

impl ElfHeader {
    pub fn read(source : &[u8], class : ElfClass, data : ElfData) -> Result<Self, ElfError> {
        let endian = Endian::try_from(data)?;
        let mut cursor = Cursor::new(source);

        match class {
            ElfClass::SIXTY_FOUR => Ok(ElfHeader {
                object_type : ElfType::new(cursor.read_u16(endian)?),
                machine : ElfMachine::new(cursor.read_u16(endian)?),
                version : ElfVersion::new(cursor.read_u32(endian)?),
                entry : cursor.read_u64(endian)?,
                program_header_table_offset :  cursor.read_u64(endian)?,
                section_header_table_offset : cursor.read_u64(endian)?,
                flags : cursor.read_u32(endian)?,
                elf_header_size : cursor.read_u16(endian)?,
                program_header_entry_size : cursor.read_u16(endian)?,
                program_header_entry_count : cursor.read_u16(endian)?,
                section_header_entry_size : cursor.read_u16(endian)?,
                section_header_entry_count : cursor.read_u16(endian)?,
                section_header_string_table_index : cursor.read_u16(endian)?
                }),
            ElfClass::THIRTY_TWO => Ok(ElfHeader {
                object_type : ElfType::new(cursor.read_u16(endian)?),
                machine : ElfMachine::new(cursor.read_u16(endian)?),
                version : ElfVersion::new(cursor.read_u32(endian)?),
                entry : cursor.read_u32(endian)? as u64,
                program_header_table_offset :  cursor.read_u32(endian)? as u64,
                section_header_table_offset : cursor.read_u32(endian)? as u64,
                flags : cursor.read_u32(endian)?,
                elf_header_size : cursor.read_u16(endian)?,
                program_header_entry_size : cursor.read_u16(endian)?,
                program_header_entry_count : cursor.read_u16(endian)?,
                section_header_entry_size : cursor.read_u16(endian)?,
                section_header_entry_count : cursor.read_u16(endian)?,
                section_header_string_table_index : cursor.read_u16(endian)?
            }),
            _ => Err(ElfError::UnknownClass)
        }
    }

    pub fn object_type(&self) -> ElfType {
        self.object_type
    }
    
    pub fn machine(&self) -> ElfMachine {
        self.machine
    }

    pub fn version(&self) -> ElfVersion {
        self.version
    }

    pub fn entry(&self) -> u64 {
        self.entry
    }

    pub fn program_header_table_offset(&self) -> u64 {
        self.program_header_table_offset
    }

    pub fn section_header_table_offset(&self) -> u64 {
        self.section_header_table_offset
    }

    pub fn flags(&self) -> u32 {
        self.flags
    }

    pub fn elf_header_size(&self) -> u16 {
        self.elf_header_size
    }

    pub fn program_header_entry_size(&self) -> u16 {
        self.program_header_entry_size
    }
    
    pub fn program_header_entry_count(&self) -> u16 {
        self.program_header_entry_count
    }

    pub fn section_header_entry_size(&self) -> u16 {
        self.section_header_entry_size
    }

    pub fn section_header_entry_count(&self) -> u16 {
        self.section_header_entry_count
    }

    pub fn section_header_string_table_index(&self) -> u16 {
        self.section_header_string_table_index
    }
}