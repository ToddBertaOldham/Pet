//**************************************************************************************************
// identity.rs                                                                                     *
// Copyright (c) 2019-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use super::error::Error;
use core::convert::TryFrom;
use io::cursor::Cursor;
use io::{Endian, EndianRead, Read};

pub const MAGIC_0: u8 = 0x7F;
pub const MAGIC_1: u8 = 0x45;
pub const MAGIC_2: u8 = 0x4C;
pub const MAGIC_3: u8 = 0x46;

c_enum!(
    pub enum Class : u8 {
        NONE = 0;
        THIRTY_TWO = 1;
        SIXTY_FOUR = 2;
    }
);

c_enum!(
    pub enum Data : u8 {
        INVALID = 0;
        LITTLE_ENDIAN = 1;
        BIG_ENDIAN = 2;
    }
);

impl TryFrom<Data> for Endian {
    type Error = Error;

    fn try_from(value: Data) -> Result<Self, Self::Error> {
        match value {
            Data::LITTLE_ENDIAN => Ok(Endian::Little),
            Data::BIG_ENDIAN => Ok(Endian::Big),
            _ => Err(Error::UnknownData),
        }
    }
}

impl From<Endian> for Data {
    fn from(value: Endian) -> Self {
        match value {
            Endian::Little => Data::LITTLE_ENDIAN,
            Endian::Big => Data::BIG_ENDIAN,
        }
    }
}

c_enum!(
    pub enum OsAbi : u8 {
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

#[derive(Clone, Debug)]
pub struct IdentityHeader {
    pub magic_0: u8,
    pub magic_1: u8,
    pub magic_2: u8,
    pub magic_3: u8,
    pub class: Class,
    pub data: Data,
    pub version: u8,
    pub os_abi: OsAbi,
    pub abi_version: u8,
    pub unused: [u8; 7],
}

impl IdentityHeader {
    pub fn read(source: &[u8]) -> Result<Self, Error> {
        let mut cursor = Cursor::new(source);
        Ok(IdentityHeader {
            magic_0: cursor.read_u8()?,
            magic_1: cursor.read_u8()?,
            magic_2: cursor.read_u8()?,
            magic_3: cursor.read_u8()?,
            class: Class::from(cursor.read_u8()?),
            data: Data::from(cursor.read_u8()?),
            version: cursor.read_u8()?,
            os_abi: OsAbi::from(cursor.read_u8()?),
            abi_version: cursor.read_u8()?,
            unused: {
                let mut unused: [u8; 7] = [0; 7];
                cursor.read_exact(&mut unused)?;
                unused
            },
        })
    }

    pub fn is_valid(&self) -> bool {
        self.magic_0 == MAGIC_0
            && self.magic_1 == MAGIC_1
            && self.magic_2 == MAGIC_2
            && self.magic_3 == MAGIC_3
    }

    pub fn is_64bit(&self) -> bool {
        self.class == Class::SIXTY_FOUR
    }

    pub fn is_32bit(&self) -> bool {
        self.class == Class::THIRTY_TWO
    }

    pub fn is_little_endian(&self) -> bool {
        self.data == Data::LITTLE_ENDIAN
    }

    pub fn is_big_endian(&self) -> bool {
        self.data == Data::BIG_ENDIAN
    }
}
