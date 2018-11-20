// *************************************************************************
// text_io.rs
// Copyright 2018 Todd Berta-Oldham
// This code is made available under the MIT License.
// ************************************************************************

use ffi::primitives::*;
use core::ffi::c_void;

pub const SIMPLE_TEXT_OUTPUT_GUID : GUID = GUID { data_1 : 0x387477c2, data_2 : 0x69c7, data_3 : 0x11d2, data_4 : [ 0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b ] };

pub const SIMPLE_TEXT_INPUT_GUID : GUID = GUID { data_1 : 0x387477c1, data_2 : 0x69c7, data_3 : 0x11d2, data_4 : [ 0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b ] };

#[repr(C)]
pub struct SimpleTextInputProtocol {
    pub reset : extern "win64" fn(this : *mut SimpleTextInputProtocol, extended_verification : bool) -> Status,
    pub read_key_stroke : extern "win64" fn(this : *mut SimpleTextInputProtocol, key : *mut InputKey) -> Status,
    pub wait_for_key : Event
}

#[repr(C)]
pub struct SimpleTextOutputProtocol {
    pub reset : extern "win64" fn(this : *mut SimpleTextOutputProtocol, extended_verification : bool) -> Status,
    pub output_string : extern "win64" fn(this : *mut SimpleTextOutputProtocol, string : *mut u16) -> Status,
    pub test_string : extern "win64" fn(this : *mut SimpleTextOutputProtocol, string : *mut u16) -> Status,
    pub query_mode : extern "win64" fn(this : *mut SimpleTextOutputProtocol, mode_number : usize, columns : *mut usize, rows : *mut usize) -> Status,
    pub set_mode : extern "win64" fn(this : *mut SimpleTextOutputProtocol, mode_number : usize) -> Status
}

#[repr(C)]
pub struct InputKey {
    pub scan_code : u16,
    pub unicode_char : u16
}