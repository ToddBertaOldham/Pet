// *************************************************************************
// lib.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

#![no_std]

use core::mem;

pub const MAGIC_0 : u8 = 0x7F;
pub const MAGIC_1 : u8 = 0x45;
pub const MAGIC_2 : u8 = 0x4C;
pub const MAGIC_3 : u8 = 0x46;

pub const CLASS_INVALID : u8 = 0;
pub const CLASS_32BIT : u8 = 1;
pub const CLASS_64BIT : u8 = 2;

pub const DATA_INVALID : u8 = 0;
pub const DATA_LITTLE_ENDIAN : u8 = 1;
pub const DATA_BIG_ENDIAN : u8 = 2;

pub const OS_ABI_SYSTEM_V : u8 = 0x0;
pub const OS_ABI_SYSTEM_HP_UX : u8 = 0x1;
pub const OS_ABI_SYSTEM_NETBSD : u8 = 0x2;
pub const OS_ABI_SYSTEM_LINUX : u8 = 0x3;
pub const OS_ABI_SYSTEM_GNU_HURD : u8 = 0x4;
pub const OS_ABI_SYSTEM_SOLARIS : u8 = 0x6;
pub const OS_ABI_SYSTEM_AIX : u8 = 0x7;
pub const OS_ABI_SYSTEM_IRIX : u8 = 0x8;
pub const OS_ABI_SYSTEM_FREEBSD : u8 = 0x9;
pub const OS_ABI_SYSTEM_TRU64 : u8 = 0xA;
pub const OS_ABI_SYSTEM_NOVELL_MODESTO : u8 = 0xB;
pub const OS_ABI_SYSTEM_OPENBSD : u8 = 0xC;
pub const OS_ABI_SYSTEM_OPENVMS : u8 = 0xD;
pub const OS_ABI_SYSTEM_NONSTOPKERNEL : u8 = 0xE;
pub const OS_ABI_SYSTEM_AROS : u8 = 0xF;
pub const OS_ABI_SYSTEM_FENIXOS : u8 = 0x10;
pub const OS_ABI_SYSTEM_CLOUDABI : u8 = 0x11;

#[derive(PartialEq)]
pub struct ElfFile<'a>(&'a[u8]);

impl<'a> ElfFile<'a> {
    pub const fn new(source : &'a[u8]) -> Self {
        ElfFile(source)
    }

    pub fn read_identity_header(&self) -> Option<&'a ElfIdentityHeader> {
        ElfIdentityHeader::read(self.0)
    }
}

#[repr(C)]
pub struct ElfIdentityHeader {
    pub magic_0 : u8,
    pub magic_1 : u8,
    pub magic_2 : u8,
    pub magic_3 : u8,
    pub class : u8,
    pub data : u8,
    pub version : u8,
    pub os_abi : u8,
    pub abi_version : u8,
    pub unused : [u8; 7]
}

impl ElfIdentityHeader {
    pub fn read<'a>(data : &'a[u8]) -> Option<&'a Self> {
        unsafe {
            if data.len() < mem::size_of::<Self>() {
                return None;
            }

            Some(&*(data.as_ptr() as *const Self))
        }
    }

    pub fn is_valid(&self) -> bool {
        self.magic_0 == MAGIC_0 && self.magic_1 == MAGIC_1 &&
            self.magic_2 == MAGIC_2 && self.magic_3 == MAGIC_3
    }

    pub fn is_64bit(&self) -> bool {
        self.class == CLASS_64BIT
    }

    pub fn is_32bit(&self) -> bool {
        self.class == CLASS_32BIT
    }
}

#[repr(C)]
pub struct Elf64Header {
    pub identity : ElfIdentityHeader,
    pub object_type : u16,
    pub architecture : u16,
    pub version : u32, 
    pub entry : u64,
    pub program_header_offset : u64,
    pub section_header_offset : u64,
    pub flags : u32,
    pub elf_header_size : u16,
    pub program_header_size : u16,
    pub program_header_count : u16,
    pub section_header_size : u16,
    pub section_header_count : u16,
    pub section_header_string_table_index : u16
}

impl Elf64Header {
    pub fn read<'a>(data : &'a[u8]) -> Option<&'a Self> {
        unsafe {
            if data.len() < mem::size_of::<Self>() {
                return None;
            }

            Some(&*(data.as_ptr() as *const Self))
        }
    }
}

#[repr(C)]
pub struct ELF64ProgramHeader {
    pub segment_type : u32,
    pub flags : u32,
    pub offset : u64,
    pub virtual_address : u64,
    pub physical_address : u64,
    pub file_size : u64,
    pub memory_size : u64,
    pub alignment : u64
}

#[repr(C)]
pub struct ELF64SectionHeader {
    pub name : u32,
    pub section_type : u32,
    pub flags : u64,
    pub address : u64,
    pub offset : u64,
    pub size : u64,
    pub link : u32,
    pub info : u32,
    pub address_align : u64,
    pub entry_size : u64
}