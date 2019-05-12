// *************************************************************************
// configuration.rs
// Copyright 2018-2019 Todd Berta-Oldham
// This code is made available under the MIT License.
// *************************************************************************

use super::primitives::Guid;
use core::ffi::c_void;

#[repr(C)]
pub struct Table {
    pub vendor_guid : Guid,
    pub vendor_table : *mut c_void
}

impl Table {
    pub const ACPI_20_GUID : Guid = Guid { data_1 : 0x8868e871, data_2 : 0xe4f1, data_3 : 0x11d3, data_4 : [ 0xbc, 0x22, 0x00, 0x80, 0xc7, 0x3c, 0x88, 0x81 ] };
    pub const ACPI_10_GUID : Guid = Guid { data_1 : 0xeb9d2d30, data_2 : 0x2d88, data_3 : 0x11d3, data_4 : [ 0x9a, 0x16, 0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d ] };
    pub const SAL_GUID : Guid = Guid { data_1 : 0xeb9d2d32, data_2 : 0x2d88, data_3 : 0x11d3, data_4 : [ 0x9a, 0x16, 0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d ] };
    pub const SMBIOS_GUID : Guid = Guid { data_1 : 0xeb9d2d31, data_2 : 0x2d88, data_3 : 0x11d3, data_4 : [ 0x9a, 0x16, 0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d ] };
    pub const SMBIOS3_GUID : Guid = Guid { data_1 : 0xf2fd1544, data_2 : 0x9794, data_3 : 0x4a2c, data_4 : [ 0x99, 0x2e, 0xe5, 0xbb, 0xcf, 0x20, 0xe3, 0x94 ] };
    pub const MPS_GUID : Guid = Guid { data_1 : 0xeb9d2d2f, data_2 : 0x2d88, data_3 : 0x11d3, data_4 : [ 0x9a, 0x16, 0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d ] };
}