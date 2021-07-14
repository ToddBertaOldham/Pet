//**************************************************************************************************
// gas.rs                                                                                          *
// Copyright (c) 2021 The Verdure Project                                                          *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use enums::c_enum;
use memory::Address64;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Gas {
    pub address_space_id: AddressSpaceId,
    pub register_bit_width: u8,
    pub register_bit_offset: u8,
    pub access_size: AccessSize,
    pub address: Address64,
}

c_enum!(
    pub enum AddressSpaceId : u8 {
        SYSTEM_MEMORY = 0x00,
        SYSTEM_IO = 0x01,
        PCI_CONFIGURATION = 0x02,
        EMBEDDED_CONTROLLER = 0x03,
        SMBUS = 0x04,
        SYSTEM_CMOS = 0x05,
        PCI_BAR_TARGET = 0x06,
        IPMI = 0x07,
        GENERAL_PURPOSE_IO = 0x08,
        GENERIC_SERIAL_BUS = 0x09,
        PCC = 0x0A,
        FUNCTION_FIXED_HARDWARE = 0x7F,
    }
);

c_enum!(
    pub enum AccessSize : u8 {
        UNDEFINED = 0,
        BYTE_ACCESS = 1,
        WORD_ACCESS = 2,
        DWORD_ACCESS = 3,
        QWORD_ACCESS = 4,
    }
);
