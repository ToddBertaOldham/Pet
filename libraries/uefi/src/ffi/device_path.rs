//**************************************************************************************************
// device_path.rs                                                                                  *
// Copyright (c) 2019-2021 The Verdure Project                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::Guid;

//TODO Better handling of subtypes and additional fields.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Protocol {
    pub node_type: NodeType,
    pub node_sub_type: u8,
    pub length: [u8; 2],
}

impl Protocol {
    pub const GUID: Guid = Guid {
        data_1: 0x09576e91,
        data_2: 0x6d3f,
        data_3: 0x11d2,
        data_4: [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
    };

    pub const LOADED_IMAGE_GUID: Guid = Guid {
        data_1: 0xbc62157e,
        data_2: 0x3e33,
        data_3: 0x4fec,
        data_4: [0x99, 0x20, 0x2d, 0x3b, 0x36, 0xd7, 0x50, 0xdf],
    };
}

c_enum!(
    pub enum NodeType : u8 {
        HARDWARE = 0x01,
        ACPI = 0x02,
        MESSAGING = 0x03,
        MEDIA = 0x04,
        BIOS_BOOT_SPECIFICATION = 0x05,
        END_OF_HARDWARE = 6,
    }
);
