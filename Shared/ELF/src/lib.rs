// *************************************************************************
// lib.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

#![no_std]

#[macro_use]
extern crate generation;

#[macro_use]
mod macros;
mod identity;
mod header;
mod program;
mod section;
mod error;

pub use identity::*;
pub use header::*;
pub use program::*;
pub use section::*;
pub use error::*;
use core::mem;
use core::slice;

#[derive(PartialEq)]
pub struct ElfFile<'a>(&'a[u8]);

impl<'a> ElfFile<'a> {
    pub const fn new(source : &'a[u8]) -> Self {
        ElfFile(source)
    }

    pub fn read_identity_header(&self) -> Result<&'a ElfIdentityHeader, ElfError> {
        ElfIdentityHeader::read(self.0)
    }

    pub fn read_header(&self) -> Result<&'a ElfHeader, ElfError> {
        let offset = mem::size_of::<ElfIdentityHeader>();
        let source = &self.0.get(offset..).ok_or(ElfError::SourceTooSmall)?;
        match self.read_identity_header()?.class {
            ElfClass::SIXTY_FOUR => Elf64Header::read(source).and_then(|header| { Ok(header as &'a ElfHeader) }),         
            ElfClass::THIRTY_TWO => Elf32Header::read(source).and_then(|header| { Ok(header as &'a ElfHeader) }),         
            _ => Err(ElfError::UnknownClass)
        }
    }

    //TODO Consider moving program and section header table code into collection like struct.

    pub fn read_program_header(&self, entry : u16) -> Result<&'a ElfProgramHeader, ElfError> {
        let header = self.read_header()?;

        if entry > header.program_header_entry_count() {
            return Err(ElfError::SourceTooSmall);
        }

        let entry_memory_offset = (entry * header.program_header_entry_size()) as u64; 
        let source_start = (header.program_header_table_offset() + entry_memory_offset) as usize;
        let source = &self.0.get(source_start..).ok_or(ElfError::SourceTooSmall)?;

        match self.read_identity_header()?.class {
            ElfClass::SIXTY_FOUR => Elf64ProgramHeader::read(source).and_then(|header| { Ok(header as &'a ElfProgramHeader) }),         
            ElfClass::THIRTY_TWO => Elf32ProgramHeader::read(source).and_then(|header| { Ok(header as &'a ElfProgramHeader) }),         
            _ => Err(ElfError::UnknownClass)
        }
    }

    pub fn read_section_header(&self, entry : u16) -> Result<&'a ElfSectionHeader, ElfError> {
        let header = self.read_header()?;

        if entry > header.section_header_entry_count() {
            return Err(ElfError::SourceTooSmall);
        }

        let entry_memory_offset = (entry * header.section_header_entry_size()) as u64; 
        let source_start = (header.section_header_table_offset() + entry_memory_offset) as usize;
        let source = &self.0.get(source_start..).ok_or(ElfError::SourceTooSmall)?;

        match self.read_identity_header()?.class {
            ElfClass::SIXTY_FOUR => Elf64SectionHeader::read(source).and_then(|header| { Ok(header as &'a ElfSectionHeader) }),         
            ElfClass::THIRTY_TWO => Elf32SectionHeader::read(source).and_then(|header| { Ok(header as &'a ElfSectionHeader) }),         
            _ => Err(ElfError::UnknownClass)
        }
    }

    pub fn memory_range(&self) -> Result<ElfMemoryRange, ElfError> {
        let mut start_address = core::usize::MAX;
        let mut end_address = core::usize::MIN;
        let mut available_load_segment = false;

        let header = self.read_header()?;

        for entry in 0..header.program_header_entry_count() {
            let program_header = self.read_program_header(entry)?;

            if program_header.segment_type() != ElfProgramSegmentType::LOAD {
                continue;
            }

            let memory_size = program_header.memory_size() as usize;
            if memory_size == 0 {
                continue;
            }

            let file_size = program_header.file_size() as usize;
            if memory_size < file_size {
                return Err(ElfError::InvalidProgramSegmentSize);
            }

            let segment_start_address = program_header.virtual_address() as usize;
            let segment_end_address = segment_start_address + memory_size;

            if segment_start_address < start_address {
                start_address = segment_start_address;
            }

            if segment_end_address > end_address {
                end_address = segment_end_address;
            }

            available_load_segment = true;
        }
        
        if !available_load_segment {
            return Err(ElfError::NoLoadProgramSegments);
        }

        Ok(ElfMemoryRange::new(start_address, end_address))
    }

    pub unsafe fn load(&self)-> Result<(), ElfError> {
        let memory_range = self.memory_range()?;
        self.load_internal(memory_range.as_mut_slice(), memory_range)
    }

    pub fn load_to(&self, memory : &mut [u8]) -> Result<(), ElfError> {
        let memory_range = self.memory_range()?;
    
        if memory_range.len() > memory.len() {
            return Err(ElfError::DestinationTooSmall);
        }

        self.load_internal(memory, memory_range)
    }

    fn load_internal(&self, memory : &mut [u8], memory_usage : ElfMemoryRange) -> Result<(), ElfError> {
        let header = self.read_header()?;

        for entry in 0..header.program_header_entry_count() {
            let program_header = self.read_program_header(entry)?;

            if program_header.segment_type() != ElfProgramSegmentType::LOAD {
                continue;
            }

            let file_size = program_header.file_size() as usize;
            let file_start = program_header.offset() as usize;
            let file_end = file_start + file_size;

            let source = self.0.get(file_start..file_end).ok_or(ElfError::SourceTooSmall)?;

            let memory_size = program_header.memory_size() as usize;
            let destination_start = (program_header.virtual_address() as usize) - memory_usage.start_address();
            let destination_end = destination_start + file_size;

            let destination = &mut memory[destination_start..destination_end];
            destination.copy_from_slice(source);

            let destination_extra_end = destination_end + (memory_size - file_size);
            let destination_extra = &mut memory[destination_end..destination_extra_end];

            for extra_byte in destination_extra {
                *extra_byte = 0;
            }
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct ElfMemoryRange {
    start_address : usize,
    end_address : usize
}

impl ElfMemoryRange {
    pub fn new(start_address : usize, end_address : usize) -> Self {
        ElfMemoryRange { start_address, end_address }
    }

    pub fn start_address(&self) -> usize {
        self.start_address
    }

    pub fn end_address(&self) -> usize {
        self.end_address
    }

    pub fn len(&self) -> usize {
        self.end_address - self.start_address
    }

    pub unsafe fn as_slice(&self) -> &[u8] {
        let address = self.start_address as *mut u8;
        slice::from_raw_parts(address, self.len())
    }

    pub unsafe fn as_mut_slice(&self) -> &mut [u8] {
        let address = self.start_address as *mut u8;
        slice::from_raw_parts_mut(address, self.len())
    }
}