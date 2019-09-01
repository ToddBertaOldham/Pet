//**************************************************************************************************
// system.rs                                                                                       *
// Copyright (c) 2018-2019 Todd Berta-Oldham                                                       *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use super::boot;
use super::configuration;
use super::primitives::{Handle, TableHeader};
use super::runtime;
use super::simple_text_input;
use super::simple_text_output;

#[repr(C)]
pub struct Table {
    pub hdr: TableHeader,
    pub firmware_vendor: *mut u16,
    pub firmware_revision: u32,
    pub console_in_handle: Handle,
    pub con_in: *mut simple_text_input::Protocol,
    pub console_out_handle: Handle,
    pub con_out: *mut simple_text_output::Protocol,
    pub standard_error_handle: Handle,
    pub std_error: *mut simple_text_output::Protocol,
    pub runtime_services: *mut runtime::Services,
    pub boot_services: *mut boot::Services,
    pub number_of_table_entries: usize,
    pub configuration_table: *mut configuration::Table,
}

impl Table {
    pub const SIGNATURE: u64 = 0x5453595320494249;
    pub const LATEST_REVISION: u32 = Self::REVISION_2_70;
    pub const REVISION_2_70: u32 = ((2 << 16) | (70));
    pub const REVISION_2_60: u32 = ((2 << 16) | (60));
    pub const REVISION_2_50: u32 = ((2 << 16) | (50));
    pub const REVISION_2_40: u32 = ((2 << 16) | (40));
    pub const REVISION_2_31: u32 = ((2 << 16) | (31));
    pub const REVISION_2_30: u32 = ((2 << 16) | (30));
    pub const REVISION_2_20: u32 = ((2 << 16) | (20));
    pub const REVISION_2_10: u32 = ((2 << 16) | (10));
    pub const REVISION_2_00: u32 = ((2 << 16) | (00));
    pub const REVISION_1_10: u32 = ((1 << 16) | (10));
    pub const REVISION_1_02: u32 = ((1 << 16) | (02));
}
