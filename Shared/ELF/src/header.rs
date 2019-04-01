// *************************************************************************
// header.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use core::mem;

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
    }
);

c_enum!(
    pub enum ElfVersion : u32 {
        NONE = 0;
        CURRENT = 1;
    }
);

pub trait ElfHeader {
    fn object_type(&self) -> ElfType;
    fn machine(&self) -> ElfMachine;
    fn version(&self) -> ElfVersion;
    fn entry(&self) -> u64;
    fn program_header_table_offset(&self) -> u64;
    fn section_header_table_offset(&self) -> u64;
    fn flags(&self) -> u32;
    fn elf_header_size(&self) -> u16;
    fn program_header_entry_size(&self) -> u16;
    fn program_header_entry_count(&self) -> u16;
    fn section_header_entry_size(&self) -> u16;
    fn section_header_entry_count(&self) -> u16;
    fn section_header_string_table_index(&self) -> u16;
}

macro_rules! standard_elf_header {
    ($name:ident, $address_type:ty) => {
        #[repr(C)]
        pub struct $name {
            pub object_type : ElfType,
            pub machine : ElfMachine,
            pub version : ElfVersion, 
            pub entry : $address_type,
            pub program_header_table_offset : $address_type,
            pub section_header_table_offset : $address_type,
            pub flags : u32,
            pub elf_header_size : u16,
            pub program_header_entry_size : u16,
            pub program_header_entry_count : u16,
            pub section_header_entry_size : u16,
            pub section_header_entry_count : u16,
            pub section_header_string_table_index : u16
        }

        impl $name {
            pub fn read<'a>(data : &'a[u8]) -> Option<&'a Self> {
                unsafe {
                    if data.len() < mem::size_of::<Self>() {
                        return None;
                    }

                    Some(&*(data.as_ptr() as *const Self))
                }
            }
        }

        impl ElfHeader for $name {
            fn object_type(&self) -> ElfType {
                self.object_type
            }
            
            fn machine(&self) -> ElfMachine {
                self.machine
            }

            fn version(&self) -> ElfVersion {
                self.version
            }

            fn entry(&self) -> u64 {
                self.entry as u64
            }

            fn program_header_table_offset(&self) -> u64 {
                self.program_header_table_offset as u64
            }

            fn section_header_table_offset(&self) -> u64 {
                self.section_header_table_offset as u64
            }

            fn flags(&self) -> u32 {
                self.flags
            }

            fn elf_header_size(&self) -> u16 {
                self.elf_header_size
            }

            fn program_header_entry_size(&self) -> u16 {
                self.program_header_entry_size
            }
            
            fn program_header_entry_count(&self) -> u16 {
                self.program_header_entry_count
            }

            fn section_header_entry_size(&self) -> u16 {
                self.section_header_entry_size
            }

            fn section_header_entry_count(&self) -> u16 {
                self.section_header_entry_count
            }

            fn section_header_string_table_index(&self) -> u16 {
                self.section_header_string_table_index
            }
        }
    };
}

standard_elf_header!(Elf64Header, u64);
standard_elf_header!(Elf32Header, u32);