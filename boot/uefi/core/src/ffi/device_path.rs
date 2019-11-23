//**************************************************************************************************
// device_path.rs                                                                                  *
// Copyright (c) 2019 Todd Berta-Oldham                                                            *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use crate::Guid;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Protocol {
    pub node_type: NodeType,
}

impl Protocol {
    pub const GUID: Guid = Guid {
        data_1: 0x09576e91,
        data_2: 0x6d3f,
        data_3: 0x11d2,
        data_4: [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
    };
}

c_enum!(
    pub enum NodeType : u8 {
        HARDWARE = 0x01;
        ACPI = 0x02;
        MESSAGING = 0x03;
        MEDIA = 0x04;
        BIOS_BOOT_SPECIFICATION = 0x05;
        END_OF_HARDWARE = 6;
    }
);
