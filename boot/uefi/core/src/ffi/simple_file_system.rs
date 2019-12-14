//**************************************************************************************************
// simple_file_system.rs                                                                           *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use super::file;
use super::primitives::{Guid, Status};

#[repr(C)]
pub struct Protocol {
    pub revision: u64,
    pub open_volume:
        extern "efiapi" fn(this: *mut Protocol, root: *mut *mut file::Protocol) -> Status,
}

impl Protocol {
    pub const GUID: Guid = Guid {
        data_1: 0x0964e5b22,
        data_2: 0x6459,
        data_3: 0x11d2,
        data_4: [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
    };
    pub const REVISION: u64 = 0x00010000;
}
