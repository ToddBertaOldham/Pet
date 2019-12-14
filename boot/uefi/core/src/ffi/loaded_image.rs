//**************************************************************************************************
// loaded_image.rs                                                                                 *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::ffi::boot::MemoryType;
use crate::ffi::{device_path, system};
use crate::{Guid, Handle, Status};
use core::ffi::c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Protocol {
    pub revision: u32,
    pub parent_handle: Handle,
    pub system_table: *mut system::Table,
    pub device_handle: Handle,
    pub file_path: *mut device_path::Protocol,
    pub reserved: *mut c_void,
    pub load_options_size: u32,
    pub load_options: *mut c_void,
    pub image_base: *mut c_void,
    pub image_size: u64,
    pub image_code_type: MemoryType,
    pub image_data_type: MemoryType,
    pub unload: extern "efiapi" fn(image_handle: Handle) -> Status,
}

impl Protocol {
    pub const GUID: Guid = Guid {
        data_1: 0x5B1B31A1,
        data_2: 0x9562,
        data_3: 0x11d2,
        data_4: [0x8e, 0x3F, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
    };

    pub const REVISION: u32 = 0x1000;
}
