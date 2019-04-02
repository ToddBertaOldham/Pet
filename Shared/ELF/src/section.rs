// *************************************************************************
// section.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use super::error::ElfError;
use core::mem;

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

pub trait ElfSectionHeader {
    fn name(&self) -> u32;
    fn segment_type(&self) -> ElfSectionSegmentType;
    fn flags(&self) -> u64;
    fn address(&self) -> u64;
    fn offset(&self) -> u64;
    fn size(&self) -> u64;
    fn link(&self) -> u32;
    fn info(&self) -> u32;
    fn address_align(&self) -> u64;
    fn entry_size(&self) -> u64;
}

macro_rules! standard_section_header {
    ($name:ident, $address_type:ty) => {
        #[repr(C)]
        pub struct $name {
            pub name : u32,
            pub segment_type : ElfSectionSegmentType,
            pub flags : $address_type,
            pub address : $address_type,
            pub offset : $address_type,
            pub size : $address_type,
            pub link : u32,
            pub info : u32,
            pub address_align : $address_type,
            pub entry_size : $address_type
        }

        impl $name {
            read_constructor!();
        }

        impl ElfSectionHeader for $name {
            fn name(&self) -> u32 {
                self.name
            }

            fn segment_type(&self) -> ElfSectionSegmentType {
                self.segment_type
            }

            fn flags(&self) -> u64 {
                self.flags as u64
            }

            fn address(&self) -> u64 {
                self.address as u64
            }

            fn offset(&self) -> u64 {
                self.offset as u64
            }

            fn size(&self) -> u64 {
                self.size as u64
            }

            fn link(&self) -> u32 {
                self.link
            }

            fn info(&self) -> u32 {
                self.info
            }

            fn address_align(&self) -> u64 {
                self.address_align as u64
            }

            fn entry_size(&self) -> u64 {
                self.entry_size as u64
            }
        }
    };
}

standard_section_header!(Elf64SectionHeader, u64);
standard_section_header!(Elf32SectionHeader, u32);