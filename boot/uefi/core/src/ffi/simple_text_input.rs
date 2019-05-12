// *************************************************************************
// simple_text_input.rs
// Copyright 2018-2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// ************************************************************************

use super::primitives::{Event, Status, Guid};

#[repr(C)]
pub struct Protocol {
    pub reset : extern "win64" fn(this : *mut Protocol, extended_verification : bool) -> Status,
    pub read_key_stroke : extern "win64" fn(this : *mut Protocol, key : *mut InputKey) -> Status,
    pub wait_for_key : Event
}

impl Protocol {
    pub const GUID : Guid = Guid { data_1 : 0x387477c1, data_2 : 0x69c7, data_3 : 0x11d2, data_4 : [ 0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b ] };
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct InputKey {
    pub scan_code : u16,
    pub unicode_char : u16
}