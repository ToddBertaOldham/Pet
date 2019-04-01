// *************************************************************************
// lib.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

#![no_std]

#[macro_use]
extern crate generation;

mod identity;
mod header;
mod program;
mod section;

pub use identity::*;
pub use header::*;
pub use program::*;
pub use section::*;
use core::mem;

#[derive(PartialEq)]
pub struct ElfFile<'a>(&'a[u8]);

impl<'a> ElfFile<'a> {
    pub const fn new(source : &'a[u8]) -> Self {
        ElfFile(source)
    }

    pub fn read_identity_header(&self) -> Option<&'a ElfIdentityHeader> {
        ElfIdentityHeader::read(self.0)
    }

    pub fn read_header(&self) -> Option<&'a ElfHeader> {
        let source = &self.0.get(mem::size_of::<ElfIdentityHeader>()..)?;
        match self.read_identity_header()?.class {
            ElfClass::SIXTY_FOUR => Elf64Header::read(source).and_then(|header| { Some(header as &'a ElfHeader) }),         
            ElfClass::THIRTY_TWO => Elf32Header::read(source).and_then(|header| { Some(header as &'a ElfHeader) }),         
            _ => None
        }
    }

    pub fn read_program_header(&self, entry : u16) -> Option<&'a ElfProgramHeader> {
        let header = self.read_header()?;

        if entry > header.program_header_entry_count() {
            return None;
        }

        let entry_memory_offset = (entry * header.program_header_entry_size()) as u64; 
        let source_start = (header.program_header_table_offset() + entry_memory_offset) as usize;
        let source = &self.0.get(source_start..)?;

        match self.read_identity_header()?.class {
            ElfClass::SIXTY_FOUR => Elf64ProgramHeader::read(source).and_then(|header| { Some(header as &'a ElfProgramHeader) }),         
            ElfClass::THIRTY_TWO => Elf32ProgramHeader::read(source).and_then(|header| { Some(header as &'a ElfProgramHeader) }),         
            _ => None
        }
    }

    pub fn read_section_header(&self, entry : u16) -> Option<&'a ElfSectionHeader> {
        let header = self.read_header()?;

        if entry > header.section_header_entry_count() {
            return None;
        }

        let entry_memory_offset = (entry * header.section_header_entry_size()) as u64; 
        let source_start = (header.section_header_table_offset() + entry_memory_offset) as usize;
        let source = &self.0.get(source_start..)?;

        match self.read_identity_header()?.class {
            ElfClass::SIXTY_FOUR => Elf64SectionHeader::read(source).and_then(|header| { Some(header as &'a ElfSectionHeader) }),         
            ElfClass::THIRTY_TWO => Elf32SectionHeader::read(source).and_then(|header| { Some(header as &'a ElfSectionHeader) }),         
            _ => None
        }
    }
}