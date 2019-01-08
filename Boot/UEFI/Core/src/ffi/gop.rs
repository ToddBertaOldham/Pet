// *************************************************************************
// gop.rs
// Copyright 2018 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

// Reference available at http://wiki.phoenix.com/wiki/index.php/EFI_GRAPHICS_OUTPUT_PROTOCOL.

use super::primitives::*;

pub const GOP_GUID : GUID = GUID { data_1 : 0x9042a9de, data_2 : 0x23dc, data_3 : 0x4a38, data_4 : [ 0x96, 0xfb, 0x7a, 0xde, 0xd0, 0x80, 0x51, 0x6a ] };

#[repr(C)]
pub struct GOP {
    pub query_mode : extern "win64" fn(this : *mut GOP, mode_number : u32, size_of_info : *mut usize, info : *mut *mut GOPModeInfo) -> Status, 
    pub set_mode : extern "win64" fn(this : *mut GOP, mode_number : u32) -> Status, 
    pub blt : extern "win64" fn(this : *mut GOP, blt_buffer : *mut BltPixel, blt_operation : BltOperation, source_x : usize, source_y : usize,
                                destination_x : usize, destination_y : usize, width : usize, height : usize, delta : usize) -> Status, 
    pub mode : *mut GOPMode
}

#[repr(C)]
pub struct GOPMode {
    pub max_mode : u32,
    pub mode : u32,
    pub info : *mut GOPModeInfo,
    pub size_of_info : usize,
    pub frame_buffer_base : PhysicalAddress,
    pub frame_buffer_size : usize
}

#[repr(C)]
pub struct GOPModeInfo {
    pub version : u32,
    pub horizontal_resolution : u32,
    pub vertical_resolution : u32,
    pub pixel_format : PixelFormat,
    pub pixel_information : PixelBitMask,
    pub pixels_per_scan_line : u32
}

#[repr(C)]
pub enum PixelFormat {
    RedGreenBlueReserved8BitPerColor,
    BlueGreenRedReserved8BitPerColor,
    BitMask,
    BltOnly
}

#[repr(C)]
pub struct PixelBitMask {
    pub red_mask : u32,
    pub green_mask : u32,
    pub blue_mask : u32,
    reserved_mask : u32
}

#[repr(C)]
pub struct BltPixel {
    pub blue : u8,
    pub green : u8,
    pub red : u8,
    reserved : u8 
}

impl BltPixel {
    pub fn new(blue : u8, green : u8, red : u8) -> Self {
        BltPixel { blue : blue, green : green, red : red, reserved : 0 }
    }
}

#[repr(C)]
pub enum BltOperation {
    VideoFill,
    VideoToBltBuffer,
    BufferToVideo,
    VideoToVideo
}