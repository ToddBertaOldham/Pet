// *************************************************************************
// identity.rs
// Copyright 2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use super::error::ElfError;
use core::mem;

pub const MAGIC_0 : u8 = 0x7F;
pub const MAGIC_1 : u8 = 0x45;
pub const MAGIC_2 : u8 = 0x4C;
pub const MAGIC_3 : u8 = 0x46;

c_enum!(
    pub enum ElfClass : u8 {
        NONE = 0;
        THIRTY_TWO = 1;
        SIXTY_FOUR = 2;
    }
);

c_enum!(
    pub enum ElfData : u8 {
        INVALID = 0;
        LITTLE_ENDIAN = 1;
        BIG_ENDIAN = 2;
    }
);

c_enum!(
    pub enum ElfOsAbi : u8 {
        SYSTEM_V = 0x0;
        HP_UX = 0x1;
        NETBSD = 0x2;
        LINUX = 0x3;
        GNU_HURD = 0x4;
        SOLARIS = 0x6;
        AIX = 0x7;
        IRIX = 0x8;
        FREEBSD = 0x9;
        TRU64 = 0xA;
        NOVELL_MODESTO = 0xB;
        OPENBSD = 0xC;
        OPENVMS = 0xD;
        NONSTOPKERNEL = 0xE;
        AROS = 0xF;
        FENIXOS = 0x10;
        CLOUDABI = 0x11;        
    }
);

#[repr(C)]
pub struct ElfIdentityHeader {
    pub magic_0 : u8,
    pub magic_1 : u8,
    pub magic_2 : u8,
    pub magic_3 : u8,
    pub class : ElfClass,
    pub data : ElfData,
    pub version : u8,
    pub os_abi : ElfOsAbi,
    pub abi_version : u8,
    pub unused : [u8; 7]
}

impl ElfIdentityHeader {
    read_constructor!();

    pub fn is_valid(&self) -> bool {
        self.magic_0 == MAGIC_0 && self.magic_1 == MAGIC_1 &&
            self.magic_2 == MAGIC_2 && self.magic_3 == MAGIC_3
    }

    pub fn is_64bit(&self) -> bool {
        self.class == ElfClass::SIXTY_FOUR
    }

    pub fn is_32bit(&self) -> bool {
        self.class == ElfClass::THIRTY_TWO
    }

    pub fn is_little_endian(&self) -> bool {
        self.data == ElfData::LITTLE_ENDIAN
    }

    pub fn is_big_endian(&self) -> bool {
        self.data == ElfData::BIG_ENDIAN
    }
}