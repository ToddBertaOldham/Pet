//**************************************************************************************************
// runtime.rs                                                                                      *
// Copyright (c) 2018-2020 Aurora Berta-Oldham                                                     *
// This code is made available under the MIT License.                                              *
//**************************************************************************************************

use super::primitives::{Guid, PhysicalAddress, Status, TableHeader, Time};
use super::system;
use core::ffi::c_void;

#[repr(C)]
pub struct Services {
    pub hdr: TableHeader,
    pub get_time:
        extern "efiapi" fn(time: *mut Time, capabilities: *mut TimeCapabilities) -> Status,
    pub set_time: extern "efiapi" fn(time: *mut Time) -> Status,
    pub get_wakeup_time:
        extern "efiapi" fn(enabled: *mut bool, pending: *mut bool, time: *mut Time) -> Status,
    pub set_wakeup_time: extern "efiapi" fn(enabled: bool, time: *mut Time) -> Status,
    pub convert_pointer:
        extern "efiapi" fn(debug_disposition: usize, address: *mut *mut c_void) -> Status,
    pub reset_system: extern "efiapi" fn(
        reset_type: ResetType,
        status: Status,
        data_size: usize,
        reset_data: *mut c_void,
    ),
    pub get_next_high_monotonic_count: extern "efiapi" fn(high_count: *mut u32) -> Status,
    pub update_capsule: extern "efiapi" fn(
        capsule_header_array: *mut *mut CapsuleHeader,
        capsule_count: usize,
        scatter_gather_list: PhysicalAddress,
    ) -> Status,
    pub query_capsule_capabilities: extern "efiapi" fn(
        capsule_header_array: *mut *mut CapsuleHeader,
        capsule_count: usize,
        maximum_capsule_size: *mut u64,
        reset_type: *mut ResetType,
    ) -> Status,
    pub query_variable_info: extern "efiapi" fn(
        attributes: u32,
        maximum_variable_storage_size: *mut u64,
        remaining_variable_storage_size: *mut u64,
        maximum_variable_size: *mut u64,
    ) -> Status,
}

impl Services {
    pub const SIGNATURE: u64 = 0x56524553544e5552;
    pub const REVISION: u32 = system::Table::LATEST_REVISION;
}

#[repr(C)]
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ResetType {
    Cold,
    Warm,
    Shutdown,
    PlatformSpecific,
}

#[repr(C)]
pub struct TimeCapabilities {
    pub resolution: u32,
    pub accuracy: u32,
    pub sets_to_zero: bool,
}

#[repr(C)]
pub struct CapsuleHeader {
    pub capsule_guid: Guid,
    pub header_size: u32,
    pub flags: CapsuleFlags,
    pub capsule_image_size: u32,
}

flags!(
    pub struct CapsuleFlags : u32 {
        PERSIST_ACROSS_RESET = 0x00010000;
        POPULATE_SYSTEM_TABLE = 0x00020000;
        INITIATE_RESET = 0x00040000;
    }
);
