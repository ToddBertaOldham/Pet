// *************************************************************************
// simple_text_output.rs
// Copyright 2018-2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// ************************************************************************

use super::primitives::{Status, Guid};

#[repr(C)]
pub struct Protocol {
    pub reset : extern "win64" fn(this : *mut Protocol, extended_verification : bool) -> Status,
    pub output_string : extern "win64" fn(this : *mut Protocol, string : *mut u16) -> Status,
    pub test_string : extern "win64" fn(this : *mut Protocol, string : *mut u16) -> Status,
    pub query_mode : extern "win64" fn(this : *mut Protocol, mode_number : usize, columns : *mut usize, rows : *mut usize) -> Status,
    pub set_mode : extern "win64" fn(this : *mut Protocol, mode_number : usize) -> Status,
    pub set_attribute : extern "win64" fn(this : *mut Protocol, attribute : ColorAttribute) -> Status,
    pub clear_screen : extern "win64" fn(this : *mut Protocol) -> Status,
    pub set_cursor_position : extern "win64" fn(this : *mut Protocol, column : usize, row : usize) -> Status,
    pub enable_cursor : extern "win64" fn(this : *mut Protocol, visible : bool) -> Status
}

impl Protocol {
    pub const GUID : Guid = Guid { data_1 : 0x387477c2, data_2 : 0x69c7, data_3 : 0x11d2, data_4 : [ 0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b ] };
}

//TODO Finish color attribute.

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FrontColor {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x02,
    Cyan = 0x03,
    Red = 0x04,
    Magenta = 0x05,
    Brown = 0x06,
    LightGray = 0x07,
    DarkGray = 0x08,
    LightBlue = 0x09,
    LightGreen = 0x0A,
    LightCyan = 0x0B,
    LightRed = 0x0C,
    LightMagenta = 0x0D,
    Yellow = 0x0E,
    White = 0x0F
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BackColor {
    Black = 0x0,
    Blue = 0x10,
    Green = 0x20,
    Cyan = 0x30,
    Red = 0x40,
    Magenta = 0x50,
    Brown = 0x60,
    LightGray = 0x70
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ColorAttribute(usize);

impl ColorAttribute {
    pub fn new(back_color : BackColor, front_color : FrontColor) -> Self {
        Self(front_color as usize | back_color as usize)
    }
}