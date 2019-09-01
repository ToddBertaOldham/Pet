//**************************************************************************************************
// graphics_output.rs                                                                              *
// Copyright (c) 2018-2019 Todd Berta-Oldham                                                       *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use super::primitives::{Guid, PhysicalAddress, Status};

#[repr(C)]
pub struct Protocol {
    pub query_mode: extern "win64" fn(
        this: *mut Protocol,
        mode_number: u32,
        size_of_info: *mut usize,
        info: *mut *mut ModeInfo,
    ) -> Status,
    pub set_mode: extern "win64" fn(this: *mut Protocol, mode_number: u32) -> Status,
    pub blt: extern "win64" fn(
        this: *mut Protocol,
        blt_buffer: *mut BltPixel,
        blt_operation: BltOperation,
        source_x: usize,
        source_y: usize,
        destination_x: usize,
        destination_y: usize,
        width: usize,
        height: usize,
        delta: usize,
    ) -> Status,
    pub mode: *mut Mode,
}

impl Protocol {
    pub const GUID: Guid = Guid {
        data_1: 0x9042a9de,
        data_2: 0x23dc,
        data_3: 0x4a38,
        data_4: [0x96, 0xfb, 0x7a, 0xde, 0xd0, 0x80, 0x51, 0x6a],
    };
}

#[repr(C)]
pub struct Mode {
    pub max_mode: u32,
    pub mode: u32,
    pub info: *mut ModeInfo,
    pub size_of_info: usize,
    pub frame_buffer_base: PhysicalAddress,
    pub frame_buffer_size: usize,
}

#[repr(C)]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ModeInfo {
    pub version: u32,
    pub horizontal_resolution: u32,
    pub vertical_resolution: u32,
    pub pixel_format: PixelFormat,
    pub pixel_information: PixelBitMask,
    pub pixels_per_scan_line: u32,
}

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum PixelFormat {
    RedGreenBlueReserved8BitPerColor,
    BlueGreenRedReserved8BitPerColor,
    BitMask,
    BltOnly,
}

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct PixelBitMask {
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub reserved_mask: u32,
}

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct BltPixel {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
    pub reserved: u8,
}

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum BltOperation {
    VideoFill,
    VideoToBltBuffer,
    BufferToVideo,
    VideoToVideo,
}
