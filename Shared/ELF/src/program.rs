// *************************************************************************
// program.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use core::mem;

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

pub trait ElfProgramHeader {
    fn segment_type(&self) -> ElfProgramSegmentType;
    fn flags(&self) -> u32;
    fn offset(&self) -> u64;
    fn virtual_address(&self) -> u64;
    fn physical_address(&self) -> u64;
    fn file_size(&self) -> u64;
    fn memory_size(&self) -> u64;
    fn alignment(&self) -> u64;
}

macro_rules! implement_program_header {
    ($structname:ident) => {
        impl $structname {
            pub fn read<'a>(data : &'a[u8]) -> Option<&'a Self> {
                unsafe {
                    if data.len() < mem::size_of::<Self>() {
                        return None;
                    }

                    Some(&*(data.as_ptr() as *const Self))
                }
            }
        }

        impl ElfProgramHeader for $structname {
            fn segment_type(&self) -> ElfProgramSegmentType {
                self.segment_type
            }

            fn flags(&self) -> u32 {
                self.flags
            }

            fn offset(&self) -> u64 {
                self.offset as u64
            }

            fn virtual_address(&self) -> u64 {
                self.virtual_address as u64
            }

            fn physical_address(&self) -> u64 {
                self.physical_address as u64
            }

            fn file_size(&self) -> u64 {
                self.file_size as u64
            }

            fn memory_size(&self) -> u64 {
                self.memory_size as u64
            }

            fn alignment(&self) -> u64 {
                self.alignment as u64
            }
        }
    };
}

#[repr(C)]
pub struct Elf64ProgramHeader {
    pub segment_type : ElfProgramSegmentType,
    pub flags : u32,
    pub offset : u64,
    pub virtual_address : u64,
    pub physical_address : u64,
    pub file_size : u64,
    pub memory_size : u64,
    pub alignment : u64
}

implement_program_header!(Elf64ProgramHeader);

#[repr(C)]
pub struct Elf32ProgramHeader {
    pub segment_type : ElfProgramSegmentType,
    pub offset : u32,
    pub virtual_address : u32,
    pub physical_address : u32,
    pub file_size : u32,
    pub memory_size : u32,
    pub flags : u32,
    pub alignment : u32
}

implement_program_header!(Elf32ProgramHeader);