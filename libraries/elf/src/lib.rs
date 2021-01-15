//**************************************************************************************************
// lib.rs                                                                                          *
// Copyright (c) 2019-2021 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

#![no_std]

#[macro_use]
extern crate enums;

mod error;
mod header;
mod identity;
mod program;
mod section;

pub use error::*;
pub use header::*;
pub use identity::*;
pub use program::*;
pub use section::*;

use core::cmp;
use core::mem;
use memory;

#[derive(Copy, Clone, Debug)]
pub struct File<'a>(&'a [u8]);

impl<'a> File<'a> {
    pub const fn new(source: &'a [u8]) -> Self {
        File(source)
    }

    pub fn read_identity_header(&self) -> Result<IdentityHeader, Error> {
        IdentityHeader::read(self.0)
    }

    pub fn read_header(&self) -> Result<Header, Error> {
        let offset = mem::size_of::<IdentityHeader>();
        let source = &self.0.get(offset..).ok_or(Error::SourceTooSmall)?;

        let identity_header = self.read_identity_header()?;

        Header::read(source, identity_header.class, identity_header.data)
    }

    //TODO Consider moving program and section header table code into collection like struct.

    pub fn read_program_header(&self, entry: u16) -> Result<ProgramHeader, Error> {
        let header = self.read_header()?;

        if entry > header.program_header_entry_count {
            return Err(Error::SourceTooSmall);
        }

        let identity_header = self.read_identity_header()?;

        let entry_memory_offset = (entry * header.program_header_entry_size) as u64;
        let source_start = (header.program_header_table_offset + entry_memory_offset) as usize;
        let source = &self.0.get(source_start..).ok_or(Error::SourceTooSmall)?;

        ProgramHeader::read(source, identity_header.class, identity_header.data)
    }

    pub fn read_section_header(&self, entry: u16) -> Result<SectionHeader, Error> {
        let header = self.read_header()?;

        if entry > header.section_header_entry_count {
            return Err(Error::SourceTooSmall);
        }

        let identity_header = self.read_identity_header()?;

        let entry_memory_offset = (entry * header.section_header_entry_size) as u64;
        let source_start = (header.section_header_table_offset + entry_memory_offset) as usize;
        let source = &self.0.get(source_start..).ok_or(Error::SourceTooSmall)?;

        SectionHeader::read(source, identity_header.class, identity_header.data)
    }

    pub fn load_memory_segment(&self) -> Result<memory::Segment, Error> {
        let mut start_address = core::usize::MAX;
        let mut end_address = core::usize::MIN;
        let mut available_load_segment = false;

        let header = self.read_header()?;

        for entry in 0..header.program_header_entry_count {
            let program_header = self.read_program_header(entry)?;

            if program_header.segment_type != ProgramSegmentType::LOAD {
                continue;
            }

            let memory_size = program_header.memory_size as usize;
            if memory_size == 0 {
                continue;
            }

            let file_size = program_header.file_size as usize;
            if memory_size < file_size {
                return Err(Error::InvalidProgramSegmentSize);
            }

            let segment_start_address = program_header.virtual_address as usize;
            let segment_end_address = segment_start_address + memory_size;

            start_address = cmp::min(segment_start_address, start_address);
            end_address = cmp::max(segment_end_address, end_address);

            available_load_segment = true;
        }

        if !available_load_segment {
            return Err(Error::NoLoadProgramSegments);
        }

        Ok(memory::Segment::with_end(start_address, end_address))
    }

    pub unsafe fn load(&self) -> Result<(), Error> {
        let memory_range = self.load_memory_segment()?;
        self.load_internal(memory_range.as_mut_slice(), memory_range)
    }

    pub fn load_to(&self, memory: &mut [u8]) -> Result<(), Error> {
        let memory_range = self.load_memory_segment()?;

        if memory_range.len() > memory.len() {
            return Err(Error::DestinationTooSmall);
        }

        self.load_internal(memory, memory_range)
    }

    fn load_internal(&self, memory: &mut [u8], memory_usage: memory::Segment) -> Result<(), Error> {
        let header = self.read_header()?;

        for entry in 0..header.program_header_entry_count {
            let program_header = self.read_program_header(entry)?;

            if program_header.segment_type != ProgramSegmentType::LOAD {
                continue;
            }

            let file_size = program_header.file_size as usize;
            let file_start = program_header.offset as usize;
            let file_end = file_start + file_size;

            let source = self
                .0
                .get(file_start..file_end)
                .ok_or(Error::SourceTooSmall)?;

            let memory_size = program_header.memory_size as usize;
            let destination_start =
                (program_header.virtual_address as usize) - memory_usage.start();
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
